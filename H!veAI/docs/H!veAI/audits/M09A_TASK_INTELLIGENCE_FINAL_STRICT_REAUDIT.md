# M09A Task Intelligence Parser Final Strict Re-Audit

Date: 2026-08-25
Branch: `H!veAI`
M09A prompt base: `247dd6707d7cb721a0909bcc2966e401e59c842b`
Primary implementation commit: `a7c228b5a4d72f844e23e756ff48c27d3f0d4164`
Audited remote branch HEAD: `fba3f98822678c84a03dfef3c52ffe8095b3f68c`

## 1. VERDICT

`FAIL`

M09A closes the seven original findings substantially and correctly in production code, but two new source-level correctness/bounding defects remain. There are no BLOCKER findings. M10 must remain blocked.

Severity summary:

- BLOCKER: 0
- MAJOR: 2
- MINOR: 4

## 2. AUDIT METHOD

The builder log was treated as claims only. The re-audit inspected the actual `H!veAI` branch, the M09A diff from `247dd670...` to current remote HEAD, current `task_intelligence.rs`, current M08 `task_sources.rs`, schema foreign keys, focused test bodies, TASKS truth, and the final follow-up commit.

The audited branch is four commits ahead of the M09A prompt base and contains only the bounded M09A task-intelligence/TASKS/log changes plus the final adapter identity evidence test. No visible UI production file, canonical visual asset, M10 state-machine implementation, Git Engine, watcher, startup-intro, or installer change is present.

## 3. ORIGINAL M09A FINDINGS F01-F07

| Finding | Re-audit | Summary |
|---|---|---|
| F01 source-change retry | PASS production / MINOR evidence note | Stable one-edit retry now rediscoveries, re-canonicalizes, rereads once, and accepts refreshed bytes. A second change returns skip after one retry. |
| F02 no silent bounds | PARTIAL / MAJOR residual | Task/title/metadata fields are improved, but several persisted source-derived scalar/evidence fields remain unbounded and can amplify one large heading into a very large snapshot. |
| F03 evidenced adapters | PASS with truthful unverified special conventions | FormuLab bonus now requires `FVL-` evidence. ScrubBots/FMCG remain selectable but convention-unmatched, so they receive no special bonus. Log wording should have marked those conventions UNVERIFIED rather than simply F03 PASS. |
| F04 structured metadata / owner gate | PASS | Nested explicit blocker/acceptance blocks and owner-gate mapping are implemented; casual prose is not inferred as a blocker. |
| F05 deterministic identity | PARTIAL / MAJOR residual | Heading normalization/movement tests are present, but the source path is normalized with a prose whitespace-collapsing function, creating possible cross-source task-ID collisions. |
| F06 status + handoff | PASS production / MINOR evidence note | Checklist prefix status tags work and handoff summaries merge in M08 iteration order; the named ordering test does not assert exact order. |
| F07 UPSERT persistence | PASS production / MINOR evidence note | Stable tasks now UPSERT without deleting retained IDs, preserving `created_at` and task-event references. Stale-only reconciliation exists; one named source-removal test does not actually remove a source. |

## 4. MAJOR FINDINGS

### R01 - MAJOR - task identity uses text normalization for filesystem paths

`task_id()` currently uses `normalize_text(path)` for the source path component. `normalize_text()` collapses whitespace with `split_whitespace().join(" ")` and lowercases it.

That is valid normalization for human prose, but not for a filesystem path. Two distinct M08-approved files such as:

- `plans/a b.md`
- `plans/a  b.md`

can both exist while normalizing to the same task-ID path component. If both contain the same heading/title or explicit task ID, M09 can generate the same `m09task:` primary key for tasks from different sources. The source rows remain distinct because `source_id()` uses the original path, so this creates an internal identity inconsistency and can make one source overwrite another in SQLite UPSERT.

This directly violates the M09 identity contract that normalized source paths must remain deterministic without allowing distinct sources to collide.

Required correction:

- add path-specific identity normalization;
- normalize separators and platform-equivalent case policy as appropriate;
- remove harmless `./` syntax if needed;
- preserve semantic filename whitespace and characters;
- never reuse prose normalization for path identity.

Required direct regression test:

Create two M08-approved source files whose names differ only by meaningful repeated whitespace and contain the same normalized task text. Parse through production M08 -> M09 flow and prove the task IDs are distinct, both snapshot tasks survive, and both SQLite task rows survive.

### R02 - MAJOR - source-derived scalar/evidence bounds remain incomplete and allow snapshot amplification

M09A added `bounded_field()` for title, next step, owner gate, external wait, blockers, dependencies, and acceptance values. However several source-derived persisted fields bypass that bound:

- `milestone: context.last().cloned()` stores a raw heading;
- every `TaskEvidenceLocator.heading_path` clones raw headings;
- `explicit_task_id` remains unbounded;
- handoff `current`, `next`, `blockers`, and `waiting` values use unbounded `clean_value()`;
- handoff evidence also carries unbounded heading-path components.

This is not only a cosmetic contract gap. One source under the M08 2 MiB file limit can contain a very large heading followed by many small tasks. That heading is cloned into each task's milestone/evidence heading path, multiplying a bounded source into a very large in-memory/persisted snapshot. The original M09 contract requires source-derived scalar fields to stay bounded with structured evidence when truncation occurs.

Required correction:

