use std::{cell::RefCell, collections::HashMap, rc::Rc};

use html5ever::{Attribute, tendril::TendrilSink};
use markup5ever_rcdom::{Node, NodeData, RcDom};
use pulldown_cmark::{Event, Options as MarkdownOptions, Parser, html::push_html};

use crate::{
    Context, Element,
    element_handler::{
        HandlerResult, Handlers,
        element_util::{serialize_element, serialize_inline_subtree},
    },
    options::TranslationMode,
    util::{
        node::{get_node_tag_name, get_parent_node},
        text::{frame_as_block, indent_text_except_first_line},
    },
};

#[derive(Clone)]
enum Role {
    Reference(String),
    Definition {
        label: String,
        label_node: Option<usize>,
    },
}

/// One document's footnotes, accepted or rejected as a group.
#[derive(Clone, Default)]
pub(crate) struct FootnotePlan {
    roles: HashMap<usize, Role>,
    notes: Vec<Note>,
    faithful: bool,
}

#[derive(Clone)]
struct Note {
    id: String,
    number: String,
    reference_count: usize,
    source: Rc<Node>,
    source_label: Option<usize>,
}

#[derive(Default)]
struct TargetLinks {
    references: Vec<(usize, String)>,
    has_ordinary_link: bool,
}

#[derive(PartialEq, Eq)]
enum CanonicalNode {
    Element(String, Vec<(String, String)>, Vec<CanonicalNode>),
    Text(String),
    Comment(String),
}

fn canonical_children(
    parent: &Node,
    omit: Option<usize>,
    skip_blank_text: bool,
) -> Vec<CanonicalNode> {
    parent
        .children
        .borrow()
        .iter()
        .filter_map(|child| {
            if omit == Some(node_key(child)) {
                return None;
            }
            match &child.data {
                NodeData::Text { contents } => {
                    let value = contents.borrow();
                    if skip_blank_text && value.trim().is_empty() {
                        return None;
                    }
                    let mut normalized = String::new();
                    let mut previous_space = false;
                    for ch in value.chars() {
                        if ch.is_whitespace() {
                            if !previous_space {
                                normalized.push(' ');
                            }
                            previous_space = true;
                        } else {
                            normalized.push(ch);
                            previous_space = false;
                        }
                    }
                    Some(CanonicalNode::Text(normalized))
                }
                NodeData::Comment { contents } => {
                    Some(CanonicalNode::Comment(contents.to_string()))
                }
                NodeData::Element { name, attrs, .. } => {
                    let mut attributes: Vec<_> = attrs
                        .borrow()
                        .iter()
                        .map(|attr| (attr.name.local.to_string(), attr.value.to_string()))
                        .collect();
                    attributes.sort();
                    Some(CanonicalNode::Element(
                        name.local.to_string(),
                        attributes,
                        canonical_children(
                            child,
                            None,
                            matches!(
                                name.local.as_ref(),
                                "ul" | "ol"
                                    | "li"
                                    | "blockquote"
                                    | "div"
                                    | "table"
                                    | "tbody"
                                    | "thead"
                                    | "tr"
                            ),
                        ),
                    ))
                }
                _ => None,
            }
        })
        .collect()
}

fn body_text(parent: &Node, omit: Option<usize>) -> String {
    fn collect(parent: &Node, omit: Option<usize>, output: &mut String) {
        for child in parent.children.borrow().iter() {
            if omit == Some(node_key(child)) {
                continue;
            }
            if let NodeData::Text { contents } = &child.data {
                output.push_str(&contents.borrow());
            } else {
                collect(child, None, output);
            }
        }
    }
    let mut content = String::new();
    collect(parent, omit, &mut content);
    content.split_whitespace().collect::<Vec<_>>().join(" ")
}

thread_local! {
    static PLANS: RefCell<Vec<FootnotePlan>> = const { RefCell::new(Vec::new()) };
}

fn node_key(node: &Rc<Node>) -> usize {
    Rc::as_ptr(node) as usize
}

