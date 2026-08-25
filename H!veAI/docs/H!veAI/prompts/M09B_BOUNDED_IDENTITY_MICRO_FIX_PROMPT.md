# M09B Bounded Identity Micro-Fix

## Mission

Fix ONLY the two production MAJOR findings and four evidence tightenings in:

`H!veAI/docs/H!veAI/audits/M09A_TASK_INTELLIGENCE_FINAL_STRICT_REAUDIT.md`

This is a micro-fix, not a new milestone.

Do not start M10.
Do not redesign M09.
Do not change visible UI.
Do not fix the separate terminal-popup or startup-audio defects in this run.
Do not create an installer.

M00-M08 remain PASS/CLOSED. M09 remains open until independent re-audit.

---

## Start

Work from:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Run:

```powershell
git fetch origin H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
```

Fast-forward only if safe. Never reset/rebase/force-push/overwrite user-owned work.

Read:

1. `H!veAI/AGENTS.md`
2. `H!veAI/TASKS.md`
3. `H!veAI/docs/H!veAI/audits/M09A_TASK_INTELLIGENCE_FINAL_STRICT_REAUDIT.md`
4. `H!veAI/src-tauri/src/task_intelligence.rs`
5. `H!veAI/src-tauri/src/task_sources.rs`
6. this prompt

Record start branch/HEAD/status/worktrees/untracked files in the M09B log.

---

# Canonical UI Assets

Canonical user-owned asset root:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Canonical assets:

- `scene 3 starting point.png`
- `videos and gifs\opening video.mp4`
- `H!veAI logo.png`
- `H!veAI small logo.png`

Repository canonical asset bytes must remain unchanged.

Do not edit visible UI production files, route CSS, Command Center, StartupIntro, Git Engine, watcher, launcher UX, or canonical assets.

---

# R01 - PATH IDENTITY MUST NOT COLLAPSE DISTINCT FILESYSTEM PATHS

## Current defect

`task_id()` uses prose `normalize_text(path)`. That function collapses internal whitespace.

Therefore distinct M08-approved files such as:

```text
plans/a b.md
plans/a  b.md
```

can map to the same task-ID source-path component.

## Required production behavior

Create a dedicated path-identity normalizer.

It MUST:

