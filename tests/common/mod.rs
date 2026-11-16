// Shared code for all integration tests.
use htmd::{
    HtmlToMarkdown,
    options::{
        BrStyle, BulletListMarker, CodeBlockFence, CodeBlockStyle, HeadingStyle, HrStyle,
        LinkReferenceStyle, LinkStyle, Options, TranslationMode,
    },
};
use html5ever::tendril::TendrilSink;
use html5ever::{QualName, local_name, ns, parse_fragment};
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use pulldown_cmark::{Options as CommonMarkOptions, Parser};

// By default, use the faithful translation mode, which is more stringent.
pub fn convert_faithful(html: &str) -> std::io::Result<String> {
    convert_faithful_options(html, faithful_baseline_options())
}

fn faithful_baseline_options() -> Options {
    Options {
        heading_style: HeadingStyle::Atx,
        hr_style: HrStyle::Asterisks,
        br_style: BrStyle::TwoSpaces,
        link_style: LinkStyle::Inlined,
        link_reference_style: LinkReferenceStyle::Full,
        code_block_style: CodeBlockStyle::Fenced,
        code_block_fence: CodeBlockFence::Backticks,
        bullet_list_marker: BulletListMarker::Asterisk,
        ul_bullet_spacing: 3,
        ol_number_spacing: 2,
        preformatted_code: false,
        translation_mode: TranslationMode::Faithful,
    }
}

// `convert_faithful` with the given options. The translation mode is
// overridden, so callers vary only the options they care about.
pub fn convert_faithful_options(html: &str, options: Options) -> std::io::Result<String> {
    HtmlToMarkdown::builder()
        .options(Options {
            translation_mode: TranslationMode::Faithful,
            ..options
        })
        .build()
        .convert(html)
}

// `convert_faithful` with setext headings, the style whose h1 and h2 leave
// their content at the start of a line.
#[allow(dead_code)]
pub fn convert_faithful_setext(html: &str) -> std::io::Result<String> {
    convert_faithful_options(
        html,
        Options {
            heading_style: HeadingStyle::Setex,
            ..faithful_baseline_options()
        },
    )
}

// The default translation mode, which drops what CommonMark cannot spell.
#[allow(dead_code)]
pub fn convert_pure(html: &str) -> std::io::Result<String> {
    HtmlToMarkdown::builder().build().convert(html)
}

// Reference helper for a round trip with default options. The case runner
// keeps the intermediate Markdown so it can check second-trip stability.
#[allow(dead_code)]
pub fn round_trip(html: &str) -> String {
    render_markdown(&convert_faithful(html).unwrap(), CommonMarkOptions::all())
}

// The second half of a round trip: the CommonMark of the first half read back
// as HTML.
#[allow(dead_code)]
pub fn render_markdown(markdown: &str, options: CommonMarkOptions) -> String {
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, Parser::new_ext(markdown, options));
    html_output
}

#[derive(Debug, PartialEq, Eq)]
enum ComparedNode {
    Document(Vec<ComparedNode>),
    Element {
        name: (String, String),
        attributes: Vec<(String, String, String)>,
        children: Vec<ComparedNode>,
        template_contents: Option<Vec<ComparedNode>>,
    },
    Text(String),
    Comment(String),
    Doctype(String, String, String),
    ProcessingInstruction(String, String),
}

fn is_html_whitespace(character: char) -> bool {
    matches!(character, '\t' | '\n' | '\u{000c}' | '\r' | ' ')
}

fn push_collapsed_text(output: &mut String, input: &str) {
    for character in input.chars() {
        if is_html_whitespace(character) {
            if !output.ends_with(' ') {
                output.push(' ');
            }
        } else {
            output.push(character);
        }
    }
}

// Match the block set in `src/util/node.rs`, which follows the HTML block
// distinction used by `translation_approach.md`.
fn is_block_element(tag: &str) -> bool {
    matches!(
        tag,
        "address"
            | "article"
            | "aside"
            | "base"
            | "basefont"
            | "blockquote"
            | "body"
            | "caption"
            | "center"
            | "col"
            | "colgroup"
            | "dd"
            | "details"
            | "dialog"
            | "dir"
            | "div"
            | "dl"
            | "dt"
            | "fieldset"
            | "figcaption"
            | "figure"
            | "footer"
            | "form"
            | "frame"
            | "frameset"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "head"
            | "header"
            | "hr"
            | "html"
            | "iframe"
            | "legend"
            | "li"
            | "link"
            | "main"
            | "menu"
            | "menuitem"
            | "nav"
            | "noframes"
            | "ol"
            | "optgroup"
            | "option"
            | "p"
            | "param"
            | "pre"
            | "script"
            | "search"
            | "section"
            | "style"
            | "summary"
            | "table"
            | "tbody"
            | "td"
            | "textarea"
            | "tfoot"
            | "th"
            | "thead"
            | "title"
            | "tr"
            | "track"
            | "ul"
    )
}

