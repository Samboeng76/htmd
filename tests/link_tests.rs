use htmd::{
    HtmlToMarkdown,
    options::{LinkStyle, Options, TranslationMode},
};
mod common;
use common::{
    Case, DifferenceKind, HtmlExpectation, StabilityExpectation, check_cases, convert_faithful,
    convert_faithful_options, convert_pure, render_markdown,
};
use pulldown_cmark::Options as CommonMarkOptions;

/// [Links](../translation_approach.md#links) in a paragraph preserve the anchor.
#[test]
fn paragraph_wrapped_link_round_trips() {
    check_cases(&[Case {
        id: "paragraph_wrapped_link",
        html: "<p><a href=\"/\">Link</a></p>",
        expected_markdown: Some("[Link](/)"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
}

/// [Links](../translation_approach.md#links).
#[test]
fn links() {
    let html = r#"
        <a href="https://example.com">Link 1</a>
        <a href="https://example.com" title="Hello">Link 2</a>
        "#;
    assert_eq!(
        "[Link 1](https://example.com) [Link 2](https://example.com \"Hello\")",
        convert_faithful(html).unwrap(),
    );
    check_cases(&[Case {
        id: "links_case_1",
        html,
        expected_markdown: Some(
            "[Link 1](https://example.com) [Link 2](https://example.com \"Hello\")",
        ),
        ..Case::default()
    }]);
}

/// [Links](../translation_approach.md#links).
#[test]
fn links_with_spaces_in_destination_and_title() {
    assert_eq!(
        r#"[Link](<https://example.com/hello world> "Hello")"#,
        convert_faithful(r#"<a href="https://example.com/hello world" title="Hello">Link</a>"#)
            .unwrap(),
    );
    check_cases(&[Case {
        id: "links_with_spaces_in_destination_and_title_case_2",
        html: r#"<a href="https://example.com/hello world" title="Hello">Link</a>"#,
        expected_markdown: Some(r#"[Link](<https://example.com/hello world> "Hello")"#),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "[Link](https://example.com/hello%20world \"Hello\")",
            reason: "pulldown-cmark percent-encodes the destination when rendering its CommonMark link or image",
            reference: "translation_approach.md#links",
        },
        ..Case::default()
    }]);
}

/// [Links](../translation_approach.md#links).
#[test]
fn links_with_spaces_around_text() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "links_with_spaces_around_text_case_3",
        html: r#"<a href="/"> bla </a>"#,
        expected_markdown: Some("[ bla ](/)"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "links_with_spaces_around_text_case_4",
        html: r#"Some <a href="/"> random </a> text"#,
        expected_markdown: Some("Some [ random ](/) text"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Links](../translation_approach.md#links).
#[test]
fn links_inlined_prefer_autolinks() {
    let converter = HtmlToMarkdown::builder()
        .options(Options {
            translation_mode: TranslationMode::Faithful,
            link_style: LinkStyle::InlinedPreferAutolinks,
            ..Default::default()
        })
        .build();

    let html = r#"<a href="https://example.com">https://example.com</a>"#;
    assert_eq!("<https://example.com>", converter.convert(html).unwrap());

    let html = r#"<a href="https://example.com">Link</a>"#;
    assert_eq!(
        "[Link](https://example.com)",
        converter.convert(html).unwrap()
    );

    let html = r#"<a href="https://example.com" title="https://example.com">Link</a>"#;
    assert_eq!(
        r#"[Link](https://example.com "https://example.com")"#,
        converter.convert(html).unwrap()
    );
    let options = Options {
        link_style: LinkStyle::InlinedPreferAutolinks,
        ..Case::default().htmd_options
    };
    check_cases(&[
        Case {
            id: "autolink_preferred_when_text_is_url",
            html: r#"<a href="https://example.com">https://example.com</a>"#,
            htmd_options: options.clone(),
            expected_markdown: Some("<https://example.com>"),
            ..Case::default()
        },
        Case {
            id: "ordinary_link_with_autolink_preference",
            html: r#"<a href="https://example.com">Link</a>"#,
            htmd_options: options.clone(),
            expected_markdown: Some("[Link](https://example.com)"),
            ..Case::default()
        },
        Case {
            id: "link_title_disables_autolink_preference",
            html,
            htmd_options: options,
            expected_markdown: Some(r#"[Link](https://example.com "https://example.com")"#),
            ..Case::default()
        },
    ]);
}

/// [Links](../translation_approach.md#links).
/// A line ending in a link destination ends the leaf block holding it: written
/// literally, `a[t](u⏎⏎v)b` is two paragraphs and no link at all. CommonMark
/// decodes a character reference in a destination, so it is encoded instead.
#[test]
fn a_link_destination_escapes_its_line_endings() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_link_destination_escapes_its_line_endings_case_5",
        html: "<p>a<a href=\"u\nv\">t</a>b</p>",
        expected_markdown: Some("a[t](u&#10;v)b"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "a[t](u%0Av)b",
            reason: "pulldown-cmark percent-encodes the destination when rendering its CommonMark link or image",
            reference: "translation_approach.md#links",
        },
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_link_destination_escapes_its_line_endings_case_6",
        html: "<p>a<a href=\"u\n\nv\">t</a>b</p>",
        expected_markdown: Some("a[t](u&#10;&#10;v)b"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "a[t](u%0A%0Av)b",
            reason: "pulldown-cmark percent-encodes the destination when rendering its CommonMark link or image",
            reference: "translation_approach.md#links",
        },
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_link_destination_escapes_its_line_endings_case_7",
        html: "<h1>a<a href=\"u\nv\">t</a>b</h1>",
        expected_markdown: Some("# a[t](u&#10;v)b"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "# a[t](u%0Av)b",
            reason: "pulldown-cmark percent-encodes the destination when rendering its CommonMark link or image",
            reference: "translation_approach.md#links",
        },
        ..Case::default()
    }]);
    // A carriage return reaches the destination only as a character reference,
    // the parser having folded any literal CRLF into a line feed.

    faithful_cases.extend([Case {
        id: "a_link_destination_escapes_its_line_endings_case_8",
        html: "<p>a<a href=\"u&#13;v\">t</a>b</p>",
        expected_markdown: Some("a[t](u&#13;v)b"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "a[t](u%0Dv)b",
            reason: "pulldown-cmark percent-encodes the destination when rendering its CommonMark link or image",
            reference: "translation_approach.md#links",
        },
        ..Case::default()
    }]);
    // An image's destination takes the same encoding.

    faithful_cases.extend([Case {
        id: "a_link_destination_escapes_its_line_endings_case_9",
        html: "<p>a<img src=\"i\nj\" alt=\"\">b</p>",
        expected_markdown: Some("a![](i&#10;j)b"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "a![](i%0Aj)b",
            reason: "pulldown-cmark percent-encodes the destination when rendering its CommonMark link or image",
            reference: "translation_approach.md#links",
        },
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_link_destination_escapes_its_line_endings_case_10",
        html: r#"<p><a href="u">t</a></p>"#,
        expected_markdown: Some("[t](u)"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_link_destination_escapes_its_line_endings_case_11",
        html: r#"<p><a href="u(v)">t</a></p>"#,
        expected_markdown: Some(r"[t](u\(v\))"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

#[test]
fn nested_convert_with_referenced_links() {
    use htmd::Element;
    use htmd::element_handler::Handlers;
    use indoc::indoc;

    let converter = HtmlToMarkdown::builder()
        .options(Options {
            link_style: LinkStyle::Referenced,
            ..Default::default()
        })
        .add_handler(
            vec!["widget"],
            |_handlers: &dyn Handlers, _element: &Element| {
                let inner_converter = HtmlToMarkdown::builder()
                    .options(Options {
                        link_style: LinkStyle::Referenced,
                        ..Default::default()
                    })
                    .build();
                let inner_md = inner_converter
                    .convert(r#"<a href="https://inner.com">Inner</a>"#)
                    .unwrap();
                Some(inner_md.into())
            },
        )
        .build();

    let html = indoc!(
        r#"
        <p><a href="https://outer1.com">Outer 1</a></p>
        <widget></widget>
        <p><a href="https://outer2.com">Outer 2</a></p>
        "#
    );
    let md = converter.convert(html).unwrap();

    assert_eq!(
        indoc!(
            r#"
            [Outer 1][1]

            [Inner][1]

            [1]: https://inner.com

            [Outer 2][2]

            [1]: https://outer1.com
            [2]: https://outer2.com"#
        ),
        md
    );
}

/// [Links](../translation_approach.md#links).
#[test]
fn discard_links_in_table_falling_back_to_raw_html() {
    use indoc::indoc;

    let converter = HtmlToMarkdown::builder()
        .options(Options {
            link_style: LinkStyle::Referenced,
            translation_mode: TranslationMode::Faithful,
            ..Default::default()
        })
        .build();

    let html = indoc!(
        r#"
        <table>
            <tr>
                <td><a href="https://discarded.com">Discarded</a></td>
                <td><custom-unsupported>test</custom-unsupported></td>
            </tr>
        </table>
        <p><a href="https://kept.com">Kept</a></p>
        "#
    );
    let md = converter.convert(html).unwrap();

    assert_eq!(
        indoc!(
            r#"
            <table>
                <tbody><tr>
                    <td><a href="https://discarded.com">Discarded</a></td>
                    <td><custom-unsupported>test</custom-unsupported></td>
                </tr>
            </tbody></table>

            [Kept][1]

            [1]: https://kept.com"#
        ),
        md
    );
    check_cases(&[Case {
        id: "discard_links_in_table_falling_back_to_raw_html",
        html,
        htmd_options: Options {
            link_style: LinkStyle::Referenced,
            ..Case::default().htmd_options
        },
        expected_markdown: Some(indoc!(
            r#"<table>
    <tbody><tr>
        <td><a href="https://discarded.com">Discarded</a></td>
        <td><custom-unsupported>test</custom-unsupported></td>
    </tr>
</tbody></table>

[Kept][1]

[1]: https://kept.com"#
        )),
        ..Case::default()
    }]);
}

/// [Links](../translation_approach.md#links).
/// A backslash in a destination escapes the character which follows it, the
/// `)` which closes the destination included: written literally, `[x](a\)`
/// reads back as the text `[x](a)` and holds no link at all. The destination's
/// own backslashes are escaped so that the delimiters htmd writes are the only
/// ones the parser sees.
#[test]
fn a_link_destination_escapes_its_backslashes() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_link_destination_escapes_its_backslashes_case_12",
        html: r#"<a href="a\">x</a>"#,
        expected_markdown: Some(r"[x](a\\)"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "[x](a%5C)",
            reason: "pulldown-cmark percent-encodes the destination when rendering its CommonMark link or image",
            reference: "translation_approach.md#links",
        },

        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p><a href=\"a%5C\">x</a></p>\n",
            reason: "the root link gains a paragraph and pulldown-cmark percent-encodes the destination backslash",
            reference: "translation_approach.md#links",
        }),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_link_destination_escapes_its_backslashes_case_13",
        html: r#"<img src="i\" alt="q">"#,
        expected_markdown: Some(r"![q](i\\)"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "![q](i%5C)",
            reason: "pulldown-cmark percent-encodes the destination when rendering its CommonMark link or image",
            reference: "translation_approach.md#links",
        },
        ..Case::default()
    }]);
    // A destination holding a space closes with a `>` instead, which a
    // trailing backslash escapes the same way.

    faithful_cases.extend([Case {
        id: "a_link_destination_escapes_its_backslashes_case_14",
        html: r#"<a href="a b\">x</a>"#,
        expected_markdown: Some(r"[x](<a b\\>)"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "[x](a%20b%5C)",
            reason: "pulldown-cmark percent-encodes the destination when rendering its CommonMark link or image",
            reference: "translation_approach.md#links",
        },
        ..Case::default()
    }]);
    // A backslash before a paren is text the destination holds, not the escape
    // htmd writes for the paren, so it needs an escape of its own.

    faithful_cases.extend([Case {
        id: "a_link_destination_escapes_its_backslashes_case_15",
        html: r#"<a href="a\(b">x</a>"#,
        expected_markdown: Some(r"[x](a\\\(b)"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "[x](a%5C\\(b)",
            reason: "pulldown-cmark percent-encodes the destination when rendering its CommonMark link or image",
            reference: "translation_approach.md#links",
        },
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Links](../translation_approach.md#links).
/// A backslash in a title escapes the character which follows it, the `"`
/// which closes the title included: written literally, `[t](x "a\")` reads
/// back as text and holds no link at all.
#[test]
fn a_link_title_escapes_its_backslashes() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_link_title_escapes_its_backslashes_case_16",
        html: r#"<a href="x" title="a\">t</a>"#,
        expected_markdown: Some(r#"[t](x "a\\")"#),
        ..Case::default()
    }]);
    // A title is a plain string rather than inline content, so the `*` needs
    // no escape -- but the backslash before it would be read as one, and the
    // title would come back a character short.

    faithful_cases.extend([Case {
        id: "a_link_title_escapes_its_backslashes_case_17",
        html: r#"<a href="x" title="a\*b">t</a>"#,
        expected_markdown: Some(r#"[t](x "a\\*b")"#),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_link_title_escapes_its_backslashes_case_18",
        html: r#"<img src="i" alt="q" title="a\">"#,
        expected_markdown: Some(r#"![q](i "a\\")"#),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Images](../translation_approach.md#images).
/// An image's alt text lands in a
/// [link label](https://spec.commonmark.org/0.31.2/#link-label), which is
/// inline content: an unescaped `]` closes the label early, so written
/// literally `![a](y)b](x)` is an image of `y` with the alt text `a`, followed
/// by the text `b](x)`. The label reads its other specials too, which an alt
/// of `a*b*c` would come back emphasized by.
#[test]
fn an_image_alt_escapes_its_markdown_specials() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "an_image_alt_escapes_its_markdown_specials_case_19",
        html: r#"<img src="x" alt="a](y)b">"#,
        expected_markdown: Some(r"![a\](y)b](x)"),

        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p><img src=\"x\" alt=\"a](y)b\" /></p>\n",
            reason: "the root image gains a paragraph and pulldown-cmark serializes the void tag with a slash",
            reference: "translation_approach.md#images",
        }),
        ..Case::default()
    }]);

    // An unmatched `[` opens a label the parser never closes.

    faithful_cases.extend([Case {
        id: "an_image_alt_escapes_its_markdown_specials_case_20",
        html: r#"<img src="x" alt="a[b">"#,
        expected_markdown: Some(r"![a\[b](x)"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "an_image_alt_escapes_its_markdown_specials_case_21",
        html: r#"<img src="x" alt="a\b*c_d`e$f">"#,
        expected_markdown: Some(r"![a\\b\*c\_d\`e\$f](x)"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Images](../translation_approach.md#images).
/// An `alt` attribute holds text, but the label it lands in is inline content,
/// where a tag is a [raw HTML](https://spec.commonmark.org/0.31.2/#raw-html)
/// inline. Raw HTML is literal, so the backslashes escaping the Markdown
/// specials inside a tag are never removed: without the `<` escape,
/// `![a<b c="\*">d](x)` comes back with the alt text `a<b c="\*">d`. A text
/// node takes the same escape, from `escape_html`, once its Markdown specials
/// are escaped.
#[test]
fn an_image_alt_escapes_its_raw_html() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "an_image_alt_escapes_its_raw_html_case_22",
        html: r#"<img src="x" alt='a<b c="*">d'>"#,
        expected_markdown: Some(r#"![a\<b c="\*">d](x)"#),

        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p><img src=\"x\" alt=\"a&lt;b c=&quot;*&quot;&gt;d\" /></p>\n",
            reason: "the root image gains a paragraph and the renderer uses double quotes, character references, and a void slash",
            reference: "translation_approach.md#images",
        }),
        ..Case::default()
    }]);

    // A `<` which opens no tag is text in either reading, so it needs no
    // backslash -- `leaves_non_html_sequences` in `src/util/escape.rs` holds
    // the rest of that line.

    faithful_cases.extend([Case {
        id: "an_image_alt_escapes_its_raw_html_case_23",
        html: r#"<img src="x" alt="a < b">"#,
        expected_markdown: Some("![a < b](x)"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Links](../translation_approach.md#links).
/// `img_handler` writes a destination holding a space in the
/// [pointy bracket form](https://spec.commonmark.org/0.31.2/#link-destination),
/// where a `<` or `>` in the destination itself has to be escaped: written
/// literally, `![q](<a b>c>)` ends the destination at the first `>`, which
/// leaves `c>)` behind as text.
#[test]
fn a_bracketed_link_destination_escapes_its_angle_brackets() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_bracketed_link_destination_escapes_its_angle_brackets_case_24",
        html: r#"<img src="a b>c" alt="q">"#,
        expected_markdown: Some(r"![q](<a b\>c>)"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "![q](a%20b%3Ec)",
            reason: "pulldown-cmark percent-encodes the destination when rendering its CommonMark link or image",
            reference: "translation_approach.md#links",
        },

        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p><img src=\"a%20b%3Ec\" alt=\"q\" /></p>\n",
            reason: "the root image gains a paragraph and pulldown-cmark percent-encodes the destination space and angle bracket",
            reference: "translation_approach.md#links",
        }),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_bracketed_link_destination_escapes_its_angle_brackets_case_25",
        html: r#"<img src="a b<c" alt="q">"#,
        expected_markdown: Some(r"![q](<a b\<c>)"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "![q](a%20b%3Cc)",
            reason: "pulldown-cmark percent-encodes the destination when rendering its CommonMark link or image",
            reference: "translation_approach.md#links",
        },
        ..Case::default()
    }]);
    // pulldown-cmark percent-encodes a destination holding a space, so the
    // trip shows `a b>c` as `a%20b%3Ec` rather than as itself.
    check_cases(&faithful_cases);
}

/// [Links](../translation_approach.md#links).
/// An [autolink](https://spec.commonmark.org/0.31.2/#autolinks) takes what
/// sits between its angle brackets as the destination itself, so it is spelled
/// from the unescaped href: a backslash there is a character of the URL rather
/// than an escape.
#[test]
fn an_autolink_spells_its_destination_without_escapes() {
    let mut faithful_cases = Vec::new();
    assert_eq!(
        r"<https://example.com/a\b>",
        convert_autolinks(r#"<a href="https://example.com/a\b">https://example.com/a\b</a>"#)
    );
    faithful_cases.extend([Case {
        id: "an_autolink_spells_its_destination_without_escapes_case_1_autolinks",
        html: r#"<a href="https://example.com/a\b">https://example.com/a\b</a>"#,
        expected_markdown: Some(r"<https://example.com/a\b>"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: r"[https://example.com/a\\b](https://example.com/a%5Cb)",
            reason: "the rendered destination is percent-encoded while its text retains the backslash, so it no longer qualifies as an autolink",
            reference: "translation_approach.md#links",
        },
        htmd_options: Options {
            link_style: LinkStyle::InlinedPreferAutolinks,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);
    // Text-node escapes don't prevent an autolink.
    assert_eq!(
        "<https://example.com/a_b*c>",
        convert_autolinks(r#"<a href="https://example.com/a_b*c">https://example.com/a_b*c</a>"#)
    );
    faithful_cases.extend([Case {
        id: "an_autolink_spells_its_destination_without_escapes_case_2_autolinks",
        html: r#"<a href="https://example.com/a_b*c">https://example.com/a_b*c</a>"#,
        expected_markdown: Some("<https://example.com/a_b*c>"),
        htmd_options: Options {
            link_style: LinkStyle::InlinedPreferAutolinks,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);
    assert_eq!(
        "<p><a href=\"https://example.com/a_b*c\">https://example.com/a_b*c</a></p>\n",
        render_markdown(
            &convert_autolinks(
                r#"<a href="https://example.com/a_b*c">https://example.com/a_b*c</a>"#
            ),
            CommonMarkOptions::empty()
        )
    );
    check_cases(&faithful_cases);
}

/// [Links](../translation_approach.md#links).
/// The conditions for an autolink are listed in the links section of
/// `translation_approach.md`; failing any one gives the inline form.
#[test]
fn an_autolink_needs_every_condition() {
    let mut faithful_cases = Vec::new();
    // No autolink can hold a `<`.
    assert_eq!(
        r"[https://example.com/a\<b](https://example.com/a<b)",
        convert_autolinks(r#"<a href="https://example.com/a<b">https://example.com/a&lt;b</a>"#)
    );
    faithful_cases.extend([Case {
        id: "an_autolink_needs_every_condition_case_3_autolinks",
        html: r#"<a href="https://example.com/a<b">https://example.com/a&lt;b</a>"#,
        expected_markdown: Some(r"[https://example.com/a\<b](https://example.com/a<b)"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: r"[https://example.com/a\<b](https://example.com/a%3Cb)",
            reason: "pulldown-cmark percent-encodes the less-than sign in the destination",
            reference: "translation_approach.md#links",
        },
        htmd_options: Options {
            link_style: LinkStyle::InlinedPreferAutolinks,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);
    // An autolink would write a `&` as `&amp;`.
    assert_eq!(
        r"[https://example.com/?a\&b](https://example.com/?a\&b)",
        convert_autolinks(
            r#"<a href="https://example.com/?a&amp;b">https://example.com/?a&amp;b</a>"#
        )
    );
    faithful_cases.extend([Case {
        id: "an_autolink_needs_every_condition_case_4_autolinks",
        html: r#"<a href="https://example.com/?a&amp;b">https://example.com/?a&amp;b</a>"#,
        expected_markdown: Some(r"[https://example.com/?a\&b](https://example.com/?a\&b)"),
        htmd_options: Options {
            link_style: LinkStyle::InlinedPreferAutolinks,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);
    // An autolink can't hold a title.
    assert_eq!(
        r#"[https://example.com](https://example.com "t")"#,
        convert_autolinks(r#"<a href="https://example.com" title="t">https://example.com</a>"#)
    );
    faithful_cases.extend([Case {
        id: "an_autolink_needs_every_condition_case_5_autolinks",
        html: r#"<a href="https://example.com" title="t">https://example.com</a>"#,
        expected_markdown: Some(r#"[https://example.com](https://example.com "t")"#),
        htmd_options: Options {
            link_style: LinkStyle::InlinedPreferAutolinks,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);
    // The content must be a single text node.
    assert_eq!(
        "[*https://example.com*](https://example.com)",
        convert_autolinks(r#"<a href="https://example.com"><em>https://example.com</em></a>"#)
    );
    faithful_cases.extend([Case {
        id: "an_autolink_needs_every_condition_case_6_autolinks",
        html: r#"<a href="https://example.com"><em>https://example.com</em></a>"#,
        expected_markdown: Some("[*https://example.com*](https://example.com)"),
        htmd_options: Options {
            link_style: LinkStyle::InlinedPreferAutolinks,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);
    // A relative href is no absolute URI: written as an autolink it would go
    // out as raw HTML.

    faithful_cases.extend([Case {
        id: "an_autolink_needs_every_condition_case_7_autolinks",
        html: r#"<a href="a/b">a/b</a>"#,
        expected_markdown: Some("[a/b](a/b)"),
        htmd_options: Options {
            link_style: LinkStyle::InlinedPreferAutolinks,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);
    // A one-letter scheme is too short.

    faithful_cases.extend([Case {
        id: "an_autolink_needs_every_condition_case_8_autolinks",
        html: r#"<a href="a:b">a:b</a>"#,
        expected_markdown: Some("[a:b](a:b)"),
        htmd_options: Options {
            link_style: LinkStyle::InlinedPreferAutolinks,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Links](../translation_approach.md#links).
/// A
/// [link reference definition](https://spec.commonmark.org/0.31.2/#link-reference-definition)
/// takes the destination an inline link takes, so
/// `AnchorElementHandler::build_referenced_anchor` writes one holding a space
/// in the same pointy bracket form `build_inlined_anchor` writes. Written bare
/// the line opens no definition, so it would stay in the output as a paragraph
/// of text and the reference pointing at it would find nothing.
#[test]
fn a_referenced_link_destination_brackets_its_spaces() {
    check_cases(&[Case {
        id: "a_referenced_link_destination_brackets_its_spaces_case_9_referenced",
        html: r#"<a href="a b">t</a>"#,
        expected_markdown: Some("[t][1]\n\n[1]: <a b>"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "[t][1]\n\n[1]: a%20b",
            reason: "pulldown-cmark percent-encodes the space in the reference destination",
            reference: "translation_approach.md#links",
        },
        htmd_options: Options {
            link_style: LinkStyle::Referenced,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);
}

/// [Links](../translation_approach.md#links).
/// A `!` immediately before a link would make it an image.
#[test]
fn a_bang_before_a_link_is_escaped() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_bang_before_a_link_is_escaped_case_26",
        html: r#"<p>Hi!<a href="x">t</a></p>"#,
        expected_markdown: Some(r"Hi\![t](x)"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    assert_eq!(
        r"Hi\![t][1]",
        convert_referenced(r#"<p>Hi!<a href="x">t</a></p>"#)
            .lines()
            .next()
            .unwrap()
    );
    // The link needn't be the `!`'s sibling.
    assert_eq!(
        r"Hi\![t](x)",
        convert_pure(r#"<p>Hi!<span><a href="x">t</a></span></p>"#).unwrap()
    );
    // A `\` before the `!` is text, escaped by its own backslash.

    faithful_cases.extend([Case {
        id: "a_bang_before_a_link_is_escaped_case_27",
        html: r#"<p>\!<a href="x">t</a></p>"#,
        expected_markdown: Some(r"\\\![t](x)"),
        ..Case::default()
    }]);
    // Anything between the two leaves the `!` alone.

    faithful_cases.extend([Case {
        id: "a_bang_before_a_link_is_escaped_case_28",
        html: r#"<p>Hi! <a href="x">t</a></p>"#,
        expected_markdown: Some("Hi! [t](x)"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_bang_before_a_link_is_escaped_case_29",
        html: r#"<p><img src="i" alt="a"><img src="j" alt="b"></p>"#,
        expected_markdown: Some("![a](i)![b](j)"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Links](../translation_approach.md#links).
/// Every `&` in link text, a destination, a title or an image description is
/// escaped, so that `&amp;copy;` comes back as the text `&copy;` rather than
/// the character it names.
#[test]
fn a_link_escapes_its_ampersands() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_link_escapes_its_ampersands_case_30",
        html: r#"<a href="a&amp;copy;b" title="t&amp;copy;">x&amp;copy;</a>"#,
        expected_markdown: Some(r#"[x\&copy;](a\&copy;b "t\&copy;")"#),

        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p><a href=\"a&amp;copy;b\" title=\"t&amp;copy;\">x&amp;copy;</a></p>\n",
            reason: "the root link gains a paragraph; escaped ampersands preserve the link values",
            reference: "translation_approach.md#links",
        }),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_link_escapes_its_ampersands_case_31",
        html: r#"<img src="i&amp;copy;" alt="a&amp;copy;" title="t&amp;copy;">"#,
        expected_markdown: Some(r#"![a\&copy;](i\&copy; "t\&copy;")"#),

        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p><img src=\"i&amp;copy;\" alt=\"a&amp;copy;\" title=\"t&amp;copy;\" /></p>\n",
            reason: "the root image gains a paragraph and a void slash; escaped ampersands preserve the image values",
            reference: "translation_approach.md#links",
        }),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_link_escapes_its_ampersands_case_10_referenced",
        html: r#"<a href="a&amp;b">t</a>"#,
        expected_markdown: Some(
            r"[t][1]

[1]: a\&b",
        ),
        htmd_options: Options {
            link_style: LinkStyle::Referenced,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Links](../translation_approach.md#links).
/// A `$` in link text is escaped, since math binds more tightly than a link;
/// for the same reason, math in link text needs no escapes.
#[test]
fn link_text_escapes_its_math_delimiters() {
    let mut faithful_cases = Vec::new();
    let math = CommonMarkOptions::ENABLE_MATH;
    let html = r#"<p><a href="u">$a](v)$</a></p>"#;

    faithful_cases.extend([Case {
        id: "link_text_escapes_its_math_delimiters_case_32",
        html,
        expected_markdown: Some(r"[\$a\](v)\$](u)"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        ..Case::default()
    }]);
    assert_eq!(
        format!("{html}\n"),
        render_markdown(&convert_faithful(html).unwrap(), math)
    );
    let html = r#"<p><a href="u"><span class="math math-inline">a](v)</span></a></p>"#;

    faithful_cases.extend([Case {
        id: "link_text_escapes_its_math_delimiters_case_33",
        html,
        expected_markdown: Some("[$a](v)$](u)"),
        markdown_options: CommonMarkOptions::ENABLE_MATH,
        ..Case::default()
    }]);
    assert_eq!(
        format!("{html}\n"),
        render_markdown(&convert_faithful(html).unwrap(), math)
    );
    check_cases(&faithful_cases);
}

/// [Links](../translation_approach.md#links).
/// A destination takes the pointy bracket form when it is empty, starts with a
/// `<`, or holds a space or ASCII control character. Only that form escapes a
/// `<` or `>`.
#[test]
fn a_link_destination_takes_pointy_brackets_where_needed() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_link_destination_takes_pointy_brackets_where_needed_case_34",
        html: r#"<a href="">t</a>"#,
        expected_markdown: Some("[t](<>)"),

        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p><a href=\"\">t</a></p>\n",
            reason: "the root link gains a paragraph",
            reference: "translation_approach.md#links",
        }),
        ..Case::default()
    }]);

    // A bare empty destination opens no link reference definition.

    faithful_cases.extend([Case {
        id: "a_link_destination_takes_pointy_brackets_where_needed_case_11_referenced",
        html: r#"<a href="">t</a>"#,
        expected_markdown: Some("[t][1]\n\n[1]: <>"),
        htmd_options: Options {
            link_style: LinkStyle::Referenced,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_link_destination_takes_pointy_brackets_where_needed_case_35",
        html: r#"<a href="&lt;a">t</a>"#,
        expected_markdown: Some(r"[t](<\<a>)"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "[t](%3Ca)",
            reason: "pulldown-cmark percent-encodes the destination when rendering its CommonMark link or image",
            reference: "translation_approach.md#links",
        },

        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p><a href=\"%3Ca\">t</a></p>\n",
            reason: "the root link gains a paragraph and pulldown-cmark percent-encodes the destination angle bracket",
            reference: "translation_approach.md#links",
        }),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_link_destination_takes_pointy_brackets_where_needed_case_36",
        html: "<a href=\"a\tb\">t</a>",
        expected_markdown: Some("[t](<a\tb>)"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "[t](a%09b)",
            reason: "pulldown-cmark percent-encodes the destination when rendering its CommonMark link or image",
            reference: "translation_approach.md#links",
        },

        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p><a href=\"a%09b\">t</a></p>\n",
            reason: "the root link gains a paragraph and pulldown-cmark percent-encodes the destination tab",
            reference: "translation_approach.md#links",
        }),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_link_destination_takes_pointy_brackets_where_needed_case_37",
        html: r#"<a href="a&lt;b&gt;c">t</a>"#,
        expected_markdown: Some("[t](a<b>c)"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "[t](a%3Cb%3Ec)",
            reason: "pulldown-cmark percent-encodes the destination when rendering its CommonMark link or image",
            reference: "translation_approach.md#links",
        },

        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p><a href=\"a%3Cb%3Ec\">t</a></p>\n",
            reason: "the root link gains a paragraph and pulldown-cmark percent-encodes the destination angle brackets",
            reference: "translation_approach.md#links",
        }),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_link_destination_takes_pointy_brackets_where_needed_case_38",
        html: r#"<img src="" alt="q">"#,
        expected_markdown: Some("![q](<>)"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Links](../translation_approach.md#links).
/// A line ending in link text would end a heading or table cell holding the
/// link, and a blank line would end any paragraph, so each is encoded.
#[test]
fn link_text_encodes_its_line_endings() {
    let mut faithful_cases = Vec::new();
    // Block content gives the link text a blank line in pure mode.
    assert_eq!(
        "[&#10;&#10;a&#10;&#10;b&#10;&#10;](x)",
        convert_pure(r#"<a href="x"><p>a</p><p>b</p></a>"#).unwrap()
    );
    // A link begins an inline context, so faithful mode writes that content as
    // raw HTML inlines instead, which survive the trip back to HTML.

    faithful_cases.extend([Case {
        id: "link_text_encodes_its_line_endings_case_39",
        html: r#"<a href="x"><p>a</p><p>b</p></a>"#,
        expected_markdown: Some("[<p>a</p><p>b</p>](x)"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "[](x)\n\n[a](x)\n\n[b](x)",
            reason: "HTML parser repairs split block children of a link into separate linked paragraphs",
            reference: "translation_approach.md#translating-html-nodes",
        },

        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p><a href=\"x\"><p>a</p><p>b</p></a></p>\n",
            reason: "the root link gains a paragraph and the HTML parser repairs block children inside a link",
            reference: "translation_approach.md#links",
        }),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "link_text_encodes_its_line_endings_case_12_referenced",
        html: r#"<a href="x"><p>a</p><p>b</p></a>"#,
        expected_markdown: Some("[<p>a</p><p>b</p>][1]\n\n[1]: x"),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "[][1]\n\n[a][2]\n\n[b][3]\n\n[1]: x\n[2]: x\n[3]: x",
            reason: "HTML parser repairs split block children of a link into separate linked paragraphs",
            reference: "translation_approach.md#translating-html-nodes",
        },
        htmd_options: Options {
            link_style: LinkStyle::Referenced,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);
    // A pure-mode `<br>` is a line ending too.
    assert_eq!(
        "# [a  &#10;b](x)",
        convert_pure(r#"<h1><a href="x">a<br>b</a></h1>"#).unwrap()
    );
    check_cases(&faithful_cases);
}

/// [Images](../translation_approach.md#images).
/// An image is written inline even when links are referenced.
#[test]
fn an_image_is_inline_in_the_referenced_style() {
    check_cases(&[Case {
        id: "an_image_is_inline_in_the_referenced_style_case_13_referenced",
        html: r#"<img src="i" alt="a" title="t">"#,
        expected_markdown: Some(r#"![a](i "t")"#),
        htmd_options: Options {
            link_style: LinkStyle::Referenced,
            ..Case::default().htmd_options
        },
        ..Case::default()
    }]);
}

/// [Links](../translation_approach.md#links).
/// Whitespace at either end of the link text stays inside the brackets, so the
/// text reads back unchanged.
#[test]
fn a_link_keeps_the_whitespace_around_its_text() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_link_keeps_the_whitespace_around_its_text_case_40",
        html: r#"<p>Hi!<a href="x"> t</a></p>"#,
        expected_markdown: Some("Hi\\![ t](x)"),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    assert_eq!(
        "Hi\\![ t][1]",
        convert_referenced(r#"<p>Hi!<a href="x"> t</a></p>"#)
            .lines()
            .next()
            .unwrap()
    );
    // A space on each side of the `<a>` tag collapses to one.

    faithful_cases.extend([Case {
        id: "a_link_keeps_the_whitespace_around_its_text_case_41",
        html: r#"<p>Hi! <a href="x"> t</a></p>"#,
        expected_markdown: Some("Hi! [ t](x)"),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_link_keeps_the_whitespace_around_its_text_case_42",
        html: r#"<p><a href="x">t </a>now</p>"#,
        expected_markdown: Some("[t ](x)now"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Images](../translation_approach.md#images).
/// An `<img>` without a `src` has no Markdown spelling -- `![q](<>)` reads
/// back with an empty `src` -- so it is written as raw HTML rather than dropped
/// along with its alt text.
#[test]
fn an_image_without_a_source_is_written_as_html() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "an_image_without_a_source_is_written_as_html_case_43",
        html: r#"<p>a<img alt="q">b</p>"#,
        expected_markdown: Some(r#"a<img alt="q">b"#),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Images](../translation_approach.md#images).
/// An `<img>` without an `alt` has no Markdown spelling -- `![](i)` reads back
/// with `alt=""`, which [means](https://html.spec.whatwg.org/multipage/images.html#alt)
/// a decorative image rather than one lacking a text alternative -- so it is
/// written as raw HTML.
#[test]
fn an_image_without_alt_text_is_written_as_html() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "an_image_without_alt_text_is_written_as_html_case_44",
        html: r#"<p>a<img src="i">b</p>"#,
        expected_markdown: Some(r#"a<img src="i">b"#),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    // An empty `alt` is the one `![](i)` spells.

    faithful_cases.extend([Case {
        id: "an_image_without_alt_text_is_written_as_html_case_45",
        html: r#"<img src="i" alt="">"#,
        expected_markdown: Some("![](i)"),

        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p><img src=\"i\" alt=\"\" /></p>\n",
            reason: "the root image gains a paragraph and a void slash",
            reference: "translation_approach.md#images",
        }),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Images](../translation_approach.md#images).
/// An `<img>` with an empty `title` has no Markdown spelling -- `![q](i "")`
/// reads back without a `title`, which
/// [lets](https://html.spec.whatwg.org/multipage/dom.html#the-title-attribute)
/// an ancestor's advisory information apply where the empty one hid it -- so
/// it is written as raw HTML.
#[test]
fn an_image_with_an_empty_title_is_written_as_html() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "an_image_with_an_empty_title_is_written_as_html_case_46",
        html: r#"<p>a<img src="i" alt="q" title="">b</p>"#,
        expected_markdown: Some(r#"a<img src="i" alt="q" title="">b"#),
        expectation: Some(HtmlExpectation::Preserved {
            renderer_suffix: "\n",
        }),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Links](../translation_approach.md#links).
/// An [email autolink](https://spec.commonmark.org/0.31.2/#email-autolink) may
/// open with a digit, a `/` or a symbol after its `<`, where a tag needs a
/// letter. Unescaped, one in link text nests a `mailto:` link inside the link,
/// and one in an alt text comes back without its angle brackets.
#[test]
fn a_link_escapes_an_email_autolink() {
    let mut faithful_cases = Vec::new();
    faithful_cases.extend([Case {
        id: "a_link_escapes_an_email_autolink_case_47",
        html: r#"<a href="x">&lt;1foo@bar.com&gt;</a>"#,
        expected_markdown: Some(r"[\<1foo@bar.com>](x)"),

        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p><a href=\"x\">&lt;1foo@bar.com&gt;</a></p>\n",
            reason: "the root link gains a paragraph while its text keeps the escaped angle brackets",
            reference: "translation_approach.md#links",
        }),
        ..Case::default()
    }]);

    faithful_cases.extend([Case {
        id: "a_link_escapes_an_email_autolink_case_48",
        html: r#"<img src="i" alt="<+a@b.co>">"#,
        expected_markdown: Some(r"![\<+a@b.co>](i)"),

        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: "<p><img src=\"i\" alt=\"&lt;+a@b.co&gt;\" /></p>\n",
            reason: "the root image gains a paragraph and a void slash; the alt text's angle brackets become references",
            reference: "translation_approach.md#links",
        }),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// `convert_faithful` with [`LinkStyle::InlinedPreferAutolinks`], the style
/// whose link text and destination decide between the two spellings.
fn convert_autolinks(html: &str) -> String {
    convert_faithful_options(
        html,
        Options {
            link_style: LinkStyle::InlinedPreferAutolinks,
            ..Default::default()
        },
    )
    .unwrap()
}

/// `convert_faithful` with [`LinkStyle::Referenced`], the style which writes
/// the destination in a link reference definition rather than inline.
fn convert_referenced(html: &str) -> String {
    convert_faithful_options(
        html,
        Options {
            link_style: LinkStyle::Referenced,
            ..Default::default()
        },
    )
    .unwrap()
}
