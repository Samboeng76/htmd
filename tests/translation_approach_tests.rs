//! Tests of the tables and rules of `translation_approach.md`. A test of a
//! table holds its rows in the order they are written there, and covers the
//! whole table or a part which needs a test of its own.
//!
//! Where a row writes `<br><br>...<br>` — one or more `<br>` elements — it is
//! checked with one and with three, except where the table gives one `<br>` and
//! a run of them a row each.

use htmd::options::{HeadingStyle, Options};
use pretty_assertions::assert_eq;

mod common;
use common::{Case, HtmlExpectation, StabilityExpectation, check_cases, convert_faithful};

/// Approved second-trip results for block children inside inline elements.
/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
const INLINE_BLOCK_INSTABILITIES: &[(&str, &str)] = &[
    (
        "a<a href=\"u\">x<p>b</p>y</a>c",
        "a[x](u)\n\n[b](u)\n\n[y](u)c",
    ),
    ("a<a href=\"u\"><p>b</p></a>c", "a[](u)\n\n[b](u)\n\nc"),
    (
        "a<a href=\"u\">x<h2>b</h2>y</a>c",
        "a[x](u)\n\n## [b](u)\n\n[y](u)c",
    ),
    ("a<a href=\"u\"><h2>b</h2></a>c", "a[](u)\n\n## [b](u)\n\nc"),
    (
        "a<a href=\"u\">x<ul><li>b</li></ul>y</a>c",
        "a[x](u)\n\n*   [b](u)\n\n[y](u)c",
    ),
    (
        "a<a href=\"u\"><ul><li>b</li></ul></a>c",
        "a[](u)\n\n*   [b](u)\n\nc",
    ),
    (
        "a<a href=\"u\">x<ol><li>b</li></ol>y</a>c",
        "a[x](u)\n\n1.  [b](u)\n\n[y](u)c",
    ),
    (
        "a<a href=\"u\"><ol><li>b</li></ol></a>c",
        "a[](u)\n\n1.  [b](u)\n\nc",
    ),
    (
        "a<a href=\"u\">x<blockquote>b</blockquote>y</a>c",
        "a[x](u)\n\n> [b](u)\n\n[y](u)c",
    ),
    (
        "a<a href=\"u\"><blockquote>b</blockquote></a>c",
        "a[](u)\n\n> [b](u)\n\nc",
    ),
    (
        "a<a href=\"u\">x<pre>b</pre>y</a>c",
        "a[x](u)\n\n<pre><a href=\"u\">b</a></pre>\n\n[y](u)c",
    ),
    (
        "a<a href=\"u\"><pre>b</pre></a>c",
        "a[](u)\n\n<pre><a href=\"u\">b</a></pre>\n\nc",
    ),
    ("a<a href=\"u\">x<hr>y</a>c", "a[x](u)\n\n* * *\n\n[y](u)c"),
    ("a<a href=\"u\"><hr></a>c", "a[](u)\n\n* * *\n\nc"),
    (
        "a<a href=\"u\">x<div>b</div>y</a>c",
        "a[x](u)\n\n<div><a href=\"u\">b</a></div>\n\n[y](u)c",
    ),
    (
        "a<a href=\"u\"><div>b</div></a>c",
        "a[](u)\n\n<div><a href=\"u\">b</a></div>\n\nc",
    ),
    ("a<em>x<p>b</p>y</em>c", "a*x*\n\n*b*\n\n*y*c"),
    ("a<em><p>b</p></em>c", "a<em></em>\n\n*b*\n\nc"),
    ("a<em>x<h2>b</h2>y</em>c", "a*x*\n\n## *b*\n\n*y*c"),
    ("a<em><h2>b</h2></em>c", "a<em></em>\n\n## *b*\n\nc"),
    (
        "a<em>x<ul><li>b</li></ul>y</em>c",
        "a*x*\n\n*   *b*\n\n*y*c",
    ),
    (
        "a<em><ul><li>b</li></ul></em>c",
        "a<em></em>\n\n*   *b*\n\nc",
    ),
    (
        "a<em>x<ol><li>b</li></ol>y</em>c",
        "a*x*\n\n1.  *b*\n\n*y*c",
    ),
    (
        "a<em><ol><li>b</li></ol></em>c",
        "a<em></em>\n\n1.  *b*\n\nc",
    ),
    (
        "a<em>x<blockquote>b</blockquote>y</em>c",
        "a*x*\n\n> *b*\n\n*y*c",
    ),
    (
        "a<em><blockquote>b</blockquote></em>c",
        "a<em></em>\n\n> *b*\n\nc",
    ),
    (
        "a<em>x<pre>b</pre>y</em>c",
        "a*x*\n\n<pre><em>b</em></pre>\n\n*y*c",
    ),
    (
        "a<em><pre>b</pre></em>c",
        "a<em></em>\n\n<pre><em>b</em></pre>\n\nc",
    ),
    ("a<em>x<hr>y</em>c", "a*x*\n\n* * *\n\n*y*c"),
    ("a<em><hr></em>c", "a<em></em>\n\n* * *\n\nc"),
    (
        "a<em>x<div>b</div>y</em>c",
        "a*x*\n\n<div><em>b</em></div>\n\n*y*c",
    ),
    (
        "a<em><div>b</div></em>c",
        "a<em></em>\n\n<div><em>b</em></div>\n\nc",
    ),
    ("a<i>x<p>b</p>y</i>c", "a*x*\n\n*b*\n\n*y*c"),
    ("a<i><p>b</p></i>c", "a<i></i>\n\n*b*\n\nc"),
    ("a<i>x<h2>b</h2>y</i>c", "a*x*\n\n## *b*\n\n*y*c"),
    ("a<i><h2>b</h2></i>c", "a<i></i>\n\n## *b*\n\nc"),
    ("a<i>x<ul><li>b</li></ul>y</i>c", "a*x*\n\n*   *b*\n\n*y*c"),
    ("a<i><ul><li>b</li></ul></i>c", "a<i></i>\n\n*   *b*\n\nc"),
    ("a<i>x<ol><li>b</li></ol>y</i>c", "a*x*\n\n1.  *b*\n\n*y*c"),
    ("a<i><ol><li>b</li></ol></i>c", "a<i></i>\n\n1.  *b*\n\nc"),
    (
        "a<i>x<blockquote>b</blockquote>y</i>c",
        "a*x*\n\n> *b*\n\n*y*c",
    ),
    (
        "a<i><blockquote>b</blockquote></i>c",
        "a<i></i>\n\n> *b*\n\nc",
    ),
    (
        "a<i>x<pre>b</pre>y</i>c",
        "a*x*\n\n<pre><em>b</em></pre>\n\n*y*c",
    ),
    (
        "a<i><pre>b</pre></i>c",
        "a<i></i>\n\n<pre><i>b</i></pre>\n\nc",
    ),
    ("a<i>x<hr>y</i>c", "a*x*\n\n* * *\n\n*y*c"),
    ("a<i><hr></i>c", "a<i></i>\n\n* * *\n\nc"),
    (
        "a<i>x<div>b</div>y</i>c",
        "a*x*\n\n<div><em>b</em></div>\n\n*y*c",
    ),
    (
        "a<i><div>b</div></i>c",
        "a<i></i>\n\n<div><i>b</i></div>\n\nc",
    ),
    ("a<strong>x<p>b</p>y</strong>c", "a**x**\n\n**b**\n\n**y**c"),
    (
        "a<strong><p>b</p></strong>c",
        "a<strong></strong>\n\n**b**\n\nc",
    ),
    (
        "a<strong>x<h2>b</h2>y</strong>c",
        "a**x**\n\n## **b**\n\n**y**c",
    ),
    (
        "a<strong><h2>b</h2></strong>c",
        "a<strong></strong>\n\n## **b**\n\nc",
    ),
    (
        "a<strong>x<ul><li>b</li></ul>y</strong>c",
        "a**x**\n\n*   **b**\n\n**y**c",
    ),
    (
        "a<strong><ul><li>b</li></ul></strong>c",
        "a<strong></strong>\n\n*   **b**\n\nc",
    ),
    (
        "a<strong>x<ol><li>b</li></ol>y</strong>c",
        "a**x**\n\n1.  **b**\n\n**y**c",
    ),
    (
        "a<strong><ol><li>b</li></ol></strong>c",
        "a<strong></strong>\n\n1.  **b**\n\nc",
    ),
    (
        "a<strong>x<blockquote>b</blockquote>y</strong>c",
        "a**x**\n\n> **b**\n\n**y**c",
    ),
    (
        "a<strong><blockquote>b</blockquote></strong>c",
        "a<strong></strong>\n\n> **b**\n\nc",
    ),
    (
        "a<strong>x<pre>b</pre>y</strong>c",
        "a**x**\n\n<pre><strong>b</strong></pre>\n\n**y**c",
    ),
    (
        "a<strong><pre>b</pre></strong>c",
        "a<strong></strong>\n\n<pre><strong>b</strong></pre>\n\nc",
    ),
    ("a<strong>x<hr>y</strong>c", "a**x**\n\n* * *\n\n**y**c"),
    (
        "a<strong><hr></strong>c",
        "a<strong></strong>\n\n* * *\n\nc",
    ),
    (
        "a<strong>x<div>b</div>y</strong>c",
        "a**x**\n\n<div><strong>b</strong></div>\n\n**y**c",
    ),
    (
        "a<strong><div>b</div></strong>c",
        "a<strong></strong>\n\n<div><strong>b</strong></div>\n\nc",
    ),
    ("a<b>x<p>b</p>y</b>c", "a**x**\n\n**b**\n\n**y**c"),
    ("a<b><p>b</p></b>c", "a<b></b>\n\n**b**\n\nc"),
    ("a<b>x<h2>b</h2>y</b>c", "a**x**\n\n## **b**\n\n**y**c"),
    ("a<b><h2>b</h2></b>c", "a<b></b>\n\n## **b**\n\nc"),
    (
        "a<b>x<ul><li>b</li></ul>y</b>c",
        "a**x**\n\n*   **b**\n\n**y**c",
    ),
    ("a<b><ul><li>b</li></ul></b>c", "a<b></b>\n\n*   **b**\n\nc"),
    (
        "a<b>x<ol><li>b</li></ol>y</b>c",
        "a**x**\n\n1.  **b**\n\n**y**c",
    ),
    ("a<b><ol><li>b</li></ol></b>c", "a<b></b>\n\n1.  **b**\n\nc"),
    (
        "a<b>x<blockquote>b</blockquote>y</b>c",
        "a**x**\n\n> **b**\n\n**y**c",
    ),
    (
        "a<b><blockquote>b</blockquote></b>c",
        "a<b></b>\n\n> **b**\n\nc",
    ),
    (
        "a<b>x<pre>b</pre>y</b>c",
        "a**x**\n\n<pre><strong>b</strong></pre>\n\n**y**c",
    ),
    (
        "a<b><pre>b</pre></b>c",
        "a<b></b>\n\n<pre><b>b</b></pre>\n\nc",
    ),
    ("a<b>x<hr>y</b>c", "a**x**\n\n* * *\n\n**y**c"),
    ("a<b><hr></b>c", "a<b></b>\n\n* * *\n\nc"),
    (
        "a<b>x<div>b</div>y</b>c",
        "a**x**\n\n<div><strong>b</strong></div>\n\n**y**c",
    ),
    (
        "a<b><div>b</div></b>c",
        "a<b></b>\n\n<div><b>b</b></div>\n\nc",
    ),
    ("a<code>x<p>b</p>y</code>c", "a`x`\n\n`b`\n\n`y`c"),
    ("a<code><p>b</p></code>c", "a<code></code>\n\n`b`\n\nc"),
    ("a<code>x<h2>b</h2>y</code>c", "a`x`\n\n## `b`\n\n`y`c"),
    ("a<code><h2>b</h2></code>c", "a<code></code>\n\n## `b`\n\nc"),
    (
        "a<code>x<ul><li>b</li></ul>y</code>c",
        "a`x`\n\n*   `b`\n\n`y`c",
    ),
    (
        "a<code><ul><li>b</li></ul></code>c",
        "a<code></code>\n\n*   `b`\n\nc",
    ),
    (
        "a<code>x<ol><li>b</li></ol>y</code>c",
        "a`x`\n\n1.  `b`\n\n`y`c",
    ),
    (
        "a<code><ol><li>b</li></ol></code>c",
        "a<code></code>\n\n1.  `b`\n\nc",
    ),
    (
        "a<code>x<blockquote>b</blockquote>y</code>c",
        "a`x`\n\n> `b`\n\n`y`c",
    ),
    (
        "a<code><blockquote>b</blockquote></code>c",
        "a<code></code>\n\n> `b`\n\nc",
    ),
    (
        "a<code>x<pre>b</pre>y</code>c",
        "a`x`\n\n```\nb\n```\n\n`y`c",
    ),
    (
        "a<code><pre>b</pre></code>c",
        "a<code></code>\n\n```\nb\n```\n\nc",
    ),
    ("a<code>x<hr>y</code>c", "a`x`\n\n* * *\n\n`y`c"),
    ("a<code><hr></code>c", "a<code></code>\n\n* * *\n\nc"),
    (
        "a<code>x<div>b</div>y</code>c",
        "a`x`\n\n<div><code>b</code></div>\n\n`y`c",
    ),
    (
        "a<code><div>b</div></code>c",
        "a<code></code>\n\n<div><code>b</code></div>\n\nc",
    ),
    ("a<del>x<p>b</p>y</del>c", "a<del>x</del>\n\nb\n\nyc"),
    ("a<del><p>b</p></del>c", "a<del></del>\n\nb\n\nc"),
    ("a<del>x<h2>b</h2>y</del>c", "a<del>x</del>\n\n## b\n\nyc"),
    ("a<del><h2>b</h2></del>c", "a<del></del>\n\n## b\n\nc"),
    (
        "a<del>x<ul><li>b</li></ul>y</del>c",
        "a<del>x</del>\n\n*   b\n\nyc",
    ),
    (
        "a<del><ul><li>b</li></ul></del>c",
        "a<del></del>\n\n*   b\n\nc",
    ),
    (
        "a<del>x<ol><li>b</li></ol>y</del>c",
        "a<del>x</del>\n\n1.  b\n\nyc",
    ),
    (
        "a<del><ol><li>b</li></ol></del>c",
        "a<del></del>\n\n1.  b\n\nc",
    ),
    (
        "a<del>x<blockquote>b</blockquote>y</del>c",
        "a<del>x</del>\n\n> b\n\nyc",
    ),
    (
        "a<del><blockquote>b</blockquote></del>c",
        "a<del></del>\n\n> b\n\nc",
    ),
    (
        "a<del>x<pre>b</pre>y</del>c",
        "a<del>x</del>\n\n<pre>b</pre>\n\nyc",
    ),
    (
        "a<del><pre>b</pre></del>c",
        "a<del></del>\n\n<pre>b</pre>\n\nc",
    ),
    ("a<del>x<hr>y</del>c", "a<del>x</del>\n\n* * *\n\nyc"),
    ("a<del><hr></del>c", "a<del></del>\n\n* * *\n\nc"),
    (
        "a<del>x<div>b</div>y</del>c",
        "a<del>x</del>\n\n<div>b</div>\n\nyc",
    ),
    (
        "a<del><div>b</div></del>c",
        "a<del></del>\n\n<div>b</div>\n\nc",
    ),
    (
        "a<span class=\"s\">x<p>b</p>y</span>c",
        "a<span class=\"s\">x</span>\n\nb\n\nyc",
    ),
    (
        "a<span class=\"s\"><p>b</p></span>c",
        "a<span class=\"s\"></span>\n\nb\n\nc",
    ),
    (
        "a<span class=\"s\">x<h2>b</h2>y</span>c",
        "a<span class=\"s\">x</span>\n\n## b\n\nyc",
    ),
    (
        "a<span class=\"s\"><h2>b</h2></span>c",
        "a<span class=\"s\"></span>\n\n## b\n\nc",
    ),
    (
        "a<span class=\"s\">x<ul><li>b</li></ul>y</span>c",
        "a<span class=\"s\">x</span>\n\n*   b\n\nyc",
    ),
    (
        "a<span class=\"s\"><ul><li>b</li></ul></span>c",
        "a<span class=\"s\"></span>\n\n*   b\n\nc",
    ),
    (
        "a<span class=\"s\">x<ol><li>b</li></ol>y</span>c",
        "a<span class=\"s\">x</span>\n\n1.  b\n\nyc",
    ),
    (
        "a<span class=\"s\"><ol><li>b</li></ol></span>c",
        "a<span class=\"s\"></span>\n\n1.  b\n\nc",
    ),
    (
        "a<span class=\"s\">x<blockquote>b</blockquote>y</span>c",
        "a<span class=\"s\">x</span>\n\n> b\n\nyc",
    ),
    (
        "a<span class=\"s\"><blockquote>b</blockquote></span>c",
        "a<span class=\"s\"></span>\n\n> b\n\nc",
    ),
    (
        "a<span class=\"s\">x<pre>b</pre>y</span>c",
        "a<span class=\"s\">x</span>\n\n<pre>b</pre>\n\nyc",
    ),
    (
        "a<span class=\"s\"><pre>b</pre></span>c",
        "a<span class=\"s\"></span>\n\n<pre>b</pre>\n\nc",
    ),
    (
        "a<span class=\"s\">x<hr>y</span>c",
        "a<span class=\"s\">x</span>\n\n* * *\n\nyc",
    ),
    (
        "a<span class=\"s\"><hr></span>c",
        "a<span class=\"s\"></span>\n\n* * *\n\nc",
    ),
    (
        "a<span class=\"s\">x<div>b</div>y</span>c",
        "a<span class=\"s\">x</span>\n\n<div>b</div>\n\nyc",
    ),
    (
        "a<span class=\"s\"><div>b</div></span>c",
        "a<span class=\"s\"></span>\n\n<div>b</div>\n\nc",
    ),
];

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// The "Translating HTML nodes" table: a CommonMark block may only process its
/// content in a block context.
#[test]
fn translating_html_nodes() {
    check_cases(&[Case {
        id: "translating_html_nodes_case_1",
        html: "<h1><p>a</p></h1>",
        expected_markdown: Some("# <p>a</p>"),
        ..Case::default()
    }]);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// The "Translating HTML nodes" rule that an inline begins an inline context,
/// applied to every inline which can hold an element: a block inside one goes
/// out as a raw HTML inline, as it does inside a heading. Images, autolinks and
/// hard line breaks hold no elements. GFM strikethrough has no handler, so
/// `<del>` is a raw HTML inline, like `<span>`.
#[test]
fn blocks_inside_inlines() {
    // An inline as HTML, then as the CommonMark expected for it with text at
    // each end of its content and with a block alone; `{}` stands for that
    // content. A raw HTML inline at the edge of an emphasis string would not
    // flank, so the "Emphasis and strong emphasis" approach writes that
    // emphasis as HTML.
    let inlines = [
        (r#"<a href="u">{}</a>"#, "[{}](u)", "[{}](u)"),
        ("<em>{}</em>", "*{}*", "<em>{}</em>"),
        ("<i>{}</i>", "*{}*", "<i>{}</i>"),
        ("<strong>{}</strong>", "**{}**", "<strong>{}</strong>"),
        ("<b>{}</b>", "**{}**", "<b>{}</b>"),
        ("<code>{}</code>", "<code>{}</code>", "<code>{}</code>"),
        ("<del>{}</del>", "<del>{}</del>", "<del>{}</del>"),
        (
            r#"<span class="s">{}</span>"#,
            r#"<span class="s">{}</span>"#,
            r#"<span class="s">{}</span>"#,
        ),
    ];
    // Each is spelled as it serializes, so the table carries the `<tbody>` the
    // HTML parser would otherwise insert.
    let blocks = [
        "<p>b</p>",
        "<h2>b</h2>",
        "<ul><li>b</li></ul>",
        "<ol><li>b</li></ol>",
        "<blockquote>b</blockquote>",
        "<pre>b</pre>",
        "<hr>",
        "<table><tbody><tr><td>b</td></tr></tbody></table>",
        "<div>b</div>",
    ];

    let mut inputs = Vec::new();
    for (inline, between_text, alone) in inlines {
        for block in blocks {
            let block_between_text = format!("x{block}y");
            let contents = [(block_between_text.as_str(), between_text), (block, alone)];
            for (content, markdown) in contents {
                let html = format!("a{}c", inline.replace("{}", content));
                let expected = format!("a{}c", markdown.replace("{}", content));
                inputs.push((html, expected));
            }
        }
    }
    let cases = inputs
        .iter()
        .map(|(html, expected)| Case {
            id: html,
            html,
            expected_markdown: Some(expected),
            stability: INLINE_BLOCK_INSTABILITIES
                .iter()
                .find(|(documented_html, _)| *documented_html == html)
                .map(|(_, second_markdown)| StabilityExpectation::DocumentedInstability {
                    second_markdown,
                    reason: "HTML parser repair moves block children out of inline elements before the second conversion",
                    reference: "translation_approach.md#translating-html-nodes",
                })
                .unwrap_or_default(),
            ..Case::default()
        })
        .collect::<Vec<_>>();
    assert_eq!(144, cases.len());
    assert_eq!(128, INLINE_BLOCK_INSTABILITIES.len());
    assert_eq!(
        INLINE_BLOCK_INSTABILITIES.len(),
        cases
            .iter()
            .filter(|case| matches!(
                case.stability,
                StabilityExpectation::DocumentedInstability { .. }
            ))
            .count()
    );
    check_cases(&cases);
}

/// [Code blocks and code spans](../translation_approach.md#code-blocks-and-code-spans).
/// Rows 1-4 of the "Code blocks and code spans" table. A code span's and a
/// fenced code block's content is literal text, so every row writes HTML rather
/// than a break. `basic_tests::faithful_mode_keeps_adjacent_code_spans_apart`
/// covers row 5.
#[test]
fn code_blocks_and_code_spans() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "code_blocks_and_code_spans_case_2",
        html: "<p>a<code>x<br>y</code>b</p>",
        expected_markdown: Some("a<code>x<br>y</code>b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "code_blocks_and_code_spans_case_3",
        html: "<p>a<code>x<br><br><br>y</code>b</p>",
        expected_markdown: Some("a<code>x<br><br><br>y</code>b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "code_blocks_and_code_spans_case_4",
        html: "<p>a<code><br></code>b</p>",
        expected_markdown: Some("a<code><br></code>b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "code_blocks_and_code_spans_case_5",
        html: "<p>a<code><br><br><br></code>b</p>",
        expected_markdown: Some("a<code><br><br><br></code>b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "code_blocks_and_code_spans_case_6",
        html: "<pre><code>a<br>b</code></pre>",
        expected_markdown: Some("<pre><code>a<br>b</code></pre>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "code_blocks_and_code_spans_case_7",
        html: "<pre><code>a<br><br><br>b</code></pre>",
        expected_markdown: Some("<pre><code>a<br><br><br>b</code></pre>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "code_blocks_and_code_spans_case_8",
        html: "<pre><code><br></code></pre>",
        expected_markdown: Some("<pre><code><br></code></pre>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "code_blocks_and_code_spans_case_9",
        html: "<pre><code><br><br><br></code></pre>",
        expected_markdown: Some("<pre><code><br><br><br></code></pre>"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Code blocks and code spans](../translation_approach.md#code-blocks-and-code-spans).
/// A code span whose text holds a line ending. A code span's content is literal
/// text, and no encoding of a break survives there (see the "Code blocks and
/// code spans" section), which makes this the same situation as the `<br>` rows
/// of [`code_blocks_and_code_spans`]: the `<code>` has to go out as a raw HTML
/// inline, whose contents the walk collapses to a single space. Writing the
/// span itself would break the "Translating HTML nodes" rule that no handler
/// may write a bare newline in an inline context.
#[test]
fn code_span_holding_a_line_ending() {
    let mut faithful_cases = Vec::new();
    // An ATX heading is a single line, so any line ending truncates it.

    faithful_cases.extend([Case {
        id: "code_span_holding_a_line_ending_case_10",
        html: "<h1>a<code>x\ny</code>b</h1>",
        expected_markdown: Some("# a<code>x y</code>b"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "# a`x y`b",
            reason: "the first trip collapses the code line ending, allowing a code span on the second trip",
            reference: "translation_approach.md#code-blocks-and-code-spans",
        },
        ..Case::default()
    }]);
    // A setext heading's underline attaches to the line above it.

    faithful_cases.extend([Case {
        id: "code_span_holding_a_line_ending_case_11_setext",
        html: "<h1>a<code>x\ny</code>b</h1>",
        expected_markdown: Some("a<code>x y</code>b\n=================="),
        htmd_options: Options {
            heading_style: HeadingStyle::Setex,
            ..Case::default().htmd_options
        },
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "a`x y`b\n=======",
            reason: "the first trip collapses the code line ending, allowing a shorter setext code span on the second trip",
            reference: "translation_approach.md#code-blocks-and-code-spans",
        },
        ..Case::default()
    }]);

    // A paragraph absorbs a lone line ending, which CommonMark reads as a
    // space; a blank one ends the paragraph instead.

    faithful_cases.extend([Case {
        id: "code_span_holding_a_line_ending_case_12",
        html: "<p>a<code>x\n\ny</code>b</p>",
        expected_markdown: Some("a<code>x y</code>b"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "a`x y`b",
            reason: "the first trip collapses the code line endings, allowing a code span on the second trip",
            reference: "translation_approach.md#code-blocks-and-code-spans",
        },
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "code_span_holding_a_line_ending_case_13",
        html: "<blockquote><p>a<code>x\n\ny</code>b</p></blockquote>",
        expected_markdown: Some("> a<code>x y</code>b"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "> a`x y`b",
            reason: "the first trip collapses the code line endings, allowing a code span on the second trip",
            reference: "translation_approach.md#code-blocks-and-code-spans",
        },
        ..Case::default()
    }]);
    // The same span in a block context, where the blank line ends the
    // blockquote rather than the paragraph inside it.

    faithful_cases.extend([Case {
        id: "code_span_holding_a_line_ending_case_14",
        html: "<blockquote><code>x\n\ny</code></blockquote>",
        expected_markdown: Some("> <code>x y</code>"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "> `x y`",
            reason: "the first trip collapses the code line endings, allowing a code span on the second trip",
            reference: "translation_approach.md#code-blocks-and-code-spans",
        },
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Code blocks and code spans](../translation_approach.md#code-blocks-and-code-spans).
/// A code span holding nothing. The backticks meant to open an empty span close
/// it instead, so CommonMark has no spelling for one and the `<code>` goes out
/// as a raw HTML inline. The walk collapses its contents, which turns the
/// whitespace-only spans below into the single space of the first row.
#[test]
fn empty_code_span() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "empty_code_span_case_15",
        html: "<p>a<code></code>b</p>",
        expected_markdown: Some("a<code></code>b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "empty_code_span_case_16",
        html: "<p>a<code> </code>b</p>",
        expected_markdown: Some("a<code> </code>b"),
        ..Case::default()
    }]);
    // Whitespace-only content reaches the emptiness test only after the trim,
    // so a line ending here is caught as an empty span rather than as the
    // line ending of [`code_span_holding_a_line_ending`].

    faithful_cases.extend([Case {
        id: "empty_code_span_case_17",
        html: "<p>a<code>\n</code>b</p>",
        expected_markdown: Some("a<code> </code>b"),
        ..Case::default()
    }]);

    // Pure mode has no fallback and drops the span, rather than write the bare
    // backticks CommonMark reads as literal text.
    assert_eq!(
        "ab",
        htmd::HtmlToMarkdown::new()
            .convert("<p>a<code></code>b</p>")
            .unwrap()
    );
    check_cases(&faithful_cases);
}

/// [Emphasis and strong emphasis](../translation_approach.md#emphasis-and-strong-emphasis).
/// Rows 1-6 of the "Emphasis and strong emphasis" table. An emphasis delimiter
/// placed against a raw `<br>` would not flank, so rows 1, 2 and 4 write the
/// element as HTML; rows 5 and 6 do so because they leave no emphasis string to
/// delimit. `basic_tests::faithful_mode_keeps_adjacent_emphasis_apart` covers
/// row 7.
#[test]
fn emphasis_and_strong_emphasis() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "emphasis_and_strong_emphasis_case_18",
        html: "<p><em><br>a</em></p>",
        expected_markdown: Some("<em><br>a</em>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "emphasis_and_strong_emphasis_case_19",
        html: "<p><em><br><br><br>a</em></p>",
        expected_markdown: Some("<em><br><br><br>a</em>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "emphasis_and_strong_emphasis_case_20",
        html: "<p><em>a<br></em></p>",
        expected_markdown: Some("<em>a<br></em>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "emphasis_and_strong_emphasis_case_21",
        html: "<p><em>a<br><br><br></em></p>",
        expected_markdown: Some("<em>a<br><br><br></em>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "emphasis_and_strong_emphasis_case_22",
        html: "<p><em>a<br>b</em></p>",
        expected_markdown: Some("*a<br>b*"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "emphasis_and_strong_emphasis_case_23",
        html: "<p><em>a<br><br><br>b</em></p>",
        expected_markdown: Some("*a<br><br><br>b*"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "emphasis_and_strong_emphasis_case_24",
        html: "<p>a<em><br></em>b</p>",
        expected_markdown: Some("a<em><br></em>b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "emphasis_and_strong_emphasis_case_25",
        html: "<p>a<em><br><br><br></em>b</p>",
        expected_markdown: Some("a<em><br><br><br></em>b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "emphasis_and_strong_emphasis_case_26",
        html: "<p>a<em> </em>b</p>",
        expected_markdown: Some("a<em> </em>b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "emphasis_and_strong_emphasis_case_27",
        html: "<p>a<strong> </strong>b</p>",
        expected_markdown: Some("a<strong> </strong>b"),
        ..Case::default()
    }]);
    // A no-break space is Unicode whitespace too, so it leaves no emphasis
    // string either.

    faithful_cases.extend([Case {
        id: "emphasis_and_strong_emphasis_case_28",
        html: "<p>a<em>&#160;</em>b</p>",
        expected_markdown: Some("a<em>\u{a0}</em>b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "emphasis_and_strong_emphasis_case_29",
        html: "<p>a<em></em>b</p>",
        expected_markdown: Some("a<em></em>b"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [At the document root](../translation_approach.md#at-the-document-root).
/// `<br>`s alone in an inline which goes out as a raw HTML inline. GFM
/// strikethrough has no handler, so `<del>` is one, like `<span>`.
#[test]
fn brs_alone_in_a_raw_html_inline() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "brs_alone_in_a_raw_html_inline_case_30",
        html: "<p>a<del><br></del>b</p>",
        expected_markdown: Some("a<del><br></del>b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "brs_alone_in_a_raw_html_inline_case_31",
        html: "<p>a<del><br><br><br></del>b</p>",
        expected_markdown: Some("a<del><br><br><br></del>b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "brs_alone_in_a_raw_html_inline_case_32",
        html: "<p>a<span><br></span>b</p>",
        expected_markdown: Some("a<span><br></span>b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "brs_alone_in_a_raw_html_inline_case_33",
        html: "<p>a<span><br><br><br></span>b</p>",
        expected_markdown: Some("a<span><br><br><br></span>b"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Links](../translation_approach.md#links).
/// `<br>`s in link text, which the "Links" section translates as CommonMark. A
/// link label has no flanking rule, so every case keeps its CommonMark link.
#[test]
fn links() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "links_case_34",
        html: r#"<p>a<a href="u"><br></a>b</p>"#,
        expected_markdown: Some("a[<br>](u)b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "links_case_35",
        html: r#"<p>a<a href="u"><br><br><br></a>b</p>"#,
        expected_markdown: Some("a[<br><br><br>](u)b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "links_case_36",
        html: r#"<p>a<a href="u"><br>c</a>b</p>"#,
        expected_markdown: Some("a[<br>c](u)b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "links_case_37",
        html: r#"<p>a<a href="u"><br><br><br>c</a>b</p>"#,
        expected_markdown: Some("a[<br><br><br>c](u)b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "links_case_38",
        html: r#"<p>a<a href="u">c<br></a>b</p>"#,
        expected_markdown: Some("a[c<br>](u)b"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "links_case_39",
        html: r#"<p>a<a href="u">c<br><br><br></a>b</p>"#,
        expected_markdown: Some("a[c<br><br><br>](u)b"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [At the document root](../translation_approach.md#at-the-document-root).
/// The "At the document root" table.
#[test]
fn at_the_document_root() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "at_the_document_root_case_40",
        html: "<br>",
        expected_markdown: Some("<br>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "at_the_document_root_case_41",
        html: "<br><br><br>",
        expected_markdown: Some("<br><br><br>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "at_the_document_root_case_42",
        html: "<br><p>a</p>",
        expected_markdown: Some("<br>\n\na"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "at_the_document_root_case_43",
        html: "<p>a</p><br>",
        expected_markdown: Some("a\n\n<br>"),
        ..Case::default()
    }]);

    // `div` is a block-level tag name, so the element is a type 6 HTML block
    // and round-trips verbatim however many `<br>`s it holds.

    faithful_cases.extend([Case {
        id: "at_the_document_root_case_44",
        html: "<div><br></div>",
        expected_markdown: Some("<div><br></div>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "at_the_document_root_case_45",
        html: "<div><br><br><br></div>",
        expected_markdown: Some("<div><br><br><br></div>"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Headings](../translation_approach.md#headings).
/// Steps 3 and 4 of the "Headings" approach, for setext. A lone `<br>` would
/// open an HTML block where a setext heading leaves its content, so a heading
/// holding only one falls back to ATX.
#[test]
fn setext_headings() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "setext_headings_case_46_setext",
        html: "<h1><br></h1>",
        expected_markdown: Some("# <br>"),
        htmd_options: Options {
            heading_style: HeadingStyle::Setex,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "setext_headings_case_47_setext",
        html: "<h1><br><br><br></h1>",
        expected_markdown: Some("<br><br><br>\n============"),
        htmd_options: Options {
            heading_style: HeadingStyle::Setex,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "setext_headings_case_48_setext",
        html: "<h1>a #</h1>",
        expected_markdown: Some("a #\n==="),
        htmd_options: Options {
            heading_style: HeadingStyle::Setex,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "setext_headings_case_49_setext",
        html: "<h1>#</h1>",
        expected_markdown: Some("\\#\n=="),
        htmd_options: Options {
            heading_style: HeadingStyle::Setex,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Headings](../translation_approach.md#headings).
/// Step 3 of the "Headings" approach, for empty content. A level 1 or 2 heading
/// holding nothing has no setext encoding: the underline is empty too, so the
/// whole heading would disappear.
#[test]
fn an_empty_setext_heading_falls_back_to_atx() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "an_empty_setext_heading_falls_back_to_atx_case_50_setext",
        html: "<h1></h1>",
        expected_markdown: Some("#"),
        htmd_options: Options {
            heading_style: HeadingStyle::Setex,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "an_empty_setext_heading_falls_back_to_atx_case_51_setext",
        html: "<h2></h2>",
        expected_markdown: Some("##"),
        htmd_options: Options {
            heading_style: HeadingStyle::Setex,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "an_empty_setext_heading_falls_back_to_atx_case_52_setext",
        html: "<h1>   </h1>",
        expected_markdown: Some("#"),
        htmd_options: Options {
            heading_style: HeadingStyle::Setex,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);

    // The heading still has to survive whatever follows it.

    faithful_cases.extend([Case {
        id: "an_empty_setext_heading_falls_back_to_atx_case_53_setext",
        html: "<h1></h1><p>a</p>",
        expected_markdown: Some("#\n\na"),
        htmd_options: Options {
            heading_style: HeadingStyle::Setex,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Headings](../translation_approach.md#headings).
/// Step 4 of the "Headings" approach, for ATX. An ATX heading's `#` has already
/// opened the line, so a raw `<br>` is safe anywhere in it. A `#` run ending
/// the content would close the heading, so it is escaped: by step 4 in `a #`,
/// and by the generic text escape in `#`, whose run opens a text node. `basic_tests::atx_heading_of_a_hash_run_from_a_handler` covers
/// step 4 for content which is nothing but the run.
#[test]
fn atx_headings() {
    let mut faithful_cases = Vec::new();
    // An empty heading drops the space as well, which would otherwise be
    // trailing whitespace.

    faithful_cases.extend([Case {
        id: "atx_headings_case_54",
        html: "<h1></h1>",
        expected_markdown: Some("#"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "atx_headings_case_55",
        html: "<h1></h1><p>a</p>",
        expected_markdown: Some("#\n\na"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "atx_headings_case_56",
        html: "<h3></h3><p>a</p>",
        expected_markdown: Some("###\n\na"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "atx_headings_case_57",
        html: "<h1><br></h1>",
        expected_markdown: Some("# <br>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "atx_headings_case_58",
        html: "<h1><br><br><br></h1>",
        expected_markdown: Some("# <br><br><br>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "atx_headings_case_59",
        html: "<h1>a #</h1>",
        expected_markdown: Some(r"# a \#"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "atx_headings_case_60",
        html: "<h1>#</h1>",
        expected_markdown: Some(r"# \#"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Paragraphs](../translation_approach.md#paragraphs).
/// The "Paragraphs" table. Rows 1 and 2 are the section's rule that a paragraph
/// whose content would open an HTML block is serialized whole, since the block
/// would dissolve the `<p>` around it: a leading `<iframe>` opens one, as does
/// a bare `<br>`. A second tag keeps the line from being type 7, so row 3 needs
/// nothing. See also the second row of [`blockquotes`].
#[test]
fn paragraphs() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "paragraphs_case_61",
        html: r#"<p><iframe src="u">a</iframe></p>"#,
        expected_markdown: Some(r#"<p><iframe src="u">a</iframe></p>"#),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "paragraphs_case_62",
        html: "<p><br></p>",
        expected_markdown: Some("<p><br></p>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "paragraphs_case_63",
        html: "<p><br><br></p>",
        expected_markdown: Some("<br><br>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "paragraphs_case_64",
        html: "<p><br><br><br></p>",
        expected_markdown: Some("<br><br><br>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "paragraphs_case_65",
        html: "<p><br><em>b</em></p>",
        expected_markdown: Some("<br>*b*"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "paragraphs_case_66",
        html: "<p><em>a</em><br></p>",
        expected_markdown: Some("*a*<br>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "paragraphs_case_67",
        html: r#"<p><br><img src="i" alt=""></p>"#,
        expected_markdown: Some("<br>![](i)"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "paragraphs_case_68",
        html: r#"<p><img src="i" alt=""><br></p>"#,
        expected_markdown: Some("![](i)<br>"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Blockquotes](../translation_approach.md#blockquotes).
/// The "Blockquotes" table. Row 2 is the paragraph row one container down: the
/// `>` is stripped before the line is scanned, so the bare tag would re-open as
/// an HTML block exactly as it does at the root.
#[test]
fn blockquotes() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "blockquotes_case_69",
        html: "<blockquote><br></blockquote>",
        expected_markdown: Some("> <br>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "blockquotes_case_70",
        html: "<blockquote><br><br><br></blockquote>",
        expected_markdown: Some("> <br><br><br>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "blockquotes_case_71",
        html: "<blockquote><p><br></p></blockquote>",
        expected_markdown: Some("> <p><br></p>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "blockquotes_case_72",
        html: "<blockquote><p><br><em>b</em></p></blockquote>",
        expected_markdown: Some("> <br>*b*"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "blockquotes_case_73",
        html: "<blockquote><p><em>a</em><br></p></blockquote>",
        expected_markdown: Some("> *a*<br>"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Tables](../translation_approach.md#tables).
/// The "Tables" table. A cell's contents are parsed as inline content, so no
/// HTML block can open inside one.
#[test]
fn tables() {
    let rows = [
        ("<br><em>b</em>", "<br>*b*"),
        ("<em>a</em><br>", "*a*<br>"),
        ("<br>", "<br>"),
        ("<br><br><br>", "<br><br><br>"),
    ];
    let inputs = rows
        .iter()
        .map(|(cell, written)| {
            let html = format!(
                "<table><thead><tr><th>h</th></tr></thead>\
                 <tbody><tr><td>{cell}</td></tr></tbody></table>"
            );
            let markdown = format!(
                "| h{} |\n| {} |\n| {written} |",
                " ".repeat(written.len().saturating_sub(1)),
                "-".repeat(written.len())
            );
            (html, markdown)
        })
        .collect::<Vec<_>>();
    let cases = inputs
        .iter()
        .map(|(html, markdown)| Case {
            id: html,
            html,
            expected_markdown: Some(markdown),
            markdown_options: pulldown_cmark::Options::ENABLE_TABLES,
            ..Case::default()
        })
        .collect::<Vec<_>>();
    check_cases(&cases);
}

/// [Tables](../translation_approach.md#tables).
/// The "Tables" section's rule that the entire table is serialized when it
/// holds a child which could only be written as HTML. A cell carrying an
/// attribute is the case that rule names; a row, a section, a `<tfoot>` and a
/// `<colgroup>` are the same kind of child. `table::extract_table_content`
/// walks the table's own sections and rows rather than routing them through
/// `tr_handler` and `table_section_handler`, so it repeats their checks.
#[test]
fn table_children_which_only_html_can_express() {
    for html in [
        // The cell carrying an attribute, the case the rule names.
        "<table><thead><tr><th scope=\"col\">h</th></tr></thead>\
         <tbody><tr><td>a</td></tr></tbody></table>",
        // A body row, a header row, a `<thead>` and a `<tbody>` carrying one.
        "<table><thead><tr><th>h</th></tr></thead>\
         <tbody><tr id=\"r\"><td>a</td></tr></tbody></table>",
        "<table><thead><tr id=\"r\"><th>h</th></tr></thead>\
         <tbody><tr><td>a</td></tr></tbody></table>",
        "<table><thead id=\"t\"><tr><th>h</th></tr></thead>\
         <tbody><tr><td>a</td></tr></tbody></table>",
        "<table><thead><tr><th>h</th></tr></thead>\
         <tbody id=\"b\"><tr><td>a</td></tr></tbody></table>",
        // A GFM table has one body, so a `<tfoot>`'s rows could only join it,
        // losing the footer.
        "<table><thead><tr><th>h</th></tr></thead>\
         <tbody><tr><td>a</td></tr></tbody>\
         <tfoot><tr><td>f</td></tr></tfoot></table>",
        // A `<colgroup>` has no Markdown spelling at all.
        "<table><colgroup><col span=\"2\"></colgroup>\
         <thead><tr><th>h</th></tr></thead>\
         <tbody><tr><td>a</td></tr></tbody></table>",
    ] {
        assert_eq!(html, convert_faithful(html).unwrap());
        check_cases(&[Case {
            id: html,
            html,
            expected_markdown: Some(html),
            expectation: Some(HtmlExpectation::Preserved {
                renderer_suffix: "",
            }),
            ..Case::default()
        }]);
    }
}
