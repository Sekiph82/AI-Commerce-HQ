# M00 RESET — Correct Local Root Re-Baseline for H!veAI

You must restart H!veAI foundation work from the correct local repository root.

Do NOT continue from earlier M00/M01 conclusions.

Those earlier milestone results are now classified as:

`SUPERSEDED / INVALID BASELINE`

because the wrong working context appears to have been inspected.

Do NOT delete the earlier work. Preserve it, but do not use it as authoritative evidence.

## Canonical product name

The product name is exactly:

`H!veAI`

The second character is an exclamation mark.

Use `H!veAI` in all user-visible product names, documentation titles, window titles, milestone names, reports, and release naming.

Technical slugs may use `hiveai` only where punctuation is unsafe or unsupported.

---

# 1. CANONICAL LOCAL PATHS

The ONLY intended local repository root is:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

A child folder exists at:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

This child `H!veAI` folder is NOT the Git repository root.

It is only a local H!veAI foundation/source-material folder unless proven otherwise.

Do not run the project from the child folder.
Do not inspect sibling repositories as if they belong to this project.
Do not auto-discover another `.git` repository and switch to it.

Before doing anything else, explicitly change directory to:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Then run:

`git rev-parse --show-toplevel`

The output MUST resolve to the same canonical root.

If it does not, STOP immediately and report the mismatch.

---

# 2. READ CONTROL-PLANE INSTRUCTIONS

From the official GitHub repository:

`https://github.com/Sekiph82/AI-Commerce-HQ`

read branch:

`hiveai-control-plane`

Read:

- `docs/hiveai/README.md`
- `docs/hiveai/audits/2026-08-24_CORRECT_LOCAL_ROOT_RESET_AUDIT.md`
- `docs/hiveai/codex-logs/README.md`
- this prompt

Do not start M01 or M02.

---

# 3. CREATE THE CODEX LOG FIRST

Inside the correct local repository root create or continue:

`docs/hiveai/codex-logs/M00_RESET_CORRECT_LOCAL_ROOT_CODEX_LOG.md`

Create it BEFORE modifying project files.

The log must be committed with the milestone.

Record chronologically:

- timestamps
- current working directory
- git root
- commands
- outputs/results
- files inspected
- architecture findings
- decisions
- failures
- fixes
- tests
- git status
- commits
- push status

Never erase earlier failures after fixing them.
Never record secrets/tokens/private keys.

---

# 4. HARD REPOSITORY IDENTITY GATE

Run and log:

`cd /d "C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ"`

Then:

`git rev-parse --show-toplevel`
`git rev-parse HEAD`
`git status --short`
`git branch --show-current`
`git remote -v`
`git log --graph --decorate --oneline -30`
`git tag --list`
`git worktree list`

Also inspect the root directory listing.

The official GitHub repository is:

`https://github.com/Sekiph82/AI-Commerce-HQ.git`

If `origin` points somewhere else:

DO NOT PUSH.
DO NOT FORCE.
DO NOT RESET.
DO NOT REBASE.

Add the official repo as a temporary second remote, for example:

`hiveai-official`

Fetch it read-only and compare histories.

The official GitHub main currently derives from the historical AI-Commerce-HQ codebase and should contain representative paths such as:

- `src/App.tsx`
- `src-tauri/`
- `backend/main.py`
- `backend/agents/base_agent.py`
- `backend/orchestrator/`

Determine whether the correct local root actually contains this code/history.

If local and official histories are unrelated, STOP migration operations and report.

Do not merge unrelated histories.

---

# 5. INSPECT ONLY THE CORRECT LOCAL PROJECT

Perform a fresh repository audit from the canonical root.

Do NOT reuse earlier architecture conclusions unless reconfirmed from this exact root.

Inspect at minimum:

- root files
- `.gitignore`
- `README.md`
- `TASKS.md`
- `GETTING_STARTED.md`
- `.claude/`
- `package.json`
- `package-lock.json`
- `src/`
- `src-tauri/`
- `backend/`
- scripts/build files
- tests
- docs

Identify the REAL current stack from evidence.

Specifically verify whether it is actually:

- React/Vite
- Tauri
- Python/FastAPI
- WebSocket
- SQLite/SQLAlchemy
- agent/orchestrator architecture

Do not infer.

---

# 6. INSPECT THE CHILD H!veAI FOLDER SEPARATELY

Inspect:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

Treat it only as source material.

List its files.

If it contains foundation documents such as:

- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`

read them fully.

Do not treat that folder as a standalone project.

Do not create a nested `.git` repository there.

Do not run Git operations from there except to prove it is not the root.

---

# 7. PRESERVE CURRENT USER WORK

Before any write, identify every pre-existing modified/untracked file in the canonical root.

Do not discard, overwrite, reset, clean, or silently stash them.

If earlier wrong-context M00/M01 files exist somewhere under the canonical root, preserve them but clearly classify them as superseded unless they are independently revalidated.

---

# 8. RE-RUN M00 FROM SCRATCH

Now perform a NEW M00 against the correct local root.

M00 scope only:

- repository audit
- architecture inventory
- baseline build/test validation
- preservation strategy
- H!veAI foundation document installation
- reuse/replace/retire classification
- safe rebuild preparation
- versioned prompt/audit/Codex-log structure

DO NOT implement H!veAI product features.

DO NOT start Tauri migration.
DO NOT start M01.
DO NOT start UI work.

---

# 9. BUILD / TEST BASELINE

Use the commands documented by the ACTUAL correct repository.

At minimum, where applicable and safe:

- frontend typecheck
- frontend build
- frontend tests
- Python compile/tests
- Cargo/Tauri checks/tests/build

Do not start external commerce automation.
Do not trigger Etsy/Fiverr/Trading/YouTube/TikTok operations.

Record:

- command
- exit code
- result
- pre-existing failure/warning

Do not fix unrelated failures during M00.

---

# 10. PRESERVATION

Only after proving the repository identity and baseline:

inspect whether these already exist:

- `ai-commerce-hq-final`
- `archive/ai-commerce-hq`
- `hiveai-rebuild`

Do not move or recreate them blindly.

If earlier tags/branches were created from the wrong repository context, DO NOT overwrite official refs.

Document the correct preservation plan first.

If safe, create the appropriate correct baseline preservation refs on the correct repository history.

No force push.
No history rewrite.

---

# 11. H!veAI FOUNDATION DOCUMENTS

The correct authoritative repository root should ultimately contain:

- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`
- `AGENTS.md`
- `docs/hiveai/README.md`
- `docs/hiveai/prompts/`
- `docs/hiveai/audits/`
- `docs/hiveai/codex-logs/`

Use the child `H!veAI` folder and `hiveai-control-plane` branch as source material.

Do not blindly overwrite existing root files.
Archive old planning docs where necessary.

All user-visible text must say `H!veAI`, not `HiveAI`.

---

# 12. CREATE NEW CORRECT-ROOT M00 DOCUMENTATION

Create:

- `docs/migration/M00_CORRECT_ROOT_BASELINE.md`
- `docs/migration/M00_CORRECT_ROOT_REUSE_MATRIX.md`
- `docs/migration/M00_CORRECT_ROOT_MIGRATION_PLAN.md`

Clearly state that previous M00/M01 outputs were superseded due to wrong working context.

Do not hide this history.

---

# 13. M00 COMPLETION RULE

M00 RESET is complete only if:

1. canonical local root is proven,
2. local repo identity is proven against official GitHub,
3. child `H!veAI` folder is correctly treated as source material only,
4. real architecture is freshly audited,
5. build/test baseline is captured,
6. existing user changes are preserved,
7. prompt/audit/log directories exist in the authoritative repo,
8. Codex log is committed,
9. H!veAI naming is corrected,
10. preservation refs are correct or explicitly blocked,
11. no H!veAI feature implementation starts,
12. no force push/history rewrite occurs.

---

# 14. COMMIT

If and only if the correct repository identity is proven and M00 reset work is valid, create a focused commit on the authoritative H!veAI development branch:

`chore(hiveai): rebaseline H!veAI from correct AI-Commerce-HQ root`

Technical commit scopes may use `hiveai`; user-visible product text remains `H!veAI`.

Push only to:

`Sekiph82/AI-Commerce-HQ`

and only with normal non-force push after verifying destination and history.

---

# 15. FINAL REPORT

Return exactly:

1. M00 RESET RESULT
2. CANONICAL LOCAL ROOT
3. CHILD H!veAI FOLDER STATUS
4. LOCAL REPOSITORY ORIGIN
5. OFFICIAL GITHUB COMPARISON
6. HISTORY / MERGE-BASE RESULT
7. REAL FRONTEND ARCHITECTURE
8. REAL TAURI ARCHITECTURE
9. REAL BACKEND ARCHITECTURE
10. REAL DATABASE ARCHITECTURE
11. REAL AGENT/ORCHESTRATOR ARCHITECTURE
12. BASELINE BUILD / TEST RESULTS
13. PRESERVED USER CHANGES
14. PREVIOUS M00/M01 STATUS
15. FOUNDATION FILES INSTALLED
16. CODEX LOG PATH
17. PRESERVATION TAG / BRANCH STATUS
18. CURRENT AUTHORITATIVE BRANCH / HEAD
19. REMOTE / PUSH STATUS
20. BLOCKERS
21. EXACT NEXT MILESTONE

If M00 passes, exact next milestone:

`M01 — Tauri 2 Foundation`

DO NOT START M01.

Stop after the corrected M00 reset.
