# M09 Task Intelligence Parser Strict Audit

Date: 2026-08-25
Branch: `H!veAI`
Prompt base: `69e312d54380b95bcef1cfc915e6e50299a54f62`
Implementation/publication commit: `cdac3774a403a04ae94db707483a8dfad27efa52`
Documentation-only publication-equality follow-up / audited branch HEAD: `d3c45320a3d8f0aff662d2d79683ce9d38b4164f`

## 1. VERDICT

`FAIL`

M09 establishes a substantial deterministic parser foundation, a narrow IPC surface, and M09-owned persistence, but the implementation does not yet satisfy several explicit production and direct-evidence contracts from `M09_TASK_INTELLIGENCE_PARSER_PROMPT.md`.

There are no BLOCKER findings, but multiple MAJOR findings remain. M10 must not start.

## 2. CONTRACT RECOVERY

The M09 prompt requires one bounded deterministic parser milestone that:

- consumes only current M08-owned AVAILABLE source inventory;
- performs bounded, containment-checked, hash-matched source reads with one real rediscovery/retry on source change;
- emits normalized deterministic task/handoff intelligence with structured bounds/warnings;
- preserves stable IDs independent of unrelated line movement;
- parses explicit checklist/status/metadata syntax without free-prose inference;
- extracts blockers, dependencies, next step, actor, owner gate, waits and acceptance criteria;
- separates handoff narrative from tasks;
- implements real Generic/FormuLab/ScrubBots/FMCG adapter behavior rather than name aliases;
- persists M09-owned task intelligence non-destructively and idempotently using the existing M04 schema;
- exposes only explicit bounded parse/list IPC with narrow ACL;
- supplies direct tests that would fail if each production contract regressed;
- does not implement M10, redesign UI, create an installer, use AI/network/shell parsing, or mutate managed project files.

## 3. BRANCH / HEAD / DIFF SCOPE

`69e312d... -> d3c4532...` is two commits ahead with no divergence in the audited GitHub branch history.

The M09 change set is bounded to:

- `src-tauri/src/task_intelligence.rs`
- native command registration / permission / capability
- `src/taskIntelligence.ts`
- focused M09 tests and synthetic fixture files
- `ARCHITECTURE.md`, `TASKS.md`, and M09 log

No visible UI production source or canonical visual asset is changed by M09. M10 is not implemented.

## 4. ACCEPTANCE CRITERIA MATRIX

| Contract | Result | Audit summary |
|---|---|---|
| P01 secure M08-to-M09 boundary | PARTIAL | M08 inventory, containment, bounded UTF-8 and hash checks exist, but the source-change retry is not a real parse retry and project-wide/scalar bound warnings are incomplete. |
| P02 normalized model + deterministic identity | PARTIAL | Model/prefixes/project isolation exist; required movement tests are absent and fallback identity uses raw rather than normalized heading path. |
| P03 generic deterministic Markdown parser | PARTIAL | Checkboxes and neutral storage mapping exist; required explicit-status/heading tests are incomplete and status tags on checklist tasks are not interpreted. |
| P04 structured metadata | FAIL | Inline labels work, but nested label blocks are not parsed, `owner_gate` is never populated, and required negative/association evidence is incomplete. |
| P05 handoff intelligence | PARTIAL | Narrative/checklist separation works for one fixture; required Current/Next/Blocker/Waiting evidence is incomplete and multiple handoff sources are not aggregated. |
| P06 adapters | FAIL | Current adapters are effectively Registry-name aliases; `convention_matched` is true from project name alone and claimed conventions are mostly generic grammar. |
| P07 persistence/reconciliation | PARTIAL | M09 ownership and transaction boundary exist, but same-identity rows are delete/reinsert rather than updated and required changed-metadata/removal SQL evidence is missing. |
| P08 native IPC / frontend boundary | PASS | Two explicit commands, narrow permission, capability entry and TS wrappers are present; no route-driven parsing was added. |
| P09 status/warnings/security | PARTIAL | ACTIVE/MISSING/ARCHIVED parse boundary and warning cap exist; several required bound/leakage tests are missing. |
| P10 evidence/confidence | PARTIAL | Locator/confidence structures are deterministic in simple cases, but adapter bonus can be granted without matching adapter-specific evidence and repeatability coverage is incomplete. |

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH

The M09 log reports P01-P10 as PASS and 20/20 focused Rust plus 2/2 focused TypeScript tests. Aggregate execution claims are plausible, but several named tests do not prove the full acceptance contract.

Examples:

- `p01_source_change_is_retried_once_then_warned` asserts that a once-changed source returns `None`; it does not prove a rediscovered stable new version is actually reread and parsed.
- `p02_ids_are_stable_and_project_scoped` parses the exact same fixture twice; it does not insert unrelated lines/move tasks as the prompt requires.
- `p03_checkbox_and_neutral_storage_mapping` does not test `[DONE]`, `[WAITING]`, `[READY]`, `[IN PROGRESS]` explicit status tokens.
- `p04_structured_metadata_and_unknown_actor` does not assert acceptance criteria, owner gate, casual-prose negative behavior, or nested label children.
- `p05_handoff_narrative_and_checklist_are_separate` does not prove narrative in Next plus blocker summary together.
- `p09_warning_bound_is_structured` exercises the warning helper directly, not the production task/scalar-bound paths.
- `p10_locator_and_confidence_are_bounded_and_deterministic` performs one parse and does not prove repeated score/reason equality.

## 6. FILE / SYMBOL EVIDENCE

### Accepted foundation

`task_intelligence::parse()` begins with `task_sources::discover()` and `is_parser_source()` requires M08 owner/schema, AVAILABLE status, task-bearing source kinds and allowed text extensions. INSTRUCTION/AGENTS/CLAUDE sources are excluded from generic task production.

`read_authoritative_source()` canonicalizes the Registry root and target and rejects physical targets outside the root. `read_bounded_text()` uses `MAX_SOURCE_BYTES + 1`, SHA-256 and UTF-8 validation.

The normalized Rust/TypeScript model includes task IDs, parsed/storage states, actor/blocker/dependency/next/wait/acceptance fields, evidence and confidence.

Persistence uses an SQLite transaction and M09 prefixes/owner metadata. IPC is only `hiveai_task_intelligence_parse` and `hiveai_task_intelligence_list` under `allow-task-intelligence`.

### F01 - MAJOR - source-change retry contract is implemented incorrectly

In `read_authoritative_source()`:

1. old M08 source is read;
2. hash mismatch triggers `task_sources::discover()`;
3. if refreshed hash differs from the old source hash, the function immediately returns `Ok(None)`.

A normal stable file edit therefore never gets the promised bounded second read against the refreshed M08 evidence. The correct behavior is to rediscover once, re-resolve/recanonicalize the refreshed source path, reread once, and accept the new bytes when they match the refreshed hash. Only a second change should yield `SOURCE_CHANGED_DURING_PARSE`.

The focused test encodes the defective behavior by expecting `None` after a single stable edit.

### F02 - MAJOR - parser bounds can truncate silently

The prompt requires project-wide maximum 4096 tasks and structured warning whenever a parser bound is reached.

Current production behavior:

- `parse_document()` enforces 4096 per source;
- after combining sources, `parse()` calls `snapshot.tasks.truncate(MAX_TASKS)` with no project-level `TASK_LIMIT_REACHED` warning;
- `truncate()` silently clips scalar text to 4096 UTF-8 bytes with no structured field-bound warning;
- focused tests do not exercise a multi-source project-wide task overflow or scalar-field overflow through the production parser.

This violates the explicit no-silent-truncation contract.

### F03 - MAJOR - repo-specific adapters are name aliases, not evidenced parser adapters

`adapter_for(name)` sets `formulab`, `scrubbots`, or `fmcg-erp-system` and `convention_matched = true` based only on exact Registry project name.

There is no adapter-specific parse/augment behavior. `explicit_id()` recognizes `TASK-`, `FVL-`, and `FMCG-` globally for every project. Generic heading/checklist parsing already handles the FMCG module/phase pattern. Therefore the claimed special adapters do not contribute a convention that is both source-evidenced and adapter-specific.

Worse, confidence uses:

`adapter.convention_matched && candidate.explicit_id.is_some()`

so a FormuLab task using a generic `TASK-123` ID can receive the repo-specific adapter bonus even when no FormuLab-specific convention matched that task/source.

This directly violates P06 and P10.

### F04 - MAJOR - structured metadata grammar is incomplete

`fields_for()` handles only same-line `Label: content` forms. If a task contains:

`Acceptance criteria:`

followed by indented child bullets/lines, the empty label is skipped and the children are ignored. The same problem applies to Blockers/Dependencies-style nested structures.

`Fields.owner_gate` and `ParsedTask.owner_gate` exist, but no production branch ever assigns `fields.owner_gate`; the field is always `None`.

The prompt requires explicit owner-gate extraction and association of indented child bullets/lines with the nearest task. Required negative evidence that casual prose containing `blocked` does not become a blocker is also absent.

### F05 - MAJOR - deterministic identity evidence and normalization are incomplete

P02 explicitly requires direct tests where unrelated lines are inserted/moved above explicit-ID and fallback tasks. The current test only reparses unchanged content.

Additionally, fallback identity includes `headings.join("/")` without applying the same normalization used for titles. The prompt calls for a normalized heading path. Case/spacing-only heading changes can therefore change a fallback task ID even though the normalized semantic heading is equivalent.

### F06 - MAJOR - explicit status and handoff evidence are incomplete

`task_line()` interprets `[DONE]`, `[BLOCKED]`, `[WAITING]`, `[READY]`, `[IN PROGRESS]` only in the non-checklist explicit-row branch. A checklist task such as `- [ ] [WAITING] vendor approval` remains parsed as OPEN with the tag in its title rather than WAITING.

The P03 tests do not exercise explicit status tags at all.

Handoff parsing proves one Current narrative and one Waiting narrative, but does not directly prove separate Next narrative plus Blocker summary as required. The outer `parse()` also keeps only the first non-None handoff summary (`if snapshot.handoff.is_none()`), so later approved HANDOFF sources are discarded instead of merged.

### F07 - MAJOR - persistence does not meet same-identity update contract

`persist()` deletes every M09-owned task for the project and reinserts it, even when the deterministic task identity is unchanged. The prompt requires changed metadata for the same identity to update rather than duplicate/recreate.

The existing idempotency test proves stable IDs and row count, but does not seed changed metadata and verify an in-place/upsert update. The required direct stale task/source removal reconciliation test is also absent.

This delete/reinsert pattern is additionally risky for M10 because `task_events.task_id` has ON DELETE CASCADE; once later milestones attach event history to stable task IDs, an M09 reparse could erase that history. M09 itself does not write task events, so this is recorded both as a present contract failure and an important cross-milestone regression risk.

## 7. FOCUSED TEST EVIDENCE

Direct production-path coverage that is accepted:

- instruction source exclusion;
- outside-root read rejection;
- invalid UTF-8 isolation;
- basic duplicate/project ID separation;
- four checkbox markers and neutral storage mapping;
- inline blocker/dependency/next/owner/wait/acceptance locator span;
- unambiguous/ambiguous dependency behavior;
- one handoff narrative/checklist case;
- Registry-name adapter selection;
- unrelated legacy task/source/settings preservation;
- persisted-list-only path;
- archived/missing parse rejection;
- warning helper cap;
- bounded locator sibling separation.

Missing or misleading direct coverage is captured in F01-F07.

The repository also contains synthetic fixture files under `tests/fixtures/m09`, but the focused Rust tests use inline strings rather than consuming those files. This is a MINOR maintainability/evidence issue, not a standalone production blocker.

## 8. REGRESSION EVIDENCE

Builder log claims:

- focused Rust 20/20;
- focused TS 2/2;
- frontend 70/70;
- Rust 157/157;
- typecheck/build/audit/fmt/check/build PASS;
- publisher failure harness 9/9;
- production `--no-bundle` publication PASS.

The M09 diff does not modify accepted UI production source or canonical assets. Asset hashes in the builder log match the previously accepted values. These execution counts remain builder claims; repository source inspection found no M09 UI-scope regression.

## 9. SECURITY / SAFETY REVIEW

PASS/accepted:

- no AI/network/shell parser path added;
- no unrestricted Tauri shell permission;
- parse source path is reconstructed beneath Registry root and canonicalized;
- only bounded two-command IPC is exposed;
- managed project files are read-only in M09 production code;
- unrelated legacy shared-table rows are not blanket-deleted by project id.

Required fix:

- the source-change retry must re-run physical containment against the refreshed source before the second read, not reuse stale physical-path evidence.

## 10. ARCHITECTURE CONSISTENCY