fn finish_flow(flows: &mut Vec<String>, current: &mut String) {
    let text = current.trim_matches(' ');
    if !text.is_empty() {
        flows.push(text.to_owned());
    }
    current.clear();
}

fn collect_flow(node: &Handle, flows: &mut Vec<String>, current: &mut String) {
    match &node.data {
        NodeData::Text { contents } => push_collapsed_text(current, &contents.borrow()),
        NodeData::Element {
            name,
            template_contents,
            ..
        } => {
            let block = name.ns == ns!(html) && is_block_element(name.local.as_ref());
            if block {
                finish_flow(flows, current);
            }
            for child in node.children.borrow().iter() {
                collect_flow(child, flows, current);
            }
            if let Some(contents) = template_contents.borrow().as_ref() {
                // Inert template contents have their own flow, separate from surrounding text.
                let mut template_flow = String::new();
                collect_flow(contents, flows, &mut template_flow);
                finish_flow(flows, &mut template_flow);
            }
            if block {
                finish_flow(flows, current);
            }
        }
        _ => {
            for child in node.children.borrow().iter() {
                collect_flow(child, flows, current);
            }
        }
    }
}

fn compared_children(node: &Handle, exact_text: bool) -> Vec<ComparedNode> {
    node.children
        .borrow()
        .iter()
        .filter_map(|child| compared_node(child, exact_text))
        .collect()
}

fn compared_node(node: &Handle, exact_text: bool) -> Option<ComparedNode> {
    match &node.data {
        NodeData::Document => Some(ComparedNode::Document(compared_children(node, exact_text))),
        NodeData::Element {
            name,
            attrs,
            template_contents,
            ..
        } => {
            let exact_text = exact_text
                || (name.ns == ns!(html) && matches!(name.local.as_ref(), "code" | "pre"));
            let mut attributes: Vec<_> = attrs
                .borrow()
                .iter()
                .map(|attribute| {
                    (
                        attribute.name.ns.to_string(),
                        attribute.name.local.to_string(),
                        attribute.value.to_string(),
                    )
                })
                .collect();
            attributes.sort();
            Some(ComparedNode::Element {
                name: (name.ns.to_string(), name.local.to_string()),
                attributes,
                children: compared_children(node, exact_text),
                template_contents: template_contents
                    .borrow()
                    .as_ref()
                    .map(|contents| compared_children(contents, exact_text)),
            })
        }
        NodeData::Text { contents } => {
            let contents = contents.borrow();
            if exact_text {
                Some(ComparedNode::Text(contents.to_string()))
            } else {
                let mut normalized = String::new();
                push_collapsed_text(&mut normalized, &contents);
                let normalized = normalized.trim_matches(' ');
                (!normalized.is_empty()).then(|| ComparedNode::Text(normalized.to_owned()))
            }
        }
        NodeData::Comment { contents } => Some(ComparedNode::Comment(contents.to_string())),
        NodeData::Doctype {
            name,
            public_id,
            system_id,
        } => Some(ComparedNode::Doctype(
            name.to_string(),
            public_id.to_string(),
            system_id.to_string(),
        )),
        NodeData::ProcessingInstruction { target, contents } => Some(
            ComparedNode::ProcessingInstruction(target.to_string(), contents.to_string()),
        ),
    }
}

fn parse_for_comparison(html: &str) -> RcDom {
    parse_fragment(
        RcDom::default(),
        Default::default(),
        QualName::new(None, ns!(html), local_name!("div")),
        Vec::new(),
        false,
    )
    .one(html)
}

fn first_child_difference(
    expected: &[ComparedNode],
    actual: &[ComparedNode],
    path: &str,
) -> Option<String> {
    for (index, (expected_child, actual_child)) in expected.iter().zip(actual).enumerate() {
        if let Some(difference) =
            first_structure_difference(expected_child, actual_child, &format!("{path}/{index}"))
        {
            return Some(difference);
        }
    }
    (expected.len() != actual.len()).then(|| {
        format!(
            "{path}: expected {} child nodes, actual {}",
            expected.len(),
            actual.len()
        )
    })
}

