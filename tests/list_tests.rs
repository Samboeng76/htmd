use htmd::options::Options;
mod common;
use common::{Case, DifferenceKind, HtmlExpectation, StabilityExpectation, check_cases};

/// [Lists](../translation_approach.md#lists) retain the items in renderer layout.
#[test]
fn simple_lists_round_trip() {
    check_cases(&[
        Case {
            id: "simple_unordered_list",
            html: "<ul>\n<li>One</li>\n<li>Two</li>\n</ul>",
            expected_markdown: Some("*   One\n*   Two"),
            expectation: Some(HtmlExpectation::Preserved {
                renderer_suffix: "\n",
            }),
            ..Case::default()
        },
        Case {
            id: "simple_ordered_list",
            html: "<ol>\n<li>One</li>\n<li>Two</li>\n</ol>",
            expected_markdown: Some("1.  One\n2.  Two"),
            expectation: Some(HtmlExpectation::Preserved {
                renderer_suffix: "\n",
            }),
            ..Case::default()
        },
    ]);
}

/// [Lists](../translation_approach.md#lists).
#[test]
fn unordered_lists() {
    let html = r#"
        <ul>
            <li>Item 1</li>
            <li>Item 2</li>
            <li>Item 3</li>
        </ul>
        "#;
    check_cases(&[Case {
        id: "unordered_lists_original",
        html,
        expected_markdown: Some("*   Item 1\n*   Item 2\n*   Item 3"),
        ..Case::default()
    }]);
}

/// [Lists](../translation_approach.md#lists).
#[test]
fn unordered_lists_custom_bullet_spacing() {
    let html = r#"
        <ul>
            <li>Item 1</li>
            <li>Item 2</li>
            <li>Item 3</li>
        </ul>
        "#;
    check_cases(&[Case {
        id: "unordered_lists_custom_bullet_spacing",
        html,
        htmd_options: Options {
            ul_bullet_spacing: 2,
            ..Case::default().htmd_options
        },
        expected_markdown: Some("*  Item 1\n*  Item 2\n*  Item 3"),
        ..Case::default()
    }]);
}

/// [Lists](../translation_approach.md#lists).
#[test]
fn ordered_lists() {
    let html = r#"
        <ol>
            <li>Item 1</li>
            <li>Item 2</li>
            <li>Item 3</li>
        </ol>
        "#;
    check_cases(&[Case {
        id: "ordered_lists_original",
        html,
        expected_markdown: Some("1.  Item 1\n2.  Item 2\n3.  Item 3"),
        ..Case::default()
    }]);
}

/// [Lists](../translation_approach.md#lists).
#[test]
fn ordered_lists_custom_bullet_spacing() {
    let html = r#"
        <ol>
            <li>Item 1</li>
            <li>Item 2</li>
            <li>Item 3</li>
        </ol>
        "#;
    check_cases(&[Case {
        id: "ordered_lists_custom_bullet_spacing",
        html,
        htmd_options: Options {
            ol_number_spacing: 1,
            ..Case::default().htmd_options
        },
        expected_markdown: Some("1. Item 1\n2. Item 2\n3. Item 3"),
        ..Case::default()
    }]);
}

/// [Lists](../translation_approach.md#lists).
#[test]
fn ordered_lists_start_with_zero_or_negative() {
    let html = r#"
        <ol start="0">
            <li>Item 1</li>
            <li>Item 2</li>
            <li>Item 3</li>
        </ol>
        <ol start="-100">
            <li>Item 1</li>
            <li>Item 2</li>
            <li>Item 3</li>
        </ol>
        "#;
    check_cases(&[Case {
        id: "ordered_lists_start_with_zero_or_negative",
        html,
        htmd_options: Options {
            ol_number_spacing: 1,
            ..Case::default().htmd_options
        },
        expected_markdown: Some(
            "1. Item 1\n2. Item 2\n3. Item 3\n\n1. Item 1\n2. Item 2\n3. Item 3",
        ),
        expectation: Some(HtmlExpectation::DocumentedDifference {
            kind: DifferenceKind::Loss,
            expected_html: concat!(
                "<ol>\n<li>\n<p>Item 1</p>\n</li>\n",
                "<li>\n<p>Item 2</p>\n</li>\n",
                "<li>\n<p>Item 3</p>\n</li>\n",
                "<li>\n<p>Item 1</p>\n</li>\n",
                "<li>\n<p>Item 2</p>\n</li>\n",
                "<li>\n<p>Item 3</p>\n</li>\n</ol>\n",
            ),
            reason: "both nonpositive starts become one, and the two lists merge into a loose list with item paragraphs",
            reference: "translation_approach.md#lists",
        }),
        stability: StabilityExpectation::DocumentedInstability {
            second_markdown: "1. Item 1\n\n2. Item 2\n\n3. Item 3\n\n4. Item 1\n\n5. Item 2\n\n6. Item 3",
            reason: "the blank line joins adjacent ordered lists into one loose list, losing the second start",
            reference: "translation_approach.md#lists",
        },
        ..Case::default()
    }]);
}