fn attributes(node: &Node) -> Option<std::cell::Ref<'_, Vec<Attribute>>> {
    match &node.data {
        NodeData::Element { attrs, .. } => Some(attrs.borrow()),
        _ => None,
    }
}

fn attribute<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a str> {
    attrs
        .iter()
        .find(|attr| attr.name.local.as_ref() == name)
        .map(|attr| attr.value.as_ref())
}

fn class_token(attrs: &[Attribute], token: &str) -> bool {
    attribute(attrs, "class")
        .is_some_and(|class| class.split_ascii_whitespace().any(|part| part == token))
}

fn exact_attrs(attrs: &[Attribute], expected: &[&str]) -> bool {
    attrs.len() == expected.len()
        && expected
            .iter()
            .all(|name| attrs.iter().any(|attr| attr.name.local.as_ref() == *name))
}

fn text(node: &Node) -> Option<String> {
    let children = node.children.borrow();
    if children.len() != 1 {
        return None;
    }
    match &children[0].data {
        NodeData::Text { contents } => Some(contents.borrow().to_string()),
        _ => None,
    }
}

fn decimal(node: &Node) -> Option<String> {
    text(node).filter(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()))
}

fn descendants(node: &Rc<Node>, nodes: &mut Vec<Rc<Node>>) {
    nodes.push(node.clone());
    for child in node.children.borrow().iter() {
        descendants(child, nodes);
    }
}

fn decode_fragment(fragment: &str) -> Option<String> {
    let bytes = fragment.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let high = bytes.get(index + 1)?.to_ascii_uppercase();
            let low = bytes.get(index + 2)?.to_ascii_uppercase();
            let hex = |byte: u8| match byte {
                b'0'..=b'9' => Some(byte - b'0'),
                b'A'..=b'F' => Some(byte - b'A' + 10),
                _ => None,
            };
            decoded.push(hex(high)? * 16 + hex(low)?);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(decoded).ok()
}

fn target_id<'a>(href: &str, ids: &'a HashMap<String, usize>) -> Option<&'a str> {
    let fragment = href
        .strip_prefix('#')
        .filter(|fragment| !fragment.is_empty())?;
    if let Some((id, count)) = ids.get_key_value(fragment) {
        return (*count == 1).then_some(id.as_str());
    }
    let decoded = decode_fragment(fragment)?;
    ids.get_key_value(&decoded)
        .and_then(|(id, count)| (*count == 1).then_some(id.as_str()))
}

fn only_element_child(parent: &Rc<Node>, allow_blank_text: bool) -> Option<Rc<Node>> {
    let mut element = None;
    for child in parent.children.borrow().iter() {
        match &child.data {
            NodeData::Element { .. } if element.is_none() => element = Some(child.clone()),
            NodeData::Text { contents }
                if allow_blank_text && contents.borrow().trim().is_empty() => {}
            _ => return None,
        }
    }
    element
}

fn reference_anchor(sup: &Rc<Node>, pure: bool) -> Option<Rc<Node>> {
    let mut current = sup.clone();
    loop {
        let child = only_element_child(&current, pure)?;
        if get_node_tag_name(&child) == Some("a") {
            return Some(child);
        }
        if !pure {
            return None;
        }
        current = child;
    }
}

fn definition_label(definition: &Rc<Node>, pure: bool) -> Option<(Rc<Node>, usize)> {
    let first = (*definition
        .children
        .borrow()
        .iter()
        .find(|child| match &child.data {
            NodeData::Text { contents } => !contents.borrow().trim().is_empty(),
            _ => true,
        })?)
    .clone();
    let omit = node_key(&first);
    let mut current = first;
    loop {
        if get_node_tag_name(&current) == Some("sup")
            && attributes(&current)
                .is_some_and(|attrs| class_token(&attrs, "footnote-definition-label"))
        {
            return Some((current, omit));
        }
        if !pure || get_node_tag_name(&current).is_none() {
            return None;
        }
        current = only_element_child(&current, true)?;
    }
}

