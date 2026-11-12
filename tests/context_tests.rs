//! Tests for the block/inline context an element is translated in, following
//! the "Given an HTML node that can't be encoded as CommonMark" rule of
//! `translation_approach.md`.

use pretty_assertions::assert_eq;
use pulldown_cmark::Options as CommonMarkOptions;

mod common;
use common::{
    Case, DifferenceKind, HtmlExpectation, StabilityExpectation, check_cases, convert_pure,
};

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn html_at_the_document_root_is_a_block() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "html_at_the_document_root_is_a_block_case_1",
        html: "<div>a</div><div>b</div>",
        expected_markdown: Some("<div>a</div>\n\n<div>b</div>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "html_at_the_document_root_is_a_block_case_2",
        html: "<div><br><br></div>",
        expected_markdown: Some("<div><br><br></div>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "html_at_the_document_root_is_a_block_case_3",
        html: "<h1><div><p>a</p></div></h1>",
        expected_markdown: Some("# <div><p>a</p></div>"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// `br` is not a block-level tag name, so a lone `<br>` on a line would open a
/// [type 7 HTML block][], which cannot interrupt a paragraph and swallows every
/// following line down to the next blank one. Such a tag is therefore a raw
/// HTML inline everywhere, block context included.
///
/// [type 7 HTML block]: https://spec.commonmark.org/0.31.2/#html-blocks
#[test]
fn a_type_7_tag_is_a_raw_inline_even_in_a_block_context() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_type_7_tag_is_a_raw_inline_even_in_a_block_context_case_4",
        html: "<br>",
        expected_markdown: Some("<br>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_type_7_tag_is_a_raw_inline_even_in_a_block_context_case_5",
        html: "<br><br>",
        expected_markdown: Some("<br><br>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_type_7_tag_is_a_raw_inline_even_in_a_block_context_case_6",
        html: "This is <em foo>really</em> important.",
        expected_markdown: Some(r#"This is <em foo="">really</em> important."#),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_type_7_tag_is_a_raw_inline_even_in_a_block_context_case_7",
        html: "<ul><li>a<br>b</li></ul>",
        expected_markdown: Some("*   a<br>b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_type_7_tag_is_a_raw_inline_even_in_a_block_context_case_8",
        html: "<blockquote>a<br>b</blockquote>",
        expected_markdown: Some("> a<br>b"),
        ..Case::default()
    }]);
    // A paragraph still frames itself as a block, so a `<br>` beside one does
    // not join it.

    faithful_cases.extend([Case {
        id: "a_type_7_tag_is_a_raw_inline_even_in_a_block_context_case_9",
        html: "<br><p>a</p>",
        expected_markdown: Some("<br>\n\na"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_type_7_tag_is_a_raw_inline_even_in_a_block_context_case_10",
        html: "<p>a</p><br>",
        expected_markdown: Some("a\n\n<br>"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn a_commonmark_block_in_an_inline_context_is_a_raw_inline() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_commonmark_block_in_an_inline_context_is_a_raw_inline_case_11",
        html: "<h1><p>a</p></h1>",
        expected_markdown: Some("# <p>a</p>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_commonmark_block_in_an_inline_context_is_a_raw_inline_case_12",
        html: "<h1>x<p>a</p>y</h1>",
        expected_markdown: Some("# x<p>a</p>y"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_commonmark_block_in_an_inline_context_is_a_raw_inline_case_13",
        html: "<h1><blockquote>a</blockquote></h1>",
        expected_markdown: Some("# <blockquote>a</blockquote>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_commonmark_block_in_an_inline_context_is_a_raw_inline_case_14",
        html: "<h1><ul><li>a</li></ul></h1>",
        expected_markdown: Some("# <ul><li>a</li></ul>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_commonmark_block_in_an_inline_context_is_a_raw_inline_case_15",
        html: "<h1><hr></h1>",
        expected_markdown: Some("# <hr>"),
        ..Case::default()
    }]);
    // Only the tags of a raw HTML inline are HTML; what it holds is still
    // translated, except in a code block, whose content is literal text.

    faithful_cases.extend([Case {
        id: "a_commonmark_block_in_an_inline_context_is_a_raw_inline_case_16",
        html: "<h1><pre><code>a</code></pre></h1>",
        expected_markdown: Some("# <pre><code>a</code></pre>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_commonmark_block_in_an_inline_context_is_a_raw_inline_case_17",
        html: "<h1><table><thead><tr><th>h</th></tr></thead>\
             <tbody><tr><td>a</td></tr></tbody></table></h1>",
        expected_markdown: Some(
            "# <table><thead><tr><th>h</th></tr></thead><tbody><tr><td>a</td></tr></tbody></table>",
        ),
        markdown_options: CommonMarkOptions::ENABLE_TABLES,
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// Not a faithful translation: CommonMark provides no way to write raw text
/// without opening a paragraph.
#[test]
fn raw_text_at_the_document_root_is_left_alone() {
    check_cases(&[Case {
        id: "raw_text_at_the_document_root_is_left_alone_case_18",
        html: "foo",
        expected_markdown: Some("foo"),
        ..Case::default()
    }]);
}

/// [Paragraphs](../translation_approach.md#paragraphs).
#[test]
fn html_in_a_paragraph_is_a_raw_inline() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "html_in_a_paragraph_is_a_raw_inline_case_19",
        html: "<p><br><br></p>",
        expected_markdown: Some("<br><br>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "html_in_a_paragraph_is_a_raw_inline_case_20",
        html: "<p><br><em>b</em></p>",
        expected_markdown: Some("<br>*b*"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "html_in_a_paragraph_is_a_raw_inline_case_21",
        html: "<p><em>a</em><br></p>",
        expected_markdown: Some("*a*<br>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "html_in_a_paragraph_is_a_raw_inline_case_22",
        html: r#"<p><br><img src="i" alt=""></p>"#,
        expected_markdown: Some("<br>![](i)"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Headings](../translation_approach.md#headings).
#[test]
fn html_in_a_heading_is_a_raw_inline() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "html_in_a_heading_is_a_raw_inline_case_23",
        html: "<h1><br><br></h1>",
        expected_markdown: Some("# <br><br>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "html_in_a_heading_is_a_raw_inline_case_24",
        html: "<h1><br><em>b</em></h1>",
        expected_markdown: Some("# <br>*b*"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "html_in_a_heading_is_a_raw_inline_case_25",
        html: "<h6><em>a</em><br></h6>",
        expected_markdown: Some("###### *a*<br>"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Blockquotes](../translation_approach.md#blockquotes).
#[test]
fn html_in_a_blockquote_is_a_block() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "html_in_a_blockquote_is_a_block_case_26",
        html: "<blockquote><div>a</div><div>b</div></blockquote>",
        expected_markdown: Some("> <div>a</div>\n> \n> <div>b</div>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "html_in_a_blockquote_is_a_block_case_27",
        html: "<blockquote><br><br></blockquote>",
        expected_markdown: Some("> <br><br>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "html_in_a_blockquote_is_a_block_case_28",
        html: "<blockquote><p><br><em>b</em></p></blockquote>",
        expected_markdown: Some("> <br>*b*"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn html_in_a_list_item_is_a_block() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "html_in_a_list_item_is_a_block_case_29",
        html: "<ul><li><div>a</div><div>b</div></li></ul>",
        expected_markdown: Some("*   <div>a</div>\n\n    <div>b</div>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "html_in_a_list_item_is_a_block_case_30",
        html: "<ul><li><br><p>a</p></li></ul>",
        expected_markdown: Some("*   <br>\n\n    a"),
        ..Case::default()
    }]);
    // Row 1 of "Lists" in `translation_approach.md` wants two HTML blocks here,
    // but `br` is a type 7 tag, so the run stays inline.

    faithful_cases.extend([Case {
        id: "html_in_a_list_item_is_a_block_case_31",
        html: "<ul><li><br><br></li></ul>",
        expected_markdown: Some("*   <br><br>"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Tables](../translation_approach.md#tables).
/// A cell's contents are parsed as inline content, so no HTML block can open
/// inside one.
#[test]
fn html_in_a_table_cell_is_a_raw_inline() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "html_in_a_table_cell_is_a_raw_inline_case_32",
        html: "<table><thead><tr><th>h</th></tr></thead>\
             <tbody><tr><td><br><em>b</em></td></tr></tbody></table>",
        expected_markdown: Some("| h       |\n| ------- |\n| <br>*b* |"),
        markdown_options: CommonMarkOptions::ENABLE_TABLES,
        ..Case::default()
    }]);
    // A block element in a cell is a raw HTML inline like any other.

    faithful_cases.extend([Case {
        id: "html_in_a_table_cell_is_a_raw_inline_case_33",
        html: "<table><thead><tr><th>h</th></tr></thead>\
             <tbody><tr><td><p>a</p></td></tr></tbody></table>",
        expected_markdown: Some("| h        |\n| -------- |\n| <p>a</p> |"),
        markdown_options: CommonMarkOptions::ENABLE_TABLES,
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "html_in_a_table_cell_is_a_raw_inline_case_34",
        html: "<table><thead><tr><th>h</th></tr></thead>\
             <tbody><tr><td><ul><li>a</li></ul></td></tr></tbody></table>",
        expected_markdown: Some(
            "| h                   |\n| ------------------- |\n| <ul><li>a</li></ul> |",
        ),
        markdown_options: CommonMarkOptions::ENABLE_TABLES,
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Tables](../translation_approach.md#tables).
/// What a cell cannot absorb as a raw HTML inline is an attribute on the cell
/// itself: a GFM row has nowhere to write one. The `<caption>` half of the same
/// rule is covered by
/// `basic_tests::faithful_mode_serializes_a_table_with_a_caption`. See the
/// tables section of `translation_approach.md`.
#[test]
fn a_cell_attribute_writes_the_whole_table_as_html() {
    let mut faithful_cases = Vec::new();
    let with_colspan = "<table><thead><tr><th>h</th></tr></thead>\
                        <tbody><tr><td colspan=\"2\">a</td></tr></tbody></table>";

    faithful_cases.extend([Case {
        id: "a_cell_attribute_writes_the_whole_table_as_html_case_35",
        html: with_colspan,
        expected_markdown: Some(with_colspan),
        ..Case::default()
    }]);
    let heading_colspan = "<table><thead><tr><th colspan=\"2\">h</th></tr></thead>\
                           <tbody><tr><td>a</td></tr></tbody></table>";

    faithful_cases.extend([Case {
        id: "a_cell_attribute_writes_the_whole_table_as_html_case_36",
        html: heading_colspan,
        expected_markdown: Some(heading_colspan),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn html_in_an_inline_element_is_a_raw_inline() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "html_in_an_inline_element_is_a_raw_inline_case_37",
        html: "<p>a<span><br></span>b</p>",
        expected_markdown: Some("a<span><br></span>b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "html_in_an_inline_element_is_a_raw_inline_case_38",
        html: "<p>a<del><br></del>b</p>",
        expected_markdown: Some("a<del><br></del>b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "html_in_an_inline_element_is_a_raw_inline_case_39",
        html: r#"<p>a<a href="u"><br></a>b</p>"#,
        expected_markdown: Some("a[<br>](u)b"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Code blocks and code spans](../translation_approach.md#code-blocks-and-code-spans).
/// The content of a code span or code block is literal text, so no encoding of
/// a break survives there.
#[test]
fn html_in_code_is_a_raw_inline() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "html_in_code_is_a_raw_inline_case_40",
        html: "<p>a<code>x<br>y</code>b</p>",
        expected_markdown: Some("a<code>x<br>y</code>b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "html_in_code_is_a_raw_inline_case_41",
        html: "<pre><code>a<br>b</code></pre>",
        expected_markdown: Some("<pre><code>a<br>b</code></pre>"),
        ..Case::default()
    }]);
    // Only the tags of a raw HTML inline are HTML: a `<pre>` written as one
    // holds CommonMark, translated like any other inline content.

    faithful_cases.extend([Case {
        id: "html_in_code_is_a_raw_inline_case_42",
        html: "<h1><pre><b>a</b></pre></h1>",
        expected_markdown: Some("# <pre>**a**</pre>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "html_in_code_is_a_raw_inline_case_43",
        html: r#"<h1><pre class="x"><b>a</b></pre></h1>"#,
        expected_markdown: Some(r#"# <pre class="x">**a**</pre>"#),
        ..Case::default()
    }]);
    // A `<code>` is the exception: a code block is a CommonMark block, so in
    // an inline context it is a raw HTML inline of its own.

    faithful_cases.extend([Case {
        id: "html_in_code_is_a_raw_inline_case_44",
        html: "<h1><pre><code>a*b\nc</code></pre></h1>",
        expected_markdown: Some(r"# <pre><code>a\*b c</code></pre>"),
        ..Case::default()
    }]);
    // An inline `<code>` outside a `<pre>` is a code span, whose content is
    // literal and so takes no escape.

    faithful_cases.extend([Case {
        id: "html_in_code_is_a_raw_inline_case_45",
        html: "<h1><div>a<code>c*d</code>b</div></h1>",
        expected_markdown: Some("# <div>a`c*d`b</div>"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// A raw text element holds literal characters rather than markup, so its
/// structure is serialized as it stands. The Markdown specials of that text are
/// still escaped, a CommonMark parser reading what sits between the tags of a
/// raw HTML inline as text.
#[test]
fn a_raw_text_element_escapes_the_markdown_it_holds() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_raw_text_element_escapes_the_markdown_it_holds_case_46",
        html: "<h1><script>a*b_c[d]</script></h1>",
        expected_markdown: Some(r"# <script>a\*b\_c\[d\]</script>"),
        ..Case::default()
    }]);
    // The `<` escape buys nothing here: CommonMark writes `&lt;`, and an HTML
    // parser decodes no character reference inside a raw text element. That is
    // the tradeoff the "Translating HTML nodes" section of
    // `translation_approach.md` takes.

    faithful_cases.extend([Case {
        id: "a_raw_text_element_escapes_the_markdown_it_holds_case_47",
        html: "<h1><script>if(a<b){}</script></h1>",
        expected_markdown: Some(r"# <script>if(a\<b){}</script>"),
        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<h1><script>if(a&lt;b){}</script></h1>\n",
            reason: "the escaped less-than sign becomes an undecoded character reference in script raw text",
            reference: "translation_approach.md#translating-html-nodes",
        }),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: r"# <script>if(a\&lt;b){}</script>",
            reason: "the second conversion escapes the literal ampersand in the raw-text reference",
            reference: "translation_approach.md#translating-html-nodes",
        },
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_raw_text_element_escapes_the_markdown_it_holds_case_48",
        html: "<p>x<script>a*b</script>y</p>",
        expected_markdown: Some(r"x<script>a\*b</script>y"),
        ..Case::default()
    }]);
    // An HTML block holds no CommonMark to escape, so it goes out as it
    // stands. Without the `<body>`, html5ever files a leading `<script>` under
    // the `<head>`, whose content is dropped.

    faithful_cases.extend([Case {
        id: "a_raw_text_element_escapes_the_markdown_it_holds_case_49",
        html: "<body><script>a*b\nc</script></body>",
        expected_markdown: Some("<script>a*b\nc</script>"),
        ..Case::default()
    }]);
    // See `round_trip_in_an_inline_context` in `basic_tests.rs`.

    faithful_cases.extend([Case {
        id: "a_raw_text_element_escapes_the_markdown_it_holds_case_50",
        html: "<h1><script>a\nb</script></h1>",
        expected_markdown: Some("# <script>a b</script>"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// `textarea` and `title` hold no markup either, but they are RCDATA rather
/// than raw text, so nothing forces their structure to be serialized and they
/// take the ordinary raw HTML inline path instead.
#[test]
fn an_rcdata_element_is_translated() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "an_rcdata_element_is_translated_case_51",
        html: "<h1><textarea>a*b*c</textarea></h1>",
        expected_markdown: Some(r"# <textarea>a\*b\*c</textarea>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "an_rcdata_element_is_translated_case_52",
        html: "<h1><title>a*b*c</title></h1>",
        expected_markdown: Some(r"# <title>a\*b\*c</title>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "an_rcdata_element_is_translated_case_53",
        html: "<h1><textarea>a\nb</textarea></h1>",
        expected_markdown: Some("# <textarea>a b</textarea>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "an_rcdata_element_is_translated_case_54",
        html: "<textarea>a*b\nc</textarea>",
        expected_markdown: Some("<textarea>a*b\nc</textarea>"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// "All raw HTML inlines will have whitespace collapsed in their contents
/// (including newlines, which would otherwise be problematic)" — the
/// "Translating HTML nodes" section of `translation_approach.md`. Encoding the
/// line ending instead would need a parser which decodes a character reference
/// there, and neither a comment nor a raw text element has one.
#[test]
fn a_raw_inline_collapses_the_whitespace_of_its_content() {
    let mut faithful_cases = Vec::new();
    // Type 6.

    faithful_cases.extend([Case {
        id: "a_raw_inline_collapses_the_whitespace_of_its_content_case_55",
        html: "<h1>x<div>a\nb</div>y</h1>",
        expected_markdown: Some("# x<div>a b</div>y"),
        ..Case::default()
    }]);
    // Type 7.

    faithful_cases.extend([Case {
        id: "a_raw_inline_collapses_the_whitespace_of_its_content_case_56",
        html: "<p>a<del>\n  b\n</del>c</p>",
        expected_markdown: Some("a<del> b </del>c"),
        ..Case::default()
    }]);
    // Type 1, whose whitespace carries the meaning of the element — the loss
    // this collapse takes on.

    faithful_cases.extend([Case {
        id: "a_raw_inline_collapses_the_whitespace_of_its_content_case_57",
        html: "<h1><pre>a\n  b</pre></h1>",
        expected_markdown: Some("# <pre>a b</pre>"),
        ..Case::default()
    }]);
    // Ordinary flow content collapses the same way.

    faithful_cases.extend([Case {
        id: "a_raw_inline_collapses_the_whitespace_of_its_content_case_58",
        html: "<p>a\nb</p>",
        expected_markdown: Some("a b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_raw_inline_collapses_the_whitespace_of_its_content_case_59",
        html: "<h1>a\n  b</h1>",
        expected_markdown: Some("# a b"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// That collapse is scoped to the contents; an attribute value keeps its
/// whitespace. A bare line ending there would end the leaf block holding the
/// element, so that alone is encoded — the open tag passes through CommonMark
/// verbatim, and the HTML parser reading the result decodes it back.
#[test]
fn a_raw_inline_escapes_the_line_endings_of_its_attributes() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_raw_inline_escapes_the_line_endings_of_its_attributes_case_60",
        html: "<p>a<em foo=\"1\n\n2\">y</em>b</p>",
        expected_markdown: Some(r#"a<em foo="1&#10;&#10;2">y</em>b"#),
        ..Case::default()
    }]);
    // Whitespace which is not a line ending is left alone.

    faithful_cases.extend([Case {
        id: "a_raw_inline_escapes_the_line_endings_of_its_attributes_case_61",
        html: "<h1>a<div title=\"x  y\">z  w</div>b</h1>",
        expected_markdown: Some(r#"# a<div title="x  y">z w</div>b"#),
        ..Case::default()
    }]);
    // A raw HTML inline nested in another one keeps its attribute value too.

    faithful_cases.extend([Case {
        id: "a_raw_inline_escapes_the_line_endings_of_its_attributes_case_62",
        html: r#"<h1>a<div title="p  q"><em foo="r  s">t</em></div>b</h1>"#,
        expected_markdown: Some(r#"# a<div title="p  q"><em foo="r  s">t</em></div>b"#),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_raw_inline_escapes_the_line_endings_of_its_attributes_case_63",
        html: "<h1>a<div title=\"p\nq\"><em foo=\"r\ns\">t</em></div>b</h1>",
        expected_markdown: Some(r#"# a<div title="p&#10;q"><em foo="r&#10;s">t</em></div>b"#),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_raw_inline_escapes_the_line_endings_of_its_attributes_case_64",
        html: "<table><thead><tr><th>h</th></tr></thead>\
             <tbody><tr><td><em foo=\"1\n2\">y</em></td></tr></tbody></table>",
        expected_markdown: Some(concat!(
            "| h                        |\n",
            "| ------------------------ |\n",
            "| <em foo=\"1&#10;2\">y</em> |"
        )),
        markdown_options: CommonMarkOptions::ENABLE_TABLES,
        ..Case::default()
    }]);
    // An HTML *block* keeps its line structure: only a blank line ends a type 6
    // one, so only that is encoded.

    faithful_cases.extend([Case {
        id: "a_raw_inline_escapes_the_line_endings_of_its_attributes_case_65",
        html: "<div>a\n\nb</div>",
        expected_markdown: Some("<div>a\n&#10;b</div>"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Code blocks and code spans](../translation_approach.md#code-blocks-and-code-spans).
/// A `|` in an open tag is encoded for the same reason as a line ending there.
/// The `\|` which `table::normalize_cell_content` writes for a `|` elsewhere in
/// a cell is a CommonMark escape, and one written inside a tag stays the two
/// characters it is, so the tag keeps a `|` that would split the row.
#[test]
fn a_raw_inline_encodes_the_pipes_of_its_attributes() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_raw_inline_encodes_the_pipes_of_its_attributes_case_66",
        html: "<table><thead><tr><th>h</th></tr></thead>\
             <tbody><tr><td><em foo=\"1|2\">y|z</em></td></tr></tbody></table>",
        expected_markdown: Some(concat!(
            "| h                            |\n",
            "| ---------------------------- |\n",
            r#"| <em foo="1&#124;2">y\|z</em> |"#
        )),
        markdown_options: CommonMarkOptions::ENABLE_TABLES,
        ..Case::default()
    }]);
    // Outside a cell the encoding is redundant but harmless: it decodes back to
    // the `|` it replaced. The contents are CommonMark text either way, where
    // `|` needs no escape at all.

    faithful_cases.extend([Case {
        id: "a_raw_inline_encodes_the_pipes_of_its_attributes_case_67",
        html: r#"<p>a<em foo="1|2">y|z</em>b</p>"#,
        expected_markdown: Some(r#"a<em foo="1&#124;2">y|z</em>b"#),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// A type 1 HTML block ends at its closing tag rather than at a blank line, so
/// escaping a blank line inside one would needlessly rewrite the script, style,
/// or preformatted text itself.
#[test]
fn a_type_1_block_keeps_its_blank_lines() {
    let mut faithful_cases = Vec::new();
    // Without the `<body>`, html5ever files a leading `<script>` or `<style>`
    // under the `<head>`, whose content is dropped.

    faithful_cases.extend([Case {
        id: "a_type_1_block_keeps_its_blank_lines_case_68",
        html: "<body><script>a\n\nb</script></body>",
        expected_markdown: Some("<script>a\n\nb</script>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_type_1_block_keeps_its_blank_lines_case_69",
        html: "<body><style>a\n\nb</style></body>",
        expected_markdown: Some("<style>a\n\nb</style>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_type_1_block_keeps_its_blank_lines_case_70",
        html: "<pre>a\n\nb</pre>",
        expected_markdown: Some("<pre>a\n\nb</pre>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_type_1_block_keeps_its_blank_lines_case_71",
        html: "<textarea>a\n\nb</textarea>",
        expected_markdown: Some("<textarea>a\n\nb</textarea>"),
        ..Case::default()
    }]);
    // In an inline context the same element is a raw HTML inline, whose blank
    // lines are collapsed with the rest of its whitespace.

    faithful_cases.extend([Case {
        id: "a_type_1_block_keeps_its_blank_lines_case_72",
        html: "<h1><script>a\n\nb</script></h1>",
        expected_markdown: Some("# <script>a b</script>"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// A link title and an image's alt text are inline content, and a line ending
/// there ends the leaf block holding it just as one in a raw HTML inline does:
/// written literally, `# [t](u "a⏎b")` is the heading `[t](u "a` followed by
/// the paragraph `b")`. CommonMark recognizes character references in both, so
/// each line ending is encoded instead.
#[test]
fn a_title_escapes_its_line_endings() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_title_escapes_its_line_endings_case_73",
        html: "<h1><a href=\"u\" title=\"a\nb\">t</a></h1>",
        expected_markdown: Some(r#"# [t](u "a&#10;b")"#),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_title_escapes_its_line_endings_case_74",
        html: "<h1><img src=\"i\" alt=\"a\nb\" title=\"c\nd\"></h1>",
        expected_markdown: Some(r#"# ![a&#10;b](i "c&#10;d")"#),
        ..Case::default()
    }]);
    // A blank line is encoded.

    faithful_cases.extend([Case {
        id: "a_title_escapes_its_line_endings_case_75",
        html: "<p><a href=\"u\" title=\"a\n\nb\">t</a></p>",
        expected_markdown: Some(r#"[t](u "a&#10;&#10;b")"#),
        ..Case::default()
    }]);
    // A carriage return reaches the title only as a character reference, the
    // parser having folded any literal CRLF into a line feed. Each line ending
    // character is encoded where it stands, so the pair comes back intact.

    faithful_cases.extend([Case {
        id: "a_title_escapes_its_line_endings_case_76",
        html: "<h1><a href=\"u\" title=\"a&#13;&#10;b\">t</a></h1>",
        expected_markdown: Some(r#"# [t](u "a&#13;&#10;b")"#),
        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<h1><a href=\"u\" title=\"a\r\nb\">t</a></h1>\n",
            reason: "the rendered literal CRLF is normalized to LF on the next HTML parse",
            reference: "translation_approach.md#links",
        }),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: r#"# [t](u "a&#10;b")"#,
            reason: "HTML input preprocessing normalizes the rendered literal CRLF to LF",
            reference: "translation_approach.md#links",
        },
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_title_escapes_its_line_endings_case_77",
        html: "<h1><a href=\"u\" title=\"a&#13;b\">t</a></h1>",
        expected_markdown: Some(r#"# [t](u "a&#13;b")"#),
        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<h1><a href=\"u\" title=\"a\rb\">t</a></h1>\n",
            reason: "the rendered literal CR is normalized to LF on the next HTML parse",
            reference: "translation_approach.md#links",
        }),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: r#"# [t](u "a&#10;b")"#,
            reason: "HTML input preprocessing normalizes the rendered literal CR to LF",
            reference: "translation_approach.md#links",
        },
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_title_escapes_its_line_endings_case_78",
        html: "<table><thead><tr><th>h</th></tr></thead>\
             <tbody><tr><td><a href=\"u\" title=\"a\nb\">t</a></td></tr></tbody></table>",
        expected_markdown: Some(concat!(
            "| h                |\n",
            "| ---------------- |\n",
            "| [t](u \"a&#10;b\") |"
        )),
        markdown_options: CommonMarkOptions::ENABLE_TABLES,
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// A comment is a type 2 HTML block, which likewise ends at its own `-->`. In
/// an inline context it is a raw HTML inline instead, whose whitespace is
/// collapsed: no parser decodes a character reference inside a comment, so
/// encoding the line ending would be no use.
#[test]
fn a_comment_follows_its_context() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_comment_follows_its_context_case_79",
        html: "<!--a\n\nb-->",
        expected_markdown: Some("<!--a\n\nb-->"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_comment_follows_its_context_case_80",
        html: "<div>a</div><!--c--><div>b</div>",
        expected_markdown: Some("<div>a</div>\n\n<!--c-->\n\n<div>b</div>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_comment_follows_its_context_case_81",
        html: "<blockquote><!--c--></blockquote>",
        expected_markdown: Some("> <!--c-->"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_comment_follows_its_context_case_82",
        html: "<h1>x<!--a\nb-->y</h1>",
        expected_markdown: Some("# x<!--a b-->y"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_comment_follows_its_context_case_83",
        html: "<p>x<!--a\n\nb-->y</p>",
        expected_markdown: Some("x<!--a b-->y"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_comment_follows_its_context_case_84",
        html: "<h1>x<!--a\r\nb-->y</h1>",
        expected_markdown: Some("# x<!--a b-->y"),
        ..Case::default()
    }]);
    // A comment inside a raw HTML inline is a comment still.

    faithful_cases.extend([Case {
        id: "a_comment_follows_its_context_case_85",
        html: "<h1><div>x<!--a\nb-->y</div></h1>",
        expected_markdown: Some("# <div>x<!--a b-->y</div>"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// The html5ever tokenizer hands a CDATA section and a processing instruction
/// back as comments, so both take the type 2 path above rather than the type 5
/// and type 3 ones they were written as.
#[test]
fn a_cdata_section_and_a_processing_instruction_are_comments() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_cdata_section_and_a_processing_instruction_are_comments_case_86",
        html: "<![CDATA[x\n\ny]]>",
        expected_markdown: Some("<!--[CDATA[x\n\ny]]-->"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_cdata_section_and_a_processing_instruction_are_comments_case_87",
        html: "<p>a<![CDATA[x\ny]]>b</p>",
        expected_markdown: Some("a<!--[CDATA[x y]]-->b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_cdata_section_and_a_processing_instruction_are_comments_case_88",
        html: "<p>a<?php echo \"x\ny\"; ?>b</p>",
        expected_markdown: Some("a<!--?php echo \"x y\"; ?-->b"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// What a raw HTML inline holds is an inline context as well, so the rule of
/// `a_commonmark_block_in_an_inline_context_is_a_raw_inline` holds one element
/// deeper.
#[test]
fn a_commonmark_block_inside_a_raw_inline_is_a_raw_inline() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_commonmark_block_inside_a_raw_inline_is_a_raw_inline_case_89",
        html: "<h1><div><blockquote>a</blockquote></div></h1>",
        expected_markdown: Some("# <div><blockquote>a</blockquote></div>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_commonmark_block_inside_a_raw_inline_is_a_raw_inline_case_90",
        html: "<h1><div><ul><li>a</li></ul></div></h1>",
        expected_markdown: Some("# <div><ul><li>a</li></ul></div>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_commonmark_block_inside_a_raw_inline_is_a_raw_inline_case_91",
        html: "<h1><div><h2>a</h2></div></h1>",
        expected_markdown: Some("# <div><h2>a</h2></div>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_commonmark_block_inside_a_raw_inline_is_a_raw_inline_case_92",
        html: "<h1><div><hr></div></h1>",
        expected_markdown: Some("# <div><hr></div>"),
        ..Case::default()
    }]);
    // What CommonMark can write inline is still translated there.

    faithful_cases.extend([Case {
        id: "a_commonmark_block_inside_a_raw_inline_is_a_raw_inline_case_93",
        html: "<h1><div>a<em>b</em>c</div></h1>",
        expected_markdown: Some("# <div>a*b*c</div>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_commonmark_block_inside_a_raw_inline_is_a_raw_inline_case_94",
        html: r#"<h1><div>a<a href="u">l*m</a>b</div></h1>"#,
        expected_markdown: Some(r"# <div>a[l\*m](u)b</div>"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn a_serialized_element_follows_its_context() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_serialized_element_follows_its_context_case_95",
        html: "<p>x <em foo>y</em> z</p>",
        expected_markdown: Some(r#"x <em foo="">y</em> z"#),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_serialized_element_follows_its_context_case_96",
        html: "<h1><em foo>y</em></h1>",
        expected_markdown: Some(r#"# <em foo="">y</em>"#),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_serialized_element_follows_its_context_case_97",
        html: "<ul><li><em foo>y</em></li></ul>",
        expected_markdown: Some(r#"*   <em foo="">y</em>"#),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Math](../translation_approach.md#math).
/// The contents of a math span are literal text, but a line ending there would
/// end the span and begin another block. LaTeX ignores whitespace, so each line
/// ending becomes a space.
#[test]
fn a_math_span_replaces_its_line_endings_with_spaces() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_math_span_replaces_its_line_endings_with_spaces_case_98",
        html: "<p>x<span class=\"math math-inline\">a\nb</span>y</p>",
        expected_markdown: Some("x$a b$y"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_math_span_replaces_its_line_endings_with_spaces_case_99",
        html: "<p><span class=\"math math-display\">a\nb</span></p>",
        expected_markdown: Some("$$a b$$"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        ..Case::default()
    }]);
    // A carriage return reaches the span only as a character reference, the
    // parser having folded any literal CRLF into a line feed. A CRLF pair is
    // one line ending, so it becomes one space...

    faithful_cases.extend([Case {
        id: "a_math_span_replaces_its_line_endings_with_spaces_case_100",
        html: "<p><span class=\"math math-inline\">a&#13;&#10;b</span></p>",
        expected_markdown: Some("$a b$"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        ..Case::default()
    }]);
    // ...while a lone carriage return is a line ending of its own.

    faithful_cases.extend([Case {
        id: "a_math_span_replaces_its_line_endings_with_spaces_case_101",
        html: "<p><span class=\"math math-inline\">a&#13;b</span></p>",
        expected_markdown: Some("$a b$"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        ..Case::default()
    }]);
    // An escaped line ending is LaTeX's control space, which an escaped space
    // spells as well.

    faithful_cases.extend([Case {
        id: "a_math_span_replaces_its_line_endings_with_spaces_case_102",
        html: "<p><span class=\"math math-inline\">a\\\nb</span></p>",
        expected_markdown: Some(r"$a\ b$"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Math](../translation_approach.md#math).
/// Joining a math span's lines would extend a LaTeX comment over every line
/// after it, so each comment is replaced first. As in LaTeX, a comment runs
/// through its line ending and the whitespace beginning the next line; a space
/// replaces it, since that ends a control word as the comment did.
#[test]
fn a_math_span_strips_its_comments() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_math_span_strips_its_comments_case_103",
        html: "<p><span class=\"math math-inline\">\\alpha%c\n  \tx</span></p>",
        expected_markdown: Some(r"$\alpha x$"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        ..Case::default()
    }]);
    // The space is wrong in text mode, where the comment joined two letters.

    faithful_cases.extend([Case {
        id: "a_math_span_strips_its_comments_case_104",
        html: "<p><span class=\"math math-inline\">\\text{fo%c\no}</span></p>",
        expected_markdown: Some(r"$\text{fo o}$"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        ..Case::default()
    }]);
    // Every line ending bounds a comment: a CRLF pair and a lone carriage
    // return included.

    faithful_cases.extend([Case {
        id: "a_math_span_strips_its_comments_case_105",
        html: "<p><span class=\"math math-display\">a%x&#13;&#10;b&#13;c%y&#13;d</span></p>",
        expected_markdown: Some("$$a b c d$$"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        ..Case::default()
    }]);
    // A comment on the last line still leaves its space, which only inline math
    // trims.

    faithful_cases.extend([Case {
        id: "a_math_span_strips_its_comments_case_106",
        html: "<p><span class=\"math math-display\">a%b</span></p>",
        expected_markdown: Some("$$a $$"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        ..Case::default()
    }]);
    // `\%` is a literal percent sign...

    faithful_cases.extend([Case {
        id: "a_math_span_strips_its_comments_case_107",
        html: r#"<p><span class="math math-inline">a\%b</span></p>"#,
        expected_markdown: Some(r"$a\%b$"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        ..Case::default()
    }]);
    // ...but `\\` is a control symbol, leaving the `%` after it unescaped.

    faithful_cases.extend([Case {
        id: "a_math_span_strips_its_comments_case_108",
        html: r#"<p><span class="math math-inline">a\\%b</span></p>"#,
        expected_markdown: Some(r"$a\\$"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        ..Case::default()
    }]);
    // A span which is all comment holds no math. Inline math cannot be empty,
    // so the span goes out as HTML, holding the space which replaced the
    // comment.

    faithful_cases.extend([Case {
        id: "a_math_span_strips_its_comments_case_109",
        html: r#"<p><span class="math math-inline">%a</span></p>"#,
        expected_markdown: Some(r#"<span class="math math-inline"> </span>"#),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Math](../translation_approach.md#math).
/// A math span written as HTML holds its comment-stripped, single-line source:
/// serializing collapses the line ending of the original, which would leave
/// `d` inside the comment.
#[test]
fn an_unwritable_math_span_strips_its_comments() {
    let html = "<p><span class=\"math math-inline\">a$b%c\nd</span></p>";

    check_cases(&[Case {
        id: "an_unwritable_math_span_strips_its_comments_case_110",
        html,
        expected_markdown: Some(r#"<span class="math math-inline">a\$b d</span>"#),
        ..Case::default()
    }]);
    assert_eq!(r"a\$b d", convert_pure(html).unwrap());
}
