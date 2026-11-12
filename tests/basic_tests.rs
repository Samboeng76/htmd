use std::{sync::Arc, thread::JoinHandle};

use indoc::indoc;
use pretty_assertions::assert_eq;
use pulldown_cmark::Options as CommonMarkOptions;

use htmd::{
    Element, HtmlToMarkdown,
    element_handler::{ElementHandler, HandlerResult, Handlers},
    options::{HeadingStyle, LinkStyle, Options, TranslationMode},
};
mod common;
use common::{
    Case, DifferenceKind, HtmlExpectation, StabilityExpectation, check_cases, convert_faithful,
    convert_pure, render_markdown,
};

/// [Headings](../translation_approach.md#headings) and
/// [Paragraphs](../translation_approach.md#paragraphs) retain their block structure.
#[test]
fn compact_headings_and_paragraphs_round_trip() {
    check_cases(&[
        Case {
            id: "compact_heading",
            html: "<h1>Heading 1</h1>",
            expected_markdown: Some("# Heading 1"),
            expectation: Some(HtmlExpectation::Preserved {
                renderer_suffix: "\n",
            }),
            ..Case::default()
        },
        Case {
            id: "paragraphs_with_renderer_layout",
            html: "<p>The first.</p>\n<p>The <span>second.</span></p>",
            expected_markdown: Some("The first.\n\nThe <span>second.</span>"),
            expectation: Some(HtmlExpectation::Preserved {
                renderer_suffix: "\n",
            }),
            ..Case::default()
        },
        Case {
            id: "consecutive_paragraphs_with_renderer_layout",
            html: "<p>One</p>\n<p>Two</p>",
            expected_markdown: Some("One\n\nTwo"),
            expectation: Some(HtmlExpectation::Preserved {
                renderer_suffix: "\n",
            }),
            ..Case::default()
        },
    ]);
}

/// [Paragraphs](../translation_approach.md#paragraphs).
#[test]
fn html_source_representation_changes() {
    check_cases(&[
        Case {
            id: "paragraph_crlf_after_closing_tag",
            html: "<p>Hello</p>\r\n",
            expectation: Some(HtmlExpectation::DocumentedDifference {
                kind: DifferenceKind::Representation,
                expected_html: "<p>Hello</p>\n",
                reason: "HTML input preprocessing normalizes CRLF to LF before parsing",
                reference: "https://html.spec.whatwg.org/multipage/parsing.html#preprocessing-the-input-stream",
            }),
            ..Case::default()
        },
        Case {
            id: "paragraph_numeric_character_reference",
            html: "<p>&#65;</p>",
            expectation: Some(HtmlExpectation::DocumentedDifference {
                kind: DifferenceKind::Representation,
                expected_html: "<p>A</p>\n",
                reason: "the HTML parser decodes the numeric character reference to the same text value",
                reference: "https://html.spec.whatwg.org/multipage/parsing.html#numeric-character-reference-state",
            }),
            ..Case::default()
        },
    ]);
}

/// [Paragraphs](../translation_approach.md#paragraphs) with attributes use raw HTML.
#[test]
fn paragraph_with_attribute_round_trips_as_html() {
    check_cases(&[Case {
        id: "paragraph_with_dir_attribute",
        html: "<p dir=\"ltr\">Test 1</p>",
        expected_markdown: Some("<p dir=\"ltr\">Test 1</p>"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "",
        }),
        ..Case::default()
    }]);
}

/// [Links](../translation_approach.md#links).
#[test]
fn links_with_spaces() {
    let html = r#"
        <a href="https://example.com/Some Page.html">Example</a>
        "#;
    assert_eq!(
        "[Example](<https://example.com/Some Page.html>)",
        convert_faithful(html).unwrap(),
    );
    check_cases(&[Case {
        id: "links_with_spaces_case_1",
        html,
        expected_markdown: Some("[Example](<https://example.com/Some Page.html>)"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "[Example](https://example.com/Some%20Page.html)",
            reason: "pulldown-cmark percent-encodes the space in the link destination",
            reference: "translation_approach.md#links",
        },
        ..Case::default()
    }]);
}

/// [Links](../translation_approach.md#links).
#[test]
fn referenced_links_with_title() {
    let html = r#"
        <a href="https://example.com" title="Some title">Example</a>
        "#;
    let md = HtmlToMarkdown::builder()
        .options(Options {
            link_style: LinkStyle::Referenced,
            translation_mode: TranslationMode::Faithful,
            ..Default::default()
        })
        .build()
        .convert(html)
        .unwrap();
    assert_eq!(
        "[Example][1]\n\n[1]: https://example.com \"Some title\"",
        &md
    );
    check_cases(&[Case {
        id: "referenced_links_with_title",
        html,
        htmd_options: Options {
            link_style: LinkStyle::Referenced,
            ..Case::default().htmd_options
        },
        expected_markdown: Some("[Example][1]\n\n[1]: https://example.com \"Some title\""),
        ..Case::default()
    }]);
}

/// [Links](../translation_approach.md#links).
#[test]
fn consecutive_referenced_links_with_title() {
    let html = r#"
        <a href="https://example.com" title="Some title">Example</a><a href="https://example.com" title="Some title">Another example</a>
        "#;
    let md = HtmlToMarkdown::builder()
        .options(Options {
            link_style: LinkStyle::Referenced,
            translation_mode: TranslationMode::Faithful,
            ..Default::default()
        })
        .build()
        .convert(html)
        .unwrap();
    assert_eq!(
        indoc!(
            r#"
        [Example][1][Another example][2]

        [1]: https://example.com "Some title"
        [2]: https://example.com "Some title""#
        ),
        &md
    );
    check_cases(&[Case {
        id: "consecutive_referenced_links_with_title",
        html,
        htmd_options: Options {
            link_style: LinkStyle::Referenced,
            ..Case::default().htmd_options
        },
        expected_markdown: Some(indoc!(
            r#"[Example][1][Another example][2]

[1]: https://example.com "Some title"
[2]: https://example.com "Some title""#
        )),
        ..Case::default()
    }]);
}

/// [Images](../translation_approach.md#images).
#[test]
fn images() {
    let html = r#"
        <img src="https://example.com" alt="" />
        <img src="https://example.com" alt="Image 1" />
        <img src="https://example.com" alt="Image 2" title="Hello" />
        "#;
    assert_eq!(
        "![](https://example.com) ![Image 1](https://example.com) \
            ![Image 2](https://example.com \"Hello\")",
        convert_faithful(html).unwrap(),
    );
    check_cases(&[Case {
        id: "images_case_2",
        html,
        expected_markdown: Some(
            "![](https://example.com) ![Image 1](https://example.com) \
            ![Image 2](https://example.com \"Hello\")",
        ),
        ..Case::default()
    }]);
}

/// [Images](../translation_approach.md#images).
#[test]
fn images_with_spaces_in_url() {
    let html = r#"
        <img src="https://example.com/Some Image.jpg" alt="" />
        "#;
    assert_eq!(
        "![](<https://example.com/Some Image.jpg>)",
        convert_faithful(html).unwrap(),
    );
    check_cases(&[Case {
        id: "images_with_spaces_in_url_case_3",
        html,
        expected_markdown: Some("![](<https://example.com/Some Image.jpg>)"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "![](https://example.com/Some%20Image.jpg)",
            reason: "pulldown-cmark percent-encodes the space in the image source",
            reference: "translation_approach.md#images",
        },
        ..Case::default()
    }]);
}

#[test]
fn image_title_stays_outside_an_angle_bracket_destination() {
    let markdown = htmd::convert(
        r#"<img src="https://example.com/image name.png" alt="diagram" title="A title">"#,
    )
    .unwrap();

    assert_eq!(
        r#"![diagram](<https://example.com/image name.png> "A title")"#,
        markdown
    );
}

/// [Headings](../translation_approach.md#headings).
#[test]
fn headings() {
    let html = r#"
        <h1>Heading 1</h1>
        <h2>Heading 2</h2>
        <h3>Heading 3</h3>
        <h4>Heading 4</h4>
        <h5>Heading 5</h5>
        <h6>Heading 6</h6>
        "#;

    check_cases(&[Case {
        id: "headings_case_4",
        html,
        expected_markdown: Some(
            "# Heading 1\n\n## Heading 2\n\n### Heading 3\n\n\
             #### Heading 4\n\n##### Heading 5\n\n###### Heading 6",
        ),
        ..Case::default()
    }]);
}