fn classify_reference(
    anchor: &Rc<Node>,
    href: &str,
    id: &str,
    pure: bool,
) -> Result<Option<(usize, String)>, ()> {
    let mut parent = get_parent_node(anchor);
    if pure {
        while parent
            .as_ref()
            .is_some_and(|node| get_node_tag_name(node) != Some("sup"))
        {
            parent = parent.as_ref().and_then(get_parent_node);
        }
    }
    let Some(sup) = parent.filter(|node| get_node_tag_name(node) == Some("sup")) else {
        return Ok(None);
    };
    let sup_attrs = attributes(&sup).ok_or(())?;
    if !class_token(&sup_attrs, "footnote-reference") {
        return Ok(None);
    }
    let contained_anchor = reference_anchor(&sup, pure).ok_or(())?;
    if !Rc::ptr_eq(&contained_anchor, anchor) {
        return Err(());
    }
    let number = decimal(anchor).ok_or(())?;
    if !pure {
        let attrs = attributes(anchor).ok_or(())?;
        if !exact_attrs(&attrs, &["href"])
            || !exact_attrs(&sup_attrs, &["class"])
            || attribute(&sup_attrs, "class") != Some("footnote-reference")
            || href != format!("#{id}")
        {
            return Err(());
        }
    }
    Ok(Some((node_key(&sup), number)))
}

impl FootnotePlan {
    /// Identify pulldown-cmark footnotes before walking the document.
    pub(crate) fn build(tree: &Rc<Node>, mode: TranslationMode) -> Option<Self> {
        let mut nodes = Vec::new();
        descendants(tree, &mut nodes);
        if nodes.iter().any(|node| {
            get_node_tag_name(node) == Some("base")
                && attributes(node).is_some_and(|attrs| attribute(&attrs, "href").is_some())
        }) {
            return None;
        }

        let mut ids = HashMap::<String, usize>::new();
        let mut definitions = HashMap::<String, Rc<Node>>::new();
        for node in &nodes {
            let Some(attrs) = attributes(node) else {
                continue;
            };
            if let Some(id) = attribute(&attrs, "id") {
                *ids.entry(id.to_owned()).or_default() += 1;
                if get_node_tag_name(node) == Some("div")
                    && class_token(&attrs, "footnote-definition")
                {
                    definitions.insert(id.to_owned(), node.clone());
                }
            }
        }

        let pure = mode == TranslationMode::Pure;
        let mut links = HashMap::<String, TargetLinks>::new();
        for node in &nodes {
            if get_node_tag_name(node) != Some("a") {
                continue;
            }
            let Some(attrs) = attributes(node) else {
                continue;
            };
            let Some(href) = attribute(&attrs, "href") else {
                continue;
            };
            let Some(id) = target_id(href, &ids) else {
                continue;
            };
            if !definitions.contains_key(id) {
                continue;
            }
            let links_to_id = links.entry(id.to_owned()).or_default();
            match classify_reference(node, href, id, pure) {
                Ok(Some(reference)) => links_to_id.references.push(reference),
                Ok(None) => links_to_id.has_ordinary_link = true,
                Err(()) => return None,
            }
        }
        if !links.values().any(|target| !target.references.is_empty()) {
            return None;
        }

        let mut plan = Self {
            faithful: !pure,
            ..Self::default()
        };
        let mut used_labels = HashMap::<String, ()>::new();
        for node in &nodes {
            if get_node_tag_name(node) != Some("div") {
                continue;
            }
            let Some(attrs) = attributes(node) else {
                continue;
            };
            let Some(id) = attribute(&attrs, "id") else {
                continue;
            };
            let Some(target) = links
                .remove(id)
                .filter(|target| !target.references.is_empty())
            else {
                continue;
            };
            if target.has_ordinary_link {
                return None;
            }
            let references = target.references;
            if ids.get(id) != Some(&1) || !class_token(&attrs, "footnote-definition") {
                return None;
            }
            if !pure
                && (!exact_attrs(&attrs, &["class", "id"])
                    || attribute(&attrs, "class") != Some("footnote-definition"))
            {
                return None;
            }
            let number = references[0].1.clone();
            if references.iter().any(|(_, display)| display != &number) {
                return None;
            }
            let label = definition_label(node, pure);
            if !pure && label.is_none() {
                return None;
            }
            if let Some((label_node, _)) = &label {
                let label_attrs = attributes(label_node)?;
                if !pure
                    && (!exact_attrs(&label_attrs, &["class"])
                        || attribute(&label_attrs, "class") != Some("footnote-definition-label"))
                {
                    return None;
                }
                if decimal(label_node).as_deref() != Some(number.as_str()) {
                    return None;
                }
            }
            if nodes.iter().any(|other| {
                !Rc::ptr_eq(other, node)
                    && get_node_tag_name(other) == Some("div")
                    && attributes(other)
                        .is_some_and(|attrs| class_token(&attrs, "footnote-definition"))
                    && is_descendant_of(other, node)
            }) {
                return None;
            }
            let markdown_label = if pure {
                number.to_owned()
            } else {
                id.to_owned()
            };
            if used_labels.insert(markdown_label.clone(), ()).is_some() {
                return None;
            }
            let reference_count = references.len();
            plan.roles.insert(
                node_key(node),
                Role::Definition {
                    label: markdown_label.clone(),
                    label_node: label.as_ref().map(|(_, omit)| *omit),
                },
            );
            for (sup_key, _) in references {
                plan.roles
                    .insert(sup_key, Role::Reference(markdown_label.clone()));
            }
            plan.notes.push(Note {
                id: if pure { markdown_label } else { id.to_owned() },
                number,
                reference_count,
                source: node.clone(),
                source_label: label.as_ref().map(|(_, omit)| *omit),
            });
        }
        (!plan.notes.is_empty()).then_some(plan)
    }

