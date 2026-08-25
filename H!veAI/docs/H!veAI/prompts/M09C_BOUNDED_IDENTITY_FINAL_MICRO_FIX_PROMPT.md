# M09C Bounded Identity Final Micro-Fix

## Mission

Fix ONLY the one remaining production MAJOR and the two direct-evidence tightenings from:

`H!veAI/docs/H!veAI/audits/M09B_BOUNDED_IDENTITY_STRICT_REAUDIT.md`

This is a tiny parser micro-fix, not a redesign and not a new milestone.

Do not start M10.
Do not touch visible UI.
Do not fix X01 terminal popup or X02 startup audio in this run.
Do not create an installer.

M00-M08 remain PASS/CLOSED. M09 remains OPEN until independent M09C re-audit.

---

## Start

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
2. `H!veAI/TASKS.md`
3. `H!veAI/docs/H!veAI/audits/M09B_BOUNDED_IDENTITY_STRICT_REAUDIT.md`
4. `H!veAI/src-tauri/src/task_intelligence.rs`
5. this prompt

Record starting branch/HEAD/status/worktrees/untracked files in the M09C log.

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

Do not edit visible UI production files, StartupIntro, Git Engine, watcher, launcher UX, Command Center, route CSS, or canonical assets.

---

# R02C - FIX UNBOUNDED IDENTITY / DUPLICATE-ORDINAL WORKING STATE

## Current defect

M09B bounds persisted milestone/evidence/handoff/explicit-ID output correctly, but parser working identity is still unbounded.

Current fallback duplicate key contains raw heading context:

```rust
format!("{}|{}", context.join("/"), normalize_text(&candidate.title))
```

and is retained inside:

```rust
HashMap<String, usize>
```

for every unique task identity in the source.

`task_id()` also constructs one potentially huge formatted identity string before hashing it.

A <=2 MiB source with one very large heading and many distinct tasks can therefore still multiply that raw heading across parser working memory.

## Required production behavior

1. `duplicate_ordinals` MUST use a fixed-size deterministic identity key, not a raw source-derived String.
   - Preferred: `[u8; 32]`, fixed-size digest type, or equivalent fixed-size key.
   - A 64-character hex digest is acceptable if the map key remains fixed-size with respect to source content.

2. The duplicate key MUST preserve the same logical identity rules:
   - explicit identity: normalized explicit ID;
   - fallback identity: normalized heading path + normalized title;
   - duplicate ordinal remains deterministic.

3. `task_id()` MUST hash identity incrementally or through fixed-size component digests so it does not allocate a giant raw formatted identity string.

4. Preserve current task-ID semantics for ordinary existing tasks wherever feasible.
   - Do NOT intentionally churn every M09 task ID merely to make this refactor convenient.
   - Prefer incremental SHA-256 updates that produce the same digest bytes as the current logical concatenation.

5. Persisted display/evidence bounds from M09B MUST remain unchanged and green.

6. No source body, heading, explicit ID, or other source-derived scalar larger than the source itself may be multiplied into O(task_count × field_size) retained working keys.

## Exact direct tests

Add:

### `r02c_duplicate_identity_key_is_fixed_size_for_oversized_heading`

- create one >4096-byte multibyte heading;
- create many distinct fallback tasks under it;
- call the same production duplicate-key helper used by `parse_document()`;
- assert every retained duplicate identity key has a fixed representation independent of heading length;
- assert the key representation does not contain/copy the raw heading text;
- assert distinct logical tasks still produce correct deterministic ordinals.

The test MUST fail on M09B `f919fb664c8b0f74c9a7c626e80e0db59d34fad3`.

### `r02c_task_ids_remain_stable_after_identity_streaming_refactor`

- use representative explicit-ID and fallback tasks;
- compare expected current M09B task IDs from a deterministic fixture or compare the old logical-concatenation digest in test-only code against the new production incremental helper;
- prove no accidental ID churn for normal tasks.

### `r02c_large_heading_many_tasks_remains_deterministic`

- use a very large heading plus enough distinct tasks to expose repeated-key amplification behavior;
- parse twice through production M08 -> M09;
- assert identical task IDs/order/semantic fields/warning codes;
- persisted milestone/evidence heading components remain <=4096 bytes.

PASS only if the current M09B raw String-key implementation would fail the fixed-size-key test.

---

# E01C - RETRY CONTAINMENT TEST MUST DIRECTLY HIT THE RETRY CONTAINMENT CHECK

