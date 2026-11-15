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

/// [HTML comparison](../spec.md#3-html-comparison-and-correctness-criteria).
#[test]
fn template_content_changes_fail() {
    for (expected, actual) in [
        (
            "<template><p>a</p></template>",
            "<template><p>b</p></template>",
        ),
        ("<template><p>a</p></template>", "<template></template>"),
        (
            "<template><span title=\"a\">x</span></template>",
            "<template><span title=\"b\">x</span></template>",
        ),
        (
            "<template><template>a</template></template>",
            "<template><template>b</template></template>",
        ),
        (
            "<template><!--a--></template>",
            "<template><!--b--></template>",
        ),
        (
            "<template><code>a\nb</code></template>",
            "<template><code>a b</code></template>",
        ),
        (
            "<pre><template><span>a\tb</span></template></pre>",
            "<pre><template><span>a b</span></template></pre>",
        ),
        (
            "<template><em>a</em> <strong>b</strong></template>",
            "<template><em>a</em><strong>b</strong></template>",
        ),
    ] {
        assert!(
            compare_html(expected, actual).is_err(),
            "{expected:?} vs {actual:?}"
        );
    }
}

/// [HTML comparison](../spec.md#3-html-comparison-and-correctness-criteria).
#[test]
fn template_formatting_preserves_separate_text_flows() {
    for (expected, actual) in [
        (
            "<template><p>a\t b</p></template>",
            "<template>\n<p>a b</p>\n</template>",
        ),
        (
            "<p>a<template>x</template> b</p>",
            "<p>a <template>x</template>b</p>",
        ),
        (
            "<template><em>a </em><strong>b</strong></template>",
            "<template><em>a</em> <strong>b</strong></template>",
        ),
    ] {
        assert!(
            compare_html(expected, actual).is_ok(),
            "{expected:?} vs {actual:?}"
        );
    }
    assert!(
        compare_html(
            "<p>a<template>x</template> b</p>",
            "<p>a<template>x</template>b</p>",
        )
        .is_err()
    );
}

/// [HTML comparison](../spec.md#3-html-comparison-and-correctness-criteria).
#[test]
fn foreign_elements_do_not_create_html_block_boundaries() {
    let expected = "<svg><text>a</text><title> b</title><text>c</text></svg>";
    let missing_space = "<svg><text>a</text><title>b</title><text>c</text></svg>";
    assert!(compare_html(expected, missing_space).is_err());
    assert!(compare_html(missing_space, expected).is_err());
    assert!(
        compare_html(
            expected,
            "<svg><text>a</text><title>\t b</title><text>c</text></svg>",
        )
        .is_ok()
    );
}
