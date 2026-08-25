# M08A — Task Source Discovery Strict Closure

## Purpose

Close the independent strict-audit findings for M08 Task Source Discovery in one bounded remediation.

This is the **only active remediation**.

Do **not** split this work into M08.01/M08.02/M08.03 prompts or stop points.
Do **not** start M09.
Do **not** create an installer.

M00-M07 remain PASS/CLOSED.
M08.00/M08.00B remain PASS/CLOSED.
The original M08 Task Source Discovery implementation remains historical evidence and must not be rewritten.

---

## Mandatory synchronization

Work from:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Run:

```powershell
git fetch origin H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
```

Fast-forward only when safe. Never reset, rebase, force-checkout, force-push, or overwrite user-owned/untracked files.

Read completely before editing:

1. `H!veAI/AGENTS.md`
2. `H!veAI/CONSTITUTION.md`
3. `H!veAI/ARCHITECTURE.md`
4. `H!veAI/TASKS.md`
5. `H!veAI/docs/H!veAI/UI_LAYOUT_GOVERNANCE.md`
6. `H!veAI/docs/H!veAI/prompts/M08_TASK_SOURCE_DISCOVERY_PROMPT.md`
7. `H!veAI/docs/H!veAI/codex-logs/M08_TASK_SOURCE_DISCOVERY_LOG.md`
8. `H!veAI/docs/H!veAI/audits/M08_TASK_SOURCE_DISCOVERY_STRICT_AUDIT.md`
9. this prompt

The strict audit is authoritative for this closure pass.

---

# Canonical UI Assets

User-owned canonical asset root:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Canonical application background:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI\scene 3 starting point.png`

Canonical opening video:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI\videos and gifs\opening video.mp4`

Repository assets:

- `H!veAI/src/assets/hiveai-app-background.png`
- `H!veAI/src/assets/opening-video.mp4`

Canonical sidebar/logo governance remains unchanged.

Do not modify, crop, regenerate, resize, recolor, recompress, or replace canonical PNG/MP4/logo bytes.

Preserve:

- approximately 220 px sidebar;
- enlarged one-piece H!veAI sidebar logo;
- post-sidebar `.main-area` background centering;
- cold-launch + native-restart opening-video behavior;
- fixed startup overlay outside normal layout flow;
- restrained neon liquid-glass/glow styling;
- approved Command Center project-selection behavior and single-viewport geometry.

This remediation is source-discovery correctness/evidence work, not a visual redesign.

---

# Historical files are immutable

Do not modify:

- `H!veAI/docs/H!veAI/codex-logs/M08_TASK_SOURCE_DISCOVERY_LOG.md`
- prior M08.00/M08.00B logs/audits
- `H!veAI/docs/H!veAI/audits/M08_TASK_SOURCE_DISCOVERY_STRICT_AUDIT.md`

Create a new remediation log only.

---

# Required closure work

## C01 — Make filesystem work genuinely bounded

Fix production discovery so bounds are real, not output-count-only.

Required:

- preserve `MAX_DISCOVERY_DEPTH = 4` as the semantic maximum relative source depth unless the original prompt requires a stricter existing policy;
- fix the current off-by-one so the first source beyond allowed depth is not processed;
- bound **filesystem entries/work visited**, not only accepted source rows;
- define a deterministic maximum visited-entry/work budget suitable for the existing 512-candidate policy;
- root scanning and approved-directory scanning must not enumerate indefinitely when a directory contains huge numbers of non-source files;
- no permanent worker threads;
- no polling.

Limit hits must be visible as structured discovery evidence, not silently truncated.

Use a bounded warning structure compatible with current response semantics. A practical design is an optional discovery-warning collection returned/persisted with source inventory, or a deterministic synthetic/non-content warning record only if it does not masquerade as a real file. Do not encode limits only in logs.

At minimum distinguish:

- candidate/file limit hit;
- visited-entry/work limit hit;
- depth limit hit.

Tests must prove the first rejected depth and a real limit warning.

## C02 — Complete custom-path operations and ordering

The original M08 contract requires explicit add/remove/update custom-path operations.

Implement a narrow update operation through production Rust + IPC + typed frontend adapter + ACL, without broad filesystem access.

Update must support at minimum:

- changing display path/path target safely;
- deterministic custom order/reordering metadata;
- the same physical containment and dedupe rules as add.

Persist explicit custom order in H!veAI-owned settings.

Ordering contract:

1. custom sources first;
2. configured custom order within CUSTOM;
3. then deterministic freshness evidence where required by the original prompt;
4. then normalized relative path;
5. standard classes remain ordered by their documented authority priority.

Do not infer semantic truth from freshness.

Fix remove-by-path equivalence so both stored and incoming paths use the same Windows-insensitive normalized comparison. Removal by id must continue to work.

Add direct tests for:

- update path;
- update/reorder;
- dedupe after update;
- equivalent-case/slash remove-by-path;
- containment rejection during update.

## C03 — Make `project_sources` reconciliation M08-owned and non-destructive

Do not blanket-delete every `project_sources` row for a project.

Add explicit M08 ownership/schema metadata to `metadata_json`, for example:

```json
{
  "schemaVersion": 1,
  "owner": "M08_TASK_SOURCE_DISCOVERY",
  ...
}
```

Exact field naming may follow Rust conventions, but M09 must be able to identify/version M08 metadata safely.

Reconciliation must:

- update/delete only M08-owned inventory rows;
- preserve unrelated/legacy `project_sources` rows;
- deterministically adopt compatible pre-version M08 rows only when identity/shape proves they are M08 source inventory;
- never duplicate preserved legacy rows blindly;
- remain transactional;
- preserve deterministic source ids.

Add direct SQL evidence that:

- repeated scan creates one M08 row, not duplicates;
- content change updates persisted hash;
- deleted standard source removes only its M08-owned row;
- unrelated legacy `project_sources` row survives discovery unchanged;
- metadata JSON contains explicit schemaVersion/owner.

## C04 — Close stale frontend races, including stale mutation completion

The selected project must own visible Task Sources inventory at all times.

Fix the real race where a custom add/remove started for project A can complete after the user selects project B and then issue a new A refresh that becomes the newest request.

Use a current-project/request-generation guard for mutation completion. A stale mutation may finish on the backend for its original project, but it must not reclaim or overwrite the current B UI.

Required mounted frontend tests:

1. project A list is delayed;
2. select project B in the same mounted app;
3. B list resolves and B row becomes visible;
4. stale A list resolves afterward;
5. visible inventory remains B and A row does not reappear.

Also test:

- stale project-A custom add completion after B selection cannot overwrite B;
- stale project-A custom remove completion after B selection cannot overwrite B.

Do not unmount/remount to fake route-race safety.

## C05 — Replace misleading frontend evidence with real transitions

Focused tests must directly exercise:

- custom add command + refreshed visible inventory;
- custom remove command + refreshed visible inventory;
- custom update/reorder command + refreshed visible inventory;
- rejected native list/discover call produces truthful error UI;
- empty response produces truthful empty UI;
- rescan discover response replaces the visible row/data, not merely calls IPC;
- selected-project delayed stale response protection;
- stale custom mutation protection;
- browser preview invokes no native task-source commands;
- source table renders path/kind/origin/authority/priority/modified/status;
- no parsed task/workflow/owner/completion claims.

Test names must describe what they truly execute.

## C06 — Close Rust direct evidence matrix

Persistence tests must inspect SQLite `project_sources` directly, as required by the original prompt.

Add/fix production-path tests for:

- idempotent persisted row count and deterministic id;
- persisted content-hash change;
- deleted STANDARD persisted-row reconciliation;
- CUSTOM AVAILABLE -> physical target deleted -> remains configured and source inventory becomes MISSING;
- unrelated legacy `project_sources` preservation;
- metadata schema/version ownership fields;
- real structured candidate/work/depth limit warning;
- exact depth edge;
- custom update/reorder/remove equivalence;
- source ordering including custom order and standard authority;
- archived-project policy from C08.

Unreadable-file isolation:

Prefer a private `#[cfg(test)]` failpoint at the production file metadata/open/hash boundary if Windows permissions cannot deterministically create unreadability. The failpoint must exercise the same production error handling path and must not create a public bypass.

Prove one unreadable source yields `UNREADABLE` while another valid source remains persisted/returned.

Physical symlink/junction case may remain exact `UNVERIFIED` with OS error 1314 if privilege remains unavailable. Do not fake PASS.

## C07 — Improve handoff matching

Implement the original reasonable root `*handoff*.md` family, case-insensitively, while preserving actual on-disk spelling.

Examples that should classify as HANDOFF when located at project root:

- `HANDOFF.md`
- `SESSION_HANDOFF.md`
- `current_handoff.md`
- `handoff-notes.md`
- `project-handoff-2026.md`

