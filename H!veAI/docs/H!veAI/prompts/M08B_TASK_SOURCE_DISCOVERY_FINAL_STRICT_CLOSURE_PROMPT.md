# M08B — Task Source Discovery Final Strict Closure

## Purpose

Close the remaining independent M08A strict re-audit findings in one tiny bounded pass.

This is the **only active remediation**.

Do not split into M08B.01/B.02/B.03 prompts.
Do not start M09.
Do not create an installer.
Do not redesign H!veAI.

M00-M07 remain PASS/CLOSED.
M08.00/M08.00B remain PASS/CLOSED.
Original M08 strict audit = FAIL.
M08A strict re-audit = FAIL.

---

## Mandatory synchronization

Work from:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Run:

```powershell
git fetch origin H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
```

Fast-forward only when safe. Never reset, rebase, force-checkout, force-push, overwrite user-owned/untracked files, or rewrite historical logs/audits.

Read completely before editing:

1. `H!veAI/AGENTS.md`
2. `H!veAI/CONSTITUTION.md`
3. `H!veAI/ARCHITECTURE.md`
4. `H!veAI/TASKS.md`
5. `H!veAI/docs/H!veAI/UI_LAYOUT_GOVERNANCE.md`
6. `H!veAI/docs/H!veAI/prompts/M08_TASK_SOURCE_DISCOVERY_PROMPT.md`
7. `H!veAI/docs/H!veAI/prompts/M08A_TASK_SOURCE_DISCOVERY_STRICT_CLOSURE_PROMPT.md`
8. `H!veAI/docs/H!veAI/audits/M08_TASK_SOURCE_DISCOVERY_STRICT_AUDIT.md`
9. `H!veAI/docs/H!veAI/audits/M08A_TASK_SOURCE_DISCOVERY_STRICT_REAUDIT.md`
10. `H!veAI/docs/H!veAI/codex-logs/M08A_TASK_SOURCE_DISCOVERY_STRICT_CLOSURE_LOG.md`
11. this prompt

The M08A strict re-audit is authoritative for this pass.

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

Canonical logo/sidebar governance remains unchanged.

Do not modify, crop, resize, regenerate, recolor, recompress, or replace PNG/MP4/logo bytes.

Preserve exactly:

- approximately 220 px sidebar;
- enlarged one-piece H!veAI sidebar logo;
- post-sidebar `.main-area` background centering;
- cold-launch and native-restart opening video behavior;
- fixed startup overlay outside normal document flow;
- restrained neon liquid-glass/glow treatment;
- approved Command Center project selection behavior;
- approved single-viewport Command Center geometry.

This pass is task-source correctness/evidence only.

---

# Historical evidence is immutable

Do not modify:

- `docs/H!veAI/codex-logs/M08_TASK_SOURCE_DISCOVERY_LOG.md`
- `docs/H!veAI/codex-logs/M08A_TASK_SOURCE_DISCOVERY_STRICT_CLOSURE_LOG.md`
- prior M08/M08A audits
- prior M08/M08A prompts

Create one new M08B log only.

---

# Required closure work

## B01 — Fix real custom reorder semantics

Current defect:

`custom_path_update()` assigns the requested numeric order, sorts by `(order, normalized_path)`, then renumbers. When two entries collide at the same requested order, lexical path order decides which one comes first. A request to move item B from order 1 to order 0 can therefore leave B second.

Implement true positional reorder semantics.

Required behavior:

- identify the requested item first;
- if only path target changes, preserve its current relative position unless an explicit order is also supplied;
- when `order = N` is supplied, remove the item from the current ordered sequence and insert it at bounded position `N`;
- clamp target order to valid range;
- renumber every configured path to contiguous `0..n-1` after insertion;
- no lexical tie-break may override the explicit requested position;
- path update must still re-run containment and dedupe validation;
- deterministic id must still follow the normalized target path when the target changes.

Add direct Rust tests with **at least three custom paths** proving:

1. move last -> first;
2. move first -> last;
3. move middle -> middle/no-op;
4. reorder without renaming;
5. path rename without explicit reorder preserves relative position;
6. containment rejection during rename;
7. duplicate target rejection after rename.

The mounted UI's `Move earlier` action must result in the chosen row actually appearing earlier in visible custom-path order.

---

## B02 — Make pre-version M08 adoption genuinely safe

Current defect:

Reconciliation considers a row compatible pre-version M08 inventory when JSON merely has `relativePath` plus `origin = STANDARD|CUSTOM|SYSTEM`. That is not sufficient identity/shape proof for a shared `project_sources` table.

Narrow the compatibility predicate.

A pre-version row may be adopted/deleted as legacy M08 inventory only when deterministic evidence proves it matches the old M08 shape.

At minimum require a coherent set such as:

- project id matches the current project;
- `relativePath` exists and equals persisted `source_path` under Windows-insensitive normalized comparison;
- `origin` is one of the old M08 origins;
- `sourceKind`, `status`, `authorityClass`, `priority`, `warnings`, `depth`, and `absolutePath` exist with expected primitive/array types;
- the persisted row id equals the deterministic old M08 id derived from `projectId|origin|normalizedRelativePath` OR another equally strong old-M08 identity proof;
- metadata does not declare a different owner/schema.

Do not adopt solely by `relativePath + origin`.

Add direct SQL tests proving:

1. a real pre-version M08-shaped row with deterministic legacy identity is adopted/reconciled correctly;
2. an unrelated legacy row with `{relativePath, origin}` but non-M08 identity survives unchanged;
3. an unrelated rich row that has several overlapping source fields but a different owner survives;
4. normal M08-owned rows still reconcile transactionally.