/// [Lists](../translation_approach.md#lists).
#[test]
fn list_spacing_outside_the_commonmark_range() {
    let ul_html = "<ul><li>Item 1</li><li>Item 2</li></ul>";
    let ol_html = "<ol><li>Item 1</li><li>Item 2</li></ol>";

    // A nested list is indented to its parent item's content column, so the
    // clamp keeps it a list: past four spaces, CommonMark reads that
    // indentation as a code block instead.
    let nested_ul_html = "<ul><li>Outer<ul><li>Inner</li></ul></li></ul>";
    let nested_ol_html = "<ol><li>Outer<ol><li>Inner</li></ol></li></ol>";

    check_cases(&[
        Case {
            id: "ul_spacing_clamps_to_one",
            html: ul_html,
            htmd_options: Options {
                ul_bullet_spacing: 0,
                ..Case::default().htmd_options
            },
            expected_markdown: Some("* Item 1\n* Item 2"),
            ..Case::default()
        },
        Case {
            id: "ul_spacing_clamps_to_four",
            html: ul_html,
            htmd_options: Options {
                ul_bullet_spacing: 9,
                ..Case::default().htmd_options
            },
            expected_markdown: Some("*    Item 1\n*    Item 2"),
            ..Case::default()
        },
        Case {
            id: "ol_spacing_clamps_to_one",
            html: ol_html,
            htmd_options: Options {
                ol_number_spacing: 0,
                ..Case::default().htmd_options
            },
            expected_markdown: Some("1. Item 1\n2. Item 2"),
            ..Case::default()
        },
        Case {
            id: "ol_spacing_clamps_to_four",
            html: ol_html,
            htmd_options: Options {
                ol_number_spacing: 9,
                ..Case::default().htmd_options
            },
            expected_markdown: Some("1.    Item 1\n2.    Item 2"),
            ..Case::default()
        },
        Case {
            id: "nested_ul_spacing_clamps",
            html: nested_ul_html,
            htmd_options: Options {
                ul_bullet_spacing: 9,
                ..Case::default().htmd_options
            },
            expected_markdown: Some("*    Outer\n     *    Inner"),
            ..Case::default()
        },
        Case {
            id: "nested_ol_spacing_clamps",
            html: nested_ol_html,
            htmd_options: Options {
                ol_number_spacing: 9,
                ..Case::default().htmd_options
            },
            expected_markdown: Some("1.    Outer\n      1.    Inner"),
            ..Case::default()
        },
    ]);
}

/// [Lists](../translation_approach.md#lists).
#[test]
fn ordered_list_alignment_across_a_digit_boundary() {
    let mut faithful_cases = Vec::new();
    // `start` can put the marker's width change in the middle of the list.
    // Here the padding the narrower markers need stays within the spacing
    // CommonMark allows, so every item's content keeps the same column.
    let html = format!(r#"<ol start="998">{}</ol>"#, "<li>Item</li>".repeat(5));
    faithful_cases.extend([Case {
        id: "ordered_list_alignment_across_a_digit_boundary_998",
        html: &html,
        expected_markdown: Some("998.   Item\n999.   Item\n1000.  Item\n1001.  Item\n1002.  Item"),
        ..Case::default()
    }]);

    let html = format!(r#"<ol start="9">{}</ol>"#, "<li>Item</li>".repeat(2));
    faithful_cases.extend([Case {
        id: "ordered_list_alignment_across_a_digit_boundary_9",
        html: &html,
        expected_markdown: Some("9.   Item\n10.  Item"),
        ..Case::default()
    }]);
    check_cases(&faithful_cases);
}

/// [Lists](../translation_approach.md#lists).
#[test]
fn ordered_list_alignment_yields_to_the_spacing_limit() {
    let html = format!("<ol>{}</ol>", "<li>Item</li>".repeat(1000));
    let expected_markdown = (1..=1000)
        .map(|number: usize| {
            let spaces = (6 - number.to_string().len()).clamp(1, 4);
            format!("{number}.{}Item", " ".repeat(spaces))
        })
        .collect::<Vec<_>>()
        .join("\n");
    check_cases(&[Case {
        id: "ordered_list_alignment_yields_to_the_spacing_limit",
        html: &html,
        expected_markdown: Some(&expected_markdown),
        ..Case::default()
    }]);
}
