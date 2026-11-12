// Shared code for all integration tests.
use htmd::{
    HtmlToMarkdown,
    options::{
        BrStyle, BulletListMarker, CodeBlockFence, CodeBlockStyle, HeadingStyle, HrStyle,
        LinkReferenceStyle, LinkStyle, Options, TranslationMode,
    },
};
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
    render_markdown(&convert_faithful(html).unwrap(), CommonMarkOptions::empty())
}

// The second half of a round trip: the CommonMark of the first half read back
// as HTML.
#[allow(dead_code)]
pub fn render_markdown(markdown: &str, options: CommonMarkOptions) -> String {
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, Parser::new_ext(markdown, options));
    html_output
}

fn can_append_renderer_lf(html: &str) -> bool {
    if html.ends_with('\n') {
        return false;
    }
    let ending = html.rsplit('<').next().unwrap_or_default();
    let tag = ending.trim_ascii_start();
    matches!(
        tag,
        "/p>"
            | "/h1>"
            | "/h2>"
            | "/h3>"
            | "/h4>"
            | "/h5>"
            | "/h6>"
            | "/ul>"
            | "/ol>"
            | "/blockquote>"
            | "/pre>"
            | "/table>"
    ) || tag.starts_with("hr ") && tag.ends_with('>')
        || tag == "hr>"
        || tag == "hr/>"
}

/// The source-level HTML result expected from a Faithful round trip.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum HtmlExpectation<'a> {
    Preserved {
        renderer_suffix: &'a str,
    },
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
            markdown_options: CommonMarkOptions::empty(),
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
    let mut ids = std::collections::HashSet::new();
    for case in cases {
        let context = format!(
            "case: {}\nhtmd options: {:?}\npulldown-cmark options: {:?}\ninput HTML: {:?}",
            case.id, case.htmd_options, case.markdown_options, case.html
        );
        if case.id.is_empty() || (case.expectation.is_none() && case.expected_markdown.is_none()) {
            failures.push(format!(
                "{context}\ncase needs an ID and at least one assertion"
            ));
            continue;
        }
        if !ids.insert(case.id) {
            failures.push(format!("{context}\nduplicate case ID"));
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
            let (label, expected) = match expectation {
                HtmlExpectation::Preserved { renderer_suffix } => {
                    if !renderer_suffix.is_empty() && renderer_suffix != "\n" {
                        failures.push(format!(
                            "{context}\nrenderer suffix must be empty or one LF: {renderer_suffix:?}"
                        ));
                    }
                    if renderer_suffix == "\n" && !can_append_renderer_lf(case.html) {
                        failures.push(format!(
                            "{context}\nrenderer LF requires a completed top-level block at input end"
                        ));
                    }
                    ("preservation", format!("{}{renderer_suffix}", case.html))
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
                    (
                        match kind {
                            DifferenceKind::Representation => "representation difference",
                            DifferenceKind::Loss => "known loss",
                        },
                        expected_html.to_owned(),
                    )
                }
            };
            if rendered != expected {
                failures.push(format!(
                    "{context}\n{label} mismatch\nexpected HTML: {expected:?}\nactual HTML: {rendered:?}"
                ));
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
