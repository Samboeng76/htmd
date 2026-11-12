use htmd::options::Options;
mod common;
use common::{
    Case, HtmlExpectation, StabilityExpectation, check_cases, convert_faithful,
    convert_faithful_options,
};

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
    assert_eq!(
        "*   Item 1\n*   Item 2\n*   Item 3",
        convert_faithful(html).unwrap()
    );
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
    let options = Options {
        ul_bullet_spacing: 2,
        ..Default::default()
    };
    let md = convert_faithful_options(html, options).unwrap();
    assert_eq!("*  Item 1\n*  Item 2\n*  Item 3", md);
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
    assert_eq!(
        "1.  Item 1\n2.  Item 2\n3.  Item 3",
        convert_faithful(html).unwrap()
    );
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
    let options = Options {
        ol_number_spacing: 1,
        ..Default::default()
    };
    let md = convert_faithful_options(html, options).unwrap();
    assert_eq!("1. Item 1\n2. Item 2\n3. Item 3", md);
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
    let options = Options {
        ol_number_spacing: 1,
        ..Default::default()
    };
    let md = convert_faithful_options(html, options).unwrap();
    assert_eq!(
        "1. Item 1\n2. Item 2\n3. Item 3\n\n1. Item 1\n2. Item 2\n3. Item 3",
        md
    );
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
    let convert = |html, options| convert_faithful_options(html, options).unwrap();
    let ul = |ul_bullet_spacing| Options {
        ul_bullet_spacing,
        ..Default::default()
    };
    let ol = |ol_number_spacing| Options {
        ol_number_spacing,
        ..Default::default()
    };
    let ul_html = "<ul><li>Item 1</li><li>Item 2</li></ul>";
    let ol_html = "<ol><li>Item 1</li><li>Item 2</li></ol>";

    assert_eq!("* Item 1\n* Item 2", convert(ul_html, ul(0)));
    assert_eq!("*    Item 1\n*    Item 2", convert(ul_html, ul(9)));
    assert_eq!("1. Item 1\n2. Item 2", convert(ol_html, ol(0)));
    assert_eq!("1.    Item 1\n2.    Item 2", convert(ol_html, ol(9)));

    // A nested list is indented to its parent item's content column, so the
    // clamp keeps it a list: past four spaces, CommonMark reads that
    // indentation as a code block instead.
    let nested_ul_html = "<ul><li>Outer<ul><li>Inner</li></ul></li></ul>";
    let nested_ol_html = "<ol><li>Outer<ol><li>Inner</li></ol></li></ol>";

    assert_eq!(
        "*    Outer\n     *    Inner",
        convert(nested_ul_html, ul(9))
    );
    assert_eq!(
        "1.    Outer\n      1.    Inner",
        convert(nested_ol_html, ol(9))
    );
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
    let md = convert_faithful(&html).unwrap();
    assert_eq!(
        "998.   Item\n999.   Item\n1000.  Item\n1001.  Item\n1002.  Item",
        md
    );
    faithful_cases.extend([Case {
        id: "ordered_list_alignment_across_a_digit_boundary_998",
        html: &html,
        expected_markdown: Some("998.   Item\n999.   Item\n1000.  Item\n1001.  Item\n1002.  Item"),
        ..Case::default()
    }]);

    let html = format!(r#"<ol start="9">{}</ol>"#, "<li>Item</li>".repeat(2));
    assert_eq!("9.   Item\n10.  Item", convert_faithful(&html).unwrap());
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
    let md = convert_faithful(&html).unwrap();
    let mut lines = md.lines();
    assert_eq!("1.    Item", lines.next().unwrap());
    assert_eq!("1000.  Item", lines.last().unwrap());
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