    /// Check the rendered labels, references, and note bodies before committing the plan.
    pub(crate) fn validate(&self, markdown: &str) -> bool {
        let mut html = String::new();
        let mut parsed_references = HashMap::<String, usize>::new();
        push_html(
            &mut html,
            Parser::new_ext(markdown, MarkdownOptions::ENABLE_FOOTNOTES).inspect(|event| {
                if let Event::FootnoteReference(label) = event {
                    *parsed_references.entry(label.to_string()).or_default() += 1;
                }
            }),
        );
        if self.notes.iter().any(|note| {
            parsed_references.get(&note.id) != Some(&note.reference_count)
        }) {
            return false;
        }
        let tree =
            html5ever::parse_document(RcDom::default(), Default::default()).one(html.as_str());
        let mut nodes = Vec::new();
        descendants(&tree.document, &mut nodes);
        for note in &self.notes {
            if nodes
                .iter()
                .filter(|node| {
                    attributes(node)
                        .is_some_and(|attrs| attribute(&attrs, "id") == Some(note.id.as_str()))
                })
                .count()
                != 1
            {
                return false;
            }
            let definitions: Vec<_> = nodes
                .iter()
                .filter(|node| {
                    get_node_tag_name(node) == Some("div")
                        && attributes(node).is_some_and(|attrs| {
                            attribute(&attrs, "id") == Some(note.id.as_str())
                                && class_token(&attrs, "footnote-definition")
                        })
                })
                .collect();
            if definitions.len() != 1 {
                return false;
            }
            let definition = definitions[0];
            let label = definition
                .children
                .borrow()
                .iter()
                .find(|child| {
                    get_node_tag_name(child) == Some("sup")
                        && attributes(child)
                            .is_some_and(|attrs| class_token(&attrs, "footnote-definition-label"))
                })
                .cloned();
            if label.as_ref().and_then(|node| decimal(node)).as_deref()
                != Some(note.number.as_str())
            {
                return false;
            }
            let references: Vec<_> = nodes
                .iter()
                .filter(|node| {
                    get_node_tag_name(node) == Some("sup")
                        && attributes(node)
                            .is_some_and(|attrs| class_token(&attrs, "footnote-reference"))
                        && node.children.borrow().iter().any(|child| {
                            get_node_tag_name(child) == Some("a")
                                && attributes(child).is_some_and(|attrs| {
                                    attribute(&attrs, "href")
                                        == Some(format!("#{}", note.id).as_str())
                                })
                        })
                })
                .collect();
            if references.len() != note.reference_count
                || references.iter().any(|node| {
                    node.children
                        .borrow()
                        .iter()
                        .filter(|child| get_node_tag_name(child) == Some("a"))
                        .any(|child| decimal(child).as_deref() != Some(note.number.as_str()))
                })
            {
                return false;
            }
            if self.faithful
                && canonical_children(&note.source, note.source_label, true)
                    != canonical_children(definition, label.as_ref().map(node_key), true)
            {
                return false;
            }
            if !self.faithful
                && body_text(&note.source, note.source_label)
                    != body_text(definition, label.as_ref().map(node_key))
            {
                return false;
            }
        }
        true
    }
}

