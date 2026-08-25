# M09B Bounded Identity Strict Re-Audit

Date: 2026-08-25
Branch: `H!veAI`
M09B prompt base: `8b16a4ce18cf4bd3f0648e8f029aba8b13843797`
Primary M09B implementation commit: `f919fb664c8b0f74c9a7c626e80e0db59d34fad3`
M09B publication-log commit: `a2d6e90b4bcc5402db377911fffaf1f14c7085f7`
Audited remote branch HEAD before this audit: `1fcc242304cdbb6e78aa082abdfc0d9072244b58`

## 1. VERDICT

`FAIL`

M09B correctly fixes R01 path identity and closes the persisted-output portion of R02, and it materially strengthens E01-E04. One production MAJOR remains: oversized source headings/explicit identifiers are still retained in unbounded identity/duplicate-working strings during parsing even though their persisted display/evidence fields are bounded. This violates the M09B requirement for a stable bounded identity representation and can still amplify one <=2 MiB source into very large parser working memory.

Severity summary:

- BLOCKER: 0
- MAJOR: 1
- MINOR: 3
- NOTE: 2

M10 remains blocked.

## 2. CONTRACT RECOVERY

M09B was required to do only six bounded things:

1. R01: replace prose normalization for source-path task identity with path-specific normalization that preserves meaningful filename whitespace.
2. R02: bound all source-derived persisted scalar output and keep identity representation bounded/deterministic.
3. E01: strengthen retry-containment evidence so the test reaches the retry branch.
4. E02: assert exact handoff merge order from M08 discovery order.
5. E03: directly prove stale M09 source/task reconciliation while retained M09 and unrelated legacy source/task/settings rows survive, with no duplicate SOURCE_EXPLICIT edges.
6. E04: record FormuLab/ScrubBots/FMCG adapter convention truthfully, including UNVERIFIED where no distinct convention exists.

It also required focused/full regression, governed no-bundle publication, immutable historical logs, no M10/UI/installer scope expansion, and final repository publication/equality evidence.

## 3. BRANCH / HEAD / DIFF SCOPE

Compared `8b16a4ce...` to audited remote HEAD `1fcc2423...`.

The M09B production change is confined to `H!veAI/src-tauri/src/task_intelligence.rs`, plus the M09B builder log and tracker documentation. Subsequent commits on the branch are documentation-only expansions of TASKS/ROADMAP/README tracking. No visible UI production source, canonical visual asset, Git Engine, watcher, StartupIntro, M10 state-machine code, or installer change is present in the audited scope.

## 4. ACCEPTANCE CRITERIA MATRIX

| Criterion | Result | Audit conclusion |
|---|---|---|
| R01 path-specific identity normalization | PASS | `normalize_path_identity()` preserves repeated filename whitespace and normalizes separators/case policy. |
| R01 production regression fixture | PASS | Two M08-approved whitespace-distinct paths survive with distinct task IDs and SQLite rows. |
| R02 bound explicitTaskId/milestone/heading/evidence/handoff/locator persisted values | PASS | Persisted task/handoff/evidence values are bounded through `bounded_field()`. |
| R02 deduplicated truncation warning | PASS | Equivalent FIELD_TRUNCATED warnings are deduplicated by code/path/message. |
| R02 bounded deterministic identity representation | FAIL | Raw oversized heading/explicit strings still feed unbounded `duplicate_ordinals` keys and a large formatted identity string. |
| R02 repeated parse deterministic output | PASS | Existing tests prove semantic repeatability for the supplied fixtures. |
| E01 retry-specific containment evidence | PARTIAL | Test now reaches retry, but changes file->directory and fails on re-read; it does not directly make refreshed path escape containment. Production re-check is visible. |
| E02 exact handoff order | PASS | Expected order is derived from M08 parser-source order and compared exactly. |
| E03 stale source/task + retained/legacy preservation | PARTIAL | Stale source/task and legacy source/task are proven, but required legacy settings preservation and meaningful SOURCE_EXPLICIT edge assertion are absent from this named fixture. |
| E04 truthful adapters | PASS | FormuLab PASS; ScrubBots/FMCG explicitly UNVERIFIED with no bonus. |
| Full tests/security/build | PASS BY BUILDER CLAIM + SOURCE CONSISTENCY | Log records 50 focused parser tests, 187 Rust, 70 frontend, type/build/audit/fmt/check/build green. No contradictory repository evidence found. |
| Governed no-bundle publication | PASS BY BUILDER CLAIM | Publication commit records stable EXE SHA/size/shortcut and PASS. |
| Final equality after all final commits | PARTIAL | Builder equality was recorded at implementation HEAD before the publication-log commit; later documentation commits also advanced remote HEAD. Remote visibility is proven, exact final builder local==origin equality is not. |
| No M10/UI/installer scope creep | PASS | No such production changes in audited diff. |

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH

