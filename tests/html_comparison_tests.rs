mod common;

use common::{Case, HtmlExpectation, check_cases, compare_html};

/// [HTML comparison](../spec.md#3-html-comparison-and-correctness-criteria).
#[test]
fn formatting_whitespace_and_inline_boundary_spaces_are_preserved() {
    for (expected, actual) in [
        ("<p>Hello</p>", "<p>Hello</p>\n"),
        ("<p>a\t \nb</p>", "<p>a b</p>\n"),
        ("<p>a</p><p>b</p>", "<p>a</p>\n<p>b</p>\n"),
        (
            "<p><em>a </em><strong>b</strong></p>",
            "<p><em>a</em> <strong>b</strong></p>",
        ),
        ("<p>&#65;</p>", "<p>A</p>\n"),
        ("<p a=\"1\" b=\"2\">x</p>", "<p b=\"2\" a=\"1\">x</p>"),
        ("<p>a\r\nb</p>", "<p>a\nb</p>"),
        (
            "<script><code>a\tb</code></script>",
            "<script><code>a b</code></script>",
        ),
    ] {
        assert!(
            compare_html(expected, actual).is_ok(),
            "{expected:?} vs {actual:?}"
        );
    }
}

/// [HTML comparison](../spec.md#3-html-comparison-and-correctness-criteria).
#[test]
fn content_and_protected_whitespace_changes_fail() {
    for (expected, actual) in [
        ("<p>a b</p>", "<p>ab</p>\n"),
        (
            "<p><em>a</em> <strong>b</strong></p>",
            "<p><em>a</em><strong>b</strong></p>",
        ),
        ("<p>a\u{00a0}b</p>", "<p>a b</p>"),
        ("<p>a\u{000b}b</p>", "<p>a b</p>"),
        (
            "<pre><code>a\nb\n</code></pre>",
            "<pre><code>a b\n</code></pre>\n",
        ),
        (
            "<pre><span>a\tb</span></pre>",
            "<pre><span>a b</span></pre>",
        ),
        ("<code>a\r\nb</code>", "<code>a b</code>"),
        ("<p title=\"a b\">x</p>", "<p title=\"ab\">x</p>"),
        ("<!--a b-->", "<!--ab-->"),
        ("<p>a</p>", "<div>a</div>"),
        ("<br><br>", "<p><br><br></p>\n"),
    ] {
        assert!(
            compare_html(expected, actual).is_err(),
            "{expected:?} vs {actual:?}"
        );
    }
}

/// [Common case format](../spec.md#13-common-test-case-format).
#[test]
fn case_label_is_optional() {
    check_cases(&[Case {
        html: "<p>Hello</p>",
        expectation: Some(HtmlExpectation::Preserved),
        ..Case::default()
    }]);
}
