# M09D Retry Containment Evidence Closure

## Mission

Close ONLY the single remaining evidence item from:

`H!veAI/docs/H!veAI/audits/M09C_BOUNDED_IDENTITY_FINAL_STRICT_REAUDIT.md`

This is a **test-only evidence closure** for M09.

There is no known remaining M09 production MAJOR.

Do not redesign M09.
Do not start M10.
Do not fix X01 terminal popups or X02 startup audio in this run.
Do not change visible UI.
Do not create an installer.

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

Read:

1. `H!veAI/AGENTS.md`
2. `H!veAI/TASKS.md`
3. `H!veAI/docs/H!veAI/audits/M09C_BOUNDED_IDENTITY_FINAL_STRICT_REAUDIT.md`
4. `H!veAI/src-tauri/src/task_intelligence.rs`
5. this prompt

Preserve user-owned untracked files.

---

# Canonical UI Assets

Canonical user-owned asset root:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Canonical assets:

- `scene 3 starting point.png`
- `videos and gifs\opening video.mp4`
- `H!veAI logo.png`
- `H!veAI small logo.png`

Do not modify any canonical asset or visible UI production file.

---

# E01D - MAKE THE RETRY TEST HIT THE ACTUAL CONTAINMENT REJECTION

## Current evidence defect

`p01_retry_rechecks_physical_containment` currently substitutes:

```text
../outside.md
```

but does not create that outside file.

Production does:

```text
join refreshed candidate
-> canonicalize refreshed candidate
-> starts_with(refreshed_root) containment check
```

Therefore the test can currently PASS because `canonicalize()` fails with file-not-found before the explicit containment check runs.

## Required change

Strengthen ONLY the test/test-only hook as necessary.

Preferred fixture:

1. create the registered project fixture normally;
2. create a real file in the project temp directory's **parent**, with a unique deterministic test filename derived from the project temp directory name;
3. set the private retry relative-path failpoint to `../<that-unique-file>`;
4. ensure that path canonicalizes successfully;
5. call the real production `read_authoritative_source()` retry path;
6. assert:
   - warning code is exactly `SOURCE_READ_FAILED`;
   - warning message is exactly `refreshed source is outside registered root`;
7. clean up the outside test file if the temp fixture does not own it automatically.

An equivalent isolated tempfile strategy is acceptable only if it guarantees the substituted refreshed target canonicalizes successfully outside the registered project root before the containment assertion.

Do not satisfy this by asserting only the warning code.
Do not satisfy this using a missing path.
Do not weaken or bypass production containment.
Do not expose test hooks in production builds.

## PASS only if

The strengthened test would FAIL on current M09C because current M09C's fixture points to a nonexistent outside target and does not assert the containment-specific message.

Production `read_authoritative_source()` should not need redesign. If the stronger test exposes a real production defect, stop and report it rather than broadening scope silently.

---

# Regression

Run the strengthened focused test first.

Then run:

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

Verify no visible UI, canonical asset, M10, Git Engine, watcher, StartupIntro, X01, or X02 changes.

---

# TASKS truth

Update `H!veAI/TASKS.md` prospectively only:

- M09C independent re-audit = `CONDITIONAL`, 0 BLOCKER / 0 MAJOR / 1 MINOR evidence item;
- add M09D as evidence-only retry-containment closure;
- mark M09D implementation/test work complete only with actual evidence;
- keep final independent M09D audit pending;
- keep M09 NOT CLOSED;
- keep M10 BLOCKED/UNSTARTED;
- keep X01/X02 queued for after M09 closure and before M10.

Do not rewrite historical logs/audits.

---

# Required log

Create:

`H!veAI/docs/H!veAI/codex-logs/M09D_RETRY_CONTAINMENT_EVIDENCE_CLOSURE_LOG.md`

Record:

```text
E01D
Production source changed: YES/NO
Test/test-only symbol(s) changed:
Exact test:
Why M09C test could false-pass:
How M09D proves canonicalizable outside-root target:
Exact asserted warning code:
Exact asserted warning message:
Status:
```

Also record focused/full regression/publication results, scope verification, implementation/log commit(s), and final remote branch HEAD.

After the final pushed log commit, run:

```powershell
git fetch origin H!veAI
git rev-parse HEAD
git rev-parse origin/H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
```

Final required result:

```text
HEAD == origin/H!veAI
0 0
```

Do not create another commit solely to describe that equality. Report the verified final SHA in the final Codex response.

---

# Stop condition

Stop only when:

1. the retry containment test uses a real canonicalizable outside-root file;
2. the test asserts the containment-specific warning message;
3. focused/full regression and publication gates pass;
4. M09D log is committed and pushed;
5. final local/origin equality is verified after the pushed log commit;
6. M10/X01/X02 remain untouched.

Then stop and wait for independent audit.