### R01
Builder claim: PASS.
Repository truth: PASS.

`task_id()` now calls `normalize_path_identity(path)` instead of prose `normalize_text(path)`. The path normalizer canonicalizes `\\` to `/`, removes empty/dot components, preserves internal filename whitespace, and applies ASCII case equivalence on Windows. `parse()` also deduplicates duplicate M08 inventory rows for the same normalized source path before parsing.

The direct `r01_distinct_whitespace_paths_never_collide` fixture creates `plans/a b.md` and `plans/a  b.md`, routes them through M08+M09, requires exactly two tasks, distinct source paths, distinct IDs, and two M09-owned SQLite task rows. This would fail on the pre-fix implementation.

### R02
Builder claim: PASS.
Repository truth: PARTIAL / MAJOR residual.

Persisted scalar bounding is substantially fixed. However the builder claim that identity is fully bounded is not supported by production source.

The fallback duplicate-ordinal key still does:

`format!("{}|{}", context.join("/"), normalize_text(&candidate.title))`

where `context` contains raw, unbounded heading strings. `duplicate_ordinals` retains this `String` key for each unique task identity during the source parse.

`task_id()` also builds one large `format!` identity containing normalized raw heading context or the raw explicit ID before hashing it.

A single source may be <=2 MiB while containing a very large heading followed by many distinct tasks. Persisted milestone/evidence values are bounded, but the same raw heading can still be copied into many `duplicate_ordinals` keys, producing large in-memory amplification. The M09B prompt explicitly required a stable bounded identity representation, not only bounded display output.

The named oversized-heading test checks persisted milestone/evidence length and warning count only. It does not prove the identity/ordinal working representation is fixed-size or bounded, so it passes despite the remaining production defect.

## 6. FILE / SYMBOL EVIDENCE

Accepted symbols:

- `parse()` source-path deduplication via `parsed_paths`.
- `normalize_path_identity()` path-specific identity normalization.
- `bounded_field()` UTF-8 safe truncation + equivalent warning dedupe.
- bounded milestone/heading context persisted into tasks/evidence.
- bounded handoff values and handoff evidence heading paths.
- bounded persisted explicit task ID and locator text.
- strengthened stale-source and handoff tests.

Residual production symbols:

- `parse_document()` `duplicate_ordinals: HashMap<String, usize>` stores raw-context-derived keys.
- fallback ordinal key uses `context.join("/")` before hashing/bounding.
- `task_id()` constructs a potentially very large intermediate identity `String` before SHA-256.

## 7. FOCUSED TEST EVIDENCE

### Accepted

- `r01_distinct_whitespace_paths_never_collide`
- `r02_oversized_heading_is_bounded_without_snapshot_amplification` for persisted output only
- `r02_oversized_handoff_value_is_bounded`
- `r02_oversized_explicit_id_is_bounded_and_deterministic`
- `r02_bounded_snapshot_repeat_is_deterministic`
- `p06_multiple_handoff_sources_merge_in_source_order`
- stale-source direct SQL portions of `p07_removed_task_and_source_reconcile_only_stale_m09_rows`

### Evidence gaps

E01 test reaches the retry path, then mutates the file into a directory. The source visibly re-canonicalizes and rechecks `starts_with(root)`, so production behavior is acceptable, but the test still does not directly exercise an escaped refreshed target.

E03 fixture does not seed/preserve a legacy settings row despite the prompt requiring one, and its fixture has no meaningful SOURCE_EXPLICIT dependency edge whose duplicate-free reconciliation is checked.

Most importantly, no R02 test proves the duplicate-ordinal/identity working representation is fixed-size or bounded. The current implementation would pass all supplied R02 tests while retaining the raw oversized heading in many internal keys.

## 8. REGRESSION EVIDENCE

Builder log reports:

- 50 task-intelligence focused tests PASS;
- 187 Rust tests PASS;
- 70 frontend tests PASS;
- typecheck/build/npm audit/cargo fmt/check/build PASS;
- publisher failure harness PASS;
- production no-bundle publisher PASS.

No source-level regression is visible in M08 boundary usage, M09 persistence UPSERT, adapter safety, status parsing, nested metadata, or handoff merge.

## 9. SECURITY / SAFETY REVIEW

PASS for M09B scope:

