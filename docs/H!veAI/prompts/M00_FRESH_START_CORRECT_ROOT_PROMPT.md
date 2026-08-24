# M00 — H!veAI Fresh Start with Dedicated Application Root

You are restarting H!veAI development from the correct local repository and the correct H!veAI application folder.

Do NOT reuse prior M00/M01 conclusions blindly.

## Canonical locations

GitHub repository:
`https://github.com/Sekiph82/AI-Commerce-HQ`

Canonical development branch:
`H!veAI`

Canonical LOCAL GIT repository root:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Canonical H!veAI APPLICATION root:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

These are intentionally different.

- Git operations belong to the parent repository root.
- All NEW H!veAI application code, H!veAI product documentation, prompts, audits, Codex logs, tests, desktop shell and application configuration belong under the child `H!veAI\` directory unless a root-level Git compatibility file is strictly necessary.
- The child `H!veAI\` directory is NOT a separate Git repository and must not contain its own `.git` directory.

The canonical product name is **H!veAI**. The second character is `!`.

## Mandatory stop conditions

Your FIRST command must be run from:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Run:
`git rev-parse --show-toplevel`

Normalize Windows path separators/casing if needed.

If the result is not:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

STOP. Do not modify files.

Then verify:
`Test-Path .\H!veAI`

If the H!veAI application directory does not exist, STOP and report.

If `.\H!veAI\.git` exists, STOP and report the nested repository conflict. Do not delete it automatically.

## Read authoritative control files first

From branch `H!veAI`, read:

- `docs/H!veAI/README.md`
- `docs/H!veAI/audits/M00_FRESH_START_AUDIT.md`
- `docs/H!veAI/codex-logs/README.md`
- this prompt

Also inspect all files currently present under local:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

Do not assume they are authoritative until compared with this prompt and the current GitHub branch.

## Logging rule

Create the durable Codex log INSIDE the H!veAI application root:

`H!veAI/docs/H!veAI/codex-logs/M00_FRESH_START_CODEX_LOG.md`

Create it before making H!veAI code changes.

Record chronologically:
- timestamps
- commands
- results
- files inspected
- architectural decisions
- failures
- fixes
- tests
- commits
- push status

Do not erase failures after they are fixed.
Do not record secrets/tokens.

## Step 1 — Prove repository identity

From the parent Git root, run and log:

- `git rev-parse --show-toplevel`
- `git status --short`
- `git branch --show-current`
- `git rev-parse HEAD`
- `git remote -v`
- `git log --graph --decorate --oneline -20`
- `git tag --list`
- `git worktree list`

Verify the official remote:
`https://github.com/Sekiph82/AI-Commerce-HQ.git`

Do not push to `iamlukethedev/Claw3D` or any unrelated remote.

If origin is wrong, inspect before changing it. Preserve evidence. Correct only if safe.

## Step 2 — Verify the official H!veAI branch

Fetch official GitHub state safely.

Verify that the local development branch is or can safely become:
`H!veAI`

Do not create `hiveai-rebuild`.
Do not use `hiveai-control-plane` as the active development branch.
Do not force-reset or force-push.

Verify that official `main` history is present and that branch `H!veAI` descends from the intended AI-Commerce-HQ repository.

## Step 3 — Audit the ACTUAL old AI-Commerce-HQ project

The parent repository contains the old AI-Commerce-HQ application.

Inspect it as SOURCE MATERIAL only.

Audit the actual parent application:

Frontend:
- framework
- entry points
- state management
- styling
- 3D/game UI
- reusable component patterns

Desktop/Tauri:
- whether Tauri exists
- version
- Rust lifecycle/process code
- packaging knowledge

Backend:
- Python/FastAPI
- WebSocket
- agent abstraction
- orchestrators
- state layer

Database:
- SQLite/SQLAlchemy
- schema
- migration behavior
- local data locations

Build/test:
- scripts
- build
- typecheck
- lint
- tests
- Rust/Python checks

Security:
- tokens/secrets
- CORS
- process permissions
- local HTTP
- risky inherited defaults

Do NOT convert the old parent application in place.
Do NOT delete old AI-Commerce-HQ runtime code during M00.

## Step 4 — Audit the H!veAI child directory

Inspect the existing contents of:
`H!veAI\`

Classify every existing file as:

A. KEEP AS H!veAI FOUNDATION
B. UPDATE / REPLACE
C. REFERENCE ONLY
D. REMOVE LATER

Do not delete blindly.

If foundation files exist there, inspect them fully.

## Step 5 — Establish H!veAI as an independent application workspace inside the repo

H!veAI is a new dashboard/product that lives under:
`H!veAI\`

The target application structure should become approximately:

`H!veAI/`
- `package.json`
- `AGENTS.md`
- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`
- `README.md`
- `src/`
- `src-tauri/`
- `docs/`
- `tests/`

Do NOT build actual product features in M00.

M00 only establishes the clean application workspace and authoritative docs.

Do not copy old AI-Commerce-HQ code wholesale into H!veAI.
Only document reusable patterns for later milestones.

## Step 6 — Move/copy canonical AI-development control files into H!veAI

The long-term canonical AI-assisted-development locations must be inside the application root:

- `H!veAI/docs/H!veAI/README.md`
- `H!veAI/docs/H!veAI/prompts/`
- `H!veAI/docs/H!veAI/audits/`
- `H!veAI/docs/H!veAI/codex-logs/`

