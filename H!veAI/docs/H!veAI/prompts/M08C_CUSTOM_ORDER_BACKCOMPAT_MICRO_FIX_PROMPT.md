# M08C — Custom Order Backward-Compatibility Micro Fix

## One job only

Fix the **one remaining production defect** and **one small evidence gap** from:

`docs/H!veAI/audits/M08B_TASK_SOURCE_DISCOVERY_FINAL_STRICT_REAUDIT.md`

Do not broaden scope.
Do not redesign anything.
Do not start M09.
Do not create an installer.

This should be a tiny final M08 correction, not another milestone implementation.

---

# Canonical UI Assets

User-owned canonical asset root:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Canonical background:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI\scene 3 starting point.png`

Canonical opening video:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI\videos and gifs\opening video.mp4`

Repository visual assets include:

- `H!veAI/src/assets/hiveai-app-background.png`
- `H!veAI/src/assets/opening-video.mp4`
- existing canonical H!veAI logo assets

**Do not modify any canonical PNG/MP4/logo bytes.**
Do not change sidebar width/logo scale, post-sidebar background positioning, opening-video lifecycle, glass/glow styling, Command Center layout, or navigation behavior.

---

# DEFECT 1 — legacy custom paths have no `order`

## Exact pre-fix bug

Original M08 persisted custom-path JSON objects with:

- `id`
- `displayPath`
- `normalizedPath`

and **no `order` field**.

Current code uses:

```rust
#[serde(default)]
order: i64
```

Therefore multiple historical entries deserialize as `order = 0`.

That breaks the current contract:

- multiple old paths can all appear order 0;
- explicit custom ordering can fall through to freshness/path;
- path-only rename of an old second/third item can move it to the front because `original_order` becomes 0;
- frontend `Move earlier` can be disabled for every old item because they all report 0.

## Required production behavior

Treat the **persisted vector sequence** as the compatibility fallback only when stored order metadata is missing or invalid.

Implement one normalization boundary for loaded custom settings:

1. Deserialize in a way that can distinguish **missing order** from explicit `order = 0`.
2. If every entry has an explicit, non-negative, unique, contiguous order `0..n-1`, honor explicit order and return the sequence sorted by that order.
3. If any entry has missing, duplicate, negative, non-contiguous, or otherwise invalid order metadata, preserve the persisted JSON vector sequence and assign in-memory contiguous orders `0..n-1` by vector position.
4. Read/list/discover must immediately use this normalized in-memory sequence.
5. The next H!veAI-owned custom-path mutation (`add`, `update`, `remove`) must save the normalized vector with explicit contiguous order values.
6. Do not mutate registered project files. Only H!veAI-owned settings may be written.
7. Path-only rename with `order = None` must preserve the normalized current relative position.
8. Explicit positional reorder from M08B must continue to work exactly as already implemented.

Do not solve this by making every missing order 0 again.
Do not use lexical path sorting as the legacy fallback.

## Required production-path test

Seed the `settings` row directly with the **actual old M08 JSON shape** for three custom paths, deliberately in non-lexical vector order, for example:

```text
z.md
A.md
m.md
```

No object may contain an `order` field.

Then prove, using production functions:

1. `custom_paths_list()` returns exactly `z.md, A.md, m.md` with orders `0,1,2`.
2. `discover()` with those files present returns those CUSTOM sources in the same configured order before STANDARD sources, not alphabetical order.
3. Rename only the middle path with `custom_path_update(... path=Some("renamed.md"), order=None)`.
4. Result remains exactly `z.md, renamed.md, m.md` with orders `0,1,2`.
5. Read the persisted settings JSON directly from SQLite after the update and assert all three objects now contain explicit contiguous order values `0,1,2`.
6. The test must fail on the current pre-fix M08B implementation.

Name the test clearly, for example:

`legacy_custom_settings_without_order_normalize_and_preserve_position`

---

# DEFECT 2 — combined ordering evidence must use 3 CUSTOM sources

The M08B prompt explicitly required at least **three CUSTOM paths plus multiple STANDARD classes** in the combined ordering evidence. Current combined test uses only two CUSTOM paths.

Modify that production ordering test so it contains at least:

- three CUSTOM files;
- explicit configured custom ordering that is not alphabetical;
- `TASKS.md`;
- `PLANS.md` or another PLAN source;
- `ROADMAP.md`.

Example final order after an explicit reorder:

```text
custom-c.md
custom-a.md
custom-b.md
TASKS.md
PLANS.md
ROADMAP.md
```

Assert the full exact production `discover()` / persisted `list()` order.

The test must prove both:

- explicit custom order beats lexical custom path order;
- STANDARD sources remain after CUSTOM and follow documented authority priority.

---

# Do not touch accepted work

Do not change unless a regression absolutely requires it:

- M08 IPC command set;
- Tauri permissions/capabilities;
- filesystem limits;
- containment logic;
- M08 ownership/schema reconciliation;
- stale frontend race guards;
- `/tasks` layout;
- publisher scripts;
- DB migration schema.

Expected production edit should be centered on:

`H!veAI/src-tauri/src/task_sources.rs`

plus focused tests, `TASKS.md`, and one new immutable M08C log.

---

# Required self-audit before claiming closure

For each defect write this in the M08C log:

```text
DEFECT 1
Production function changed:
Exact test:
Pre-fix behavior that would fail the test:
Post-fix behavior proved:
PASS/FAIL:

DEFECT 2
Exact test:
Why it has >=3 CUSTOM + multiple STANDARD:
Exact asserted order:
PASS/FAIL:
```

Do **not** mark PASS because a test with a plausible name exists. State what transition/assertion proves it.

---

# Verification

Run the focused Rust task-source tests first.

Then run all existing M08 focused frontend tests unchanged unless a direct compatibility regression requires a test adjustment.

Then from `H!veAI` run:

```powershell
npm run typecheck
npm test -- --reporter=dot
npm run build
npm audit --audit-level=high
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo check --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
powershell -ExecutionPolicy Bypass -File scripts\tests\publish-dev-qa-failure-harness.ps1
```

Only if all gates pass:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\publish-dev-qa.ps1
```

No installer.

---

# Tracker and log

Update `H!veAI/TASKS.md` truthfully:

- historical M08/M08A/M08B audit results remain history;
- M08C is the only active micro-fix while executing;
- M09 stays BLOCKED/UNSTARTED;
- do not mark M08 PASS yourself;
- leave independent strict re-audit and user visual acceptance open.

Create only this new log:

`H!veAI/docs/H!veAI/codex-logs/M08C_CUSTOM_ORDER_BACKCOMPAT_MICRO_FIX_LOG.md`

Record:

- synchronized base HEAD;
- exact changed files;
- DEFECT 1 and DEFECT 2 self-audit blocks above;
- every focused Rust test name/result;
- focused frontend result;
- full regression/security/harness result;
- stable EXE SHA-256;
- canonical asset hashes unchanged;
- no installer;
- no M09;
- local/origin publication commit equality after push;
- native `/tasks` remains `PENDING USER VISUAL ACCEPTANCE`.

---

# Stop condition

Stop after this micro-fix, tests, publication, log, commit, and push.

If the exact legacy three-path test or exact three-CUSTOM-plus-STANDARD ordering test fails, report `FAIL` and stop.

Do not start M09.
