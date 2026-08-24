# M01.5 — Repository Identity Repair + Versioned AI/Codex Logging Bootstrap

You are working on the local H!veAI development folder previously used for M00 and M01.

Do NOT start M02.

This is a blocking integrity milestone whose purpose is to reconcile the local repository with the intended official GitHub repository:

`https://github.com/Sekiph82/AI-Commerce-HQ`

The official repository has a control branch:

`hiveai-control-plane`

Read these files from that branch first:

- `docs/hiveai/README.md`
- `docs/hiveai/audits/2026-08-24_REPOSITORY_IDENTITY_AUDIT.md`
- `docs/hiveai/codex-logs/README.md`
- this prompt

## 1. Start the durable Codex log immediately

Create or continue, inside the LOCAL working repository:

`docs/hiveai/codex-logs/M01_5_REPOSITORY_IDENTITY_CODEX_LOG.md`

Create it before making repository changes.

Record chronologically:
- timestamps,
- commands,
- outputs/results,
- files inspected,
- decisions,
- failures,
- fixes,
- git state,
- commits,
- push status.

Never erase earlier failures after they are resolved.
Never put secrets/tokens into the log.

## 2. Preserve everything before repair

Inspect and record:

`git rev-parse --show-toplevel`
`git status --short`
`git branch --show-current`
`git rev-parse HEAD`
`git remote -v`
`git log --graph --decorate --oneline -30`
`git tag --list`
`git worktree list`

Explicitly preserve the previously reported user-owned changes:
- `package.json`
- `package-lock.json`
- `start-demo.bat`
- `task.md`

Do not discard, reset, overwrite, or silently stash them.

Also preserve M00/M01 commits, including:
- `f0ea451`
- `fedc44b388d5bf3fed9e6bfd4f288de8e0815118`

if those commits exist locally.

## 3. Do NOT replace origin immediately

The local origin was previously reported as:

`https://github.com/iamlukethedev/Claw3D.git`

Do not push to it.

Add the intended official repository temporarily as a SECOND remote named:

`hiveai-official`

pointing to:

`https://github.com/Sekiph82/AI-Commerce-HQ.git`

Then fetch it normally, without force and without altering local branches.

## 4. Prove repository identity

Compare the local repository against `hiveai-official/main`.

At minimum determine:

- whether `git merge-base HEAD hiveai-official/main` exists,
- whether histories are related,
- whether the official baseline commit `2ab25ef17ae4d2ee2d2f123364277e252ce144f4` exists locally,
- whether the local historical tree contained the official AI-Commerce-HQ files,
- whether the local M00/M01 work was actually based on Claw3D rather than AI-Commerce-HQ.

Compare representative official paths such as:
- `src/App.tsx`
- `src-tauri/`
- `backend/main.py`
- `backend/agents/base_agent.py`
- `backend/orchestrator/`

with the local pre-M01 history where possible.

Document factual evidence in the Codex log.

## 5. Choose one safe resolution path

### Path A — histories are genuinely related

If ancestry proves the local repo belongs to the official AI-Commerce-HQ history, document exactly how, then safely correct the remote without rewriting history.

### Path B — local repo is a different Claw3D-derived repository

If evidence shows the local M00/M01 work was based on a different repository, DO NOT merge unrelated histories and DO NOT force-push it over `Sekiph82/AI-Commerce-HQ`.

Instead:

1. Preserve the current local H!veAI work in a local backup branch such as:
   `backup/claw3d-hiveai-m00-m01`
2. Preserve all uncommitted user changes.
3. Clone or create a clean worktree/check-out of the official `Sekiph82/AI-Commerce-HQ` repository in a NEW sibling directory.
4. Do not delete the current local folder.
5. Reapply only the H!veAI foundation documents and genuinely reusable M01 decisions/code after explicit comparison.
6. Do not blindly copy Claw3D product code into the official repository.
7. Keep a migration ledger of every file intentionally transferred.

### Path C — ambiguous

If identity cannot be proven safely, stop destructive/migration work, leave both repos untouched, and report the evidence needed to decide.

## 6. Establish the durable repo logging convention

Whichever repository becomes the authoritative H!veAI development checkout must contain:

- `docs/hiveai/README.md`
- `docs/hiveai/prompts/`
- `docs/hiveai/audits/`
- `docs/hiveai/codex-logs/`

Copy these canonical files from the official `hiveai-control-plane` branch into the authoritative H!veAI development branch after repository identity is resolved.

From this point forward:

- ChatGPT prompts live in `docs/hiveai/prompts/`
- ChatGPT audits live in `docs/hiveai/audits/`
- Codex milestone logs live in `docs/hiveai/codex-logs/`

The Codex log is committed with each milestone.

## 7. Do not start M02

This milestone ends after:

- repository identity is proven or safely isolated,
- the correct authoritative working checkout is established,
- M00/M01 work is preserved,
- user changes are preserved,
- AI/Codex logging directories are installed in the authoritative branch,
- remote configuration is safe,
- no incorrect remote has received a push.

## 8. Validation

Before finishing run and log:

`git status`
`git branch --show-current`
`git remote -v`
`git log -10 --oneline --decorate`
`git worktree list`

If a new authoritative checkout was created, run the safe baseline build/test commands appropriate to that repository and record results.

## 9. Commit

If a safe authoritative H!veAI branch has been established and only documentation/control-plane changes are ready, create a focused commit such as:

`chore(hiveai): repair repository identity and establish versioned agent logs`

Do not force push.

Do not push to `iamlukethedev/Claw3D`.

Push only if the destination is unquestionably `Sekiph82/AI-Commerce-HQ` and history is safe.

## 10. Final report

Return exactly:

1. M01.5 RESULT
2. LOCAL REPOSITORY IDENTITY
3. OFFICIAL REPOSITORY IDENTITY
4. MERGE-BASE / HISTORY RESULT
5. CHOSEN RESOLUTION PATH
6. PRESERVED M00/M01 COMMITS
7. PRESERVED USER CHANGES
8. AUTHORITATIVE H!VEAI LOCAL PATH
9. AUTHORITATIVE BRANCH
10. REMOTE CONFIGURATION
11. CODEX LOG PATH
12. FILES TRANSFERRED OR CREATED
13. BUILD / TEST RESULT
14. PUSH STATUS
15. BLOCKERS
16. EXACT NEXT MILESTONE

The next milestone is M02 — H!veAI UI Shell and Design System.

DO NOT START M02.
