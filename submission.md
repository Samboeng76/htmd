# Team submission report

Project: htmd testing standardization.
Specification: [spec.md](spec.md).
PR target: [bjones1/htmd](https://github.com/bjones1/htmd).

## 1. Team contributions

The table gives final-style contribution wording assuming each assigned section
has been completed. Sam's and Alibek's contributions are recorded in Git;
Yuriy and Kamila must confirm their entries before final submission.

| Team member | One-sentence contribution |
| --- | --- |
| Yuriy Kim | Defined the project scope and common test-case format in section 1. |
| Sam English | Developed the test-migration strategy in section 2 and added the supporting per-test inventory. |
| Alibek Baisholanov | Specified the HTML comparison and correctness criteria in section 3 and completed two LLM review rounds of that section. |
| Kamila | Defined the round-trip runner configuration, failure diagnostics, and project-verification procedure in section 4. |

## 2. Compile and run verification

Exact prompt:

```text
Verify that you can compile and run the project in the current git repo.
```

Recorded LLM response:

> I verified that I can compile and run htmd in the current repository.

Verification was performed on **2026-11-08** at commit
`f596ee5fb3b045b9a2b34302b556662ed4de55e0`, on macOS ARM64 with Rust 1.99.1
and Cargo 1.99.1. Subsequent changes in this branch affect Markdown documents;
the Rust sources, tests, examples, and dependency files remain unchanged.

| Command executed | Exit code | Recorded result |
| --- | --- | --- |
| `cargo build` | 0 | Project compiled successfully. |
| `cargo clippy -- --deny warnings` | 0 | Clippy passed with warnings denied. |
| `cargo test` | 0 | 263 unit/integration tests and 4 doctests passed; 1 table-generation test was intentionally ignored; 0 failed. |
| `cargo run --example simple` | 0 | The example executable ran and its conversion assertions passed. |

The ignored test generates the Unicode punctuation/symbol table instead of
checking it. These results verify the existing project; the specification
describes the proposed future test migration.

## 3. Review rounds

Review scope: Alibek's section,
[HTML comparison and correctness criteria](spec.md#3-html-comparison-and-correctness-criteria).
Two successive self-review passes were performed in the same Codex session on
2026-11-08. Other team members' sections were outside these reviews.

| Round | Findings | Main changes made |
| --- | --- | --- |
| 1 | 3 | Defined the exact final-LF predicate and its permitted context, bounded Representation versus Loss, and required independent expected results grounded in the conversion rules. |
| 2 | 0 | No further changes: the revised section produced no relevant correctness findings. |

All three findings were applied; none were rejected or crossed out.

## 4. Last LLM review, prompt, and model

Model: **GPT-6.1 Sol**. Reasoning: **Extra High**.
The user confirmed these settings from the Codex interface.

Exact prompt used in both rounds, followed by the text of section 3:

```text
Review only the section named "HTML comparison and correctness criteria" from spec.md, supplied below. Check for contradictions, ambiguous pass/fail rules, and ways the comparison could hide lost content. Report only actionable correctness findings; do not review other sections or rewrite the text. Number the findings and give a short suggested correction for each. Do not target a predetermined finding count. If there are no relevant findings, say so.
```

Last review output, verbatim:

> No relevant correctness findings in the supplied section. The exact equality predicates, the scope of the final-LF allowance, the distinction between Representation and Loss, and the requirement for independent expected results are now explicit. No further changes recommended.

## Before final hand-in

Confirm all four contribution statements and that the team-identification
email has been sent to all teammates with Dr. Jones copied. Use the exact
assigned team/project label, since similarly named teams are separate teams.
