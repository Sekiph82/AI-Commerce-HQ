# M01 — H!veAI Tauri 2 Foundation

You are continuing H!veAI development after an independently approved M00.

Do NOT start M02.

## Canonical locations

Git root:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

H!veAI application root:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

GitHub repository:
`https://github.com/Sekiph82/AI-Commerce-HQ`

Development branch:
`H!veAI`

Canonical product name:
`H!veAI`

The second character is an exclamation mark.

## Read first

Read completely before coding:

- `H!veAI/AGENTS.md`
- `H!veAI/CONSTITUTION.md`
- `H!veAI/ARCHITECTURE.md`
- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/docs/H!veAI/README.md`
- `H!veAI/docs/H!veAI/audits/M00_FRESH_START_AUDIT_APPROVAL.md`
- `H!veAI/docs/H!veAI/codex-logs/README.md`
- this prompt

## Mandatory repository preflight

Run and log:

- `git rev-parse --show-toplevel`
- `git branch --show-current`
- `git rev-parse HEAD`
- `git remote -v`
- `git status --short`
- `git stash list`

Stop if:

- Git root is not the canonical parent root,
- current branch is not `H!veAI`,
- origin is not `https://github.com/Sekiph82/AI-Commerce-HQ.git` or equivalent HTTPS form.

Do not modify files if any of those checks fail.

## Preserve legacy parent state

A pre-existing stash was recorded during M00:

`stash@{0}: preserve pre-M00 user package changes before H!veAI branch switch`

Do NOT apply it.
Do NOT drop it.
Do NOT pop it.
Do NOT inspect secret-bearing content unnecessarily.

Also preserve untouched user files:

- `start-demo.bat`
- `task.md`

M01 must not repair, modernize, or refactor the legacy parent application.

The parent app's broken local `npm run build` and Tauri 1 `dist` dependency are legacy baseline issues, not M01 targets.

## Durable Codex log

Create or continue before code changes:

`H!veAI/docs/H!veAI/codex-logs/M01_TAURI2_FOUNDATION_CODEX_LOG.md`

Record chronologically:

- timestamps,
- commands,
- relevant outputs,
- files changed,
- decisions and reasons,
- failures,
- fixes,
- tests,
- git state,
- commit,
- push status.

Never erase failures after fixing them.
Never record secrets or token values.

## M01 objective

Create a clean, NEW Tauri 2 desktop foundation entirely under the H!veAI application workspace.

Do NOT upgrade the legacy parent `src-tauri/`.
Do NOT copy the legacy parent Tauri 1 app wholesale.
Do NOT implement H!veAI dashboard features yet.

M01 should establish only:

- H!veAI frontend bootstrap sufficient for Tauri 2,
- H!veAI Tauri 2 shell,
- native app identity,
- secure minimal capabilities,
- native logging,
- native notifications foundation,
- lifecycle/status commands,
- clean Windows build/run baseline.

## Step 1 — inspect child workspace

From `H!veAI/`, inspect:

- `package.json`
- `README.md`
- `ARCHITECTURE.md`
- `TASKS.md`
- existing `src/`
- existing `src-tauri/`
- existing `tests/`
- child `.gitignore` if any

Confirm no real product UI was accidentally implemented in M00.

## Step 2 — choose minimal frontend bootstrap

For M01 use a minimal React + TypeScript + Vite frontend inside `H!veAI/` unless the authoritative architecture docs explicitly require a different already-approved choice.

The frontend only needs to prove the Tauri app can launch.

Create a minimal placeholder screen showing:

- product name `H!veAI`
- subtitle `AI Development Command Center`
- foundation status

This is not the M02 design system.
Do not build dashboard navigation, project cards, agent UI, audit UI, or command center features yet.

## Step 3 — create Tauri 2 shell

Create a fresh Tauri 2 project under:

`H!veAI/src-tauri/`

Use current Tauri 2 conventions.

Requirements:

- app product name: `H!veAI`
- app identifier: use a valid identifier without `!`, e.g. `ai.hiveai.desktop`
- Windows desktop target
- development URL points to child Vite dev server
- production frontend points to child Vite build output
- minimal capabilities only
- no broad shell access
- no inherited commerce permissions
- no inherited parent HTTP-all or shell-open permission sets unless explicitly needed

## Step 4 — native foundation

Implement minimal native commands such as:

- `hiveai_native_status`
- `hiveai_request_restart`

The status command should return structured data sufficient to prove native IPC works.

The restart command should be implemented safely using supported Tauri 2 APIs.

Do not expose a generic arbitrary shell command.

## Step 5 — logging and notification plugins

Add current supported Tauri 2 logging and notification plugins.

Configure them minimally and securely.

The app must not request unnecessary capabilities at startup.

Document where logs are expected to be written on Windows.

