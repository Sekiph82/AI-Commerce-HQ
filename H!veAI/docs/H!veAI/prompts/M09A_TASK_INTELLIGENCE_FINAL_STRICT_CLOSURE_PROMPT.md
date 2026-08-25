# M09A Task Intelligence Parser Final Strict Closure

## Mission

Fix only the seven MAJOR findings in:

`H!veAI/docs/H!veAI/audits/M09_TASK_INTELLIGENCE_PARSER_STRICT_AUDIT.md`

This is one bounded remediation run. Do not split it into M09.01/M09.02/etc.

M00-M08 remain PASS/CLOSED.
M09 original implementation is historical FAIL pending this closure.
M10 is BLOCKED/UNSTARTED.

Do not implement M10.
Do not redesign the UI.
Do not fix the separate terminal-popup/startup-audio defects in this run; they are queued as a separate post-M09 native UX hotfix.
Do not create an installer.

Builder logs are claims only. Do not mark M09 PASS yourself.

---

## Start / synchronization

Work from:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Run:

```powershell
git fetch origin H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
```

Fast-forward only if safe. Never reset/rebase/force-push/overwrite user work.

Read before editing:

1. `H!veAI/AGENTS.md`
2. `H!veAI/CONSTITUTION.md`
3. `H!veAI/ARCHITECTURE.md`
4. `H!veAI/TASKS.md`
5. `H!veAI/docs/H!veAI/audits/M09_TASK_INTELLIGENCE_PARSER_STRICT_AUDIT.md`
6. `H!veAI/docs/H!veAI/prompts/M09_TASK_INTELLIGENCE_PARSER_PROMPT.md`
7. `H!veAI/src-tauri/src/task_intelligence.rs`
8. M08 Task Source Discovery production/tests
9. current M09 log

Record starting branch/HEAD/status/worktrees/untracked files in the new M09A log.

---

# Canonical UI Assets

Canonical user-owned asset root:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Canonical assets:

- `scene 3 starting point.png`
- `videos and gifs\opening video.mp4`
- `H!veAI logo.png`
- `H!veAI small logo.png`

Repository canonical visual assets must remain byte-identical.

M09A is backend/domain remediation. Do not edit visible UI production files, route layout CSS, Command Center CSS, startup intro, Git Engine, watcher, publisher UX, or canonical assets.

---

# F01 - REAL SOURCE-CHANGE RETRY

## Current defect

`read_authoritative_source()` rediscoveries after a hash mismatch, but if the refreshed hash differs from the old hash it immediately returns `None`. A single stable edit is therefore skipped instead of receiving the required one real retry.

## Required production behavior

Exact state machine:

1. receive current M08 source row;
2. freshly reconstruct/canonicalize/contain target under Registry root;
3. bounded read + SHA-256;
4. if bytes match the supplied M08 hash -> parse them;
5. if mismatch -> perform exactly ONE M08 rediscovery;
6. find the refreshed M08-owned AVAILABLE row for the same normalized source path;
7. freshly reconstruct/canonicalize/contain the refreshed target again;
8. bounded read exactly once more;
9. if second bytes match refreshed M08 hash -> return the refreshed body/hash and parse it;
10. if they mismatch again / source disappears / becomes unavailable -> skip with `SOURCE_CHANGED_DURING_PARSE`;
11. never loop.

Do not reuse stale physical-path evidence for retry containment.

## Direct tests required

`p01_single_stable_edit_is_parsed_after_one_refresh`

- discover old `TASKS.md`;
- change it once to stable new contents;
- call the production reader/parse path;
- prove the new task is parsed and no SOURCE_CHANGED warning is emitted.

`p01_second_change_after_refresh_is_skipped_after_exactly_one_retry`

- use a narrow private `cfg(test)` failpoint/hook if necessary;
- mutate once before refresh and once between refresh and second read;
- prove exactly one refresh/retry and final `SOURCE_CHANGED_DURING_PARSE`;
- test must call production reader logic, not copied test logic.

`p01_retry_rechecks_physical_containment`

- prove retry does not trust the first canonical path.
- Windows symlink/junction fixture may remain environment-UNVERIFIED if OS 1314 prevents creation, but code-path containment must be direct.