fn first_structure_difference(
    expected: &ComparedNode,
    actual: &ComparedNode,
    path: &str,
) -> Option<String> {
    match (expected, actual) {
        (ComparedNode::Document(expected), ComparedNode::Document(actual)) => {
            first_child_difference(expected, actual, path)
        }
        (
            ComparedNode::Element {
                name: expected_name,
                attributes: expected_attributes,
                children: expected_children,
                template_contents: expected_template,
            },
            ComparedNode::Element {
                name: actual_name,
                attributes: actual_attributes,
                children: actual_children,
                template_contents: actual_template,
            },
        ) => {
            if expected_name != actual_name {
                return Some(format!(
                    "{path}: expected element {expected_name:?}, actual {actual_name:?}"
                ));
            }
            if expected_attributes != actual_attributes {
                return Some(format!(
                    "{path}: expected attributes {expected_attributes:?}, actual {actual_attributes:?}"
                ));
            }
            if let Some(difference) =
                first_child_difference(expected_children, actual_children, path)
            {
                return Some(difference);
            }
            match (expected_template, actual_template) {
                (Some(expected), Some(actual)) => {
                    first_child_difference(expected, actual, &format!("{path}/template"))
                }
                (None, None) => None,
                _ => Some(format!("{path}: template content fragments differ")),
            }
        }
        _ if expected != actual => {
            Some(format!("{path}: expected {expected:?}, actual {actual:?}"))
        }
        _ => None,
    }
}

/// Compare parsed HTML structure and text flow using the rules in `spec.md#3`.
#[allow(dead_code)]
pub fn compare_html(expected: &str, actual: &str) -> Result<(), String> {
    let expected = parse_for_comparison(expected);
    let actual = parse_for_comparison(actual);
    let expected_tree = compared_node(&expected.document, false).expect("document node");
    let actual_tree = compared_node(&actual.document, false).expect("document node");
    if let Some(difference) = first_structure_difference(&expected_tree, &actual_tree, "fragment") {
        return Err(format!("parsed structure or text differs at {difference}"));
    }
    let flow = |document: &Handle| {
        let mut flows = Vec::new();
        let mut current = String::new();
        collect_flow(document, &mut flows, &mut current);
        finish_flow(&mut flows, &mut current);
        flows
    };
    let expected_flow = flow(&expected.document);
    let actual_flow = flow(&actual.document);
    if expected_flow != actual_flow {
        return Err(format!(
            "word separation differs\nexpected text flow: {expected_flow:?}\nactual text flow: {actual_flow:?}"
        ));
    }
    Ok(())
}

/// The HTML preservation claim or approved exact result of a Faithful round trip.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum HtmlExpectation<'a> {
    Preserved,
    DocumentedDifference {
        kind: DifferenceKind,
        expected_html: &'a str,
        reason: &'a str,
        reference: &'a str,
    },
}

/// Whether a documented HTML difference changes the parsed document.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum DifferenceKind {
    Representation,
    Loss,
}

/// The expected result of converting rendered HTML back to Markdown.
#[derive(Debug, Clone, Copy, Default)]
#[allow(dead_code)]
pub enum StabilityExpectation<'a> {
    #[default]
    Stable,
    DocumentedInstability {
        second_markdown: &'a str,
        reason: &'a str,
        reference: &'a str,
    },
}

/// One Faithful input, its conversion options, and its independent assertions.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Case<'a> {
    pub id: &'a str,
    pub html: &'a str,
    pub htmd_options: Options,
    pub markdown_options: CommonMarkOptions,
    pub expectation: Option<HtmlExpectation<'a>>,
    pub expected_markdown: Option<&'a str>,
    pub stability: StabilityExpectation<'a>,
}

impl Default for Case<'_> {
    fn default() -> Self {
        Self {
            id: "",
            html: "",
            htmd_options: faithful_baseline_options(),
            markdown_options: CommonMarkOptions::all(),
            expectation: None,
            expected_markdown: None,
            stability: StabilityExpectation::Stable,
        }
    }
}

