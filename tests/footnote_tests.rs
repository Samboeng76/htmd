mod common;

use common::{convert_faithful, convert_pure, render_markdown};
use pulldown_cmark::Options;

const ONE_NOTE: &str = r##"<p>a statement<sup class="footnote-reference"><a href="#1">1</a></sup></p>
<div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup>
<p>this is true</p>
</div>"##;

fn render_footnotes(markdown: &str) -> String {
    render_markdown(markdown, Options::ENABLE_FOOTNOTES)
}

#[test]
fn translates_the_pulldown_cmark_example() {
    let markdown = convert_faithful(ONE_NOTE).unwrap();
    assert_eq!("a statement[^1]\n\n[^1]: this is true", markdown);
    let rendered = render_footnotes(&markdown);
    assert!(rendered.contains("<sup class=\"footnote-reference\"><a href=\"#1\">1</a></sup>"));
    assert!(rendered.contains("<div class=\"footnote-definition\" id=\"1\">"));
}

#[test]
fn keeps_multiple_references_and_the_definition_in_source_order() {
    let html = r##"<p>first<sup class="footnote-reference"><a href="#1">1</a></sup> and second<sup class="footnote-reference"><a href="#1">1</a></sup>.</p><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><p>note</p></div><p>after</p>"##;
    let markdown = convert_faithful(html).unwrap();
    assert_eq!("first[^1] and second[^1].\n\n[^1]: note\n\nafter", markdown);
    let rendered = render_footnotes(&markdown);
    assert_eq!(2, rendered.matches("class=\"footnote-reference\"").count());
    assert!(rendered.find("</div>").unwrap() < rendered.find("<p>after</p>").unwrap());
}

#[test]
fn falls_back_when_a_reference_is_preserved_inside_html() {
    let html = r##"<div class="box"><p>raw<sup class="footnote-reference"><a href="#1">1</a></sup></p></div><p>converted<sup class="footnote-reference"><a href="#1">1</a></sup></p><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><p>note</p></div>"##;
    let markdown = convert_faithful(html).unwrap();
    assert_eq!(
        2,
        markdown
            .matches("<sup class=\"footnote-reference\">")
            .count()
    );
    assert!(markdown.contains("<div class=\"footnote-definition\" id=\"1\">"));
    assert!(!markdown.contains("[^1]"));
}

#[test]
fn indents_following_paragraphs_inside_the_definition() {
    let html = r##"<p>x<sup class="footnote-reference"><a href="#1">1</a></sup></p><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><p>first</p><p>second</p></div>"##;
    let markdown = convert_faithful(html).unwrap();
    assert_eq!("x[^1]\n\n[^1]: first\n\n    second", markdown);
    let rendered = render_footnotes(&markdown);
    assert!(rendered.contains("<p>first</p>"));
    assert!(rendered.contains("<p>second</p>"));
}

#[test]
fn leaves_an_ordinary_link_to_the_target_resolvable() {
    let html = format!("{ONE_NOTE}<p><a href=\"#1\">see note</a></p>");
    let markdown = convert_faithful(&html).unwrap();
    assert!(markdown.contains("<sup class=\"footnote-reference\"><a href=\"#1\">1</a></sup>"));
    assert!(markdown.contains("<div class=\"footnote-definition\" id=\"1\">"));
    assert!(!markdown.contains("[^1]:"));
}

#[test]
fn rejects_an_ambiguous_id_and_a_base_href() {
    let duplicate = format!("{ONE_NOTE}<span id=\"1\"></span>");
    let based = format!("<base href=\"https://example.com/other\">{ONE_NOTE}");
    for html in [duplicate, based] {
        let markdown = convert_faithful(&html).unwrap();
        assert!(markdown.contains("<div class=\"footnote-definition\" id=\"1\">"));
        assert!(!markdown.contains("[^1]:"));
    }
}

#[test]
fn faithful_mode_rejects_extra_attributes_and_a_changed_fragment() {
    let extra = ONE_NOTE.replace(
        "class=\"footnote-reference\"",
        "class=\"footnote-reference\" title=\"source\"",
    );
    let encoded = ONE_NOTE.replace("href=\"#1\"", "href=\"#%31\"");
    for html in [extra, encoded] {
        let markdown = convert_faithful(&html).unwrap();
        assert!(markdown.contains("<sup class=\"footnote-reference\""));
        assert!(!markdown.contains("[^1]:"));
    }
}

#[test]
fn preserves_a_rejected_reference_inside_a_heading_with_a_multiline_attribute() {
    let html = r##"<h1>heading<sup class="footnote-reference" title="first&#10;second"><a href="#1">1</a></sup></h1><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><p>note</p></div>"##;
    let markdown = convert_faithful(html).unwrap();
    assert!(markdown.contains("title=\"first&#10;second\""), "{markdown}");
    assert!(!markdown.contains("[^1]"));
    let rendered = render_footnotes(&markdown);
    assert!(rendered.contains("<h1>heading<sup"), "{rendered}");
    assert!(rendered.contains("<a href=\"#1\">1</a></sup></h1>"), "{rendered}");
}

#[test]
fn pure_mode_can_use_the_display_number_as_a_label() {
    let html = r##"<p>x<sup class="footnote-reference" title="source"><a href="#note" data-x="y">1</a></sup></p><div class="footnote-definition" id="note" data-x="y"><p>text</p></div>"##;
    let markdown = convert_pure(html).unwrap();
    assert_eq!("x[^1]\n\n[^1]: text", markdown);
    assert!(render_footnotes(&markdown).contains("href=\"#1\""));
}