## Step 6 — frontend/native IPC smoke surface

Add a minimal foundation screen that:

- renders `H!veAI`,
- invokes `hiveai_native_status`,
- displays a safe status result,
- provides a restart control only if safe for the dev foundation,
- clearly labels the screen as `M01 Foundation`.

Do not style beyond basic readable layout.
M02 owns the real UI system.

## Step 7 — dependency isolation

All H!veAI dependencies must belong to:

`H!veAI/package.json`

Do not depend on parent `node_modules`.
Do not modify parent `package.json` or parent `package-lock.json`.

Generate child lockfile normally if needed.

## Step 8 — security baseline

Document:

- Tauri capabilities enabled,
- commands exposed,
- plugins enabled,
- filesystem/shell/network permissions,
- whether notifications require runtime permission,
- log output location,
- any remaining security questions.

Create:

`H!veAI/docs/migration/M01_TAURI2_FOUNDATION.md`

## Step 9 — tests and verification

Run from the H!veAI workspace as appropriate:

Frontend:

- install dependencies using the child package manifest,
- typecheck,
- production build.

Rust/Tauri:

- `cargo check --manifest-path H!veAI/src-tauri/Cargo.toml`
- `cargo test --manifest-path H!veAI/src-tauri/Cargo.toml`
- `cargo build --manifest-path H!veAI/src-tauri/Cargo.toml`

Tauri CLI:

- record `npx tauri --version` from child workspace or equivalent child-local invocation.

Windows smoke test:

- launch the H!veAI desktop app in a bounded dev/build smoke test,
- confirm the window opens,
- confirm the visible product name is `H!veAI`,
- confirm native status IPC succeeds,
- confirm the app closes cleanly,
- confirm no legacy commerce runtime starts.

If restart cannot be safely verified automatically, leave that item explicitly blocked/manual rather than falsely marking complete.

## Step 10 — TASKS.md

Update only M01-related items in:

`H!veAI/TASKS.md`

Use `[x]` only for completed and verified items.
Use `[!]` for blocked verification.
Do not mark M02 or later milestones complete.

## Step 11 — repository containment

Before commit verify:

- all new application code is under `H!veAI/`,
- no parent application source was changed,
- root `.gitignore` is changed only if strictly necessary,
- no `.env`, DB, target, dist, node_modules, logs containing secrets, caches or generated binaries are staged.

## Step 12 — final validation

Run and record:

- `git status --short`
- `git branch --show-current`
- `git rev-parse HEAD`
- `git remote -v`
- `git diff --check`
- `git diff --cached --stat` before commit if staged
- child frontend verification
- child Rust/Tauri verification

## Commit and push

If M01 is genuinely complete, create one focused commit:

`feat(H!veAI): establish Tauri 2 desktop foundation`

Push normally to:

`origin/H!veAI`

Do not force push.

## M01 acceptance criteria

M01 is complete only if:

1. H!veAI has an independent child frontend dependency set.
2. H!veAI has a fresh Tauri 2 shell under `H!veAI/src-tauri/`.
3. Product name displays as `H!veAI`.
4. Valid technical identifier is used where `!` is illegal.
5. Native IPC status command works.
6. Logging plugin is installed/configured.
7. Notification plugin foundation is installed/configured.
8. Capabilities are minimal and documented.
9. Child frontend build passes.
10. Rust check/test/build pass.
11. Windows H!veAI smoke launch succeeds.
12. No legacy commerce runtime starts.
13. Parent app source/package files are not modified.
14. The preserved stash remains untouched.
15. Codex log is committed.
16. M01 migration document exists.
17. TASKS.md reflects verified state only.

## Final response

Return exactly:

1. M01 RESULT
2. VERIFIED GIT ROOT
3. VERIFIED H!veAI APPLICATION ROOT
4. CURRENT BRANCH / HEAD
5. CHILD FRONTEND STACK
6. TAURI VERSION
7. APP IDENTITY
8. CAPABILITIES / PERMISSIONS
9. NATIVE COMMANDS
10. LOGGING / NOTIFICATION STATUS
11. FILES ADDED
12. FILES MODIFIED
13. PARENT FILES MODIFIED
14. FRONTEND BUILD RESULT
15. RUST / TAURI RESULTS
16. WINDOWS SMOKE TEST RESULT
17. RESTART VERIFICATION STATUS
18. CODEX LOG PATH
19. MIGRATION DOC PATH
20. PRESERVED STASH STATUS
21. COMMIT / PUSH STATUS
22. BLOCKERS / OPEN DECISIONS
23. EXACT NEXT MILESTONE
24. RECOMMENDED NEXT CODEX PROMPT

The exact next milestone is:

`M02 — H!veAI UI Shell and Design System`

Do NOT start M02.
Stop after M01.