/// Run every case and report all failures together, with whitespace visible.
#[allow(dead_code)]
pub fn check_cases(cases: &[Case<'_>]) {
    let mut failures = Vec::new();
    let test_thread = std::thread::current();
    let test_name = test_thread.name().unwrap_or("unnamed test");
    for (index, case) in cases.iter().enumerate() {
        let label = if case.id.is_empty() {
            String::new()
        } else {
            format!(", {}", case.id)
        };
        let context = format!(
            "test: {test_name} (row {}{label})\nhtmd options: {:?}\npulldown-cmark options: {:?}\ninput HTML: {:?}",
            index + 1,
            case.htmd_options,
            case.markdown_options,
            case.html
        );
        if case.html.is_empty() {
            failures.push(format!("{context}\ncase needs input HTML"));
            continue;
        }
        if case.expectation.is_none() && case.expected_markdown.is_none() {
            failures.push(format!("{context}\ncase needs at least one assertion"));
            continue;
        }
        if case.htmd_options.translation_mode != TranslationMode::Faithful {
            failures.push(format!("{context}\ncase must use Faithful mode"));
            continue;
        }
        let markdown = match convert_faithful_options(case.html, case.htmd_options.clone()) {
            Ok(markdown) => markdown,
            Err(error) => {
                failures.push(format!("{context}\nfirst conversion failed: {error}"));
                continue;
            }
        };
        let context = format!("{context}\nM(H): {markdown:?}");
        if let Some(expected) = case.expected_markdown
            && markdown != expected
        {
            failures.push(format!(
                "{context}\nexact Markdown mismatch\nexpected: {expected:?}\nactual: {markdown:?}"
            ));
        }
        let rendered = render_markdown(&markdown, case.markdown_options);
        if let Some(expectation) = case.expectation {
            match expectation {
                HtmlExpectation::Preserved => {
                    if let Err(difference) = compare_html(case.html, &rendered) {
                        failures.push(format!(
                            "{context}\npreservation mismatch: {difference}\nexpected HTML: {:?}\nactual HTML: {rendered:?}", case.html
                        ));
                    }
                }
                HtmlExpectation::DocumentedDifference {
                    kind,
                    expected_html,
                    reason,
                    reference,
                } => {
                    if reason.is_empty() || reference.is_empty() {
                        failures.push(format!(
                            "{context}\ndocumented {kind:?} needs a reason and reference"
                        ));
                    }
                    if expected_html == case.html {
                        failures.push(format!(
                            "{context}\ndocumented {kind:?} must differ from input HTML"
                        ));
                    }
                    let parsed_difference = compare_html(case.html, expected_html);
                    match (kind, parsed_difference) {
                        (DifferenceKind::Representation, Err(difference)) => failures.push(format!(
                            "{context}\nrepresentation changes parsed content: {difference}"
                        )),
                        (DifferenceKind::Loss, Ok(())) => failures.push(format!(
                            "{context}\nknown loss is preserved under the whitespace-aware comparison"
                        )),
                        _ => {}
                    }
                    if rendered != expected_html {
                        let label = match kind {
                            DifferenceKind::Representation => "representation difference",
                            DifferenceKind::Loss => "known loss",
                        };
                        failures.push(format!(
                            "{context}\n{label} mismatch\nexpected HTML: {expected_html:?}\nactual HTML: {rendered:?}"
                        ));
                    }
                }
            }
        }
        let second_markdown = match convert_faithful_options(&rendered, case.htmd_options.clone()) {
            Ok(markdown) => markdown,
            Err(error) => {
                failures.push(format!(
                    "{context}\nR(H): {rendered:?}\nsecond conversion failed: {error}"
                ));
                continue;
            }
        };
        let expected = match case.stability {
            StabilityExpectation::Stable => markdown.as_str(),
            StabilityExpectation::DocumentedInstability {
                second_markdown,
                reason,
                reference,
            } => {
                if reason.is_empty() || reference.is_empty() {
                    failures.push(format!(
                        "{context}\ndocumented instability needs a reason and reference"
                    ));
                }
                if second_markdown == markdown {
                    failures.push(format!(
                        "{context}\ndocumented instability must differ from first Markdown"
                    ));
                }
                second_markdown
            }
        };
        if second_markdown != expected {
            failures.push(format!(
                "{context}\nstability mismatch\nR(H): {rendered:?}\nM(R(H)): {second_markdown:?}\nexpected Markdown: {expected:?}"
            ));
        }
    }
    if !failures.is_empty() {
        panic!("{}", failures.join("\n\n"));
    }
}