M08 remains discovery authority and M09 does not create a second crawler. M10 workflow states are not introduced; DONE/BLOCKED/BACKLOG storage mapping stays neutral.

However F03 and F07 conflict with the intended adapter and stable-task foundations required by future M10-M12 consumers.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

`TASKS.md` correctly leaves M09 pending independent audit and M10 blocked.

The M09 log is detailed and appropriately says builder completion is pending audit, but its P01-P10 PASS self-assessment overstates the evidence in F01-F07.

`ARCHITECTURE.md` reflects the intended M09 boundary, but final acceptance must wait for remediation.

## 12. FINAL REPOSITORY STATE

Audited branch HEAD: `d3c45320a3d8f0aff662d2d79683ce9d38b4164f`.

The implementation commit is `cdac3774...`; `d3c4532...` is the immediate documentation-only publication-equality follow-up. No M10 implementation is present.

## 13. OPEN CROSS-MILESTONE FINDINGS

These are not M09 implementation findings but remain real user-reported production defects and must not be forgotten:

### X01 - native Git child-process console windows

The current Git engine spawns `git` with `std::process::Command` and piped stdio but no Windows `CREATE_NO_WINDOW` creation flag. The filesystem watcher can trigger Git snapshots automatically, causing repeated child Git process launches while H!veAI remains open. The user reports visible terminal windows appearing repeatedly. Queue a bounded native UX hotfix after M09 closure and before M10.

### X02 - startup intro audio muted

`StartupIntro.tsx` still renders the canonical opening video with the `muted` attribute. The user confirms the source video contains audio and expects sound. Queue the startup-audio fix in the same bounded native UX hotfix after M09 closure and before M10.

Neither X01 nor X02 should be mixed into M09 parser remediation.

## 14. DEFECTS BY SEVERITY

BLOCKER: 0

MAJOR:

1. F01 real source-change rediscovery/retry is missing.
2. F02 project/scalar bounds can truncate silently.
3. F03 repo-specific adapters are aliases and adapter bonus is not convention-evidenced.
4. F04 nested metadata/owner-gate contract incomplete.
5. F05 movement identity evidence + normalized heading identity incomplete.
6. F06 explicit status/handoff contracts incomplete.
7. F07 same-identity persistence update contract incomplete.

MINOR:

- Synthetic fixture files exist but focused Rust tests do not consume them directly.

NOTE:

- Builder execution totals are claims pending independent local rerun capability; source-level direct test bodies were inspected.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

- Avoid O(tasks x source-lines) rescanning for heading context if large task files become common; a one-pass parser context can reduce future latency.
- Persist stable task rows with UPSERT/update semantics before M10 starts attaching durable event history.
- Make task-level warnings useful or remove the currently always-empty `ParsedTask.warnings` field in a later schema cleanup.

## 16. UNVERIFIED ITEMS

- Builder-reported local inspection of real FormuLab/ScrubBots/FMCG private source structure cannot be independently reproduced from GitHub evidence alone. More importantly, current production adapter code does not actually enforce those claimed conventions, so P06 fails regardless.
- Windows physical symlink/junction creation retains the previously accepted OS error 1314 environment limitation from M08; M09 does not change that boundary.

## 17. REGRESSION RISK

`MEDIUM-HIGH`

The parser is bounded and local-first, but adapter confidence, stable persistence, metadata completeness and source-change behavior feed directly into future workflow/Command Center truth. Advancing to M10 now would bake ambiguous parser semantics into later state-machine logic.

## 18. AUDIT CONFIDENCE

`HIGH`

The audit inspected the actual M09 diff, production Rust/TS code, direct focused test bodies, Tauri permissions/capability, branch HEAD and prompt contracts. The primary failures are visible in production symbols rather than inferred solely from logs.

## 19. FINAL VERDICT

`M09 = FAIL / BOUNDED REMEDIATION REQUIRED`

M00-M08 remain PASS/CLOSED. M10 remains BLOCKED/UNSTARTED.

## 20. REQUIRED REMEDIATION

Create one bounded M09A strict-closure pass. Do not split into M09.01/M09.02/etc.

M09A must fix F01-F07 only, add direct regression tests that demonstrate the pre-fix failures, preserve all accepted M08/M09 boundaries, and stop before M10.

After M09 is independently closed, run a separate native UX hotfix for X01/X02 before authorizing M10.
