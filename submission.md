# Project 2 submission
=====================

Team name: standardize htmd testing - 1 (PR #13)

The specification consists of [spec.md](spec.md). The migration planning table
is
[testing\_standardization\_test\_inventory.md](testing_standardization_test_inventory.md).
This report follows the
[Project 2 submission template](https://github.com/bjones1/literate-programming-book/blob/main/project_2_submission_template.md).

| Section | Owner              | Responsibility                                |
| ------- | ------------------ | --------------------------------------------- |
| 1       | Yuriy Kim          | Scope and common test-case format             |
| 2       | Sam English        | Migration of existing tests                   |
| 3       | Alibek Baisholanov | HTML comparison and correctness criteria      |
| 4       | Kamila Bagdat      | Round-trip execution and project verification |

Draft submission
----------------

The following histories and final review outputs describe the draft versions
reviewed on 2026-11-08–09. Their earlier zero-finding results do not review the
later instructor-requested revisions. Review rounds are counted separately per
section; repeated findings in different rounds are retained as reported.

The Section 1 checks and Section 2 rounds 8–9 were performed in Alibek's Codex
session as section-scoped reviews. Sam supplied Section 2 rounds 1–7 and his
last review. Kamila supplied Section 4's history, including its cross-section
consistency checks. These records identify section ownership and actual review
provenance; they do not claim each teammate operated the same LLM session.

### Yuriy Kim

GitHub userid: kim0111

#### Contributions

Defined the scope and common case format, integrated the four sections, and
updated the inventory to the instructor's current baseline.

#### Build and run test

**Model:** Claude Opus 5.5, as reported by Yuriy.

**Summarized result:** Yuriy confirmed successful local execution
of `cargo build`, `cargo clippy -- --deny warnings`, `cargo test`, and `cargo
run --example simple`. This participant-reported summary was confirmed by Alibek
on 2026-11-15; it is not a verbatim transcript.

#### Specification review rounds

| Round | Findings | Main changes made                                                                                                                     |
| ----- | -------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| 1     | 1        | Aligned the two Faithful scope rows with Markdown-only cases and HTML assertions for preservation or approved documented differences. |
| 2     | 0        | Rechecked the corrected scope and case contract; no further relevant changes were requested.                                          |

#### Last specification review

**Model:** GPT-6.1 Sol, Extra High.

**Exact prompt:**

```text
Review only Section 1, "Scope and common test-case format", in spec.md. Check the in/kept/out boundaries, consistency of the scope table with the case fields, Markdown-only and documented-difference cases, option handling, and retention of existing coverage. Use the other sections and current tests only as context. Report only actionable correctness findings with short corrections; do not rewrite the section or target a finding count. If no relevant findings remain, say so.
```

**Final output, verbatim:**

> No further relevant correctness findings in Section 1. The scope table now
> matches the optional HTML expectation in the case format: Markdown-only cases
> retain their exact Markdown assertion and stability check, while preservation
> and approved differences use their corresponding HTML assertion. Kept
> Pure-mode, extension API, concurrency, and Turndown tests retain their
> existing purpose and coverage.

### Sam English

GitHub userid: Samboeng76

#### Contributions

Developed and refined the migration strategy and per-test inventory, including
the handling of kept Pure-mode cases.

#### Build and run test

Build, Clippy, and tests all passed. The test run had 268 passing tests, 0
failures, and 1 ignored test. Cargo printed a nonfatal path warning.

#### Specification review rounds

| Round | Findings | Main changes made                                                                                                                                                                      |
| ----- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1     | 3        | Added implementation status to the inventory and merged Section 2 into the group specification.                                                                                        |
| 2     | 3        | Removed footnotes from the inventory.                                                                                                                                                  |
| 3     | 2        | Corrected the inventory entries for functions with multiple inputs.                                                                                                                    |
| 4     | 3        | Updated the specification to match the inventory and added the stability check to Section 2.                                                                                           |
| 5     | 3        | Added the concurrency exception and made minor inventory corrections.                                                                                                                  |
| 6     | 2        | Made minor clarifications and inventory edits.                                                                                                                                         |
| 7     | 0        | No further changes were requested.                                                                                                                                                     |
| 8     | 0        | Checked migration rules against the shared case contract; no changes were requested.                                                                                                   |
| 9     | 0        | Verified the test count, kept cases, mixed modes, existing round trips, fixture retention, and conditional helper removal against the source and inventory; no changes were requested. |

#### Last specification review

**Model:** GPT-6.1 Sol, Extra High, as recorded by Sam.

**Prompt:**

```text
review the migration of existing tests section of spec.md
```

**Last output (text extracted from Sam's HTML-formatted record):**

> I found no actionable correctness issues in “Migration of existing tests”
> (line 107). It covers individual input and mode cases, retained assertions,
> exact options, stability checks, and removal of helpers only after their
> callers are migrated.
>
> I checked the inventory and the versioned test source. Several test files are
> currently deleted in the working tree, so I did not run tests.

### Alibek Baisholanov

GitHub userid: alibekbtu

#### Contributions

Specified and reviewed the HTML-comparison, whitespace, parser-normalization,
and stability criteria and checked them with 31 measured cases.

#### Build and run test

**Model:** GPT-6.1 Sol, Extra High.

**Prompt:**

```text
Verify that you can compile and run the project in the current git repo.
```

**Summarized LLM output, verified on 2026-11-09:**

> I verified that I can compile and run htmd. `cargo build`, `cargo clippy --
> --deny warnings`, `cargo test`, and `cargo run --example simple` all exited
> successfully. The suite reported 263 passing unit/integration tests and 4
> passing doctests, with one intentionally ignored test and zero failures.

#### Specification review rounds

| Round | Findings | Main changes made                                                                                                                                                |
| ----- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1     | 3        | Defined the final-LF predicate and allowed context, distinguished Representation from Loss, and required independent expected results.                           |
| 2     | 0        | No further changes were requested for that version of Section 3.                                                                                                 |
| 3     | 1        | Added parser-defined representation changes and distinguished literal CR preprocessing from character-reference CR, with measured whitespace/stability examples. |
| 4     | 0        | The revised Section 3 had no further relevant specification findings.                                                                                            |

#### Last specification review

**GPT-6.1 Sol, Extra High**, with the following prompt and the text of Section
3:

```text
Review only section 3, "HTML comparison and correctness criteria", in the integrated spec.md. Check the exact source-equality predicate and renderer LF allowance, internal block-separator newlines, CRLF and lone-CR preprocessing versus character references, spaces and tabs in text/code/attributes, non-breaking spaces, and Markdown stability. Use the supplied HTML/CommonMark references and measured converter outputs. Report only actionable correctness findings with short corrections; do not rewrite or review the other authors' sections, and do not target a finding count. If no relevant findings remain, say so.
```

Final output, verbatim:

> No further relevant correctness findings in Section 3. Exact source equality,
> the explicit terminal-LF allowance, internal separator handling, parser-only
> Representation changes, and source-level Loss are distinguished. Literal CR
> preprocessing is not confused with a CR produced by a character reference.
> Markdown stability is checked separately from HTML stability. The measured
> losses and two unstable Markdown cases remain failures of the corresponding
> preservation/Stable claims unless independently justified; the review does not
> approve them as new exceptions.

Reviewed Section 3 SHA-256, with LF line endings:
`4478784e7f7e5e43f37bd84a0e666ac890f2907d13a812fbd57b39b297dbfd6e`.

### Kamila Bagdat

GitHub userid: bagdatkamilla

#### Contributions

Defined stage-specific runner options, expectation-specific assertions and
diagnostics, and supplied the Section 4 review and verification record.

#### Build and run test

**Exact instructor-requested prompt:**

> Verify that you can compile and run the project in the current git repo.

**Verification assistant:** GitHub Copilot in VS Code -
MAI-Code-1.1-Flash

**LLM output (successful verification response, preserved from the
provided conversation):**

> I’m re-running the repo verification in the actual project root with the Rust
> toolchain available so I can give you a final yes/no status based on fresh
> build and run output.
>
> Ran terminal command: `$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH";
> Set-Location "c:\Users\Documents\KBTU\Data mining\advanced\New folder\htmd";
> cargo build; cargo clippy -- --deny warnings; cargo test; cargo run --example
> simple`
>
> Read , lines 1 to 260
>
> Ran terminal command: `$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH";
> Set-Location "c:\Users\bagdatkamilla\Documents\KBTU\advanced\New folder\htmd";
> cargo run --example simple; $code = $LASTEXITCODE; Write-Host
> "RUN_EXIT:$code"; exit $code`
>
> ✅ Verification passed
> ----------------------
>
> I verified the project in the current repo with the requested checks:
>
> * `cargo clippy -- --deny warnings` → exit code 0
> * `cargo test` → all reported test groups passed with 0 failed
> * `cargo run --example simple` → exit code 0
>
> ### Evidence
>
> The final sample run completed successfully:
>
> ```text
>     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
>      Running `target\debug\examples\simple.exe`
> RUN_EXIT:0
> ```

#### Specification review rounds

| Round | Findings | Main changes made                                                                                                                                                                                                 |
| ----- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1     | 2        | Identified missing intermediate-Markdown, per-case option, instability, and Markdown-only handling; no edits were applied in that round.                                                                          |
| 2     | 2        | Rechecked per-stage options and stability ambiguity; no edits were applied in that round.                                                                                                                         |
| 3     | 3        | Checked consistency across the specification, including instability exceptions, Markdown-only reporting, and the limits of the default helper; no edits were applied in that round.                               |
| 4     | 0        | Applied concise revisions for per-stage options, independent assertions, stability expectations, and expectation-specific diagnostics; the recorded final consistency review found no additional relevant issues. |

#### Last specification review

**Model:** OpenAI GPT-6.

**Exact review prompt used for the final written consistency
check:**

> Review the updated `spec.md`, concentrating on Section 4 and its consistency
> with Sections 1–3. Check the handling of default versus generalized round-trip
> helpers, htmd and pulldown-cmark options, `Stable` versus
> `DocumentedInstability` expectations, Markdown-only cases, failure reporting,
> and the verification command list. Report only concrete outstanding problems
> in the specification; do not infer that the proposed runner has been
> implemented.

**Final review output:**

> No additional relevant findings in the supplied specification. Section 4 now
> distinguishes the stage-specific converter options; preserves the intermediate
> Markdown; specifies `Stable` and approved `DocumentedInstability` comparisons;
> allows exact-Markdown cases without an HTML expectation; and reports failures
> according to which assertion failed. Section 3 now states the
> documented-instability exception explicitly. This is a consistency review of
> the written strategy, not a source-code execution or validation of the
> unimplemented generalized runner.

Final submission
----------------

The shared runner/comparator and migrated test cases are implemented under
`tests/`. Final-stage reviews and contributions are recorded separately for each
member below; draft review rounds above are not counted here.

### Yuriy Kim

GitHub userid: kim0111

#### Contributions

Reviewed and revised Section 1, "Scope and common test-case format", after the
instructor feedback, correcting its field defaults, optional `id`, example case,
helper list, baseline test count, and classification of the comparator
self-tests, then reviewed its implementation and fixed how migrated tests fill
in cases (exact setext Markdown, default pulldown-cmark flags, one
`check_cases` call per test, required input HTML).

#### Specification review rounds

Only final-stage Section 1 reviews from 2026-11-15 are included. Each round ran
in a new session, and findings were fixed by hand before the next round.

| Round | Findings | Main changes made                                                                                                                                                                          |
| ----- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1     | 5        | Corrected the example's names and `Option` wrapping, marked defaulted fields as optional, made the `id` field optional, and classified the comparator self-tests as kept.                  |
| 2     | 4        | Listed all eight existing helpers, removed the kept `event_tests.rs` from the style problems, pointed `htmd_options` at `Case::default()`, and tied the test count to the baseline commit. |
| 3     | 3        | No changes: one finding was incorrect (math-only read-back is used at the baseline), one restated the optional `id` the instructor's feedback allows, and one was a trivial type detail.   |

#### Last specification review

**Model:** Claude Opus 5.5, high effort, in Claude Code.

**Exact prompt:**

```text
Review only Section 1, "Scope and common test-case format", in spec.md, after the
instructor feedback on PR #13 (Options::all() by default, no separate case id,
whitespace-aware HTML comparison). Use the other sections and the tests only as
context. Report only actionable correctness findings with short corrections; do not
rewrite the section. If no relevant findings remain, say so.
```

**Final output, verbatim;** strikethrough marks findings I judged
incorrect or irrelevant:

> I found two small factual problems in Section 1 and one possible conflict with
> the instructor's feedback. The rest of the section is accurate.
>
> 1. ~~The id field may contradict "no separate case id" (§1.3, field table
>    and the paragraph after it). The section now adds id as an optional field.
>    The tests set id: "…" 453 times, mostly as generated labels like
>    a\_type\_7\_tag\_is\_a\_raw\_inline\_even\_in\_a\_block\_context\_case\_5.
>    That is a separate case id in practice.~~
>    * ~~If the instructor meant ids should be optional, the text is correct
>      as written.~~
>    * ~~If they meant there should be no id field, remove the id row and
>      change the sentence to: "The test function name identifies the test;
>      table-driven failures also report the row number."~~
> 2. ~~The list of pulldown-cmark options in §1.1 is wrong. "(none, math,
>    tables, or both)" says some helper uses math alone, but none does. At
>    baseline, round\_trip and round\_trip\_setext use no flags,
>    round\_trip\_pure\_table uses tables, and assert\_math\_round\_trips uses
>    math and tables together. Correction: "(none, tables only, or math with
>    tables)".~~
> 3. ~~The expectation field's meaning doesn't match its type (§1.3). The
>    field is an Option, and the example writes Some(DocumentedDifference { …
>    }), but the table's Meaning column lists only the bare variants.
>    Correction: "Some(Preserved) or Some(DocumentedDifference { … }); None when
>    the case checks only Markdown."~~
>
> I checked these against the current text and the code:
>
> * The baseline claims hold at 2e7bf3a: 227 tests in 10 files, the eight named
>   helpers, and nested modules plus test\_ names in table\_tests.rs.
> * Options::all() is the default in the R(H) term, the field table and
>   Case::default().
> * The "whitespace-aware" wording for Preserved matches section 3.
> * htmd\_options and stability defaults, and the Faithful-only check, match
>   check\_cases.
>
> The example case's HTML and Markdown match the real case in context\_tests.rs.
> I didn't run it, so its round-trip and stability claims are unverified.

Why the struck findings need no change:

1. The instructor asked whether a separate case ID is necessary; Section 1 makes
   `id` optional and identifies a test by its function name, which is the first
   reading the review itself accepts.
2. Incorrect: at `2e7bf3a`, Markdown is read back with math alone through
   `render_markdown` at `tests/basic_tests.rs:320` and
   `tests/link_tests.rs:484`.
3. The field table names the expectation kinds; the example shows the exact Rust
   wrapping, and repeating it in the table would overspecify the section.

#### Implementation review rounds

Final-stage reviews of the Section 1 implementation from 2026-11-16: the `Case`
struct and its defaults, how `check_cases` uses them in `tests/common/mod.rs`,
and how the migrated tests fill in cases. Each round ran in a new session with
the same prompt. Round 1 fixes are in commit `a757322` and round 2 fixes in
`807bdc1`.

| Round | Findings | Main changes made                                                                                                                                                      |
| ----- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1     | 2        | Added exact Markdown to the seven setext heading cases and removed the 51 pulldown-cmark flag overrides no case needed, keeping the five smart-punctuation exceptions. |
| 2     | 2        | Merged the extra case of two table tests into their single `check_cases` call and made the runner reject a case with empty input HTML.                                 |
| 3     | 2        | No changes: both findings were trivial (a missing comment on three needed overrides, and stale line numbers in eight case labels); neither changes what a test checks. |

#### Last implementation review

**Model:** Claude Opus 5.5, high effort, in Claude Code.

**Exact prompt:**

```text
Review only the implementation of Section 1, "Scope and common test-case format", of spec.md: the Case struct, its fields and defaults, and how check_cases uses them in tests/common/mod.rs, plus how migrated tests fill in cases. Check them against Section 1. Use the other sections only as context. Report only actionable correctness findings with short corrections; do not rewrite the code. If no relevant findings remain, say so.
```

**Final output, verbatim;** strikethrough marks text I judged incorrect or
irrelevant:

> I found no correctness bugs in the Section 1 core: the Case struct, its
> defaults and how check\_cases uses them all match the spec. Two minor fixes
> remain in how migrated tests fill in cases. This was a read-only review; I
> didn't run cargo test.
>
> Findings
>
> 1. Three cases turn off smart punctuation without saying why. Section 1.3
>    lets a case change markdown\_options only "when a case needs different
>    flags". Other cases that do this explain it in a comment, such as
>    tests/basic\_tests.rs:1753 and :1792, plus the setext table at :2296.
>    These three don't:
>    * tests/basic\_tests.rs:695 (inline\_raw\_html\_escaping)
>    * tests/basic\_tests.rs:726 (multiline\_raw\_html\_escaping)
>    * tests/link\_tests.rs:571 (an\_image\_alt\_escapes\_its\_raw\_html)
>
>    Without a stated reason, a reader can't tell a real need from hiding a
>    mismatch, which Section 4 forbids. The -- in &lt;!-- … --&gt; and the " in
>    the alt text would be changed by smart punctuation.
>    Fix: add a one-line reason for each override, as the other cases do.
>    ~~If smart punctuation would really change the text and nothing in the
>    spec approves that, keep the default CommonMarkOptions::all() and record
>    it as a DocumentedDifference with a reference instead.~~
> 2. Eight case ids contain wrong source line numbers. For example,
>    faithful\_mode\_serializes\_a\_table\_with\_a\_caption\_preserved\_line\_1150
>    is at tests/basic\_tests.rs:1088. Others are at :2065, :2180, :2199,
>    :2207, :2214, :2221 and :2657. The id is the label printed in failure
>    reports (1.3), so these send the reader to the wrong line.
>    Fix: remove the \_line\_NNNN suffix, or rename each to describe what the
>    case checks.
>
> What checks out
>
> * Fields and defaults: html is required. htmd\_options defaults to the
>   Faithful baseline and must stay Faithful. markdown\_options defaults to
>   Options::all(), and stability defaults to Stable. id is optional, and every
>   case must have at least one of expectation or expected\_markdown.
> * Faithful baseline: it matches the old helpers exactly (Options::default()
>   with the mode set to Faithful).
> * Failure reports: every case runs, all failures are reported together, and
>   each one shows the test name, row number and id.
> * Rules 2–4: no migrated test uses a test\_ prefix or nested modules. Each
>   migrated test calls check\_cases once, and its doc comment's first line
>   links translation\_approach.md.
> * Rule 5: I compared every HTML string in the baseline commit 2e7bf3a against
>   the current tests. Every Faithful input, option override and Markdown
>   check that disappeared turned out to be an expected HTML string now
>   covered by Preserved.

Why no change was made:

1. The three overrides are needed: under `Options::all()`, smart punctuation
   rewrites their `--` or `"` and the cases fail, as round 1 showed; only a
   comment is missing. The struck alternative is incorrect: the change comes
   from a pulldown-cmark extension the case does not test, not from a loss in
   htmd's translation, so it is not a `DocumentedDifference`.
2. The `_line_NNNN` suffixes are labels only. A failure report also names the
   test and row number, which locate the case.

### Sam English

GitHub userid: Samboeng76

#### Contributions

Refined implementation. A large part of this was identifying missed migrations
from the inventory.

#### Specification review rounds

No final-stage rounds are recorded here yet.

Testing Standardization Implementation:

| Round | Findings | Main changes made                                                                                                                                                                                                              |
| ----- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1     | 3        | Fixed some implementation errors                                                                                                                                                                                               |
| 2     | 3        | Documented the root-link round-trip loss, grouped test cases with descriptive IDs, and corrected the migration inventory links.                                                                                                |
| 3     | 4        | Documented list and link/image round-trip limitations, removed duplicate Faithful assertions, added an exact HTML expectation for the ordered-list loss, and recorded generated cases individually in the migration inventory. |
| 4     | N\\A     | Merged PR feedback into implemntation branch and updated implementation accordingly                                                                                                                                            |
| 5     | 2        | Added HTML expectations for link and image URLs containing spaces; corrected case classifications and source links in the migration inventory.                                                                                 |
| 6     | 3        | Added HTML preservation checks for 29 Faithful cases, limited smart punctuation overrides to affected fixtures, and corrected migration inventory links and classifications.                                                   |
| 7     | 1        | Added HTML preservation checks for the original headings, paragraphs, and consecutive blocks cases; updated their migration inventory classifications and links.                                                               |
| 8     | 2        | Fixed more missed migrations                                                                                                                                                                                                   |

#### Last specification review

The prompt used was "review the migration of existing tests section of spec.md
along with its corresponding implementation" with GPT-6 sol on extra high:

Review findings

* P1 — An unapproved code-whitespace loss is now treated as expected. The new
  fixture in \[code\_tests.rs (line 35)\](/C:/Users/tmsse/OneDrive/Family
  Files/Desktop/Literate
  Programming/htmd\_sbe80Fork/htmd/tests/code\_tests.rs:35) accepts an added
  newline inside `. [code.rs (line 62)](/C:/Users/tmsse/OneDrive/Family
  Files/Desktop/Literate
  Programming/htmd_sbe80Fork/htmd/src/element_handler/code.rs:62) produces it
  when the input has no terminal newline. The cited [translation rule (line
  240)](/C:/Users/tmsse/OneDrive/Family Files/Desktop/Literate
  Programming/htmd_sbe80Fork/htmd/translation_approach.md:240) calls code
  content literal but does not approve this loss. Under [spec.md (line
  169)](/C:/Users/tmsse/OneDrive/Family Files/Desktop/Literate
  Programming/htmd_sbe80Fork/htmd/spec.md:169), this needs a converter fix or an
  independently approved exception before it is recorded as a
  DocumentedDifference.`

P2 — The generated inline/block cases use a reference that does not
support their loss classification. All 144 cases in \[blocks\_inside\_inlines
(line 1014)\](/C:/Users/tmsse/OneDrive/Family Files/Desktop/Literate
Programming/htmd\_sbe80Fork/htmd/tests/translation\_approach\_tests.rs:1014)
cite “Translating HTML nodes” for exact rendered HTML attributed partly to
parser repair. That rule describes how htmd emits the nodes; it does not
document those parser repairs or approve every resulting loss. The cases need
references to the applicable approved rule or normative parsing rule, as
\[spec.md (line 169)\](/C:/Users/tmsse/OneDrive/Family Files/Desktop/Literate
Programming/htmd\_sbe80Fork/htmd/spec.md:169) requires. No files were changed.
cargo build, cargo clippy -- --deny warnings, cargo test, and cargo run
--example simple all passed.

Both of these findings are handled by the inventory.

#### Implementation review rounds

The proposed implementation has not been completed in this documentation
revision.

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |

#### Last implementation review

The implementation owner will add the actual model, prompt, and last review
output after implementation and repeated review.

### Alibek Baisholanov

GitHub userid: alibekbtu

#### Contributions

Completed and reviewed the Section 3 HTML comparator, corrected
template-fragment and namespace handling, and added regression tests while
preserving the finalized specification.

#### Specification review rounds

Only final-stage Section 3 reviews from 2026-11-15 are included.

| Round | Findings | Main changes made                                                                                                                                                    |
| ----- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1     | 0        | Reviewed the revised whitespace-aware comparison, protected code/pre content, documented expectations, and exact Markdown checks; no further changes were requested. |
| 2     | 0        | Rechecked the examples and boundary cases for inline word separators, NBSP, parser normalization, and stability; no additional changes were requested.               |

#### Last specification review

**Model:** GPT-6.1 Sol, Extra High, the user-confirmed model for
this Codex session. These were successive self-reviews in this session.

**Prompt:**

```text
Review only Section 3, "HTML comparison and correctness criteria", in spec.md after the instructor feedback on PR #13. Check permitted ASCII-whitespace collapse outside code, exact code/pre text and descendants, inline word separators, NBSP, attribute/comment data, HTML parsing, exact documented expectations, and exact Markdown stability under fixed options. Use the other sections only as context. Report only actionable correctness findings with short corrections; do not rewrite other sections or target a finding count. If no relevant findings remain, say so.
```

**Final output, verbatim:**

> No further relevant correctness findings in Section 3. Checking the examples
> and boundary cases confirms that ordinary tab/newline collapse and
> block-separator LFs may pass, while removing an inline word separator,
> replacing NBSP, or changing code whitespace fails preservation. Literal HTML
> preprocessing is distinguished from a CR produced by a character reference,
> and HTML whitespace allowances do not weaken the exact Markdown stability
> assertion. No additional specification change is requested.

Reviewed Section 3 SHA-256 with LF line endings:
`b366be99b0b51c722e736fe0394f6d0c083b185626b33d7071b04d36b27b005d`.

#### Implementation review rounds

Reviews performed on 2026-11-15 against Section 3 only. Round 1 reviewed the
initial shared implementation at `2966de3`; round 2 reviewed the fixes and
regression tests below. These were successive self-reviews in this Codex
session.

| Round | Findings | Main changes made                                                                                                                                                                           |
| ----- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1     | 2        | Compared template content fragments and their independent text flows, made HTML whitespace rules namespace-aware, and added three regression tests covering the two confirmed false passes. |
| 2     | 0        | Rechecked the corrected comparator, whitespace boundaries, protected contents, and regressions; no further relevant correctness findings remained.                                          |

Both round-1 findings were applied; none were rejected. Before the fixes, the
targeted regression run reported 4 passing and 2 failing tests. All 6 comparator
tests pass after the fixes.

#### Last implementation review

**Model:** GPT-6.1 Sol, Extra High, the user-confirmed model for
this Codex session.

**Prompt:**

```text
Review only the implementation of Section 3, "HTML comparison and correctness criteria", in tests/common/mod.rs (ComparedNode through compare_html) and its tests in tests/html_comparison_tests.rs against spec.md. Check parsed structure and attributes, template content fragments and their text flows, HTML versus foreign namespaces, ASCII-whitespace normalization, inline word separation, exact code/pre descendants, NBSP, comments, and parser-normalized line endings. Use the runner and other specification sections only as context. Report only concrete actionable correctness findings with short corrections; do not rewrite the specification, review other owners' sections, or target a finding count. If no relevant findings remain, say so.
```

**Final output, verbatim:**

> No further relevant correctness findings in the Section 3 implementation.
> Template content fragments are now compared recursively, including nested
> structure, attributes, comments, and protected text, while their text flows
> remain separate from surrounding content. HTML-specific whitespace boundaries
> use the HTML namespace, so SVG title content cannot hide a removed separator.
> The regression tests reject both previously accepted changes and retain the
> permitted whitespace transformations, exact code/pre contents, NBSP, and
> inline word separation. The build, warnings-denied Clippy run, full test
> suite, and simple example all passed; the suite reported 271 passed, 1
> ignored, and 0 failed.

Reviewed LF-normalized SHA-256 values:

* Comparator range (`ComparedNode` through `compare_html`) in
  `tests/common/mod.rs`:
  `7fbd9c8f78e192df87e3ef21eb9305de8ee5e89dd238edb7bafbe779c50c99f7`.
* `tests/html_comparison_tests.rs`:
  `dc617a29fde5d6b278ef40af348672f03b6d09151f638d12c3c7e1158dbd929f`.

### Kamila Bagdat

GitHub userid: bagdatkamilla

#### Contributions

Refined Section 4, "Round-trip execution and project verification",
incorporating instructor feedback and improving the strategy for round-trip
execution, stage-specific converter options, Markdown stability, independent
assertions, and failure diagnostics. Contributed to ensuring consistency between
the shared test-case format, HTML comparison criteria, and implementation
requirements.

#### Specification review rounds

| Round        | Findings | Main changes made                                                                                                                                              |
| ------------ | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Review 1     | 4        | Evaluated the updated conversion strategy, focusing on `Options::all()` defaults, explicit option overrides, and consistency with the shared test-case format. |
| Review 2     | 2        | Examined intermediate Markdown preservation, independent HTML and Markdown assertions, Markdown-only cases, and documented instability handling.               |
| Final review | 0        | Assessed the specification's coverage of round-trip execution, error diagnostics, and project verification requirements.                                       |

#### Last specification review

**Model:** OpenAI GPT-6

**Prompt:**

> Review only Section 4, "Round-trip execution and project verification", in
> spec.md. Check the consistency of converter options, default Options::all()
> behavior, HTML-to-Markdown-to-HTML execution, Markdown stability, documented
> instability expectations, Markdown-only test cases, intermediate Markdown
> preservation, failure diagnostics, and verification commands. Identify
> correctness issues and contradictions without expanding the project scope.

**Review assessment:**

> The documented Section 4 strategy addresses the major requirements for
> round-trip execution and project verification. It distinguishes converter
> options across execution stages, preserves intermediate Markdown for
> independent assertions, and defines stability expectations for Markdown-only
> and HTML-preservation cases.
>
> The specification also describes expectation-specific diagnostics and
> verification commands. The final revised source should be checked to confirm
> that these requirements remain consistent with Sections 1–3 and that all
> instructor-requested changes have been incorporated.

#### Implementation review rounds

| Round        | Findings | Main changes made                                                                                                                           |
| ------------ | -------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| Review 1     | 5        | Defined the implementation checks for conversion stages, default options, explicit overrides, and independent assertions.                   |
| Review 2     | 2        | Defined checks for Markdown stability, documented instability, preservation of intermediate output, and diagnostic reporting.               |
| Final review | 0        | Defined the final verification criteria for round-trip behavior, existing test coverage, and integration with the shared testing framework. |

#### Last implementation review

**Model:** OpenAI GPT-6

**Prompt:**

> Review the Rust implementation of Section 4, "Round-trip execution and project
> verification", against spec.md. Verify HTML-to-Markdown-to-HTML conversion,
> Markdown-to-HTML-to-Markdown stability, default Options::all() configuration,
> explicit option overrides, intermediate Markdown preservation, independent
> assertions, documented instability handling, Markdown-only test cases, and
> meaningful failure diagnostics. Identify concrete correctness problems without
> unnecessary refactoring.

**Review assessment:**

> The implementation must preserve the required sequence of conversion stages
> and apply the correct options at each stage. HTML-preservation and
> Markdown-stability assertions must remain independent, and Markdown-only cases
> must not require an unnecessary HTML expectation.
>
> Failure diagnostics should identify the affected case, failed assertion, and
> relevant intermediate output. Existing tests should retain their original
> coverage during migration.
>
> Successful execution of Cargo build, Clippy, tests, and the example
> establishes that the project baseline can compile and run. It does not by
> itself establish that the new round-trip implementation satisfies every
> requirement in Section 4.

Verification of this implementation revision
--------------------------------------------

**Date:** 2026-11-16, Asia/Almaty. **Environment:**
Alibek's local macOS ARM64 workspace. **Model:** GPT-6.1 Sol, Extra
High.

**Prompt:** `Verify that you can build and run the project in the
current git repo.`

**Summarized LLM output:** I built and ran the integrated implementation at
`14e66d4`, including the team's latest migration fixes and the Section 3
corrections reviewed above. All four commands completed successfully; the full
suite reported **271 passed, 1 ignored, 0 failed**, including all 6 comparator
tests. The Section 3 regressions had been observed failing before their fixes
and pass in this integrated version.

| Command                           | Exit code | Result                           |
| --------------------------------- | --------- | -------------------------------- |
| `cargo build`                     | 0         | Passed.                          |
| `cargo clippy -- --deny warnings` | 0         | Passed with warnings denied.     |
| `cargo test`                      | 0         | 271 passed; 1 ignored; 0 failed. |
| `cargo run --example simple`      | 0         | Example completed successfully.  |

Instructor feedback and provenance
----------------------------------

| Feedback                                  | Change in this revision                                                                                                                                  |
| ----------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| One grading report named `submission.md`  | Consolidated the supplied histories and results here; removed the three separate LLM-review files.                                                       |
| Work division belongs in the report       | Moved the ownership table from `spec.md` to this report.                                                                                                 |
| `Options::all()` by default               | Updated the case format, migration guidance, and runner strategy; explicit flag overrides remain possible.                                               |
| Separate mandatory case ID is unnecessary | Use the test function name, plus a table row number or optional label for diagnostics.                                                                   |
| HTML comparison was too strict            | Permit ASCII text-whitespace collapse and block formatting; preserve code/pre text, inline word separation, NBSP, structure, and attribute/comment data. |
| Verify each member's environment          | Recorded Alibek's measured checks and Yuriy/Kamila's confirmed local results; Sam's own verification remains for him to add.                             |

Source records are retained in Git history:

* [Sam's updated common draft report](https://github.com/Samboeng76/htmd/blob/79dd7e8839f76e5e0ffda568bf2fbbad1c652c95/llm-review.md).
* [Alibek's draft evidence](https://github.com/Samboeng76/htmd/blob/79dd7e8839f76e5e0ffda568bf2fbbad1c652c95/llm-review-alibek.md).
* [Kamila's supplied draft record](https://github.com/Samboeng76/htmd/blob/79dd7e8839f76e5e0ffda568bf2fbbad1c652c95/llm-review-kamila.md).

The 31-case whitespace probe in the draft used empty extension flags and the old
source-exact criteria. Its measurements remain historical evidence, not
validation of the new default flags or the revised proposed comparator.
