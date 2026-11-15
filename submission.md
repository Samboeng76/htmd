# Project 2 submission

Team name: standardize htmd testing - 1 (PR #13)

The specification consists of [spec.md](spec.md). The migration planning table
is [testing_standardization_test_inventory.md](testing_standardization_test_inventory.md).
This report follows the [Project 2 submission template](https://github.com/bjones1/literate-programming-book/blob/main/project_2_submission_template.md).

| Section | Owner | Responsibility |
| --- | --- | --- |
| 1 | Yuriy Kim | Scope and common test-case format |
| 2 | Sam English | Migration of existing tests |
| 3 | Alibek Baisholanov | HTML comparison and correctness criteria |
| 4 | Kamila Bagdat | Round-trip execution and project verification |

## Draft submission

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

Defined the scope and common case format, integrated the four sections, and updated the inventory to the instructor's current baseline.

#### Build and run test

**Model:** Claude Opus 5.5, as reported by Yuriy.

**Summarized result:** Yuriy confirmed successful local execution of
`cargo build`, `cargo clippy -- --deny warnings`, `cargo test`, and
`cargo run --example simple`. This participant-reported summary was confirmed
by Alibek on 2026-11-15; it is not a verbatim transcript.

#### Specification review rounds

| Round | Findings | Main changes made |
| --- | --- | --- |
| 1 | 1 | Aligned the two Faithful scope rows with Markdown-only cases and HTML assertions for preservation or approved documented differences. |
| 2 | 0 | Rechecked the corrected scope and case contract; no further relevant changes were requested. |

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

Developed and refined the migration strategy and per-test inventory, including the handling of kept Pure-mode cases.

#### Build and run test

Build, Clippy, and tests all passed. The test run had 268 passing tests, 0 failures, and 1 ignored test. Cargo printed a nonfatal path warning.

#### Specification review rounds

| Round | Findings | Main changes made |
| --- | --- | --- |
| 1 | 3 | Added implementation status to the inventory and merged Section 2 into the group specification. |
| 2 | 3 | Removed footnotes from the inventory. |
| 3 | 2 | Corrected the inventory entries for functions with multiple inputs. |
| 4 | 3 | Updated the specification to match the inventory and added the stability check to Section 2. |
| 5 | 3 | Added the concurrency exception and made minor inventory corrections. |
| 6 | 2 | Made minor clarifications and inventory edits. |
| 7 | 0 | No further changes were requested. |
| 8 | 0 | Checked migration rules against the shared case contract; no changes were requested. |
| 9 | 0 | Verified the test count, kept cases, mixed modes, existing round trips, fixture retention, and conditional helper removal against the source and inventory; no changes were requested. |

#### Last specification review

**Model:** GPT-6.1 Sol, Extra High, as recorded by Sam.

**Prompt:**

```text
review the migration of existing tests section of spec.md
```

**Last output (text extracted from Sam's HTML-formatted record):**

> I found no actionable correctness issues in “Migration of existing tests” (line 107). It covers individual input and mode cases, retained assertions, exact options, stability checks, and removal of helpers only after their callers are migrated.
>
> I checked the inventory and the versioned test source. Several test files are currently deleted in the working tree, so I did not run tests.

### Alibek Baisholanov

GitHub userid: alibekbtu

#### Contributions

Specified and reviewed the HTML-comparison, whitespace, parser-normalization, and stability criteria and checked them with 31 measured cases.

#### Build and run test

**Model:** GPT-6.1 Sol, Extra High.

**Prompt:**

```text
Verify that you can compile and run the project in the current git repo.
```

**Summarized LLM output, verified on 2026-11-09:**

> I verified that I can compile and run htmd. `cargo build`, `cargo clippy -- --deny warnings`, `cargo test`, and `cargo run --example simple` all exited successfully. The suite reported 263 passing unit/integration tests and 4 passing doctests, with one intentionally ignored test and zero failures.

#### Specification review rounds

| Round | Findings | Main changes made |
| --- | --- | --- |
| 1 | 3 | Defined the final-LF predicate and allowed context, distinguished Representation from Loss, and required independent expected results. |
| 2 | 0 | No further changes were requested for that version of Section 3. |
| 3 | 1 | Added parser-defined representation changes and distinguished literal CR preprocessing from character-reference CR, with measured whitespace/stability examples. |
| 4 | 0 | The revised Section 3 had no further relevant specification findings. |

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

Defined stage-specific runner options, expectation-specific assertions and diagnostics, and supplied the Section 4 review and verification record.

#### Build and run test

**Exact instructor-requested prompt:**

> Verify that you can compile and run the project in the current git repo.

**Verification assistant:** GitHub Copilot in VS Code - MAI-Code-1.1-Flash

**LLM output (successful verification response, preserved from the provided conversation):**

> I’m re-running the repo verification in the actual project root with the Rust toolchain available so I can give you a final yes/no status based on fresh build and run output.
>
> Ran terminal command: `$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; Set-Location "c:\Users\Documents\KBTU\Data mining\advanced\New folder\htmd"; cargo build; cargo clippy -- --deny warnings; cargo test; cargo run --example simple`
>
> Read [](file:///c%3A/Users/AppData/Roaming/Code/User/workspaceStorage/85bbe8ed1e7a15c7c5aad0216f8c0faa/GitHub.copilot-chat/chat-session-resources/6379a0aa-2159-4942-9666-4dc444547949/call_En95w4LXJJBKApVGZA6tasmd__vscode-1794233934546/content.txt#1-1), lines 1 to 260
>
> Ran terminal command: `$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; Set-Location "c:\Users\bagdatkamilla\Documents\KBTU\advanced\New folder\htmd"; cargo run --example simple; $code = $LASTEXITCODE; Write-Host "RUN_EXIT:$code"; exit $code`
>
> ## ✅ Verification passed
>
> I verified the project in the current repo with the requested checks:
>
> - `cargo clippy -- --deny warnings` → exit code 0
> - `cargo test` → all reported test groups passed with 0 failed
> - `cargo run --example simple` → exit code 0
>
> ### Evidence
> The final sample run completed successfully:
>
> ```text
>     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
>      Running `target\debug\examples\simple.exe`
> RUN_EXIT:0
> `

#### Specification review rounds

| Round | Findings | Main changes made |
| --- | --- | --- |
| 1 | 2 | Identified missing intermediate-Markdown, per-case option, instability, and Markdown-only handling; no edits were applied in that round. |
| 2 | 2 | Rechecked per-stage options and stability ambiguity; no edits were applied in that round. |
| 3 | 3 | Checked consistency across the specification, including instability exceptions, Markdown-only reporting, and the limits of the default helper; no edits were applied in that round. |
| 4 | 0 | Applied concise revisions for per-stage options, independent assertions, stability expectations, and expectation-specific diagnostics; the recorded final consistency review found no additional relevant issues. |

#### Last specification review

**Model:** OpenAI GPT-6.

<strong>Exact review prompt used for the final written consistency
check:</strong>

> Review the updated `spec.md`, concentrating on Section 4 and its consistency
> with Sections 1–3. Check the handling of default versus generalized round-trip
> helpers, htmd and pulldown-cmark options, `Stable` versus
> `DocumentedInstability` expectations, Markdown-only cases, failure reporting,
> and the verification command list. Report only concrete outstanding problems
> in the specification; do not infer that the proposed runner has been
> implemented.

<strong>Final review output:</strong>

> No additional relevant findings in the supplied specification. Section 4 now
> distinguishes the stage-specific converter options; preserves the intermediate
> Markdown; specifies `Stable` and approved `DocumentedInstability` comparisons;
> allows exact-Markdown cases without an HTML expectation; and reports failures
> according to which assertion failed. Section 3 now states the
> documented-instability exception explicitly. This is a consistency review of
> the written strategy, not a source-code execution or validation of the
> unimplemented generalized runner.

## Final submission

Status: in progress. The instructor-feedback revisions to the strategy are
recorded below. Implementing the proposed runner/comparator and completing the
migration, implementation reviews, and each member's additional authored commit
remain final-submission work. Draft review rounds above are not counted here.

### Yuriy Kim

GitHub userid: kim0111

#### Contributions

Yuriy will record final-stage contributions here.

#### Specification review rounds

No final-stage rounds are recorded here yet.

| Round | Findings | Main changes made |
| --- | --- | --- |

#### Last specification review

The section owner will add the final-stage model, prompt, and output here.

#### Implementation review rounds

The proposed implementation has not been completed in this documentation revision.

| Round | Findings | Main changes made |
| --- | --- | --- |

#### Last implementation review

The implementation owner will add the actual model, prompt, and last review
output after implementation and repeated review.

### Sam English

GitHub userid: Samboeng76

#### Contributions

Sam will record final-stage contributions here.

#### Specification review rounds

No final-stage rounds are recorded here yet.

| Round | Findings | Main changes made |
| --- | --- | --- |

#### Last specification review

The section owner will add the final-stage model, prompt, and output here.

#### Implementation review rounds

The proposed implementation has not been completed in this documentation revision.

| Round | Findings | Main changes made |
| --- | --- | --- |

#### Last implementation review

The implementation owner will add the actual model, prompt, and last review
output after implementation and repeated review.

### Alibek Baisholanov

GitHub userid: alibekbtu

#### Contributions

Revised the HTML-comparison strategy in response to instructor feedback, integrated the shared specification corrections, and organized the common submission report.

#### Specification review rounds

Only final-stage Section 3 reviews from 2026-11-15 are included.

| Round | Findings | Main changes made |
| --- | --- | --- |
| 1 | 0 | Reviewed the revised whitespace-aware comparison, protected code/pre content, documented expectations, and exact Markdown checks; no further changes were requested. |
| 2 | 0 | Rechecked the examples and boundary cases for inline word separators, NBSP, parser normalization, and stability; no additional changes were requested. |

#### Last specification review

**Model:** GPT-6.1 Sol, Extra High, the user-confirmed model for this Codex session.
These were successive self-reviews in this session.

**Prompt:**

```text
Review only Section 3, "HTML comparison and correctness criteria", in spec.md after the instructor feedback on PR #13. Check permitted ASCII-whitespace collapse outside code, exact code/pre text and descendants, inline word separators, NBSP, attribute/comment data, HTML parsing, exact documented expectations, and exact Markdown stability under fixed options. Use the other sections only as context. Report only actionable correctness findings with short corrections; do not rewrite other sections or target a finding count. If no relevant findings remain, say so.
```

**Final output, verbatim:**

> No further relevant correctness findings in Section 3. Checking the examples and boundary cases confirms that ordinary tab/newline collapse and block-separator LFs may pass, while removing an inline word separator, replacing NBSP, or changing code whitespace fails preservation. Literal HTML preprocessing is distinguished from a CR produced by a character reference, and HTML whitespace allowances do not weaken the exact Markdown stability assertion. No additional specification change is requested.

Reviewed Section 3 SHA-256 with LF line endings:
`b366be99b0b51c722e736fe0394f6d0c083b185626b33d7071b04d36b27b005d`.

#### Implementation review rounds

The proposed implementation has not been completed in this documentation revision.

| Round | Findings | Main changes made |
| --- | --- | --- |

#### Last implementation review

The implementation owner will add the actual model, prompt, and last review
output after implementation and repeated review.

### Kamila Bagdat

GitHub userid: bagdatkamilla

#### Contributions

Refined Section 4, "Round-trip execution and project verification", incorporating instructor feedback and improving the strategy for round-trip execution, stage-specific converter options, Markdown stability, independent assertions, and failure diagnostics. Contributed to ensuring consistency between the shared test-case format, HTML comparison criteria, and implementation requirements.

#### Specification review rounds

| Round | Findings | Main changes made |
| --- | --- | --- |
| Review 1 | 4 | Evaluated the updated conversion strategy, focusing on `Options::all()` defaults, explicit option overrides, and consistency with the shared test-case format. |
| Review 2 | 2 | Examined intermediate Markdown preservation, independent HTML and Markdown assertions, Markdown-only cases, and documented instability handling. |
| Final review | 0 | Assessed the specification's coverage of round-trip execution, error diagnostics, and project verification requirements. |

#### Last specification review

**Model:** OpenAI GPT-6

**Prompt:**

> Review only Section 4, "Round-trip execution and project verification", in spec.md. Check the consistency of converter options, default Options::all() behavior, HTML-to-Markdown-to-HTML execution, Markdown stability, documented instability expectations, Markdown-only test cases, intermediate Markdown preservation, failure diagnostics, and verification commands. Identify correctness issues and contradictions without expanding the project scope.

**Review assessment:**

> The documented Section 4 strategy addresses the major requirements for round-trip execution and project verification. It distinguishes converter options across execution stages, preserves intermediate Markdown for independent assertions, and defines stability expectations for Markdown-only and HTML-preservation cases.
>
> The specification also describes expectation-specific diagnostics and verification commands. The final revised source should be checked to confirm that these requirements remain consistent with Sections 1–3 and that all instructor-requested changes have been incorporated.

#### Implementation review rounds

| Round | Findings | Main changes made |
| --- | --- | --- |
| Review 1 | 5 | Defined the implementation checks for conversion stages, default options, explicit overrides, and independent assertions. |
| Review 2 | 2 | Defined checks for Markdown stability, documented instability, preservation of intermediate output, and diagnostic reporting. |
| Final review | 0 | Defined the final verification criteria for round-trip behavior, existing test coverage, and integration with the shared testing framework. |

#### Last implementation review

**Model:** OpenAI GPT-6

**Prompt:**

> Review the Rust implementation of Section 4, "Round-trip execution and project verification", against spec.md. Verify HTML-to-Markdown-to-HTML conversion, Markdown-to-HTML-to-Markdown stability, default Options::all() configuration, explicit option overrides, intermediate Markdown preservation, independent assertions, documented instability handling, Markdown-only test cases, and meaningful failure diagnostics. Identify concrete correctness problems without unnecessary refactoring.

**Review assessment:**

> The implementation must preserve the required sequence of conversion stages and apply the correct options at each stage. HTML-preservation and Markdown-stability assertions must remain independent, and Markdown-only cases must not require an unnecessary HTML expectation.
>
> Failure diagnostics should identify the affected case, failed assertion, and relevant intermediate output. Existing tests should retain their original coverage during migration.
>
> Successful execution of Cargo build, Clippy, tests, and the example establishes that the project baseline can compile and run. It does not by itself establish that the new round-trip implementation satisfies every requirement in Section 4.

## Verification of this documentation revision

**Date:** 2026-11-15, Asia/Almaty. **Environment:** Alibek's local macOS ARM64
workspace. **Model:** GPT-6.1 Sol, Extra High.

**Prompt:** `Verify that you can compile and run the project in the current git repo.`

**Summarized LLM output:** All four commands passed again on the PR #13
baseline `79dd7e8`: `cargo build`, `cargo clippy -- --deny warnings`,
`cargo test`, and `cargo run --example simple`. The suite reported **267
passed, 1 ignored, 0 failed**. The subsequent changes are documentation only;
Rust sources, tests, examples, and Cargo files are unchanged. This local check
does not substitute for the other members' individual environments or validate
the proposed generalized runner, which remains subsequent work.

## Instructor feedback and provenance

| Feedback | Change in this revision |
| --- | --- |
| One grading report named `submission.md` | Consolidated the supplied histories and results here; removed the three separate LLM-review files. |
| Work division belongs in the report | Moved the ownership table from `spec.md` to this report. |
| `Options::all()` by default | Updated the case format, migration guidance, and runner strategy; explicit flag overrides remain possible. |
| Separate mandatory case ID is unnecessary | Use the test function name, plus a table row number or optional label for diagnostics. |
| HTML comparison was too strict | Permit ASCII text-whitespace collapse and block formatting; preserve code/pre text, inline word separation, NBSP, structure, and attribute/comment data. |
| Verify each member's environment | Recorded Alibek's measured checks and Yuriy/Kamila's confirmed local results; Sam's own verification remains for him to add. |

Source records are retained in Git history:

- [Sam's updated common draft report](https://github.com/Samboeng76/htmd/blob/79dd7e8839f76e5e0ffda568bf2fbbad1c652c95/llm-review.md).
- [Alibek's draft evidence](https://github.com/Samboeng76/htmd/blob/79dd7e8839f76e5e0ffda568bf2fbbad1c652c95/llm-review-alibek.md).
- [Kamila's supplied draft record](https://github.com/Samboeng76/htmd/blob/79dd7e8839f76e5e0ffda568bf2fbbad1c652c95/llm-review-kamila.md).

The 31-case whitespace probe in the draft used empty extension flags and the
old source-exact criteria. Its measurements remain historical evidence, not
validation of the new default flags or the revised proposed comparator.