fn is_descendant_of(node: &Rc<Node>, ancestor: &Rc<Node>) -> bool {
    let mut parent = get_parent_node(node);
    while let Some(current) = parent {
        if Rc::ptr_eq(&current, ancestor) {
            return true;
        }
        parent = get_parent_node(&current);
    }
    false
}

struct BorrowedChildrenGuard(Rc<Node>);

impl Drop for BorrowedChildrenGuard {
    fn drop(&mut self) {
        // rcdom's Node::drop drains descendants even when another tree owns them.
        self.0.children.borrow_mut().clear();
    }
}

/// Make a conversion plan available to handlers during one document walk.
pub(crate) fn with_plan<R>(plan: FootnotePlan, convert: impl FnOnce() -> R) -> R {
    struct PlanGuard;
    impl Drop for PlanGuard {
        fn drop(&mut self) {
            PLANS.with(|plans| {
                plans.borrow_mut().pop();
            });
        }
    }
    PLANS.with(|plans| plans.borrow_mut().push(plan));
    let _guard = PlanGuard;
    convert()
}

pub(super) fn footnote_handler(
    handlers: &dyn Handlers,
    element: &Element,
) -> Option<HandlerResult> {
    let role = PLANS.with(|plans| {
        plans
            .borrow()
            .last()
            .and_then(|plan| plan.roles.get(&node_key(element.node)).cloned())
    });
    match role {
        Some(Role::Reference(label)) => Some(format!("[^{label}]").into()),
        Some(Role::Definition { label, label_node }) if element.context.is_block() => {
            let body = Node::new(NodeData::Document);
            let _guard = BorrowedChildrenGuard(body.clone());
            body.children.borrow_mut().extend(
                element
                    .node
                    .children
                    .borrow()
                    .iter()
                    .filter(|child| label_node != Some(node_key(child)))
                    .cloned(),
            );
            let content = handlers.walk_children_content(&body, Context::BLOCK);
            let content = content.trim_matches('\n').trim();
            let first_is_paragraph = body
                .children
                .borrow()
                .iter()
                .find(|child| get_node_tag_name(child).is_some())
                .is_some_and(|child| get_node_tag_name(child) == Some("p"));
            let mut definition = format!("[^{label}]:");
            if !content.is_empty() {
                let indented = indent_text_except_first_line(content, 4, false);
                if first_is_paragraph {
                    definition.push(' ');
                } else {
                    definition.push_str("\n    ");
                }
                definition.push_str(&indented);
            }
            Some(frame_as_block(&definition).into())
        }
        _ if element.tag == "sup"
            && handlers.options().translation_mode == TranslationMode::Faithful
            && class_token(element.attrs, "footnote-reference") =>
        {
            Some(HandlerResult::html(
                serialize_inline_subtree(handlers, element)
                    .unwrap_or_else(|_| serialize_element(handlers, element)),
            ))
        }
        _ => handlers.fallback(element),
    }
}
