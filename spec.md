# Round-trip testing strategy

This specification defines the team's strategy for standardizing htmd tests
around Faithful-mode HTML → Markdown → HTML round trips.
Conversion behavior remains governed by
[translation_approach.md](https://github.com/bjones1/htmd/blob/2e7bf3a8dc0eb9348a3a15d8f04783c3669116da/translation_approach.md).
Sam's [initial note](https://github.com/Samboeng76/htmd/blob/main/testing_standardization_ideas.md)
provides the starting goal: test Faithful conversion through HTML → Markdown → HTML.

| Section | Section lead | Responsibility |
| --- | --- | --- |
| 1 | Yuriy | Scope and common test-case format |
| 2 | Sam | Migration of existing tests |
| 3 | Alibek | HTML comparison and correctness criteria |
| 4 | Kamila | Round-trip execution and project verification |

This submission specifies the strategy. Implementing the helper changes and
migrating the complete test suite are subsequent work.

## 1. Scope and common test-case format

Start with deterministic integration tests in `TranslationMode::Faithful`.
Keep Pure-mode behavior, option-specific Markdown spelling, API events, and
error handling covered by their existing assertions. Fuzzing is outside this
project's initial scope.

Each case records:

| Field | Meaning |
| --- | --- |
| `id` | Unique descriptive name, retained from the original test where possible |
| `html` | Literal input HTML, with significant whitespace made visible in the source |
| `htmd_options` | Faithful mode plus all overrides of the pinned baseline defaults |
| `markdown_options` | Exact pulldown-cmark extension flags; empty unless explicitly overridden |
| `expectation` | `Preserved { renderer_suffix }` or `DocumentedDifference { kind, expected_html, reason, reference }`, as defined in section 3 |
| `expected_markdown` | Optional additional exact assertion for a Markdown spelling requirement |
| `check_stability` | Whether to perform the additional repeated-round-trip check |

Keep cases in the existing feature test files and reuse `tests/common/mod.rs`.
Changing the format must preserve the original input, option coverage, and
purpose of each migrated test.

## 2. Migration of existing tests

Review each input and mode case for the behavior it checks, including cases
grouped inside one test function. Classify it as preservation, a documented
representation difference or loss, or a test of another contract. The
[per-test inventory](https://github.com/Samboeng76/htmd/blob/9f27b8196213fad9516b0b4dba6e683d27e01583/testing_standardization_test_inventory.md) records proposed
Faithful treatment, Markdown assertions to retain, and existing round-trip
coverage. Its `Partial` entries identify cases still lacking a round trip.
Keep the inventory in its own file, and inventory the individual cases in
`tests/html/turndown_test_index.html` before migrating them.

Select Faithful-mode cases whose claim concerns preserved HTML or an approved
documented difference. A test function may also contain Pure-mode assertions;
leave those assertions in place. Do not add a duplicate round trip for an input
already checked, though an existing check may still need the case format and
correctness criteria in sections 1 and 3.

| Case purpose | Selection |
| --- | --- |
| Preserve ordinary HTML structure | Add a `Preserved` round trip, using a compact input when formatting whitespace would change the claim. |
| Check Markdown syntax, escaping, or style options | Keep the exact Markdown assertion; add a round trip only when the case also checks preservation. |
| Check raw-HTML fallback or an intentional transformation | Keep the Markdown assertion and select `Preserved` or `DocumentedDifference` according to the approved conversion rule. |
| Check Pure mode, handler events, or Turndown compatibility | Keep the existing assertion; these are not Faithful round-trip candidates. |

Pilot compact Faithful paragraph and heading cases in `basic_tests.rs`,
including `paragraphs`, `consecutive_blocks`, and `headings`. Preserve the
original indented fixtures and Markdown checks when adding compact variants.
Then review simple lists in `list_tests.rs`, paragraph-wrapped links in
`link_tests.rs`, and raw-HTML fallback such as `faithful_mode_p` in
`basic_tests.rs`. The main link input in `a_bang_before_a_link_is_escaped`
already has a round trip; its other Faithful inputs remain partial candidates.

After the base cases, review full documents with `<head>` content and then
tables, math, and footnotes. Use the case's exact htmd options and matching
pulldown-cmark extensions. For a selected intentional transformation, record
its approved rule and expected HTML rather than treating the current output as
the expectation. For example, flow text `<p>a\nb</p>` becoming `<p>a b</p>`
changes text, so it requires an approved `DocumentedDifference` of kind `Loss`.

Record each selected case's destination, classification, mode, options, and
reference HTML. An unexpected mismatch needs investigation against the
translation specification; migration must not silently replace an expected
result with current converter output.

## 3. HTML comparison and correctness criteria

Let `H` be the literal input HTML and `R(H)` the HTML produced by one round trip
under the case's fixed conversion options.

For `Preserved`, require exactly `R(H) == H + renderer_suffix`, where the
suffix is either `""` or one LF (`"\n"`). The nonempty suffix is permitted only
when `H` ends with a completed top-level paragraph, heading, list, blockquote,
`pre`, table, or `hr`, optionally followed by LF characters. The added LF must
be outside that element and all other open constructs. It never removes or
replaces existing whitespace. Other inputs use the empty suffix.

Compare UTF-8 strings exactly. Do not trim strings, collapse whitespace, decode
entities, reorder attributes, normalize Unicode, or remove elements before
comparison. Whitespace inside `pre`, `code`, `textarea`, attribute values, and
ordinary text remains part of the assertion.

The source comparison is intentionally stricter than rendered or DOM equality.
`DocumentedDifference` with `kind = Representation` is restricted to attribute
quote style or order, the optional slash of HTML void elements, equivalent
character references, and the same final-LF convention above. Each change must
preserve parsed node names and namespaces, child order, attribute names and
values, and text and comment contents, apart from that explicit final LF, under
the converter's HTML parsing options. For example, entity spellings in raw-text
elements and comments are not interchangeable. A structural or value change
uses `kind = Loss`; other unapproved differences fail.

Both kinds require literal `expected_html`, a short explanation of every
allowed difference, and a reference to an approved conversion rule or a
limitation explicitly documented in `translation_approach.md`. Specify and
review the expected result independently of actual converter output; a failing
case cannot become accepted merely by copying its output into the fixture.
Require exactly `R(H) == expected_html`, with no additional suffix or
normalization. Report Representation and Loss separately from preservation;
a Loss result passing its regression assertion is not a preservation success.

When `check_stability` is enabled, also require `R(R(H)) == R(H)` under the same
options. This is an additional assertion: a stable result may already have
lost content on its first trip, so stability does not establish preservation.

The following pairs illustrate the comparator; they do not promise that the
converter currently produces each right-hand value. `\n` means one LF.

| Input `H` | Candidate result | Decision |
| --- | --- | --- |
| `<p>Hello</p>` | `<p>Hello</p>\n` | Preserved when the selected suffix is `"\n"` |
| `<div>a</div>` | `<div>a</div>` | Preserved with an empty suffix |
| `<p>a b</p>` | `<p>ab</p>\n` | Fails preservation: a text space was lost |
| `<pre>a\nb</pre>` | `<pre>a b</pre>` | Fails preservation: an internal LF changed |
| `<p>&#65;</p>` | `<p>A</p>\n` | Fails preservation; needs an explicit Representation expectation |
| `<br><br>` | `<p><br><br></p>\n` | Documented Loss: an extra paragraph, as described under "At the document root" in the translation specification |

## 4. Round-trip execution and project verification

The runner converts `html` with the case's htmd options, then renders that
Markdown using `render_markdown` and the case's exact pulldown-cmark flags.
Reuse the existing `round_trip` for its default profile; the generalized runner
must also pass option overrides through both stages. Keep raw HTML enabled.
Enable extensions such as tables or math only for cases that need them.

Conversion errors fail the named case. A mismatch reports the case ID, options,
input HTML, intermediate Markdown, expected HTML, and actual HTML, with escaped
whitespace in the diff. Report preservation, representation differences, and
known losses separately; stability is an additional result.

Migrate in reviewable feature batches. A batch is complete when its inventory
is accounted for, previous coverage is retained, and these commands pass:

```sh
cargo build
cargo clippy -- --deny warnings
cargo test
cargo run --example simple
```

The current helper and existing tests are implementation references, not a
claim that this proposed format has already been implemented.
