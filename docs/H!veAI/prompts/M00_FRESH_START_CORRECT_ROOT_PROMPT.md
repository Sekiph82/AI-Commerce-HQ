# M00 — H!veAI Fresh Start from Correct Local Root

You are restarting H!veAI development from the correct repository.

Do NOT reuse prior M00/M01 conclusions blindly.

## Canonical repository

GitHub:
`https://github.com/Sekiph82/AI-Commerce-HQ`

Canonical development branch:
`H!veAI`

Canonical local Git repository root:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

The nested folder:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

is NOT the repository root. It may contain reference files only.

## Mandatory stop condition

Your FIRST command must establish the actual Git root:

`git rev-parse --show-toplevel`

Normalize Windows path separators/casing for comparison if needed.

If the result is not exactly the canonical root above, STOP. Do not modify files. Report the detected root.

## Read authoritative control files first

From branch `H!veAI`, read:

- `docs/H!veAI/README.md`
- `docs/H!veAI/audits/M00_FRESH_START_AUDIT.md`
- `docs/H!veAI/codex-logs/README.md`
- this prompt

The canonical product name is **H!veAI**. The second character is `!`.

## Start durable Codex log before changes

Create or continue:

`docs/H!veAI/codex-logs/M00_FRESH_START_CODEX_LOG.md`

Record all meaningful commands, results, decisions, failures, fixes, tests, commits and push status chronologically.

## Step 1 — Prove local repository identity

Run and log:

- `git rev-parse --show-toplevel`
- `git status --short`
- `git branch --show-current`
- `git rev-parse HEAD`
- `git remote -v`
- `git log --graph --decorate --oneline -20`
- `git tag --list`
- `git worktree list`

Verify that the official repository remote is or can safely become:

`https://github.com/Sekiph82/AI-Commerce-HQ.git`

Do not push to `iamlukethedev/Claw3D` or any unrelated remote.

If `origin` is wrong, do not overwrite it blindly. Inspect first, preserve evidence, then correct only if safe.

## Step 2 — Verify official main ancestry

Fetch official GitHub state safely.

Prove that the local repository contains the official AI-Commerce-HQ history and representative files from official main.

At minimum inspect:

- `src/App.tsx`
- `src-tauri/` if present
- `backend/main.py` if present
- `backend/agents/base_agent.py` if present
- `backend/orchestrator/` if present
- `package.json`
- `TASKS.md`
- `README.md`

Do not assume architecture from old reports. Inspect actual files.

## Step 3 — Inspect nested H!veAI reference folder

Inspect:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

List and read its foundation/reference files.

Do NOT treat it as a separate Git repository unless evidence proves it actually contains a nested `.git` directory.

Do NOT run development commands from that child folder.

Use it only as an input/reference source during M00.

## Step 4 — Full fresh baseline audit

Audit the ACTUAL official AI-Commerce-HQ repository from scratch.

Map:

Frontend:
- framework
- entry points
- routing
- state management
- styling
- 3D/UI systems

Desktop/Tauri:
- whether Tauri exists
- version
- Rust structure
- lifecycle
- packaging

Backend:
- Python/FastAPI presence
- API/WebSocket architecture
- agent/orchestrator structure

Database:
- technology
- models
- migrations
- local data paths

Build/test:
- scripts
- current build
- typecheck
- lint
- tests
- Rust/Python checks where applicable

Security:
- tokens/secrets handling
- local HTTP/CORS
- shell/process permissions
- risky inherited defaults

## Step 5 — Reuse classification

Create or update a fresh reuse matrix using exactly:

A. REUSE WITH MINOR CHANGES
B. REUSE AFTER REFACTOR
C. ARCHIVE / REFERENCE ONLY
D. REMOVE FROM ACTIVE H!veAI RUNTIME

Do not inherit previous classifications without re-verifying actual source files.

## Step 6 — Install canonical H!veAI foundation docs

Use the nested `H!veAI` folder only as a reference source.

At the official repository root, establish canonical H!veAI documents such as:

- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`
- `AGENTS.md`

Preserve any existing AI-Commerce-HQ planning docs before replacing conflicting root files.

Archive old planning docs under `docs/archive/` when appropriate.

Do not delete old runtime code in M00.

## Step 7 — M00 documentation

Create:

- `docs/migration/M00_AI_COMMERCE_HQ_BASELINE.md`
- `docs/migration/M00_REUSE_MATRIX.md`
- `docs/migration/M00_HIVEAI_MIGRATION_PLAN.md`
- `docs/migration/M00_TECHNICAL_DEBT.md` if useful

All product naming in prose must use **H!veAI**.

## Step 8 — Build/test baseline

Run safe baseline checks based on the ACTUAL repository architecture.

Do not trigger real external commerce operations.

Record exact commands, pass/fail, warnings and pre-existing failures.

M00 is baseline capture and foundation setup, not mass cleanup.

## Step 9 — Branch discipline

Work only on branch:

`H!veAI`

If local branch does not exist, fetch/check out the official `H!veAI` branch safely.

Do not create `hiveai-rebuild`.

Do not use `hiveai-control-plane` as the development branch.

Do not force-push.

## Step 10 — Final verification

Run and log:

- `git status`
- `git branch --show-current`
- `git rev-parse HEAD`
- `git remote -v`
- `git log -10 --oneline --decorate`
- `git diff --check`
- `git worktree list`

Ensure no secrets, local DBs, user-specific dumps, build artifacts or unrelated files are accidentally committed.

## Commit

If corrected M00 is genuinely complete, create one focused commit:

`chore(H!veAI): establish fresh baseline from official repository`

Push only to:

`Sekiph82/AI-Commerce-HQ` branch `H!veAI`

using a normal non-force push.

## Final report

Return exactly:

1. M00 RESULT
2. VERIFIED LOCAL REPOSITORY ROOT
3. VERIFIED GITHUB REPOSITORY
4. CURRENT BRANCH / HEAD
5. REMOTE STATUS
6. NESTED H!veAI FOLDER FINDINGS
7. ACTUAL FRONTEND ARCHITECTURE
8. ACTUAL TAURI ARCHITECTURE
9. ACTUAL BACKEND ARCHITECTURE
10. DATABASE FINDINGS
11. REUSE MATRIX SUMMARY
12. FILES ADDED
13. FILES MODIFIED
14. FILES ARCHIVED
15. BUILD / TEST RESULTS
16. SECURITY FINDINGS
17. CODEX LOG PATH
18. COMMIT / PUSH STATUS
19. BLOCKERS OR OPEN DECISIONS
20. EXACT NEXT MILESTONE

The exact next milestone is:

`M01 — Tauri 2 Foundation`

Do NOT start M01.
Stop after M00.