Do not classify arbitrary nested repository Markdown outside approved source directories.

Add direct tests.

## C08 — Enforce/document registered project status boundary

The original M08 scope is ACTIVE/MISSING registered projects.

At the native production boundary, explicitly define one policy:

Preferred:

- ACTIVE: discover/list allowed;
- MISSING: return bounded unavailable state/error as appropriate;
- ARCHIVED: reject discovery/custom mutation with a bounded `project is archived`-style error.

Do not silently scan archived project roots merely because the caller knows the id.

Add direct production tests.

## C09 — Make custom path status containment-aware

A previously safe custom path may later become a symlink/junction to an outside target.

`custom_paths_list` must not report such a target as ordinary `CONFIGURED` without containment validation.

Use a bounded status/warning such as `UNREADABLE` / `OUTSIDE_ROOT` according to current type policy. Do not read the outside target.

Keep the actual discovery read boundary physically contained.

Test when the platform permits link creation; otherwise preserve exact Windows UNVERIFIED reason while still testing ordinary changed/missing states.

## C10 — Tracker and log truth

Update `H!veAI/TASKS.md` truthfully:

- M08 original implementation strict audit = FAIL;
- M08A strict closure = ACTIVE while implementing;
- do not mark stale-response race, persistence evidence, custom update, or bounded warning complete until direct tests pass;
- M09 remains blocked/unstarted;
- historical M08 builder log stays immutable.

Create:

`H!veAI/docs/H!veAI/codex-logs/M08A_TASK_SOURCE_DISCOVERY_STRICT_CLOSURE_LOG.md`

The new log must record:

- synchronized base HEAD;
- exact changed files from Git;
- each strict-audit finding F01-F07 and N01-N02 with closure evidence;
- exact filesystem limits and warning semantics;
- exact metadata schema/version ownership contract;
- exact custom update/order contract;
- exact persisted legacy-row preservation behavior;
- exact native IPC commands including new update command;
- exact ACL/capability changes;
- **every focused Rust test name and PASS/FAIL result individually**;
- **every focused frontend test name and PASS/FAIL result individually**;
- symlink/junction PASS or exact UNVERIFIED OS reason;
- unreadable-source evidence classification (`REAL_OS_FAILURE` or `REAL_PRODUCTION_PATH_WITH_TEST_FAILPOINT`);
- full frontend/Rust/security/harness results;
- publication result + stable EXE SHA-256;
- final local HEAD;
- final `origin/H!veAI` HEAD;
- explicit local == remote equality proof;
- canonical PNG/MP4/logo bytes unchanged;
- no installer;
- no M09 work;
- final native `/tasks` visual status as `PENDING USER VISUAL ACCEPTANCE` unless the user has inspected the post-M08A build.

Do not rewrite the historical M08 log to repair missing evidence.

---

# Expected implementation scope

Expected files may include only what is necessary, such as:

- `H!veAI/src-tauri/src/task_sources.rs`
- `H!veAI/src-tauri/src/lib.rs`
- `H!veAI/src-tauri/permissions/foundation.toml`
- `H!veAI/src-tauri/capabilities/default.json`
- `H!veAI/src/taskSources.ts`
- `H!veAI/src/pages.tsx`
- focused M08 frontend tests
- `H!veAI/TASKS.md`
- bounded architecture/governance docs if the contract changes materially
- the new M08A log

Do not modify M09 code or create M09 files.
Do not modify canonical visual asset bytes.
Do not create a migration unless direct source inspection proves absolutely necessary. Prefer metadata_json/settings within the existing schema.

---

# Verification order

Run focused Rust task-source tests first.
Run focused frontend Task Sources tests second.

Then from `H!veAI`:

```powershell
npm run typecheck
npm test -- --reporter=dot
npm run build
npm audit --audit-level=high
```

Then:

```powershell
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo check --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
```

Then:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\tests\publish-dev-qa-failure-harness.ps1
```

Only if every automated gate passes:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\publish-dev-qa.ps1
```

Preserve stable QA executable/shortcut behavior.

---

# Final stop condition

M08A may report automated closure only if every MAJOR strict-audit finding has direct production/evidence closure.

If any MAJOR remains, report FAIL truthfully in the new log and stop.

If automated closure is clean:

- publish stable QA build;
- push all commits;
- keep M08 pending independent re-audit + final user visual acceptance;
- do not start M09.

Stop after M08A.