Copy the current control-plane documents from branch `H!veAI` into these locations where appropriate.

Do not destroy the GitHub branch-level originals during M00 unless instructed later.

From this milestone onward:
- ChatGPT prompts live under `H!veAI/docs/H!veAI/prompts/`
- ChatGPT audits live under `H!veAI/docs/H!veAI/audits/`
- Codex logs live under `H!veAI/docs/H!veAI/codex-logs/`

## Step 7 — Install H!veAI foundation docs in the application root

Ensure these exist under `H!veAI\`:

- `README.md`
- `AGENTS.md`
- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`

Use the previously prepared H!veAI foundation material where valid, but re-check it against the actual old repo audit.

All prose must use the product spelling **H!veAI**.

Technical slugs may use `hiveai` only when punctuation is unsafe or invalid, for example package IDs, Rust crate IDs, internal identifiers, or filesystem-safe technical keys.

## Step 8 — Create fresh M00 migration docs inside H!veAI

Create:

- `H!veAI/docs/migration/M00_AI_COMMERCE_HQ_BASELINE.md`
- `H!veAI/docs/migration/M00_REUSE_MATRIX.md`
- `H!veAI/docs/migration/M00_HIVEAI_APPLICATION_PLAN.md`
- `H!veAI/docs/migration/M00_TECHNICAL_DEBT.md` if useful

The application plan must explicitly state:

Git root:
`...\AI-Commerce-HQ`

Application root:
`...\AI-Commerce-HQ\H!veAI`

and explain why this monorepo-style separation is intentional.

## Step 9 — Reuse matrix

Classify relevant old AI-Commerce-HQ components using exactly:

A. REUSE WITH MINOR CHANGES
B. REUSE AFTER REFACTOR
C. ARCHIVE / REFERENCE ONLY
D. DO NOT COPY INTO H!veAI

Evaluate at minimum:
- React/Vite tooling
- Tailwind
- Zustand
- Framer Motion
- Tauri shell concepts
- Rust lifecycle/process management
- FastAPI backend
- WebSocket manager
- BaseAgent
- async SQLite
- migrations
- 3D UI
- commerce orchestrators
- revenue/XP/gamification
- installer/build scripts

## Step 10 — Baseline validation

Run safe baseline validation of the OLD parent AI-Commerce-HQ application without triggering real external commerce activity.

Also validate the H!veAI child directory for structural correctness.

M00 is not the milestone to install the final Tauri/React application stack unless needed only to establish minimal project metadata.

Do not implement dashboard screens yet.

## Step 11 — Parent repo modification rule

Minimize edits outside `H!veAI\`.

Allowed parent-root modifications in M00 only when needed:
- `.gitignore` entries required for H!veAI build/user data
- a small root README pointer if truly useful
- repository-level CI/workspace configuration if absolutely necessary

Do not replace the old parent `package.json`, `TASKS.md`, `README.md`, `src/`, `src-tauri/`, or backend with H!veAI equivalents.

H!veAI owns its own application files under `H!veAI\`.

## Step 12 — Branch discipline

All M00 changes must be committed on branch:
`H!veAI`

Do not create another H!veAI development branch.

Do not force-push.

## Step 13 — TASKS state

Update:
`H!veAI/TASKS.md`

Mark M00 complete only when evidence supports it.

Do not begin M01.

The exact next milestone remains:
`M01 — Tauri 2 Foundation`

## Step 14 — Final verification

From the Git root run and log:

- `git status`
- `git branch --show-current`
- `git rev-parse HEAD`
- `git remote -v`
- `git log -10 --oneline --decorate`
- `git diff --check`
- `git worktree list`

Verify specifically:

- H!veAI is NOT a nested Git repository
- new H!veAI files live under `H!veAI\`
- old AI-Commerce-HQ runtime remains intact
- no secrets/local DB/build junk were committed
- product spelling is H!veAI in user-facing content

## Commit

If corrected M00 is genuinely complete, create one focused commit:

`chore(H!veAI): establish dedicated application workspace`

Push only to:
`Sekiph82/AI-Commerce-HQ`
branch:
`H!veAI`

using a normal non-force push.

## Final report

Return exactly:

1. M00 RESULT
2. VERIFIED GIT ROOT
3. VERIFIED H!veAI APPLICATION ROOT
4. VERIFIED GITHUB REPOSITORY / BRANCH
5. CURRENT HEAD
6. REMOTE STATUS
7. OLD AI-COMMERCE-HQ ARCHITECTURE SUMMARY
8. H!veAI CHILD FOLDER INITIAL FINDINGS
9. H!veAI FOUNDATION STRUCTURE CREATED
10. REUSE MATRIX SUMMARY
11. FILES ADDED UNDER H!veAI
12. FILES MODIFIED OUTSIDE H!veAI
13. BUILD / TEST RESULTS
14. SECURITY FINDINGS
15. CODEX LOG PATH
16. COMMIT / PUSH STATUS
17. BLOCKERS OR OPEN DECISIONS
18. EXACT NEXT MILESTONE
19. RECOMMENDED NEXT CODEX PROMPT

The exact next milestone is:
`M01 — Tauri 2 Foundation`

Do NOT start M01.
Stop after M00.