- normalize `\` and `/` separators to one canonical separator;
- remove harmless leading `./` components if present;
- apply the repository/platform case-equivalence policy already used by M08 when appropriate;
- preserve meaningful filename whitespace and all other semantic filename characters;
- never call prose whitespace-collapse normalization for filesystem identity.

Use it everywhere M09 task identity depends on source path.

Do not change the user-visible/evidence source path text merely to compute identity.

## Exact direct test

Add:

`r01_distinct_whitespace_paths_never_collide`

Production-path fixture:

1. same registered project;
2. create two approved task sources:
   - `plans/a b.md`
   - `plans/a  b.md`
3. put the same heading and same task text in both;
4. discover through M08;
5. parse through production M09;
6. assert exactly two tasks;
7. assert their `sourcePath` values are distinct;
8. assert their `m09task:` IDs are distinct;
9. query SQLite and assert both task rows exist.

Also add an explicit-ID variant if the same test can cover it cheaply.

## PASS only if

This test FAILS on the current `fba3f98822678c84a03dfef3c52ffe8095b3f68c` implementation.

---

# R02 - BOUND ALL SOURCE-DERIVED PERSISTED SCALARS

## Current defect

M09A bounds task title/body metadata, but these source-derived fields remain unbounded:

- `explicitTaskId`;
- milestone / heading context strings;
- `TaskEvidenceLocator.headingPath` components;
- handoff `current`, `next`, `blockers`, `waiting` values;
- handoff evidence heading-path components;
- source-derived locator text where present.

A large heading can be cloned into many task records and amplify one <=2 MiB source into a very large snapshot.

## Required production behavior

Apply the existing `MAX_FIELD_BYTES = 4096` contract to EVERY source-derived persisted scalar string.

At minimum:

- explicit task ID;
- milestone;
- each heading-path component;
- handoff values;
- locator text;
- all task-body scalar/list values already handled by M09A.

Rules:

- truncation must be UTF-8 safe;
- emit stable `FIELD_TRUNCATED` warning evidence when source-derived content is truncated;
- warning text must name the field kind but must not copy the full source content;
- warnings remain capped at 512;
- avoid one warning per repeated task when one oversized heading is reused many times: deduplicate equivalent field/path truncation warnings or otherwise keep warning growth bounded;
- deterministic task identity must use a stable bounded/normalized identity representation, not a randomly truncated display value;
- repeated parsing of unchanged bounded input must return the same IDs, fields, confidence, and warnings except timestamps.

Do not arbitrarily truncate fixed internal enum/status strings.

## Exact direct tests

Add all four:

### `r02_oversized_heading_is_bounded_without_snapshot_amplification`

- one multibyte heading >4096 bytes;
- multiple tasks beneath it;
- assert every task milestone <=4096 bytes;
- assert every evidence heading component <=4096 bytes;
- assert valid UTF-8;
- assert `FIELD_TRUNCATED` exists;
- assert warning count stays bounded and does not grow once per task for the same heading/path truncation.

### `r02_oversized_handoff_value_is_bounded`

- HANDOFF source with a >4096-byte multibyte Current or Next narrative;
- assert persisted/returned handoff value <=4096 bytes, valid UTF-8, and warning evidence exists.

### `r02_oversized_explicit_id_is_bounded_and_deterministic`

- syntactically valid explicit ID >4096 bytes;
- assert stored `explicitTaskId` <=4096 bytes;
- parse twice and assert task ID remains identical;
- warning evidence exists.

### `r02_bounded_snapshot_repeat_is_deterministic`

- repeat the oversized fixture;
- compare normalized semantic output excluding parse timestamps;
- IDs, task fields, handoff fields, confidence and warning codes/order remain deterministic.

## PASS only if

At least the oversized heading/handoff/explicit-ID tests FAIL on current M09A.

---

# E01 - STRENGTHEN RETRY-CONTAINMENT EVIDENCE

Production F01 is already accepted. Do not redesign it.

Strengthen the existing test so the test actually reaches the retry branch before proving the refreshed target is containment-checked.

Preferred:

- private `cfg(test)` hook/failpoint changes the target between first mismatch and retry resolution;
- production `read_authoritative_source()` remains the code under test.

If Windows refuses the required symlink/junction operation with OS error 1314, record that exact case UNVERIFIED and keep the direct source-path containment proof. Do not fake a PASS.

---

# E02 - ASSERT EXACT HANDOFF MERGE ORDER

Strengthen:

`p06_multiple_handoff_sources_merge_in_source_order`

Do not merely assert both values exist.

Read the M08-discovered parser-source order in the fixture and assert the merged handoff values follow that exact order.

No production redesign unless the stronger test exposes a real defect.

---

# E03 - REAL STALE SOURCE RECONCILIATION TEST

Strengthen the existing stale reconciliation evidence.

Fixture must contain:

- retained M09 source/task;
- second M09 source/task that will become stale;
- unrelated legacy task/source/settings row.

Then remove or unconfigure the second source and reparse.

Direct SQL assertions:

- stale M09 source row gone;
- stale M09 task row gone;
- retained M09 source/task survive;
- unrelated legacy rows survive unchanged;
- no duplicate SOURCE_EXPLICIT edges.

No blanket-delete implementation is allowed.

---

# E04 - TRUTHFUL ADAPTER STATUS

Do not invent new ScrubBots/FMCG conventions only to turn a checkbox green.

Current safe behavior is acceptable if no distinct non-generic convention was actually established:

- adapter selectable by exact Registry identity;
- `conventionMatched=false`;
- no special confidence bonus.

In the NEW M09B log, explicitly record for FormuLab / ScrubBots / FMCG:

```text
Adapter:
Inspected Registry/M08 source path(s):
Distinct non-generic convention found: YES/NO
Implemented convention:
Evidence status: PASS / UNVERIFIED
Special bonus possible: YES/NO
```

If a real distinct convention was not established for ScrubBots/FMCG, write `UNVERIFIED`. That is acceptable and must not block the generic-safe adapter.

Do not modify historical M09/M09A logs.

---

# Regression gates

Run focused tests first, then:

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

Run the existing publisher failure harness and governed production `--no-bundle` QA publisher.

No installer.

Verify:

- canonical asset hashes unchanged;
- no visible UI production source changed;
- no M10 code introduced;
- terminal-popup/audio defects intentionally unchanged;
- stable `H!veAI.exe` and shortcut remain valid.

---

# TASKS truth

Update prospectively only:

- original M09 strict audit = historical FAIL;
- M09A strict re-audit = historical FAIL due R01/R02;
- M09B implementation may be marked complete only by evidence;
- independent M09B re-audit remains pending;
- M10 remains BLOCKED/UNSTARTED;
- do NOT mark M09 PASS/CLOSED.

---

# Required M09B log

Create:

`H!veAI/docs/H!veAI/codex-logs/M09B_BOUNDED_IDENTITY_MICRO_FIX_LOG.md`

The log must include:

```text
R01
Production symbol(s):
Exact test(s):
Why old fba3f988 code fails:
Post-fix behavior:
Status:

R02
Production symbol(s):
Exact test(s):
Why old fba3f988 code fails:
Post-fix behavior:
Status:

E01/E02/E03/E04
Exact evidence:
Status: PASS / UNVERIFIED
```

Then record:

- focused test names + results;
- full Rust/frontend totals;
- security/build/publisher results;
- canonical asset hashes;
- implementation commit;
- final local HEAD;
- final `origin/H!veAI` HEAD;
- exact local/origin equality AFTER every final log/test commit.

A test name alone is not evidence.

---

# Stop condition

Stop when and only when:

1. R01 and R02 production fixes are complete;
2. required direct tests pass and would fail on `fba3f988...`;
3. E01-E04 are truthfully resolved/UNVERIFIED;
4. full regression and publisher gates pass;
5. M09B log is committed and pushed;
6. final local/origin equality is recorded after all commits;
7. M10 remains untouched.

Then stop and wait for independent audit.