Production behavior is already accepted. Do not redesign the reader.

Current M09B test reaches retry but changes the target from file to directory, then fails during read. Strengthen evidence so the retry branch itself receives an escaped refreshed relative path or equivalent test-only substitution and the production refreshed containment check rejects it.

Preferred bounded method:

- add a private `cfg(test)` hook that substitutes the refreshed relative path after M08 rediscovery but before refreshed canonicalization;
- set it to an outside-root relative path such as `../outside.md` in the test;
- call real `read_authoritative_source()`;
- assert `SOURCE_READ_FAILED` from the refreshed containment path;
- no symlink/junction privilege is required.

Do not weaken real M08 containment. Do not expose the hook in production builds.

Required test name may remain:

`p01_retry_rechecks_physical_containment`

but the body must now directly exercise the retry containment rejection, not only retry re-read failure.

---

# E03C - COMPLETE THE STALE SOURCE FIXTURE CONTRACT

Do not redesign persistence. Production selective reconciliation is accepted.

Strengthen `p07_removed_task_and_source_reconcile_only_stale_m09_rows` so the fixture additionally contains:

- unrelated legacy settings row;
- at least one meaningful `SOURCE_EXPLICIT` dependency edge in the retained M09 data.

After removing/unconfiguring the stale source and reparsing, directly assert:

- stale M09 source gone;
- stale M09 task gone;
- retained M09 source/task survive;
- legacy source survives unchanged;
- legacy task survives unchanged;
- legacy settings row survives unchanged;
- expected retained SOURCE_EXPLICIT edge count is exact and no duplicate edge exists.

No blanket delete is allowed.

---

# E05 - FINAL PUBLICATION / REMOTE TRUTH

Do not mutate historical M09/M09A/M09B logs.

Create a new M09C log and record:

- implementation commit;
- publication result;
- stable EXE SHA/size/shortcut;
- final pushed commit containing the M09C log;
- final remote branch HEAD visible after push.

Because a commit cannot record equality with itself before it exists, do not create an endless equality-commit loop. Record the exact final pushed log commit SHA and then verify with commands that local HEAD and `origin/H!veAI` both resolve to that SHA. Put the command output in the terminal/session evidence and summarize the verified SHA in the final Codex response. The M09C log may record the final log commit as `SELF / verified after push in session` if necessary rather than creating another commit solely to describe itself.

No force-push.

---

# Regression gates

After focused tests pass, run:

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

Verify:

- canonical asset hashes unchanged;
- no visible UI production source changed;
- no M10 code introduced;
- no installer;
- X01/X02 intentionally unchanged;
- stable `H!veAI.exe` and shortcut valid.

---

# TASKS truth

Update prospectively only:

- M09 original audit = historical FAIL;
- M09A audit = historical FAIL;
- M09B strict re-audit = historical FAIL due R02C;
- M09C implementation may be marked automated-complete only with evidence;
- independent M09C re-audit remains pending;
- M09 MUST NOT be marked PASS/CLOSED by builder;
- M10 remains BLOCKED/UNSTARTED.

Do not modify historical builder logs/audits.

---

# Required M09C log

Create:

`H!veAI/docs/H!veAI/codex-logs/M09C_BOUNDED_IDENTITY_FINAL_MICRO_FIX_LOG.md`

Required self-audit:

```text
R02C
Production symbol(s):
Exact test(s):
Why f919fb66 fails:
Fixed-size working identity representation:
Task-ID stability proof:
Status: PASS / FAIL

E01C
Exact retry-containment evidence:
Status: PASS / UNVERIFIED

E03C
Exact stale/retained/legacy/settings/dependency SQL evidence:
Status: PASS / FAIL
```

Also record focused/full test totals, security/build/publisher results, asset hashes, implementation commit, stable EXE evidence, and pushed-log commit evidence.

A test name alone is not evidence.

---

# Stop condition

Stop when and only when:

1. duplicate identity working keys are fixed-size;
2. task-ID hashing no longer allocates/retains giant raw identity strings and normal task IDs remain stable;
3. R02C direct tests pass and fail on M09B pre-fix behavior;
4. E01C directly exercises refreshed retry containment;
5. E03C fixture includes legacy settings + dependency exactness;
6. full regression/publisher gates pass;
7. M09C log is committed/pushed;
8. final pushed branch HEAD is verified without force-push;
9. M10/X01/X02 remain untouched.

Then stop and wait for independent audit.