---

## B03 — Close direct persisted SQL evidence

Add direct production-path SQL assertions for the requirements still missing from M08A.

### Persisted content-hash change

- create a source file;
- run production discovery;
- read its `content_hash` directly from `project_sources`;
- modify file contents;
- run production discovery again;
- read persisted `content_hash` again;
- assert same deterministic M08 id, one owned row, different persisted hash.

### Deleted STANDARD persisted-row reconciliation

- insert/discover a STANDARD source;
- also seed an unrelated legacy row;
- verify both rows exist;
- delete the physical STANDARD source;
- run production discovery;
- assert the M08-owned STANDARD persisted row is gone;
- assert unrelated legacy row is byte/text unchanged and still present.

### Custom + standard ordering

Use at least three CUSTOM paths with explicit configured order plus multiple STANDARD classes.

Prove production `discover()`/`list()` order exactly follows:

1. CUSTOM first;
2. explicit custom order;
3. freshness only as the documented next tie-break where relevant;
4. normalized relative path;
5. STANDARD sources by documented authority priority.

The test must fail if explicit reorder is ignored.

---

## B04 — Close mounted frontend transition evidence

Replace or supplement tests so names match real behavior.

### Custom add visible refresh

Mounted test must:

1. start with existing source/custom state;
2. enter a new custom path;
3. click Add path;
4. assert production adapter invokes add for selected project;
5. make subsequent native list/custom-list response contain the new item/source;
6. assert refreshed new item/source becomes visible without remount.

### Custom update/reorder visible refresh

Mounted test must use at least two configured custom paths.

1. render path A before path B;
2. invoke Move earlier on B;
3. assert update IPC requests target order 0;
4. subsequent refresh returns B order 0, A order 1;
5. assert DOM visible order is B then A.

Do not count IPC-call-only assertions as refresh/order evidence.

### Complete table metadata

One mounted test must directly assert a real rendered source row exposes:

- relative path;
- kind;
- origin;
- authority class;
- numeric priority;
- modified/freshness evidence;
- status.

### Preserve already-passing race evidence

Keep the same-mounted-app stale list, stale add and stale remove tests green.
Do not weaken browser-preview isolation, empty/error, or rescan replacement tests.

---

## B05 — Tracker truth

Update `H!veAI/TASKS.md` truthfully at the beginning of implementation:

- original M08 audit = FAIL historical;
- M08A re-audit = FAIL historical;
- M08B = ACTIVE;
- M09 remains BLOCKED/UNSTARTED.

At the end:

- mark individual M08B implementation/evidence items `[x]` only if directly proven;
- leave independent strict re-audit and user visual acceptance unchecked;
- do not mark M08 PASS yourself.

---

# Allowed scope

Expected changed files should be limited to what is necessary:

- `H!veAI/src-tauri/src/task_sources.rs`
- `H!veAI/src/pages.tsx`
- `H!veAI/tests/m08-task-sources-focused.test.tsx`
- `H!veAI/TASKS.md`
- `H!veAI/docs/H!veAI/codex-logs/M08B_TASK_SOURCE_DISCOVERY_FINAL_STRICT_CLOSURE_LOG.md`

Only change `src/taskSources.ts`, `src-tauri/src/lib.rs`, permission/capability files, architecture/governance, Cargo files, or publisher code if a direct regression proves they are necessary.

Do not create a migration.
Do not start M09.
Do not create an installer.
Do not modify canonical visual assets.

---

# Required focused evidence

Run all existing M08/M08A focused tests plus the new M08B tests.

The new log must list every focused Rust and frontend test name with its individual result.

Do not use source-code string assertions where a real production state transition can be exercised.

Windows physical symlink/junction containment may remain exact `UNVERIFIED` with OS error 1314 if privilege is still unavailable. Do not fake PASS.

---

# Full verification

From `H!veAI`:

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

Only if all automated gates pass:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\publish-dev-qa.ps1
```

Preserve stable QA EXE/shortcut and M08.00 presentation behavior.

---

# Required immutable M08B log

Create:

`H!veAI/docs/H!veAI/codex-logs/M08B_TASK_SOURCE_DISCOVERY_FINAL_STRICT_CLOSURE_LOG.md`

Record:

- synchronized base HEAD;
- exact changed files;
- B01-B05 closure evidence;
- exact reorder algorithm contract;
- exact legacy pre-version compatibility predicate;
- direct SQL persisted hash-change result;
- direct SQL deleted-standard + legacy-preservation result;
- exact custom + standard ordering evidence;
- exact mounted add-refresh evidence;
- exact mounted multi-item reorder-visible-order evidence;
- complete table metadata rendering evidence;
- every focused Rust test name + individual PASS/FAIL;
- every focused frontend test name + individual PASS/FAIL;
- full frontend/Rust/security/harness results;
- publication result and stable EXE SHA-256;
- canonical PNG/MP4/logo bytes unchanged;
- symlink/junction exact PASS or UNVERIFIED reason;
- no installer;
- no M09 work;
- final native `/tasks` status `PENDING USER VISUAL ACCEPTANCE`;
- implementation/publication local HEAD and `origin/H!veAI` HEAD equality after push.

The M08B log is claims only and will be independently audited.

---

# Final stop condition

Stop after M08B implementation, focused evidence, full regression, stable QA publication, log, commit, and push.

If any B01-B04 automated MAJOR remains, log `FAIL` and stop.

If all automated closure evidence is clean, log `PENDING INDEPENDENT STRICT RE-AUDIT + USER VISUAL ACCEPTANCE` and stop.

Do not start M09.