#[test]
fn leaves_mismatched_display_numbers_as_html() {
    let html = ONE_NOTE.replace(
        "footnote-definition-label\">1",
        "footnote-definition-label\">2",
    );
    let markdown = convert_faithful(&html).unwrap();
    assert!(markdown.contains("<div class=\"footnote-definition\" id=\"1\">"));
    assert!(!markdown.contains("[^1]:"));
}

#[test]
fn falls_back_for_all_notes_when_one_cannot_convert() {
    let second = ONE_NOTE
        .replace("#1", "#2")
        .replace("id=\"1\"", "id=\"2\"")
        .replace(">1<", ">2<");
    let second = second.replace(
        "footnote-reference\"",
        "footnote-reference\" title=\"extra\"",
    );
    let html = format!("{ONE_NOTE}{second}");
    let markdown = convert_faithful(&html).unwrap();
    assert!(!markdown.contains("[^1]:"));
    assert!(!markdown.contains("[^2]:"));
    assert_eq!(
        2,
        markdown
            .matches("<div class=\"footnote-definition\"")
            .count()
    );
}

#[test]
fn rejects_a_definition_nested_inside_another() {
    let html = r##"<p>x<sup class="footnote-reference"><a href="#1">1</a></sup></p><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><p>first</p><div class="footnote-definition" id="2"><sup class="footnote-definition-label">2</sup><p>second</p></div></div>"##;
    let markdown = convert_faithful(html).unwrap();
    assert!(markdown.contains("<div class=\"footnote-definition\" id=\"1\">"));
    assert!(!markdown.contains("[^1]:"));
}

#[test]
fn pure_mode_accepts_empty_wrappers_around_the_structural_elements() {
    let html = r##"<p>x<sup class="footnote-reference"><em> <a href="#note">1</a> </em></sup></p><div class="footnote-definition" id="note"><span> <sup class="footnote-definition-label">1</sup> </span><p>text</p></div>"##;
    assert_eq!("x[^1]\n\n[^1]: text", convert_pure(html).unwrap());
}

#[test]
fn a_full_url_does_not_count_as_a_same_document_link() {
    let html = format!("{ONE_NOTE}<p><a href=\"https://example.com/page#1\">elsewhere</a></p>");
    let markdown = convert_faithful(&html).unwrap();
    assert!(markdown.contains("[^1]: this is true"));
}

#[test]
fn faithful_mode_keeps_a_nonnumeric_id_when_the_display_number_survives() {
    let html = ONE_NOTE
        .replace("#1", "#note")
        .replace("id=\"1\"", "id=\"note\"");
    let markdown = convert_faithful(&html).unwrap();
    assert_eq!("a statement[^note]\n\n[^note]: this is true", markdown);
    let rendered = render_footnotes(&markdown);
    assert!(rendered.contains("href=\"#note\">1</a>"));
    assert!(rendered.contains("id=\"note\"><sup class=\"footnote-definition-label\">1</sup>"));
}

#[test]
fn places_a_first_nonparagraph_block_below_the_marker() {
    let html = r##"<p>x<sup class="footnote-reference"><a href="#1">1</a></sup></p><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><ul><li>item</li></ul></div>"##;
    let markdown = convert_faithful(html).unwrap();
    assert!(markdown.contains("[^1]:\n    *"), "{markdown}");
    let rendered = render_footnotes(&markdown);
    assert!(rendered.contains("<ul>"));
    assert!(rendered.contains("<li>item</li>"));
}

#[test]
fn checks_the_display_numbers_across_the_complete_document() {
    let html = r##"<p>x<sup class="footnote-reference"><a href="#2">2</a></sup> y<sup class="footnote-reference"><a href="#1">1</a></sup></p><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><p>one</p></div><div class="footnote-definition" id="2"><sup class="footnote-definition-label">2</sup><p>two</p></div>"##;
    let markdown = convert_faithful(html).unwrap();
    assert!(!markdown.contains("[^1]:"));
    assert!(!markdown.contains("[^2]:"));
    assert_eq!(
        2,
        markdown
            .matches("<div class=\"footnote-definition\"")
            .count()
    );
}

#[test]
fn retains_inline_formatting_in_a_note_body() {
    let html = r##"<p>x<sup class="footnote-reference"><a href="#1">1</a></sup></p><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><p>some <em>emphasis</em></p></div>"##;
    let markdown = convert_faithful(html).unwrap();
    assert_eq!("x[^1]\n\n[^1]: some *emphasis*", markdown);
    assert!(render_footnotes(&markdown).contains("<em>emphasis</em>"));
}

#[test]
fn resolves_utf8_percent_encoded_fragments_before_pure_conversion() {
    let html = r##"<p>x<sup class="footnote-reference"><a href="#caf%C3%A9">1</a></sup></p><div class="footnote-definition" id="café"><sup class="footnote-definition-label">1</sup><p>text</p></div>"##;
    assert_eq!("x[^1]\n\n[^1]: text", convert_pure(html).unwrap());
    let faithful = convert_faithful(html).unwrap();
    assert!(faithful.contains("href=\"#caf%C3%A9\""));
    assert!(!faithful.contains("[^café]:"));
}