## PASS only if

The old implementation fails the stable-one-edit test.

---

# F02 - NO SILENT PROJECT/FIELD TRUNCATION

## Current defect

Project tasks are globally truncated by `snapshot.tasks.truncate(MAX_TASKS)` without warning. Scalar `truncate()` silently clips >4096-byte fields without structured evidence.

## Required production behavior

- enforce `MAX_TASKS = 4096` across the ENTIRE project snapshot, not independently per source;
- once project limit is reached, retain at most 4096 tasks and add one stable `TASK_LIMIT_REACHED` warning;
- no silent scalar clipping;
- when a title/next step/owner gate/external wait/blocker/dependency/acceptance value exceeds `MAX_FIELD_BYTES`, safely truncate on UTF-8 boundary and emit a stable structured warning such as `FIELD_TRUNCATED` with source path and field kind, without copying the full source body;
- >128 entry lists keep their existing bound but use a specific metadata-bound warning, not a misleading generic task-limit message;
- warnings remain capped at 512.

## Direct tests required

`p02_project_task_limit_across_multiple_sources_warns`

- create at least two M08-approved task sources whose combined tasks exceed 4096 while neither source alone proves the project-wide boundary;
- assert exactly <=4096 persisted/returned tasks and `TASK_LIMIT_REACHED`.

`p02_scalar_utf8_bound_warns_without_breaking_utf8`

- use a >4096-byte multibyte UTF-8 field;
- assert retained field <=4096 bytes, valid UTF-8, and `FIELD_TRUNCATED`.

`p02_metadata_entry_limit_warns`

- production parser path with >128 acceptance/blocker entries;
- assert 128 retained plus stable metadata-limit warning.

## PASS only if

No parser bound is silently reached.

---

# F03 - REAL EVIDENCED ADAPTERS; NO NAME-ONLY BONUS

## Current defect

`adapter_for(project_name)` sets `convention_matched=true` solely from Registry name. Adapter-specific parse behavior does not exist, and `TASK-`, `FVL-`, `FMCG-` are recognized globally. Confidence can receive a repo-specific bonus merely because project name matches and any explicit ID exists.

## Required production behavior

Implement a small real adapter boundary. It can be a trait, enum strategy, or explicit adapter functions, but it must separate:

- project adapter selection;
- actual source/task convention match;
- generic parse behavior;
- adapter-specific augmentation.

Rules:

1. Registry identity may select which adapter is eligible.
2. `conventionMatched` must be FALSE until actual source/task structure matches an evidenced convention.
3. Adapter confidence bonus may be added only to a task for which that adapter-specific convention actually matched.
4. A generic `TASK-123` inside FormuLab must NOT receive a FormuLab bonus solely because the project is FormuLab.
5. A similarly named unrelated project must remain generic.
6. Generic parser must remain safe fallback.
7. Do not invent conventions.

Re-inspect the actual registered FormuLab, ScrubBots and fmcg-erp-system through Registry + M08, read-only.

For each project:

- identify a structural convention NOT merely identical to generic checklist/heading grammar;
- implement and test it if real evidence exists;
- if no distinct convention exists, keep the adapter selectable/generic-safe but set convention evidence to UNVERIFIED / `conventionMatched=false` and do not grant bonus.

Do not copy private document bodies into H!veAI. Record only source path + concise structural convention in log.

## Direct tests required

At minimum:

`p03_formulab_bonus_requires_formulab_specific_match`

- exact FormuLab name + generic TASK id only -> adapter selected, no FormuLab convention bonus;
- evidenced FormuLab-specific fixture -> convention match and bonus only on matching task.

Equivalent positive/negative tests for ScrubBots and FMCG where a distinct evidenced convention exists.

`p03_similarly_named_project_never_selects_special_adapter`

`p03_generic_parser_does_not_claim_special_convention`

## PASS only if

Special adapter PASS is based on source structure, not project name alone.

---

# F04 - STRUCTURED METADATA + OWNER GATE

## Current defect

Only `Label: content` on the same line is parsed. Empty labels followed by indented child lines are ignored. `owner_gate` exists in the model but production never assigns it.