- bound every source-derived persisted scalar string, not only task-body metadata;
- at minimum cover explicit task ID, milestone/heading components, handoff values, and locator text/heading components;
- use UTF-8-safe truncation;
- emit stable structured `FIELD_TRUNCATED` evidence without copying the full source body;
- preserve deterministic identity by applying the same bounded/normalized identity representation consistently where needed;
- avoid warning explosion and keep the existing 512 project warning cap.

Required direct regression tests:

1. A >4096-byte multibyte heading plus multiple tasks must keep every persisted milestone/evidence heading component <=4096 bytes, valid UTF-8, with structured warning evidence.
2. A >4096-byte handoff narrative value must be bounded with warning evidence.
3. An oversized explicit task ID must not create an unbounded persisted scalar or unstable identity.
4. Repeated parse of the bounded fixture must remain deterministic.

## 5. MINOR EVIDENCE / TRUTHFULNESS FINDINGS

### E01 - retry containment test name overstates its path

`p01_retry_rechecks_physical_containment` supplies an initial `../outside.md` path and fails on the first containment check. It does not itself prove containment changed between first read and retry. Production code does visibly re-canonicalize and re-check the refreshed target, so F01 production behavior is accepted. A stronger retry-specific test should be added when practical; Windows symlink/junction creation may remain environment-UNVERIFIED under OS error 1314.

### E02 - handoff merge test does not assert source order

`p06_multiple_handoff_sources_merge_in_source_order` asserts both values are present but not their exact order. Production `parse()` iterates M08's sorted discovery list and `merge_handoff()` extends vectors in iteration order, so the implementation is accepted. Strengthen the test to assert the exact M08 order.

### E03 - stale source test does not remove a source

`p07_removed_task_and_source_reconcile_only_stale_m09_rows` removes one task from `TASKS.md` but does not remove a second M09 source. Production source reconciliation is stale-ID selective, but the named test does not prove its full claim. Add a second approved source, remove/unconfigure it, then assert only that M09 source row disappears while retained/legacy rows survive.

### E04 - ScrubBots/FMCG adapter convention status should be explicitly UNVERIFIED

Current production behavior is safe: both adapters remain selectable but `convention_matched=false`, and generic `TASK-`/heading/checklist grammar receives no special bonus. The M09A prompt explicitly allowed this state when no distinct non-generic convention could be established, but required it to be reported as UNVERIFIED. The M09A log instead marks F03 simply PASS. Do not mutate the historical log; record the truthful UNVERIFIED convention status prospectively in the next closure log.

## 6. ACCEPTED PRODUCTION CORRECTIONS

The following M09A corrections are source-level accepted:

- one real M08 rediscovery + second bounded read for a stable changed source;
- fresh retry canonicalization/containment;
- project-wide 4096 task budget with structured task-limit warning;
- UTF-8-safe bounds for the implemented task-body fields;
- specific metadata-limit warning;
- FormuLab FVL-specific adapter bonus only on actual FVL match;
- no special bonus for generic TASK syntax in FormuLab/ScrubBots/FMCG;
- nested explicit metadata blocks;
- owner gate separate from required actor;
- normalized heading identity and movement-stability tests;
- checklist WAITING/READY/IN PROGRESS prefix status parsing;
- deterministic handoff merge in discovery iteration order;
- task UPSERT/update rather than blanket delete/reinsert;
- stable task IDs preserve event history;
- SOURCE_EXPLICIT dependencies are reconciled transactionally;
- M09 still writes no workflow transitions/M10 state machine.

## 7. PERSISTENCE / FOREIGN-KEY REVIEW

The M04 schema uses:

- `tasks.source_id -> task_sources(id) ON DELETE SET NULL`;
- `task_events.task_id -> tasks(id) ON DELETE CASCADE`.

M09A's stable-task UPSERT no longer deletes retained tasks, so the earlier event-history cascade risk is closed for same-identity reparses. Stale tasks may still be deleted intentionally, which correctly cascades their task events. Current task source IDs are updated transactionally after source reconciliation.

## 8. REGRESSION / PUBLICATION EVIDENCE

Builder claims in the M09A log:

- 45 focused task-intelligence tests after final test addition;
- full Rust 182/182;
- frontend 70/70;
- typecheck/build/audit/fmt/check/build PASS;
- publisher failure harness 9/9;
- production no-bundle publication PASS;
- canonical assets byte-identical.

The audited diff contains no visible UI or canonical asset change. The final remote HEAD `fba3f988...` adds only an adapter identity evidence test/log correction after the main implementation/log commits, so it does not change the production executable logic.

The log does not prove final local/origin equality after the last `fba3f988...` follow-up commit. This is bookkeeping only because the audited remote branch itself contains that commit; the next closure log should record exact final local HEAD, final origin HEAD, and equality after all commits.

## 9. CROSS-MILESTONE USER-REPORTED DEFECTS

These remain intentionally outside M09A and must not be forgotten:

- X01: visible Windows terminal/console windows from spawned Git child processes. Queue `CREATE_NO_WINDOW`-style native process handling after M09 closure and before M10.
- X02: startup intro audio remains muted because `StartupIntro.tsx` still uses the `muted` video attribute. Queue audible startup playback in the same post-M09 native UX hotfix.

Do not mix X01/X02 into the M09 parser micro-fix.

## 10. FINAL DECISION

`M09A = FAIL`

The seven original remediation themes are largely corrected, but R01 and R02 are real production correctness/bounding defects and prevent M09 closure.

M10 remains BLOCKED/UNSTARTED.

Recommended next action: one very small M09B micro-fix containing only R01, R02, and the four direct-evidence tightenings above. Do not redesign the parser or add features.

Confidence: HIGH
Regression risk after current M09A: MEDIUM