/// Step 1 of the headings approach in `translation_approach.md`: an inline
/// context holds no block, so pure mode writes only a heading's content there.
/// Faithful mode writes a raw HTML inline instead; see `context_tests.rs`.
#[test]
fn pure_heading_in_an_inline_context() {
    assert_eq!("*b*", convert_pure("<em><h2>b</h2></em>").unwrap());
    assert_eq!(
        "[b](u)",
        convert_pure(r#"<a href="u"><h2>b</h2></a>"#).unwrap()
    );
    assert_eq!(
        "| b |\n| - |",
        convert_pure("<table><thead><tr><th><h2>b</h2></th></tr></thead></table>").unwrap()
    );
    // The content is trimmed as it is in a block context. Its `#` run is
    // escaped by the enclosing heading, the only one spelled as ATX.
    assert_eq!(
        r"# a \#",
        convert_pure("<h1><div><h2> a # </h2></div></h1>").unwrap()
    );
}

/// [Headings](../translation_approach.md#headings).
/// Step 2 of the headings approach: only document whitespace is trimmed from
/// a heading's content, so a non-breaking space is content.
#[test]
fn heading_content_trims_only_document_whitespace() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "heading_content_trims_only_document_whitespace_case_5",
        html: "<h1> \t\r\n a \t\r\n </h1>",
        expected_markdown: Some("# a"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "heading_content_trims_only_document_whitespace_case_6",
        html: "<h1>&nbsp;</h1>",
        expected_markdown: Some("# \u{a0}"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "heading_content_trims_only_document_whitespace_case_7_setext",
        html: "<h1>&nbsp;</h1>",
        expected_markdown: Some("\u{a0}\n="),
        htmd_options: Options {
            heading_style: HeadingStyle::Setex,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Headings](../translation_approach.md#headings).
/// Step 4 of the headings approach: a setext underline has one character per
/// Unicode scalar value of the content as written, escapes and tags included.
/// Levels 3-6 have no setext spelling.
#[test]
fn setext_underline() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "setext_underline_case_8_setext",
        html: "<h2>a</h2>",
        expected_markdown: Some("a\n-"),
        htmd_options: Options {
            heading_style: HeadingStyle::Setex,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "setext_underline_case_9_setext",
        html: "<h1>é</h1>",
        expected_markdown: Some("é\n="),
        htmd_options: Options {
            heading_style: HeadingStyle::Setex,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "setext_underline_case_10_setext",
        html: "<h1>a*b</h1>",
        expected_markdown: Some("a\\*b\n===="),
        htmd_options: Options {
            heading_style: HeadingStyle::Setex,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "setext_underline_case_11_setext",
        html: "<h1><br><em>b</em></h1>",
        expected_markdown: Some("<br>*b*\n======="),
        htmd_options: Options {
            heading_style: HeadingStyle::Setex,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "setext_underline_case_12_setext",
        html: "<h1><em>a</em><br></h1>",
        expected_markdown: Some("*a*<br>\n======="),
        htmd_options: Options {
            heading_style: HeadingStyle::Setex,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "setext_underline_case_13_setext",
        html: "<h3>a</h3>",
        expected_markdown: Some("### a"),
        htmd_options: Options {
            heading_style: HeadingStyle::Setex,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Paragraphs](../translation_approach.md#paragraphs).
#[test]
fn paragraphs() {
    let html = r#"
        <p>The first.</p>
        <p>The <span>second.</span></p>
        "#;

    check_cases(&[Case {
        id: "paragraphs_case_14",
        html,
        expected_markdown: Some("The first.\n\nThe <span>second.</span>"),
        ..Case::default()
    }]);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn quotes() {
    let html = r#"
        <blockquote>Once upon a time</blockquote>
        "#;

    check_cases(&[Case {
        id: "quotes_case_15",
        html,
        expected_markdown: Some("> Once upon a time"),
        ..Case::default()
    }]);
}

/// [At the document root](../translation_approach.md#at-the-document-root).
#[test]
fn br() {
    let html = r#"
        <p>Hi<br>there<br><br>!</p>"#;

    check_cases(&[Case {
        id: "br_case_16",
        html,
        expected_markdown: Some("Hi<br>there<br><br>!"),
        ..Case::default()
    }]);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn hr() {
    let html = r#"Hi <hr/> there"#;

    check_cases(&[Case {
        id: "hr_case_17",
        html,
        expected_markdown: Some("Hi\n\n* * *\n\nthere"),
        ..Case::default()
    }]);
}

/// [Emphasis and strong emphasis](../translation_approach.md#emphasis-and-strong-emphasis).
#[test]
fn strong_italic() {
    let html = r#"<i>Italic</i><em>Also italic</em><strong>Strong</strong><b>Stronger</b>"#;

    check_cases(&[Case {
        id: "strong_italic_case_18",
        html,
        expected_markdown: Some(
            "<i>Italic</i><em>Also italic</em><strong>Strong</strong><b>Stronger</b>",
        ),
        ..Case::default()
    }]);
    assert_eq!(
        "*ItalicAlso italic***StrongStronger**",
        convert_pure(html).unwrap()
    );
}

/// [Emphasis and strong emphasis](../translation_approach.md#emphasis-and-strong-emphasis).
/// A pair of adjacent emphases is merged so that it does not go out as
/// `*a**b*`, which CommonMark reads as one emphasis holding `a**b`. Merging is
/// a pure-mode spelling, though: it writes two elements as one, which faithful
/// mode owes the reader as HTML instead.
#[test]
fn faithful_mode_keeps_adjacent_emphasis_apart() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "faithful_mode_keeps_adjacent_emphasis_apart_case_19",
        html: "<p><em>a</em><em>b</em></p>",
        expected_markdown: Some("<em>a</em><em>b</em>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "faithful_mode_keeps_adjacent_emphasis_apart_case_20",
        html: "<p><i>a</i><em>b</em></p>",
        expected_markdown: Some("<i>a</i><em>b</em>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "faithful_mode_keeps_adjacent_emphasis_apart_case_21",
        html: "<p><b>a</b><strong>b</strong></p>",
        expected_markdown: Some("<b>a</b><strong>b</strong>"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Code blocks and code spans](../translation_approach.md#code-blocks-and-code-spans).
/// Writing the two code spans one after the other is no answer either: the
/// backtick strings of ``` `a``b` ``` pair outside in, making one span whose
/// content is ``` a``b ```. Faithful mode writes both as HTML.
#[test]
fn faithful_mode_keeps_adjacent_code_spans_apart() {
    check_cases(&[Case {
        id: "faithful_mode_keeps_adjacent_code_spans_apart_case_22",
        html: "<p><code>a</code><code>b</code></p>",
        expected_markdown: Some("<code>a</code><code>b</code>"),
        ..Case::default()
    }]);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// An element CommonMark cannot spell is already written as a raw HTML inline,
/// so merging a pair of them buys nothing and loses an element.
#[test]
fn faithful_mode_keeps_adjacent_untranslated_elements_apart() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "faithful_mode_keeps_adjacent_untranslated_elements_apart_case_23",
        html: "<p><sub>a</sub><sub>b</sub></p>",
        expected_markdown: Some("<sub>a</sub><sub>b</sub>"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "faithful_mode_keeps_adjacent_untranslated_elements_apart_case_24",
        html: r#"<p><abbr title="t">a</abbr><abbr title="t">b</abbr></p>"#,
        expected_markdown: Some(r#"<abbr title="t">a</abbr><abbr title="t">b</abbr>"#),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "faithful_mode_keeps_adjacent_untranslated_elements_apart_case_25",
        html: r#"<p><span class="x">a</span><span class="x">b</span></p>"#,
        expected_markdown: Some(r#"<span class="x">a</span><span class="x">b</span>"#),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// Pure mode keeps the merge: it has no HTML to fall back on.
#[test]
fn pure_mode_merges_adjacent_inline_elements() {
    assert_eq!("*ab*", convert_pure("<p><em>a</em><em>b</em></p>").unwrap());
    assert_eq!(
        "ab",
        convert_pure("<p><sub>a</sub><sub>b</sub></p>").unwrap()
    );
}

/// [Math](../translation_approach.md#math).
/// pulldown-cmark reads `$a$$b$` as two math expressions, so adjacent math
/// spans, unlike other runs of inline elements, are neither merged nor written
/// as HTML.
#[test]
fn adjacent_math_spans_stay_math() {
    for (html, markdown) in [
        (
            r#"<p><span class="math math-inline">a</span><span class="math math-inline">b</span></p>"#,
            "$a$$b$",
        ),
        (
            r#"<p><span class="math math-display">a</span><span class="math math-display">b</span></p>"#,
            "$$a$$$$b$$",
        ),
        (
            r#"<p><span class="math math-inline">a</span><span class="math math-display">b</span></p>"#,
            "$a$$$b$$",
        ),
        (
            r#"<p><span class="math math-display">a</span><span class="math math-inline">b</span></p>"#,
            "$$a$$$b$",
        ),
    ] {
        check_cases(&[Case {
            id: "adjacent_math_spans_stay_math_case_26",
            html,
            expected_markdown: Some(markdown),
            markdown_options: CommonMarkOptions::ENABLE_MATH,
            ..Case::default()
        }]);
        assert_eq!(markdown, convert_pure(html).unwrap());
        assert_eq!(
            format!("{html}\n"),
            render_markdown(markdown, CommonMarkOptions::ENABLE_MATH)
        );
    }
}

// Faithful mode keeps an emphasis with nothing to emphasize as HTML; see rows 5
// and 6 of the "Emphasis and strong emphasis" table of
// `translation_approach.md`. Pure mode has no such spelling, so the element
// goes and only the whitespace it held survives, still separating the words
// around it.
#[test]
fn blank_emphasis_leaves_its_whitespace_behind() {
    assert_eq!("a b", convert_pure("<p>a<em> </em>b</p>").unwrap());
    assert_eq!("a b", convert_pure("<p>a<strong> </strong>b</p>").unwrap());
    assert_eq!(
        "a\u{a0}b",
        convert_pure("<p>a<em>&#160;</em>b</p>").unwrap()
    );
    assert_eq!("ab", convert_pure("<p>a<em></em>b</p>").unwrap());
}

/// [Emphasis and strong emphasis](../translation_approach.md#emphasis-and-strong-emphasis).
#[test]
fn italic_inside_word() {
    let html = r#"It<i>al</i>ic St<b>ro</b>ng"#;

    check_cases(&[Case {
        id: "italic_inside_word_case_27",
        html,
        expected_markdown: Some("It*al*ic St**ro**ng"),
        ..Case::default()
    }]);
}

/// [Emphasis and strong emphasis](../translation_approach.md#emphasis-and-strong-emphasis).
#[test]
fn italic_with_br() {
    let html = "a<i>b<br></i>c d<b><br>e</b>f";

    check_cases(&[Case {
        id: "italic_with_br_case_28",
        html,
        expected_markdown: Some("a<i>b<br></i>c d<b><br>e</b>f"),
        ..Case::default()
    }]);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn inline_raw_html_escaping() {
    let html = r#"Test &lt;code&gt;tags&lt;/code&gt;, &lt;!-- comments --&gt;, &lt;?processing instructions?&gt;, &lt;!A declaration&gt;, and &lt;![CDATA[character data]]&gt;."#;

    check_cases(&[Case {
        id: "inline_raw_html_escaping_case_29",
        html,
        expected_markdown: Some(
            r#"Test \<code>tags\</code>, \<!-- comments -->, \<?processing instructions?>, \<!A declaration>, and <!\[CDATA\[character data\]\]>."#,
        ),
        ..Case::default()
    }]);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn multiline_raw_html_escaping() {
    let html = indoc!(
        r#"
    Test &lt;code&gt;multi-line
    tags&lt;/code&gt;, &lt;!-- multi-line
    comments --&gt;, &lt;?multi-line
    processing instructions?&gt;, &lt;!A multi-line
    declaration&gt;, and &lt;![CDATA[multi-line
    character data]]&gt;.
    "#
    );

    check_cases(&[Case {
        id: "multiline_raw_html_escaping_case_30",
        html,
        expected_markdown: Some(
            r#"Test \<code>multi-line tags\</code>, \<!-- multi-line comments -->, \<?multi-line processing instructions?>, \<!A multi-line declaration>, and <!\[CDATA\[multi-line character data\]\]>."#,
        ),
        ..Case::default()
    }]);
}

/// [Links](../translation_approach.md#links).
/// An [email autolink](https://spec.commonmark.org/0.31.2/#email-autolink)
/// needs the escape a tag gets, though its `<` may be followed by a digit, a
/// `/` or a symbol rather than a letter.
#[test]
fn inline_email_autolink_escaping() {
    let html = "<p>Mail &lt;1foo@bar.com&gt;, &lt;/1@b.co&gt; or &lt;+a@b.co&gt;.</p>";

    check_cases(&[Case {
        id: "inline_email_autolink_escaping_case_31",
        html,
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        expected_markdown: Some(r"Mail \<1foo@bar.com>, \</1@b.co> or \<+a@b.co>."),
        ..Case::default()
    }]);
}

/// Pure mode writes a `span`, an `a` without an `href` and an empty `em` as
/// their bare content, so the text node after one continues the text node
/// before it. A `<` opening a tag or an autolink which the first ends partway
/// through is escaped, since that node cannot see whether the rest follows.
/// Unescaped, each of these reads back as raw HTML or a link instead of the
/// text it was.
#[test]
fn a_sequence_finished_by_a_sibling_stays_text() {
    for (html, expected_markdown, expected_content) in [
        (
            "<p>&lt;<span>div</span> class&gt;x</p>",
            r"\<div class>x",
            "&lt;div class&gt;x",
        ),
        (
            "<p>&lt;<a>!--</a> x --&gt; y</p>",
            r"\<!-- x --> y",
            "&lt;!-- x --&gt; y",
        ),
        ("<p>a&lt;<em></em>b&gt;c</p>", r"a\<b>c", "a&lt;b&gt;c"),
        ("<p>&lt;/<span>p</span> &gt;</p>", r"\</p >", "&lt;/p &gt;"),
        (
            "<p>&lt;<span>http://x.y</span>&gt;</p>",
            r"\<http://x.y\>",
            "&lt;http://x.y&gt;",
        ),
        (
            "<p>&lt;1<span>@b.co&gt;</span></p>",
            r"\<1@b.co>",
            "&lt;1@b.co&gt;",
        ),
    ] {
        let markdown = convert_pure(html).unwrap();
        assert_eq!(
            format!("<p>{expected_content}</p>\n"),
            render_markdown(&markdown, CommonMarkOptions::empty()),
            "round trip of {html}"
        );
        assert_eq!(expected_markdown, markdown, "markdown for {html}");
    }
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn html_escaping() {
    let html = indoc!(
        r#"
        <p>&lt;pre</p>
        <p>&lt;script</p>
        <p>&lt;style</p>
        <p>&lt;textarea</p>
        <p>&lt;address</p>
        <p>&lt;ul</p>
        "#
    );

    check_cases(&[Case {
        id: "html_escaping_case_32",
        html,
        expected_markdown: Some(indoc!(
            r#"\<pre

            \<script

            \<style

            \<textarea

            \<address

            \<ul"#,
        )),
        ..Case::default()
    }]);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn faithful_mode_inline() {
    assert_eq!(
        convert_faithful(indoc!(
            r#"<p>
                <img src="one.png" alt="yyy" title="zzz" scale="50%">
                <em bar>Testing</em>
                <strong foo>Testing</strong>
                <a href="http://foo.com" bar>link</a>
                <code class="not-a-language">code</code>
                <br foo>
            </p>"#
        ))
        .unwrap(),
        indoc!(
            r#"<img src="one.png" alt="yyy" title="zzz" scale="50%"> <em bar="">Testing</em> <strong foo="">Testing</strong> <a href="http://foo.com" bar="">link</a> <code class="not-a-language">code</code> <br foo="">"#
        )
    );
    check_cases(&[Case {
        id: "faithful_mode_inline_case_33",
        html: indoc!(
            r#"<p>
                <img src="one.png" alt="yyy" title="zzz" scale="50%">
                <em bar>Testing</em>
                <strong foo>Testing</strong>
                <a href="http://foo.com" bar>link</a>
                <code class="not-a-language">code</code>
                <br foo>
            </p>"#
        ),
        expected_markdown: Some(
            r#"<img src="one.png" alt="yyy" title="zzz" scale="50%"> <em bar="">Testing</em> <strong foo="">Testing</strong> <a href="http://foo.com" bar="">link</a> <code class="not-a-language">code</code> <br foo="">"#,
        ),
        ..Case::default()
    }]);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn faithful_mode_hr() {
    check_cases(&[Case {
        id: "faithful_mode_hr_case_34",
        html: indoc!(r#"<hr bar>"#),
        expected_markdown: Some(r#"<hr bar="">"#),
        ..Case::default()
    }]);
}

/// [Blockquotes](../translation_approach.md#blockquotes).
#[test]
fn faithful_mode_blockquote() {
    check_cases(&[Case {
        id: "faithful_mode_blockquote_case_35",
        html: indoc!(
            r#"<blockquote style="foo">
            <em>Testing</em>

            <blockquote>Nested</blockquote>
        </blockquote>"#
        ),
        expected_markdown: Some(indoc!(
            r#"<blockquote style="foo">
                <em>Testing</em>
            &#10;    <blockquote>Nested</blockquote>
            </blockquote>"#,
        )),
        ..Case::default()
    }]);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn faithful_mode_h1() {
    check_cases(&[Case {
        id: "faithful_mode_h1_case_36",
        html: indoc!(r#"<h1 class="foo">Heading</h1>"#),
        expected_markdown: Some(r#"<h1 class="foo">Heading</h1>"#),
        ..Case::default()
    }]);
}

/// [Headings](../translation_approach.md#headings).
/// Step 1 of the headings approach in `translation_approach.md`: CommonMark has
/// no syntax for a heading's attributes, so faithful mode serializes a heading
/// holding any, even one with an empty value. Pure mode drops them.
#[test]
fn heading_attributes() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "heading_attributes_case_37",
        html: r#"<h1 class="">a</h1>"#,
        expected_markdown: Some(r#"<h1 class="">a</h1>"#),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "heading_attributes_case_38",
        html: "<h2 id>a</h2>",
        expected_markdown: Some(r#"<h2 id="">a</h2>"#),
        ..Case::default()
    }]);
    assert_eq!("# a", convert_pure(r#"<h1 class="x">a</h1>"#).unwrap());
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn faithful_mode_p() {
    check_cases(&[Case {
        id: "faithful_mode_p_case_39",
        html: indoc!(r#"<p dir="ltr">Test 1</p>"#),
        expected_markdown: Some(r#"<p dir="ltr">Test 1</p>"#),
        ..Case::default()
    }]);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn faithful_mode_ol1() {
    check_cases(&[Case {
        id: "faithful_mode_ol1_case_40",
        html: indoc!(
            r#"<ol>
            <li>Test 1</li>
            <li foo>Test 2</li>
            <li>Test 3</li>
        </ol>"#
        ),
        expected_markdown: Some(indoc!(
            r#"<ol>
                <li>Test 1</li>
                <li foo="">Test 2</li>
                <li>Test 3</li>
            </ol>"#,
        )),
        ..Case::default()
    }]);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn faithful_mode_ol2() {
    check_cases(&[Case {
        id: "faithful_mode_ol2_case_41",
        html: indoc!(
            r#"<ol foo>
            <li>Test</li>
        </ol>"#
        ),
        expected_markdown: Some(indoc!(
            r#"<ol foo="">
                <li>Test</li>
            </ol>"#,
        )),
        ..Case::default()
    }]);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn faithful_mode_comment() {
    check_cases(&[Case {
        id: "faithful_mode_comment_case_42",
        html: indoc!(r#"<!-- Test -->"#),
        expected_markdown: Some(r#"<!-- Test -->"#),
        ..Case::default()
    }]);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn faithful_mode_html() {
    let html = indoc!(
        r#"<details>
            <summary>Test

                1</summary>
            Test 2
        </details>"#
    );
    let md = convert_faithful(html).unwrap();
    assert_eq!(
        indoc!(
            r#"<details>
                <summary>Test
            &#10;        1</summary>
                Test 2
            </details>"#
        ),
        md
    );
    check_cases(&[Case {
        id: "faithful_mode_html",
        html,
        expected_markdown: Some(indoc!(
            r#"<details>
                <summary>Test
            &#10;        1</summary>
                Test 2
            </details>"#
        )),
        ..Case::default()
    }]);
}

/// [Tables](../translation_approach.md#tables).
#[test]
fn faithful_mode_table() {
    check_cases(&[Case {
        id: "faithful_mode_table_case_43",
        html: indoc!(
            r#"<table>
            <tr>
                <th>Header 1</th>
                <th>Header 2</th>
            </tr>
            <tr>
                <td foo>Cell 1</td>
                <td>Cell 2</td>
            </tr>
            <tr>
                <td>Cell 3</td>
                <td>Cell 4</td>
            </tr>
        </table>
"#
        ),
        expected_markdown: Some(indoc!(
            r#"<table>
            <tbody><tr>
                <th>Header 1</th>
                <th>Header 2</th>
            </tr>
            <tr>
                <td foo="">Cell 1</td>
                <td>Cell 2</td>
            </tr>
            <tr>
                <td>Cell 3</td>
                <td>Cell 4</td>
            </tr>
        </tbody></table>"#,
        )),
        markdown_options: CommonMarkOptions::ENABLE_TABLES,
        ..Case::default()
    }]);
}

/// [Tables](../translation_approach.md#tables).
/// CommonMark has no caption, so faithful mode always writes the element as
/// HTML — and a `<caption>` is only valid inside a `<table>`, so the whole table
/// is serialized around it.
#[test]
fn faithful_mode_serializes_a_table_with_a_caption() {
    let mut faithful_cases = Vec::new();
    let html = concat!(
        r#"<table><caption><span class="label">Caption</span></caption>"#,
        "<tr><th>Header</th></tr><tr><td>Cell</td></tr></table>"
    );
    // html5ever inserts the `<tbody>` around the rows.
    let expected = concat!(
        r#"<table><caption><span class="label">Caption</span></caption>"#,
        "<tbody><tr><th>Header</th></tr><tr><td>Cell</td></tr></tbody></table>"
    );

    faithful_cases.extend([Case {
        id: "faithful_mode_serializes_a_table_with_a_caption_case_44",
        html,
        expected_markdown: Some(expected),
        ..Case::default()
    }]);
    // The point of serializing the whole table: the `<caption>` comes back as
    // an element rather than as the bare text an HTML block outside the table
    // would give. The block is the whole document here, so pulldown-cmark
    // writes it without a trailing line ending.

    faithful_cases.extend([Case {
        id: "faithful_mode_serializes_a_table_with_a_caption_preserved_line_1150",
        html: expected,
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "",
        }),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// Pure mode has no HTML to fall back on, so it keeps writing the caption as a
/// paragraph above the table.
#[test]
fn pure_mode_writes_a_caption_as_a_paragraph() {
    let html = concat!(
        "<table><caption>Caption</caption>",
        "<tr><th>Header</th></tr><tr><td>Cell</td></tr></table>"
    );

    assert_eq!(
        "Caption\n\n| Header |\n| ------ |\n| Cell   |",
        htmd::convert(html).unwrap()
    );
}

/// The blank line the caption is followed by is what keeps the table a block of
/// its own. A caption which translates to a list or a blockquote opens a block
/// which the rows below it would otherwise join as lazy continuation lines,
/// leaving the table dissolved into that block's text.
#[test]
fn pure_mode_separates_a_caption_from_the_table_it_sits_above() {
    for caption in ["<ul><li>a</li></ul>", "<blockquote>a</blockquote>"] {
        let html = format!(
            "<table><caption>{caption}</caption>\
             <tr><th>Header</th></tr><tr><td>Cell</td></tr></table>"
        );
        let markdown = htmd::convert(&html).unwrap();

        assert!(
            markdown.contains("\n\n| Header |"),
            "the caption ran into the table: {markdown:?}"
        );
        assert!(round_trip_pure_table(&html).contains("<table>"));
    }
}

/// [`round_trip`] in pure mode and with the
/// [GFM table extension](https://github.github.com/gfm/#tables-extension-),
/// which a Markdown table needs to be read back as a table at all.
fn round_trip_pure_table(html: &str) -> String {
    render_markdown(
        &htmd::convert(html).unwrap(),
        CommonMarkOptions::ENABLE_TABLES,
    )
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn faithful_mode_nested_inline_html() {
    check_cases(&[Case {
        id: "faithful_mode_nested_inline_html_case_45",
        html: "<p>Nested <foo><bar><em>content</em></bar></foo></p>",
        expected_markdown: Some("Nested <foo><bar>*content*</bar></foo>"),
        ..Case::default()
    }]);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn spaces_check() {
    let html = r#"<i>Italic</i> <em>Also italic</em>  <strong>Strong</strong> <b>Stronger </b>"#;

    check_cases(&[Case {
        id: "spaces_check_case_46",
        html,
        expected_markdown: Some("*Italic* *Also italic* **Strong** **Stronger**"),
        ..Case::default()
    }]);
}

/// [Paragraphs](../translation_approach.md#paragraphs).
#[test]
fn consecutive_blocks() {
    let html = r#"<p>One</p><p>Two</p>"#;

    check_cases(&[Case {
        id: "consecutive_blocks_case_47",
        html,
        expected_markdown: Some("One\n\nTwo"),
        ..Case::default()
    }]);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn raw_text() {
    let html = r#"Hello world!"#;

    check_cases(&[Case {
        id: "raw_text_case_48",
        html,
        expected_markdown: Some("Hello world!"),
        ..Case::default()
    }]);
}

#[test]
fn nested_divs() {
    let html = r#"
    <div>
        <div>
            <div>Hi</div>
        </div>
        <div></div>
        <div>there</div>
    </div>
    "#;
    assert_eq!("Hi\n\nthere", htmd::convert(html).unwrap());
}

#[test]
fn with_head() {
    let html = r#"
    <html>
        <head>
            <title>Demo</title>
            <script>console.log('Hello');</script>
            <style>body {}</style>
        </head>
        <body>
            Content
        </body>
    </html>
    "#;
    assert_eq!(
        "Demo\n\nconsole.log('Hello');\n\nbody {}\n\nContent",
        htmd::convert(html).unwrap()
    );
}

#[test]
fn with_custom_rules() {
    // Remove element
    let html = r#"<img src="https://example.com"/>"#;
    let md = HtmlToMarkdown::builder()
        .add_handler(vec!["img"], |_: &dyn Handlers, _element: &Element| None)
        .build()
        .convert(html)
        .unwrap();
    assert_eq!("", &md);
}

#[test]
fn with_custom_rules_and_fallback() {
    let html = r#"<img src="https://example.com"/>"#;
    let converter = HtmlToMarkdown::builder()
        .add_handler(vec!["img"], |handlers: &dyn Handlers, element: &Element| {
            if element
                .attrs
                .iter()
                .any(|attr| &attr.name.local == "id" && attr.value.as_ref() == "do_not_skip_me")
            {
                handlers.fallback(element)
            } else {
                None
            }
        })
        .options(Options {
            ..Default::default()
        })
        .build();
    assert_eq!("", &converter.convert(html).unwrap());

    let html = r#"<img src="https://example.com" id="do_not_skip_me"/>"#;
    assert_eq!(
        "![](https://example.com)",
        &converter.convert(html).unwrap()
    );
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn upper_case_tags() {
    let html = r#"<H1>Hello</H1> <P>World</P>"#;

    check_cases(&[Case {
        id: "upper_case_tags_case_49",
        html,
        expected_markdown: Some("# Hello\n\nWorld"),
        ..Case::default()
    }]);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// Every `&` is escaped, since one which begins an
/// [entity or numeric character reference](https://spec.commonmark.org/0.31.2/#entity-and-numeric-character-references)
/// would otherwise be decoded.
#[test]
fn html_entities() {
    let mut faithful_cases = Vec::new();
    let html = r#"<p><a href="/my%20&amp;uri" title="my%20&amp;title">my%20&amp;link</a></p>"#;

    faithful_cases.extend([Case {
        id: "html_entities_case_50",
        html,
        expected_markdown: Some(r#"[my%20\&link](/my%20\&uri "my%20\&title")"#),
        ..Case::default()
    }]);

    let html_plain = r#"<p>This &amp; that, then &lt; &gt; now.</p>"#;

    faithful_cases.extend([Case {
        id: "html_entities_case_51",
        html: html_plain,
        expected_markdown: Some(r#"This \& that, then < > now."#),
        ..Case::default()
    }]);

    // `&amp;copy;` is the text `&copy;`, not the character it names.
    faithful_cases.extend([Case {
        id: "html_entities_preserved_1",
        html: "<p>&amp;copy; &amp;#169; &amp;#xA9;</p>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

#[test]
fn scripting_option() {
    let html = r#"<noscript><p>Hello</p></noscript>"#;
    let md = HtmlToMarkdown::builder()
        .scripting_enabled(true)
        .build()
        .convert(html)
        .unwrap();
    assert_eq!(r#"\<p>Hello\</p>"#, md);

    let md = HtmlToMarkdown::builder()
        .scripting_enabled(false)
        .build()
        .convert(html)
        .unwrap();
    assert_eq!("Hello", md);
}

#[test]
fn multithreading() {
    let html = r#"<a href="https://example.com">Example</a>
    <a href="https://example.com">Example</a>
    <a href="https://example.com">Example</a>
    <a href="https://example.com">Example</a>
    <a href="https://example.com">Example</a>
    "#;
    let expected = "[Example][1] [Example][2] [Example][3] [Example][4] [Example][5]\n\n\
    [1]: https://example.com\n[2]: https://example.com\n[3]: https://example.com\n\
    [4]: https://example.com\n[5]: https://example.com";
    let converter = HtmlToMarkdown::builder()
        .options(Options {
            // We use a global vec to store all referenced links of the doc in
            // the anchor element handler, this is unsafe for multithreading
            // usage if we do nothing
            link_style: LinkStyle::Referenced,
            translation_mode: TranslationMode::Faithful,
            ..Default::default()
        })
        .build();
    let converter = Arc::new(converter);
    let mut handlers: Vec<JoinHandle<()>> = vec![];
    for _ in 0..20 {
        let converter_clone = converter.clone();
        let handle = std::thread::spawn(move || {
            let md = converter_clone.convert(html).unwrap();
            assert_eq!(expected, md);
        });
        handlers.push(handle);
    }
    for handle in handlers {
        handle.join().unwrap();
    }
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn unterminated_html() {
    // The `<i>` tag isn't terminated. Make sure the conversion still works.

    check_cases(&[Case {
        id: "unterminated_html_case_52",
        html: "<h1><i>A</h1>",
        expected_markdown: Some("# *A*"),
        ..Case::default()
    }]);
}

#[test]
fn misnested_formatting_does_not_duplicate_or_lose_text() {
    let markdown = htmd::convert("<p><b>one<i>two</b>three</i>four").unwrap();

    assert_eq!("**one*two****three*four", markdown);
}

/// [Math](../translation_approach.md#math).
#[test]
fn math() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "math_case_53",
        html: r#"<p><span class="math math-inline">x^2</span></p>"#,
        expected_markdown: Some("$x^2$"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "math_case_54",
        html: r#"<p><span class="math math-display">x^2</span></p>"#,
        expected_markdown: Some("$$x^2$$"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        ..Case::default()
    }]);

    // Test escaping -- values inside math should not be escaped.

    faithful_cases.extend([Case {
        id: "math_case_55",
        html: r#"<p><span class="math math-display">{a}_1, b_{2}, a*1, b*2, [a](b), 3 &lt;a&gt; b, a \; b</span></p>"#,
        expected_markdown: Some("$${a}_1, b_{2}, a*1, b*2, [a](b), 3 <a> b, a \\; b$$"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Math](../translation_approach.md#math).
/// pulldown-cmark reads `$$$$` as display math holding nothing, but inline math
/// cannot be empty: it reads `$$` as display math's delimiter.
#[test]
fn empty_math_span() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "empty_math_span_math_1",
        html: r#"<p><span class="math math-display"></span></p>"#,
        expected_markdown: Some("$$$$"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "empty_math_span_math_2",
        html: r#"<p><span class="math math-inline"></span></p>"#,
        expected_markdown: Some(r#"<span class="math math-inline"></span>"#),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    assert_eq!(
        "",
        convert_pure(r#"<p><span class="math math-inline"></span></p>"#).unwrap()
    );
    check_cases(&faithful_cases);
}

/// [Math](../translation_approach.md#math).
/// Inline math cannot begin or end with whitespace, so it is trimmed; display
/// math keeps its whitespace.
#[test]
fn math_span_whitespace() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "math_span_whitespace_case_57",
        html: "<p><span class=\"math math-inline\"> \u{b}x\t</span></p>",
        expected_markdown: Some("$x$"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "math_span_whitespace_math_1",
        html: r#"<p><span class="math math-display"> x </span></p>"#,
        expected_markdown: Some("$$ x $$"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    // Trimming a control space leaves a trailing `\`, which would escape the
    // closing `$`; display math keeps the space, so needs no HTML.
    faithful_cases.extend([Case {
        id: "math_span_whitespace_math_2",
        html: r#"<p><span class="math math-inline">a\ </span></p>"#,
        expected_markdown: Some(r#"<span class="math math-inline">a\\ </span>"#),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "math_span_whitespace_math_3",
        html: r#"<p><span class="math math-display">a\ </span></p>"#,
        expected_markdown: Some(r"$$a\ $$"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Math](../translation_approach.md#math).
/// Math which pulldown-cmark would not read back between `$`s is written as
/// HTML. A backslash escapes the character after it, so an escaped `\` escapes
/// nothing else.
#[test]
fn unwritable_math_span() {
    for (math, markdown) in [
        // An unescaped `$` ends the expression early.
        ("a$b", r"a\$b"),
        (r"a\\$b", r"a\\\\\$b"),
        // A trailing `\` escapes the closing `$`.
        (r"a\", r"a\\"),
        // pulldown-cmark pairs delimiters only around balanced braces.
        ("}a{", "}a{"),
        ("a{", "a{"),
        (r"\\{a", r"\\\\{a"),
    ] {
        check_cases(&[Case {
            id: "unwritable_math_span_math_1",
            html: &format!(r#"<p><span class="math math-inline">{math}</span></p>"#),
            expected_markdown: Some(&format!(
                r#"<span class="math math-inline">{markdown}</span>"#
            )),
            markdown_options: CommonMarkOptions::ENABLE_MATH,
            expectation: Some(HtmlExpectation::Preserved {
                renderer_suffix: "\n",
            }),
            ..Case::default()
        }]);
    }
    for math in [r"a\$b", r"a\\", "{a}", r"\{a", r"a\}"] {
        check_cases(&[Case {
            id: "unwritable_math_span_math_2",
            html: &format!(r#"<p><span class="math math-inline">{math}</span></p>"#),
            expected_markdown: Some(&format!("${math}$")),
            markdown_options: CommonMarkOptions::ENABLE_MATH,
            expectation: Some(HtmlExpectation::Preserved {
                renderer_suffix: "\n",
            }),
            ..Case::default()
        }]);
    }
}

/// [Math](../translation_approach.md#math).
/// Only a span whose sole attribute is its math class and which holds nothing
/// or a single text node is math; any other is translated like an ordinary
/// span.
#[test]
fn span_not_of_the_math_form() {
    let mut faithful_cases = Vec::new();
    let html = r#"<p><span class="math math-inline">a<em>b</em></span></p>"#;
    faithful_cases.extend([Case {
        id: "span_not_of_the_math_form_math_1",
        html,
        expected_markdown: Some(r#"<span class="math math-inline">a*b*</span>"#),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    assert_eq!("a*b*", convert_pure(html).unwrap());
    let html = r#"<p><span class="math math-inline">a<!--c--></span></p>"#;
    faithful_cases.extend([Case {
        id: "span_not_of_the_math_form_math_2",
        html,
        expected_markdown: Some(r#"<span class="math math-inline">a<!--c--></span>"#),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    assert_eq!("a", convert_pure(html).unwrap());
    let html = r#"<p><span class="math math-inline" id="x">a</span></p>"#;
    faithful_cases.extend([Case {
        id: "span_not_of_the_math_form_math_3",
        html,
        expected_markdown: Some(r#"<span class="math math-inline" id="x">a</span>"#),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    assert_eq!("a", convert_pure(html).unwrap());
    check_cases(&faithful_cases);
}

/// [Math](../translation_approach.md#math).
/// pulldown-cmark reads `\|` in a table cell as `|` even inside math, so a `|`
/// there takes the escape it takes in text.
#[test]
fn math_span_in_a_table_cell() {
    check_cases(&[Case {
        id: "math_span_in_a_table_cell",
        html: concat!(
            "<table><thead><tr><th>h</th></tr></thead><tbody>\n",
            r#"<tr><td><span class="math math-inline">\|x\| = a|b</span></td></tr>"#,
            "\n</tbody></table>"
        ),
        expected_markdown: Some(concat!(
            "| h                |\n",
            "| ---------------- |\n",
            r"| $\\|x\\| = a\|b$ |"
        )),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        markdown_options: CommonMarkOptions::ENABLE_MATH | CommonMarkOptions::ENABLE_TABLES,
        ..Case::default()
    }]);
}

/// [Math](../translation_approach.md#math).
/// pulldown-cmark looks for math delimiters in all text, so a text `$` is
/// escaped; without the escape, this reads back as math holding `5,`.
#[test]
fn dollar_sign_in_text() {
    check_cases(&[Case {
        id: "dollar_sign_in_text_math_1",
        html: "<p>$5,$6</p>",
        expected_markdown: Some(r"\$5,\$6"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    assert_eq!(r"\$5,\$6", convert_pure("<p>$5,$6</p>").unwrap());
}

// Document white space characters don't include non-breaking spaces; these should be preserved.
/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
#[test]
fn document_whitespace() {
    check_cases(&[Case {
        id: "document_whitespace_case_58",
        html: indoc!(
            "
            <p>bar&nbsp;</p>
            <ul>
              <li>foo&nbsp;</li>
            </ul>
            "
        ),
        expected_markdown: Some("bar\u{a0}\n\n*   foo\u{a0}"),
        ..Case::default()
    }]);
}

/// [Paragraphs](../translation_approach.md#paragraphs) keep multibyte text
/// before apparent list and heading markers in the same paragraph.
#[test]
fn multibyte_paragraph_markers() {
    check_cases(&[
        Case {
            id: "multibyte_ordered_list_escape_half",
            html: "<p>2½. Long shot</p>",
            expected_markdown: Some("2½. Long shot"),
            ..Case::default()
        },
        Case {
            id: "multibyte_ordered_list_escape_accented",
            html: "<p>1é. text</p>",
            expected_markdown: Some("1é. text"),
            ..Case::default()
        },
        Case {
            id: "multibyte_ordered_list_escape_trademark",
            html: "<p>3™. text</p>",
            expected_markdown: Some("3™. text"),
            ..Case::default()
        },
        Case {
            id: "ascii_ordered_list_escape",
            html: "<p>10. normal</p>",
            expected_markdown: Some(r"10\. normal"),
            ..Case::default()
        },
        Case {
            id: "multibyte_no_dot",
            html: "<p>2½</p>",
            expected_markdown: Some("2½"),
            ..Case::default()
        },
        Case {
            id: "cjk_before_ordered_list",
            html: "<p>日本語1. test</p>",
            expected_markdown: Some("日本語1. test"),
            ..Case::default()
        },
        Case {
            id: "multibyte_atx_heading_escape",
            html: "<p># héading</p>",
            expected_markdown: Some(r"\# héading"),
            ..Case::default()
        },
        Case {
            id: "multibyte_atx_heading_escape_umlaut",
            html: "<p>## über</p>",
            expected_markdown: Some(r"\## über"),
            ..Case::default()
        },
    ]);
}

/// [Paragraphs](../translation_approach.md#paragraphs).
/// The "Paragraphs" section of `translation_approach.md` keeps a
/// paragraph a paragraph where its content would otherwise open an HTML block.
/// Content which opens any other leaf block dissolves it the same way, and for
/// the same reason: the block scan reaches the content because nothing precedes
/// it on the line. A marker needs no text after it to open its block, so the
/// end of the line closes it just as a space does.
#[test]
fn a_paragraph_of_leaf_block_syntax_stays_a_paragraph() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_paragraph_of_leaf_block_syntax_stays_a_paragraph_paragraph_1",
        html: "<p>-</p>",
        expected_markdown: Some(r"\-"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "a_paragraph_of_leaf_block_syntax_stays_a_paragraph_paragraph_2",
        html: "<p>*</p>",
        expected_markdown: Some(r"\*"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "a_paragraph_of_leaf_block_syntax_stays_a_paragraph_paragraph_3",
        html: "<p>+</p>",
        expected_markdown: Some(r"\+"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "a_paragraph_of_leaf_block_syntax_stays_a_paragraph_paragraph_4",
        html: "<p>---</p>",
        expected_markdown: Some(r"\---"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "a_paragraph_of_leaf_block_syntax_stays_a_paragraph_paragraph_5",
        html: "<p>===</p>",
        expected_markdown: Some(r"\==="),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "a_paragraph_of_leaf_block_syntax_stays_a_paragraph_paragraph_6",
        html: "<p>1.</p>",
        expected_markdown: Some(r"1\."),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "a_paragraph_of_leaf_block_syntax_stays_a_paragraph_paragraph_7",
        html: "<p>1)</p>",
        expected_markdown: Some(r"1\)"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "a_paragraph_of_leaf_block_syntax_stays_a_paragraph_paragraph_8",
        html: "<p>- - -</p>",
        expected_markdown: Some(r"\- - -"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);

    // A `-` run opens a break or an underline only where nothing else shares
    // the line, so text behind it needs no escape.
    faithful_cases.extend([Case {
        id: "a_paragraph_of_leaf_block_syntax_stays_a_paragraph_paragraph_9",
        html: "<p>--foo</p>",
        expected_markdown: Some("--foo"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Headings](../translation_approach.md#headings).
/// A `#` run closed by the end of the line is an ATX heading holding nothing,
/// so it needs the escape although no text follows it.
#[test]
fn an_atx_marker_alone_in_a_paragraph_is_escaped() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "an_atx_marker_alone_in_a_paragraph_is_escaped_paragraph_1",
        html: "<p>#</p>",
        expected_markdown: Some(r"\#"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "an_atx_marker_alone_in_a_paragraph_is_escaped_paragraph_2",
        html: "<p>######</p>",
        expected_markdown: Some(r"\######"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);

    // Seven `#` open no heading, so this escape is one the conversion does not
    // need; it costs a backslash and still reads back as the text it came from.
    faithful_cases.extend([Case {
        id: "an_atx_marker_alone_in_a_paragraph_is_escaped_paragraph_3",
        html: "<p>#######</p>",
        expected_markdown: Some(r"\#######"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);

    // A `#` run which something other than a space or the line end closes is
    // not a marker at all.
    faithful_cases.extend([Case {
        id: "an_atx_marker_alone_in_a_paragraph_is_escaped_paragraph_4",
        html: "<p>#x</p>",
        expected_markdown: Some("#x"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

// The tests below spell the cases above with the whitespace a pretty-printer
// writes: a line ending or a tab in place of the space which closes the
// marker, and spaces before the marker itself. The walk compresses the one to
// the single space the marker needs and trims the other, so each opens the
// same block its plainer spelling does and needs the same escape.

/// [Paragraphs](../translation_approach.md#paragraphs).
/// A bullet marker which a line ending or a tab closes opens a list once the
/// whitespace is compressed, so it needs the escape "Special case for
/// paragraphs" calls for.
#[test]
fn a_bullet_marker_closed_by_a_line_ending_is_escaped() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_bullet_marker_closed_by_a_line_ending_is_escaped_paragraph_1",
        html: "<p>-\nfoo</p>",
        expected_markdown: Some(r"\- foo"),
        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p>- foo</p>\n",
            reason: "paragraph whitespace is trimmed or collapsed during translation",
            reference: "translation_approach.md#paragraphs",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "a_bullet_marker_closed_by_a_line_ending_is_escaped_paragraph_2",
        html: "<p>+\nfoo</p>",
        expected_markdown: Some(r"\+ foo"),
        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p>+ foo</p>\n",
            reason: "paragraph whitespace is trimmed or collapsed during translation",
            reference: "translation_approach.md#paragraphs",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "a_bullet_marker_closed_by_a_line_ending_is_escaped_paragraph_3",
        html: "<p>-\tfoo</p>",
        expected_markdown: Some(r"\- foo"),
        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p>- foo</p>\n",
            reason: "paragraph whitespace is trimmed or collapsed during translation",
            reference: "translation_approach.md#paragraphs",
        }),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Headings](../translation_approach.md#headings).
/// An ATX marker which a line ending or a tab closes opens a heading once the
/// whitespace is compressed.
#[test]
fn an_atx_marker_closed_by_a_line_ending_is_escaped() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "an_atx_marker_closed_by_a_line_ending_is_escaped_paragraph_1",
        html: "<p>#\nfoo</p>",
        expected_markdown: Some(r"\# foo"),
        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p># foo</p>\n",
            reason: "paragraph whitespace is trimmed or collapsed during translation",
            reference: "translation_approach.md#paragraphs",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "an_atx_marker_closed_by_a_line_ending_is_escaped_paragraph_2",
        html: "<p>###\nfoo</p>",
        expected_markdown: Some(r"\### foo"),
        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p>### foo</p>\n",
            reason: "paragraph whitespace is trimmed or collapsed during translation",
            reference: "translation_approach.md#paragraphs",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "an_atx_marker_closed_by_a_line_ending_is_escaped_paragraph_3",
        html: "<p>#\tfoo</p>",
        expected_markdown: Some(r"\# foo"),
        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p># foo</p>\n",
            reason: "paragraph whitespace is trimmed or collapsed during translation",
            reference: "translation_approach.md#paragraphs",
        }),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// An ordered item's delimiter which a line ending or a tab closes opens a list
/// once the whitespace is compressed.
#[test]
fn an_ordered_item_delimiter_closed_by_a_line_ending_is_escaped() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "an_ordered_item_delimiter_closed_by_a_line_ending_is_escaped_paragraph_1",
        html: "<p>1.\nfoo</p>",
        expected_markdown: Some(r"1\. foo"),
        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p>1. foo</p>\n",
            reason: "paragraph whitespace is trimmed or collapsed during translation",
            reference: "translation_approach.md#paragraphs",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "an_ordered_item_delimiter_closed_by_a_line_ending_is_escaped_paragraph_2",
        html: "<p>1)\nfoo</p>",
        expected_markdown: Some(r"1\) foo"),
        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p>1) foo</p>\n",
            reason: "paragraph whitespace is trimmed or collapsed during translation",
            reference: "translation_approach.md#paragraphs",
        }),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Paragraphs](../translation_approach.md#paragraphs).
/// Whitespace before a marker hides it only until the walk trims it: what a
/// block's content opens with is its first non-whitespace character, so the
/// marker starts the line after all. The last case is the shape a formatter
/// produces from a paragraph whose text happens to start with a number.
#[test]
fn a_marker_behind_leading_whitespace_is_escaped() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_marker_behind_leading_whitespace_is_escaped_paragraph_1",
        html: "<p> 1. foo</p>",
        expected_markdown: Some(r"1\. foo"),
        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p>1. foo</p>\n",
            reason: "paragraph whitespace is trimmed or collapsed during translation",
            reference: "translation_approach.md#paragraphs",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "a_marker_behind_leading_whitespace_is_escaped_paragraph_2",
        html: "<p> - foo</p>",
        expected_markdown: Some(r"\- foo"),
        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p>- foo</p>\n",
            reason: "paragraph whitespace is trimmed or collapsed during translation",
            reference: "translation_approach.md#paragraphs",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "a_marker_behind_leading_whitespace_is_escaped_paragraph_3",
        html: "<p>\n# foo</p>",
        expected_markdown: Some(r"\# foo"),
        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p># foo</p>\n",
            reason: "paragraph whitespace is trimmed or collapsed during translation",
            reference: "translation_approach.md#paragraphs",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "a_marker_behind_leading_whitespace_is_escaped_paragraph_4",
        html: "<p>\n  1.\n  Item numbering\n</p>",
        expected_markdown: Some(r"1\. Item numbering"),
        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p>1. Item numbering</p>\n",
            reason: "paragraph whitespace is trimmed or collapsed during translation",
            reference: "translation_approach.md#paragraphs",
        }),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Paragraphs](../translation_approach.md#paragraphs).
/// Whitespace inside a line opens nothing, so the same text needs no escape
/// where something precedes it on the line.
#[test]
fn a_marker_which_starts_no_line_is_not_escaped() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_marker_which_starts_no_line_is_not_escaped_paragraph_1",
        html: "<p>a <em>b</em> - c</p>",
        expected_markdown: Some("a *b* - c"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "a_marker_which_starts_no_line_is_not_escaped_paragraph_2",
        html: "<p>a <em>b</em> 1. c</p>",
        expected_markdown: Some("a *b* 1. c"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// Only the first delimiter after a digit run can open a list: a second
/// delimiter closes the first, which leaves it no whitespace terminator. Both
/// spellings are the text they came from, so neither is escaped.
#[test]
fn a_run_of_ordered_item_delimiters_opens_no_list() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_run_of_ordered_item_delimiters_opens_no_list_paragraph_1",
        html: "<p>1)))</p>",
        expected_markdown: Some("1)))"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "a_run_of_ordered_item_delimiters_opens_no_list_paragraph_2",
        html: "<p>1.)</p>",
        expected_markdown: Some("1.)"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Paragraphs](../translation_approach.md#paragraphs).
/// An [ordered list marker](https://spec.commonmark.org/0.31.2/#ordered-list-marker)
/// counts ASCII digits alone, so a run holding a Unicode numeric character
/// opens no list and the paragraph carrying it needs no escape. A backslash
/// reads back as the text it came from, which is why the trips below hold
/// either way: what the escape costs is a character in the Markdown.
#[test]
fn a_paragraph_of_unicode_numerics_needs_no_escape() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_paragraph_of_unicode_numerics_needs_no_escape_paragraph_1",
        html: "<p>2½. text</p>",
        expected_markdown: Some("2½. text"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "a_paragraph_of_unicode_numerics_needs_no_escape_paragraph_2",
        html: "<p>1١. text</p>",
        expected_markdown: Some("1١. text"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// What `escape_html` owes a [CDATA section](https://spec.commonmark.org/0.31.2/#cdata-section),
/// which no conversion can reach: `escape_if_needed` escapes the brackets
/// before `escape_html` runs, so the spelling htmd emits is the escaped one,
/// and the first trip below is the one that spelling takes -- proof that
/// `escape_html` must go on leaving it alone.
///
/// The last two read Markdown directly, which is the only way to reach the
/// unescaped spelling, and show what emitting it would cost: it is raw HTML,
/// so it passes through to the output for an HTML parser to read as a bogus
/// comment and show as nothing.
#[test]
fn a_literal_cdata_section_is_lost_unescaped() {
    check_cases(&[Case {
        id: "a_literal_cdata_section_is_lost_unescaped_preserved_line_1884",
        html: "<p>&lt;![CDATA[x]]&gt;</p>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    assert_eq!(
        "<p>a <![CDATA[x]]> b</p>\n",
        render_markdown("a <![CDATA[x]]> b", CommonMarkOptions::empty())
    );
    assert_eq!(
        "<p>a &lt;![CDATA[x]]&gt; b</p>\n",
        render_markdown(r"a \<![CDATA[x]]> b", CommonMarkOptions::empty())
    );
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// `script` and `style` open a
/// [type 1 HTML block](https://spec.commonmark.org/0.31.2/#html-blocks), which
/// runs to the line holding its closing tag: the element passes through whole,
/// blank lines and Markdown specials alike. `div` is type 6, which ends at a
/// blank line instead, so a blank line inside one has to be escaped. A tag
/// outside both lists is type 7, which is written as a raw HTML inline whatever
/// the context.
///
/// The tags a special case of the "Translating HTML nodes" section of
/// `translation_approach.md` sets aside — `iframe`, `xmp`, `noscript`,
/// `noembed`, `noframes` and `plaintext` — are deliberately left uncovered here
/// and in `round_trip_in_an_inline_context`.
/// The tokenizer hands their content back as literal characters, so neither the
/// escape below nor the walk an inline context uses can encode a line ending
/// inside one; this implementation ignores those cases.
#[test]
fn round_trip_in_a_block_context() {
    // Type 1: verbatim, both ways. Without the `<body>`, html5ever files a
    // leading `<script>` or `<style>` under the `<head>`, whose content is
    // dropped.
    let cases = [
        (
            "script_in_body",
            "<body><script>a*b\n\nc</script></body>",
            "<script>a*b\n\nc</script>",
            "the HTML parser omits the body wrapper",
        ),
        (
            "style_in_body",
            "<body><style>a*b\n\nc</style></body>",
            "<style>a*b\n\nc</style>",
            "the HTML parser omits the body wrapper",
        ),
        (
            "escaped_blank_line_in_div",
            "<div>a*b\n\nc</div>",
            "<div>a*b\n&#10;c</div>",
            "a blank line in a type 6 HTML block is encoded as a character reference",
        ),
        (
            "root_del_inline",
            "<del>a*b\n\nc</del>",
            "<p><del>a*b c</del></p>\n",
            "a type 7 raw HTML inline gains a paragraph and its whitespace collapses",
        ),
        (
            "root_pair_of_breaks",
            "<br><br>",
            "<p><br><br></p>\n",
            "two root breaks read back inside a paragraph",
        ),
    ];
    let mut cases = cases
        .iter()
        .map(|(id, html, expected_html, reason)| Case {
            id,
            html,
            expectation: Some(HtmlExpectation::DocumentedDifference {
                kind: DifferenceKind::Loss,
                expected_html,
                reason,
                reference: "translation_approach.md#translating-html-nodes",
            }),
            ..Case::default()
        })
        .collect::<Vec<_>>();
    cases[2].expectation = Some(HtmlExpectation::DocumentedDifference {
        kind: DifferenceKind::Representation,
        expected_html: "<div>a*b\n&#10;c</div>",
        reason: "the character reference decodes to the original text newline",
        reference: "translation_approach.md#translating-html-nodes",
    });

    // Type 6: the escaped blank line decodes when an HTML parser reads this
    // output, so the trip is faithful.

    // Type 7 is a raw HTML inline, so CommonMark opens a paragraph around it —
    // the tradeoff the "Translating HTML nodes" section of
    // `translation_approach.md` takes deliberately. The `*` survives, escaped
    // as `a\*b`; the blank line does not, a raw HTML inline having its
    // whitespace collapsed.
    // The loss the same section names: `<br><br>` at the root stays `<br><br>`,
    // which reads back inside the paragraph CommonMark opens around it.
    // A lone `<br>` meets the type 7 start condition, so it comes back as the
    // HTML block it opened.

    cases.push(Case {
        id: "round_trip_in_a_block_context_preserved_line_1945",
        html: "<br>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "",
        }),
        ..Case::default()
    });
    check_cases(&cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// The "Paragraphs" section of `translation_approach.md`: content
/// which opens an HTML block dissolves the paragraph or setext heading holding
/// it, so those two write themselves out whole instead. Each trip below would
/// otherwise come back without its `<p>` or `<h1>`.
#[test]
fn round_trip_of_a_leaf_block_an_html_block_would_dissolve() {
    let mut cases = Vec::new();
    // A lone type 7 tag.

    cases.extend([Case {
        id: "round_trip_of_a_leaf_block_an_html_block_would_dissolve_preserved_line_1955",
        html: "<p><br></p>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "",
        }),
        ..Case::default()
    }]);
    // A type 1-6 tag opening the content. A comment is type 2, `iframe` type 6.

    cases.extend([Case {
        id: "round_trip_of_a_leaf_block_an_html_block_would_dissolve_preserved_line_1957",
        html: "<p><!--c--></p>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "",
        }),
        ..Case::default()
    }]);

    cases.extend([Case {
        id: "round_trip_of_a_leaf_block_an_html_block_would_dissolve_preserved_line_1958",
        html: "<p><!--c-->a</p>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "",
        }),
        ..Case::default()
    }]);

    cases.extend([Case {
        id: "round_trip_of_a_leaf_block_an_html_block_would_dissolve_preserved_line_1959",
        html: r#"<p><iframe src="u">a</iframe></p>"#,
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "",
        }),
        ..Case::default()
    }]);
    // A list item's marker and a blockquote's `>` are stripped before the line
    // is scanned, so they protect the paragraph no better than the root does.
    cases.extend([Case {
        id: "blockquote_with_break_paragraph",
        html: "<blockquote><p><br></p></blockquote>",
        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<blockquote>\n<p><br></p></blockquote>\n",
            reason: "CommonMark inserts a line ending after the opening blockquote",
            reference: "translation_approach.md#blockquotes",
        }),
        ..Case::default()
    }]);

    // A second tag keeps the line from being type 7, and a tag which does not
    // open the content cannot start a block at all: both stay paragraphs.
    cases.extend([Case {
        id: "round_trip_of_a_leaf_block_an_html_block_would_dissolve_preserved_1",
        html: "<p><br><br></p>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    cases.extend([Case {
        id: "round_trip_of_a_leaf_block_an_html_block_would_dissolve_preserved_2",
        html: "<p><br>a</p>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    cases.extend([Case {
        id: "round_trip_of_a_leaf_block_an_html_block_would_dissolve_preserved_3",
        html: "<p><del><br></del></p>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);

    // A setext heading falls back to ATX, whose `#` the block scan matches
    // before any HTML block start condition.
    let setext = Options {
        heading_style: HeadingStyle::Setex,
        ..Case::default().htmd_options
    };
    // Content which opens no HTML block keeps the setext form.
    let headings = [
        "<h1><br></h1>",
        "<h1><div>a</div></h1>",
        "<h1><br><br></h1>",
    ];
    cases.extend(headings.iter().map(|html| Case {
        id: html,
        html,
        htmd_options: setext.clone(),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }));
    check_cases(&cases);
}

/// [Headings](../translation_approach.md#headings).
/// A setext heading's content opens a line, so content which starts any leaf
/// block dissolves the heading exactly as it dissolves a paragraph. The escape
/// a paragraph puts on such content keeps the setext spelling whole, so the
/// headings section's fallback to ATX is not needed here.
#[test]
fn round_trip_of_a_setext_heading_of_leaf_block_syntax() {
    let headings = ["<h1>-</h1>", "<h2>-</h2>", "<h1>---</h1>", "<h1>1.</h1>"];
    check_cases(
        &headings
            .iter()
            .map(|html| Case {
                id: html,
                html,
                htmd_options: Options {
                    heading_style: HeadingStyle::Setex,
                    ..Case::default().htmd_options
                },
                expectation: Some(HtmlExpectation::Preserved {
                    renderer_suffix: "\n",
                }),
                ..Case::default()
            })
            .collect::<Vec<_>>(),
    );
}

/// [Headings](../translation_approach.md#headings).
/// Step 4 of the headings approach: CommonMark reads a `#` run ending an ATX
/// heading as its closing sequence when it follows a space or tab, so each of
/// these trips would otherwise lose the run.
#[test]
fn round_trip_of_an_atx_heading_ending_in_a_hash_run() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "round_trip_of_an_atx_heading_ending_in_a_hash_run_preserved_1",
        html: "<h1>a #</h1>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "round_trip_of_an_atx_heading_ending_in_a_hash_run_preserved_2",
        html: "<h1>a ##</h1>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "round_trip_of_an_atx_heading_ending_in_a_hash_run_preserved_3",
        html: "<h1><em>a</em> #</h1>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "round_trip_of_an_atx_heading_ending_in_a_hash_run_preserved_4",
        html: "<h3>#</h3>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    // The walk collapses the tab to a space.
    faithful_cases.extend([Case {
        id: "heading_tab_before_hash",
        html: "<h1>a\t#</h1>",
        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<h1>a #</h1>\n",
            reason: "the heading walk collapses a tab to a space",
            reference: "translation_approach.md#headings",
        }),
        ..Case::default()
    }]);

    // These runs follow no space or tab, so they need no escape.

    faithful_cases.extend([Case {
        id: "round_trip_of_an_atx_heading_ending_in_a_hash_run_case_60",
        html: "<h1>a#</h1>",
        expected_markdown: Some("# a#"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "round_trip_of_an_atx_heading_ending_in_a_hash_run_case_61",
        html: "<h1>a&nbsp;#</h1>",
        expected_markdown: Some("# a\u{a0}#"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "round_trip_of_an_atx_heading_ending_in_a_hash_run_case_62",
        html: r"<h1>a \#</h1>",
        expected_markdown: Some(r"# a \\#"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// Step 4 of the headings approach, for content which is nothing but a `#` run.
/// The generic text escape has already escaped a text node which opens with
/// one, so only a custom handler reaches this case.
#[test]
fn atx_heading_of_a_hash_run_from_a_handler() {
    let converter = HtmlToMarkdown::builder()
        .options(Options {
            translation_mode: TranslationMode::Faithful,
            ..Default::default()
        })
        .add_handler(
            vec!["x-hash"],
            |_: &dyn Handlers, _: &Element| -> Option<HandlerResult> { Some("#".into()) },
        )
        .build();
    let markdown = converter.convert("<h1><x-hash></x-hash></h1>").unwrap();
    assert_eq!(r"# \#", markdown);
    assert_eq!(
        "<h1>#</h1>\n",
        render_markdown(&markdown, CommonMarkOptions::empty())
    );
    assert_eq!(
        r"# \##",
        converter
            .convert("<h1><x-hash></x-hash><x-hash></x-hash></h1>")
            .unwrap()
    );
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// A heading is a leaf block, so each element below is written as a raw HTML
/// inline. Only its tags are HTML; what sits between them is CommonMark text,
/// which is what makes the escape of `a\*b` work here — the CommonMark parser
/// consumes the backslash before any HTML parser sees a `<script>` element.
/// `round_trip_losses_of_a_walked_raw_inline` holds the cases where reading
/// that content as text costs something.
#[test]
fn round_trip_in_an_inline_context() {
    let preserved = [
        (
            "script_without_line_ending",
            "<h1>x<script>a*b*c</script>y</h1>",
        ),
        (
            "style_without_line_ending",
            "<h1>x<style>a*b*c</style>y</h1>",
        ),
        (
            "script_holding_markdown_link",
            "<h1>x<script>[a](b)</script>y</h1>",
        ),
        ("div_without_line_ending", "<h1>x<div>a*b*c</div>y</h1>"),
    ];
    let losses = [
        (
            "script_line_ending",
            "<h1>x<script>a*b\nc</script>y</h1>",
            "<h1>x<script>a*b c</script>y</h1>\n",
        ),
        (
            "style_line_ending",
            "<h1>x<style>a*b\nc</style>y</h1>",
            "<h1>x<style>a*b c</style>y</h1>\n",
        ),
        (
            "textarea_line_ending",
            "<h1>x<textarea>a*b\nc</textarea>y</h1>",
            "<h1>x<textarea>a*b c</textarea>y</h1>\n",
        ),
        (
            "pre_line_ending",
            "<h1>x<pre>a*b\nc</pre>y</h1>",
            "<h1>x<pre>a*b c</pre>y</h1>\n",
        ),
        (
            "div_line_ending",
            "<h1>x<div>a*b\nc</div>y</h1>",
            "<h1>x<div>a*b c</div>y</h1>\n",
        ),
        (
            "del_line_ending",
            "<h1>x<del>a*b\nc</del>y</h1>",
            "<h1>x<del>a*b c</del>y</h1>\n",
        ),
        (
            "comment_line_ending",
            "<h1>x<!--a\nb-->y</h1>",
            "<h1>x<!--a b-->y</h1>\n",
        ),
    ];
    let mut cases = preserved
        .iter()
        .map(|(id, html)| Case {
            id,
            html,
            expectation: Some(HtmlExpectation::Preserved {
                renderer_suffix: "\n",
            }),
            ..Case::default()
        })
        .collect::<Vec<_>>();
    cases.extend(losses.iter().map(|(id, html, expected_html)| Case {
        id,
        html,
        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html,
            reason: "the raw inline walk collapses a line ending to one space",
            reference: "translation_approach.md#translating-html-nodes",
        }),
        ..Case::default()
    }));
    check_cases(&cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// Every raw HTML inline has its content walked, `<pre>`, `<script>` and
/// `<style>` included, so each of the trips below returns the HTML it started
/// from. `round_trip_losses_of_a_walked_raw_inline` holds the cases which do
/// not.
#[test]
fn round_trip_of_a_walked_raw_inline() {
    let mut faithful_cases = Vec::new();
    // A nested element.
    faithful_cases.extend([Case {
        id: "round_trip_of_a_walked_raw_inline_preserved_1",
        html: "<h1>x<div>a<em>b</em>c</div>y</h1>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "round_trip_of_a_walked_raw_inline_preserved_2",
        html: "<h1>x<del>a<em>b</em>c</del>y</h1>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "round_trip_of_a_walked_raw_inline_preserved_3",
        html: "<h1>x<pre>a<em>b</em>c</pre>y</h1>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);

    // A literal `<`.
    faithful_cases.extend([Case {
        id: "round_trip_of_a_walked_raw_inline_preserved_4",
        html: "<h1>x<div>a&lt;b&gt;c</div>y</h1>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "round_trip_of_a_walked_raw_inline_preserved_5",
        html: "<h1>x<del>a&lt;b&gt;c</del>y</h1>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "round_trip_of_a_walked_raw_inline_preserved_6",
        html: "<h1>x<pre>a&lt;b&gt;c</pre>y</h1>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "round_trip_of_a_walked_raw_inline_preserved_7",
        html: "<h1>x<textarea>a&lt;b&gt;c</textarea>y</h1>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);

    // A literal `&`.
    faithful_cases.extend([Case {
        id: "round_trip_of_a_walked_raw_inline_preserved_8",
        html: "<h1>x<div>a&amp;b</div>y</h1>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "round_trip_of_a_walked_raw_inline_preserved_9",
        html: "<h1>x<pre>a&amp;b</pre>y</h1>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);

    // Markdown specials.
    faithful_cases.extend([Case {
        id: "round_trip_of_a_walked_raw_inline_preserved_10",
        html: "<h1>x<div>a*b_c[d]</div>y</h1>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "round_trip_of_a_walked_raw_inline_preserved_11",
        html: "<h1>x<pre>a*b_c[d]</pre>y</h1>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "round_trip_of_a_walked_raw_inline_preserved_12",
        html: "<h1>x<script>a*b_c[d]</script>y</h1>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "round_trip_of_a_walked_raw_inline_preserved_13",
        html: "<h1>x<style>a*b_c[d]</style>y</h1>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);

    // A `<code>` child: a code span where CommonMark allows one, and a raw
    // HTML inline of its own inside a `<pre>`.
    faithful_cases.extend([Case {
        id: "round_trip_of_a_walked_raw_inline_preserved_14",
        html: "<h1>x<div>a<code>c*d</code>b</div>y</h1>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "round_trip_of_a_walked_raw_inline_preserved_15",
        html: "<h1>x<pre>a<code>c*d</code>b</pre>y</h1>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "round_trip_of_a_walked_raw_inline_preserved_16",
        html: "<h1><pre><code>a</code></pre></h1>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    faithful_cases.extend([Case {
        id: "round_trip_of_a_walked_raw_inline_preserved_17",
        html: r#"<h1>x<div>a<code class="q">c*d</code>b</div>y</h1>"#,
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);

    // A block context is untouched by all of this: this is an HTML block,
    // written as it stands. `round_trip_in_a_block_context` has the rest.

    faithful_cases.extend([Case {
        id: "round_trip_of_a_walked_raw_inline_preserved_line_2176",
        html: "<pre>a*b\n\nc</pre>",
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "",
        }),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Translating HTML nodes](../translation_approach.md#translating-html-nodes).
/// The trips of `round_trip_of_a_walked_raw_inline` which lose something. Each
/// assertion holds what actually comes back, and says what went missing.
#[test]
fn round_trip_losses_of_a_walked_raw_inline() {
    let cases = [
        (
            "script_less_than",
            "<h1>x<script>if(a<b){}</script>y</h1>",
            "<h1>x<script>if(a&lt;b){}</script>y</h1>\n",
            "a raw text element retains the CommonMark escape as literal text",
        ),
        (
            "script_apparent_emphasis",
            "<h1>x<script>a<em>b</em>c</script>y</h1>",
            "<h1>x<script>a&lt;em&gt;b&lt;/em&gt;c</script>y</h1>\n",
            "raw text preserves escaped apparent tags",
        ),
        (
            "script_ampersand",
            "<h1>x<script>a&amp;b</script>y</h1>",
            "<h1>x<script>a&amp;amp;b</script>y</h1>\n",
            "raw text preserves escaped ampersands",
        ),
        (
            "script_references",
            "<h1>x<script>a&lt;b&gt;c</script>y</h1>",
            "<h1>x<script>a&amp;lt;b&amp;gt;c</script>y</h1>\n",
            "raw text preserves escaped character references",
        ),
        (
            "pre_b_to_strong",
            "<h1><pre><b>a</b></pre></h1>",
            "<h1><pre><strong>a</strong></pre></h1>\n",
            "a b element becomes the strong element meant by its Markdown",
        ),
        (
            "textarea_apparent_emphasis",
            "<h1>x<textarea>a<em>b</em>c</textarea>y</h1>",
            "<h1>x<textarea>a&lt;em&gt;b&lt;/em&gt;c</textarea>y</h1>\n",
            "RCDATA treats apparent tags as text",
        ),
        (
            "code_block_missing_final_line_ending",
            "<pre><code>a*b\nc</code></pre>",
            "<pre><code>a*b\nc\n</code></pre>\n",
            "a code block adds its required final line ending",
        ),
        (
            "div_line_ending",
            "<h1>x<div>a\nb</div>y</h1>",
            "<h1>x<div>a b</div>y</h1>\n",
            "the raw inline walk collapses a line ending",
        ),
        (
            "pre_line_ending",
            "<h1>x<pre>a\nb</pre>y</h1>",
            "<h1>x<pre>a b</pre>y</h1>\n",
            "the raw inline walk collapses a line ending",
        ),
        (
            "script_line_ending",
            "<h1>x<script>a\nb</script>y</h1>",
            "<h1>x<script>a b</script>y</h1>\n",
            "the raw inline walk collapses a line ending",
        ),
        (
            "textarea_line_ending",
            "<h1>x<textarea>a\nb</textarea>y</h1>",
            "<h1>x<textarea>a b</textarea>y</h1>\n",
            "the raw inline walk collapses a line ending",
        ),
        (
            "nested_code_line_ending",
            "<h1><pre><code>a*b\nc</code></pre></h1>",
            "<h1><pre><code>a*b c</code></pre></h1>\n",
            "the raw inline walk collapses a line ending",
        ),
    ];
    let mut cases = cases
        .iter()
        .map(|(id, html, expected_html, reason)| Case {
            id,
            html,
            expectation: Some(HtmlExpectation::DocumentedDifference {
                kind: DifferenceKind::Loss,
                expected_html,
                reason,
                reference: "translation_approach.md#translating-html-nodes",
            }),
            ..Case::default()
        })
        .collect::<Vec<_>>();
    for (case, second_markdown) in cases[..4].iter_mut().zip([
        "# x<script>if(a\\&lt;b){}</script>y",
        "# x<script>a\\&lt;em\\&gt;b\\&lt;/em\\&gt;c</script>y",
        "# x<script>a\\&amp;amp;b</script>y",
        "# x<script>a\\&amp;lt;b\\&amp;gt;c</script>y",
    ]) {
        case.stability = StabilityExpectation::DocumentedInstability {
            second_markdown,
            reason: "a raw text element retains CommonMark's escape as literal text, so the second conversion escapes it again",
            reference: "translation_approach.md#translating-html-nodes",
        };
    }
    check_cases(&cases);
}

#[test]
fn append_content_when_the_body_is_empty() {
    struct Footnote;

    impl ElementHandler for Footnote {
        fn handle(&self, _handlers: &dyn Handlers, _element: &Element) -> Option<HandlerResult> {
            None
        }

        fn append(&self) -> Option<String> {
            Some("\n\n[^1]: A footnote.".to_string())
        }
    }

    let converter = HtmlToMarkdown::builder()
        .add_handler(vec!["img"], Footnote)
        .build();

    // Appended content opens with a blank line separating it from the body.
    // With the body empty, that blank line must not lead the document.
    assert_eq!(
        "[^1]: A footnote.",
        converter.convert("<img src=\"a.png\">").unwrap()
    );
    assert_eq!(
        "Hello\n\n[^1]: A footnote.",
        converter
            .convert("<p>Hello</p><img src=\"a.png\">")
            .unwrap()
    );
}