## Required production behavior

Continue to parse existing inline labels, and additionally support explicit nested metadata blocks associated with the nearest parent task until sibling task/heading boundary.

Examples that MUST work:

```md
- [ ] Ship feature
  Blockers:
    - dependency A
    - dependency B
  Acceptance criteria:
    - unit test
    - integration test
```

Support an explicit owner-gate family such as:

- `Owner gate:`
- `Owner decision:`
- `Decision gate:`

Keep `Owner:` / `Actor:` / `Required actor:` for required-actor normalization.

Unknown actor text remains source evidence and must not populate canonical `required_actor`.

Casual prose like `this is blocked by weather` without structured label must not become a blocker.

## Direct tests required

`p04_nested_metadata_blocks_attach_only_to_parent`

- two sibling tasks;
- first has nested blockers + AC;
- prove no leakage to sibling.

`p04_owner_gate_is_preserved_separately_from_required_actor`

- `Owner: Human` + `Owner gate: approve design`;
- assert required_actor Human and owner_gate exact bounded value.

`p04_casual_blocked_prose_is_not_structured_blocker`

`p04_unknown_actor_remains_null_without_losing_locator_evidence`

## PASS only if

The `ownerGate` model is no longer permanently null and nested explicit metadata works without free-prose inference.

---

# F05 - ID MOVEMENT TESTS + NORMALIZED HEADING IDENTITY

## Current defect

Required movement tests are absent. Fallback task IDs hash raw heading strings instead of normalized heading path.

## Required production behavior

- explicit-ID identity: project + normalized source path + normalized explicit ID, with deterministic duplicate ordinal only when duplicate explicit IDs require disambiguation;
- fallback identity: project + normalized source path + normalized heading path + normalized title + deterministic duplicate ordinal;
- normalize each heading component with whitespace collapse and case normalization for identity only;
- preserve original display heading in evidence/milestone.

## Direct tests required

`p05_explicit_id_survives_unrelated_line_insertion_and_movement`

`p05_fallback_id_survives_unrelated_line_insertion_above`

`p05_heading_case_and_whitespace_normalization_preserves_fallback_id`

`p05_identical_siblings_remain_distinct_and_repeatable`

`p05_same_text_different_projects_never_collides`

## PASS only if

Tests mutate fixture text between parses rather than merely parsing identical text twice.

---

# F06 - EXPLICIT STATUS + COMPLETE HANDOFF CONTRACT

## Current defect

Explicit status tags are parsed only in non-checklist explicit rows. Required handoff evidence is incomplete, and only the first handoff summary survives when multiple approved handoff sources exist.

## Required production behavior

Support prefix status tags syntactically attached to checklist/open task text, e.g.:

```md
- [ ] [WAITING] vendor
- [ ] [READY] deploy
- [ ] [IN PROGRESS] parser
```

The tag must become parsedStatus while storage remains neutral M09 mapping.

Do not classify a status word/tag occurring casually later in task prose.

Handoff:

- Current, Next, Blocker, Waiting/External sections remain separate;
- narrative remains summary only;
- checklist remains task;
- one-based evidence remains correct;
- if more than one approved HANDOFF source exists, merge summaries deterministically in M08 source order instead of discarding every handoff after the first.

## Direct tests required

`p06_checklist_prefix_status_tags_are_parsed`

- WAITING/READY/IN PROGRESS;
- storage BACKLOG;
- title cleaned of prefix tag.

`p06_status_word_inside_prose_does_not_override_status`

`p06_handoff_current_next_blocker_waiting_are_separate`

- include narrative in every section and a checklist under Next;
- assert narrative and task behavior plus line locators.

`p06_multiple_handoff_sources_merge_in_source_order`

## PASS only if

P03/P05 original contracts are directly exercised rather than inferred from helper behavior.

---

# F07 - UPSERT SAME IDENTITY; DO NOT DELETE/REINSERT STABLE TASKS

## Current defect

`persist()` deletes all M09-owned tasks then reinserts them. This violates the same-identity update contract and would later cascade-delete M10 `task_events` attached to otherwise stable task IDs.

## Required production behavior

For M09-owned stable identity:

1. UPSERT/UPDATE current `task_sources` and `tasks` by deterministic ID;
2. preserve existing `created_at` for an existing task;
3. update `source_id`, title/state/actor/milestone/metadata and `updated_at` when content changes;
4. reconcile M09 dependency edges transactionally;
5. remove only stale M09-owned tasks/sources no longer present after current snapshot is known;
6. preserve unrelated/legacy tasks/sources/settings/dependencies;
7. M09 itself writes zero `task_events`;
8. reparsing an unchanged or metadata-changed stable task must not delete existing external/event history attached to that same task ID.

Use the existing schema. No migration should be necessary.

## Direct SQL tests required

`p07_metadata_change_updates_same_task_without_recreate`

- parse a stable-ID task;
- record task id + created_at;
- seed a synthetic task_event referencing that task to detect delete/reinsert;
- modify only structured metadata while preserving task identity;
- reparse;
- assert same task id, same created_at, changed metadata/updated_at, seeded event still present.

`p07_removed_task_and_source_reconcile_only_stale_m09_rows`

- start with multiple M09 tasks/sources plus unrelated legacy rows;
- remove one source/task;
- reparse;
- prove only stale M09 rows/edges disappear and retained/legacy rows survive.

`p07_dependency_edges_reconcile_exactly_without_duplicates`

`p07_unchanged_parse_is_idempotent`

## PASS only if

A blanket `DELETE FROM tasks ... owner=M09` before reinsertion would fail the new tests.

---

# Regression / security gates

After F01-F07 focused tests pass, run:

```powershell
npm run typecheck
npm test -- --run
npm run build
npm audit --audit-level=high
cargo fmt --manifest-path H!veAI/src-tauri/Cargo.toml -- --check
cargo check --manifest-path H!veAI/src-tauri/Cargo.toml
cargo test --manifest-path H!veAI/src-tauri/Cargo.toml
cargo build --manifest-path H!veAI/src-tauri/Cargo.toml
```

Run the existing publisher failure harness and production `--no-bundle` QA publication exactly as governed by AGENTS.md.

Do not create an installer.

Verify canonical repository asset hashes remain unchanged.

Verify no visible UI production file changed.

Verify no M10 code/state-machine implementation was introduced.

---

# TASKS / architecture truth

Update prospectively only:

- original M09 strict audit = historical FAIL;
- M09A implementation may be marked automated implementation complete only when evidence exists;
- independent M09A re-audit remains pending;
- M10 remains BLOCKED/UNSTARTED;
- do not claim M09 PASS/CLOSED.

Keep original M09 log immutable. Create a new log:

`H!veAI/docs/H!veAI/codex-logs/M09A_TASK_INTELLIGENCE_FINAL_STRICT_CLOSURE_LOG.md`

---

# Mandatory self-audit format

For every F01-F07, the new log MUST include exactly this structure:

```text
F0X
Production symbol(s) changed:
Exact direct test(s):
Pre-fix behavior that fails these tests:
Post-fix behavior proved:
Would these tests fail on cdac3774a403a04ae94db707483a8dfad27efa52? YES/NO + why
Status: PASS / FAIL / UNVERIFIED
```

A test name is not evidence by itself.

If any required direct test would also pass on the old implementation, do not claim the finding closed; strengthen the test or report FAIL/UNVERIFIED.

List every focused test name and result individually.

Record:

- synchronized base HEAD;
- implementation commit;
- final local HEAD;
- final `origin/H!veAI` HEAD;
- local/origin equality;
- changed file list;
- full regression results;
- stable EXE SHA-256 and size;
- shortcut target/icon;
- canonical asset hashes;
- no installer;
- no M10;
- separate X01 terminal-popup and X02 intro-audio hotfix remains queued and intentionally untouched.

---

# Final stop condition

Stop after:

1. F01-F07 production fixes;
2. direct focused tests;
3. full regression/security gates;
4. no-bundle QA publication;
5. truthful TASKS/architecture updates only if needed;
6. new M09A log;
7. commit + push;
8. local/origin equality verification.

Do not start M10.
Do not start the terminal/audio hotfix.
Do not create another remediation sub-milestone yourself.