- no network/AI/parser-side shell expansion;
- no project-file writes by M09 parser;
- registered-root physical validation remains in place;
- no unrestricted frontend permissions added;
- no installer;
- no M10 workflow mutations.

The remaining R02 issue is resource-bounding/safety rather than privilege escalation.

## 10. ARCHITECTURE CONSISTENCY

M08 remains the sole source-discovery authority. M09 consumes only M08-owned AVAILABLE sources. M09 still stores neutral parser state and does not implement M10 transitions. SQLite stable task IDs remain anchors for future task events.

R01 is now consistent with filesystem identity. R02 is not fully consistent with the local-first bounded-parser architecture until identity/ordinal working keys stop carrying raw oversized source context.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

Current detailed `TASKS.md` is appropriately conservative: M09B implementation items remain active/pending independent re-audit, M09 is NOT CLOSED, and M10 is blocked.

M09B log truthfully records ScrubBots/FMCG as UNVERIFIED. Its R02 PASS statement overstates production closure because internal identity working strings remain unbounded.

Historical M09/M09A logs remain unchanged.

## 12. FINAL REPOSITORY STATE

Audited remote branch before this audit: `1fcc242304cdbb6e78aa082abdfc0d9072244b58`.

M09B implementation commit `f919fb66...` and publication-log commit `a2d6e90b...` are visible on the remote branch. Later branch commits are task-tracking documentation only.

Builder-local equality after the final publication-log commit is not proven. This is a bookkeeping/evidence issue, not a production defect.

## 13. OPEN CROSS-MILESTONE FINDINGS

Still queued outside M09 parser scope:

- X01: visible Windows console/terminal windows from Git child processes. Fix before M10.
- X02: startup intro is muted despite canonical video audio. Fix before M10.

Do not mix these into the M09C parser micro-fix.

## 14. DEFECTS BY SEVERITY

### MAJOR R02C - unbounded parser identity/ordinal working representation

A raw oversized heading/explicit identifier is still incorporated into large intermediate identity strings and retained duplicate-ordinal keys. This violates the bounded identity requirement and can amplify parser memory independently of persisted-output bounds.

### MINOR E01C - retry containment test still proves path refresh more than escaped containment

Production code rechecks containment, but direct retry-escape evidence remains weak.

### MINOR E03C - stale-source fixture misses required legacy settings and dependency-edge assertions

The named test does not fully satisfy its own acceptance contract.

### MINOR E05 - final builder equality bookkeeping

Exact local==origin equality after the final publication-log commit is not recorded. Remote commit visibility is proven.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

- Consider a fixed-size digest key (`[u8; 32]` or 64-char hex) for duplicate ordinal identity.
- Compute task ID SHA-256 incrementally instead of allocating a potentially large formatted identity string. Preserve exact current hash semantics for normal tasks if possible to avoid unnecessary ID churn.
- Keep truncation-warning dedupe centralized so new source-derived fields cannot reintroduce warning amplification.

## 16. UNVERIFIED ITEMS

- ScrubBots distinct special convention: UNVERIFIED by design, generic fallback accepted.
- FMCG distinct special convention: UNVERIFIED by design, generic fallback accepted.
- Real Windows symlink/junction retry-escape fixture remains environment-dependent; current production containment code is source-level verified.
- Exact builder-local HEAD equality after final publication-log commit is unverified.

## 17. REGRESSION RISK

`MEDIUM`

The remaining fix is narrow, but task identity is a high-leverage area. A careless change can churn stable task IDs and later task-event anchors. M09C must preserve current deterministic IDs while changing only the internal working representation where practical.

## 18. AUDIT CONFIDENCE

`HIGH`

The audit inspected the prompt, current production source, direct test bodies, M08 ordering/identity boundary, M09B implementation/publication commits, current tracker truth, and remote branch scope.

## 19. FINAL VERDICT

`M09B = FAIL`

R01 is closed. Persisted R02 scalar bounds are closed. One production MAJOR remains in the bounded identity/ordinal working representation, plus three non-blocking evidence/bookkeeping gaps.

M09 remains OPEN. M10 remains BLOCKED.

## 20. REQUIRED REMEDIATION

One tiny M09C micro-fix only:

1. replace raw-context duplicate-ordinal keys with a fixed-size deterministic digest/key;
2. compute task identity hashing without retaining/formatting huge raw context strings, while preserving deterministic/stable IDs where feasible;
3. add a direct oversized-heading identity-working-key regression that fails on M09B;
4. strengthen retry-escape evidence without requiring unsupported OS privileges;
5. extend stale-source fixture with a legacy settings row and meaningful dependency-edge duplicate assertion;
6. run full regression/publication and stop before M10/X01/X02.
