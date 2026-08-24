# M06 — H!veAI Local Git Engine

You are continuing H!veAI development after independent M05 audit approval.

Do NOT start M07.

## Mandatory fetch-before-prompt sync

Before reading milestone prompt files:

```powershell
git fetch origin H!veAI
```

Then compare:

```powershell
git rev-list --left-right --count HEAD...origin/H!veAI
```

If local HEAD is behind `origin/H!veAI` and there are no conflicting local tracked changes:

```powershell
git merge --ff-only origin/H!veAI
```

Then read the authoritative audit and milestone prompt from the updated local checkout.

Never assume missing local prompt/audit files are absent from GitHub before fetching.

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

## Canonical UI Assets

Canonical UI reference assets are located at:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Use the assets in this folder as authoritative visual references:

- dashboard reference image
- Akilta logo
- H!veAI logo

Do not redesign these assets unless explicitly instructed.

Use the dashboard reference to reproduce the layout, spacing, visual hierarchy,
panels, cards, navigation, right-side assistant/status columns, typography density,
and overall dark visual language as closely as practical whenever M06 adds or
changes visible UI.

Use the H!veAI logo in product branding.
Use the Akilta logo in the footer with:

`Built with ♥ for maximum productivity by Akilta`

This section is mandatory in addition to the permanent Canonical UI Assets rule
in `H!veAI/AGENTS.md`.

## Read first

Read completely before coding:

- `H!veAI/AGENTS.md`
- `H!veAI/CONSTITUTION.md`
- `H!veAI/ARCHITECTURE.md`
- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/docs/H!veAI/audits/M05_PROJECT_REGISTRY_AUDIT.md`
- `H!veAI/docs/H!veAI/codex-logs/README.md`
- historical M00-M05 Codex logs
- this prompt

## Repository preflight

Run and log:

- `git rev-parse --show-toplevel`
- `git branch --show-current`
- `git rev-parse HEAD`
- `git remote -v`
- `git status --short`
- `git stash list`

Stop without modifying files if Git root, branch, or origin is wrong.

Preserve unchanged:

- pre-M00 stash
- untracked parent `start-demo.bat`
- untracked parent `task.md`
- legacy parent application code
- historical M00-M05 logs

## Durable M06 Codex log

Create a NEW log before implementation:

`H!veAI/docs/H!veAI/codex-logs/M06_LOCAL_GIT_ENGINE_CODEX_LOG.md`

Record chronologically:

- sync/preflight
- architecture decisions
- Git library/command strategy
- permission boundaries
- commands and relevant outputs
- failures and corrections
- tests
- Windows smoke
- git state
- commit/push
- GitHub verification

Never rewrite historical logs.
Never record secrets, tokens, credential values, or private key contents.

## M06 objective

Build the H!veAI Local Git Engine used by later task intelligence, agent,
audit, GitHub and workflow milestones.

M06 must support safe repository inspection first and narrowly controlled Git
mutation interfaces second.

Do NOT implement M07 filesystem watcher.
Do NOT implement task-file discovery/parser logic.
Do NOT implement Codex/Claude agents.
Do NOT implement GitHub API integration.
Do NOT expose an unrestricted shell.

## Step 1 — inspect M05 Project Registry boundary

Use registered repository metadata from the M05 Project Registry.

Git Engine operations must resolve a project/repository through the registry.
Do not accept arbitrary frontend filesystem paths for normal product operations.

Missing/unavailable registered paths must return a structured safe error.

## Step 2 — technology choice

Choose and document a Rust-owned approach appropriate for Windows local Git.

Acceptable examples:

- `git2` for read operations plus carefully scoped CLI fallback when necessary
- narrowly executed `git` CLI commands through Rust with explicit arguments,
  fixed working directory, timeout, output limits and no shell interpolation

Do not invoke through `cmd /C`, PowerShell command strings, `sh -c`, or another
generic shell.

Document why the selected approach is appropriate for:

- compatibility with ordinary user repositories
- worktrees
- remotes
- ahead/behind
- diff/status fidelity
- later Codex/Claude worktree integration

## Step 3 — read-only Git snapshot model

Create typed Rust models for at least:

- repository/project ID
- repository path identifier (do not unnecessarily expose secrets)
- current branch / detached HEAD
- HEAD SHA
- staged files
- unstaged files
- untracked files
- conflicted files
- ahead count
- behind count
- upstream
- remotes with sanitized URLs
- recent commits
- worktrees
- repository cleanliness/health
- snapshot timestamp

Do not infer clean state merely from absence of one status category.

## Step 4 — Git inspection APIs

Provide narrowly allowlisted read IPC such as:

- repository status/snapshot
- recent commits
- diff summary / bounded diff
- remotes
- worktrees

Use registered project/repository IDs as primary product inputs.

No arbitrary command string parameter.
No arbitrary executable path parameter.

## Step 5 — status semantics

Correctly distinguish:

- staged modified/added/deleted/renamed
- unstaged modified/deleted
- untracked
- conflicts
- detached HEAD
- unborn/new repository when practical
- missing repository
- non-Git project

Define deterministic status mapping documented for frontend/domain services.

## Step 6 — ahead / behind

Determine ahead/behind relative to configured upstream when available.

If there is no upstream, return explicit `UNAVAILABLE`/null semantics rather
than fabricated zero counts.

Do not automatically fetch remotes merely to compute status in M06.
Network Git operations must not happen implicitly.

## Step 7 — diff safety

Support bounded text diff inspection suitable for later GPT audits.

Requirements:

- staged and/or working-tree scope explicitly identified
- maximum byte/line output limits
- binary files represented as metadata, not raw bytes
- truncation indicated clearly
- paths treated as data, not commands
- no secrets intentionally harvested

M06 does not need semantic code review.

## Step 8 — recent commit inspection

Provide recent commit metadata:

- SHA
- subject
- author display name/email only if already in commit metadata
- authored/committed timestamp
- parent count

Keep result count bounded.

Do not send anything externally.

## Step 9 — worktree inspection

Inspect existing Git worktrees and expose:

- path
- branch when available
- HEAD
- locked/prunable indicators where possible

Do not create worktrees automatically in M06 unless explicitly required by the
narrow mutation API tests below.

## Step 10 — mutation boundary

Define a separate write/mutation module/API from read-only inspection.

Permitted M06 mutation interfaces may include only carefully scoped operations
needed by future milestones, for example:

- create branch
- stage explicit registered-repository relative paths
- create commit from already staged changes
- push explicit branch to an existing sanitized remote

However:

- write operations MUST be permission-gated in architecture/code
- normal M06 UI must not silently execute them
- no force push
- no hard reset
- no checkout that discards tracked changes
- no stash pop/drop
- no arbitrary Git subcommand input
- no arbitrary shell

If a mature permission engine is scheduled for a later milestone, implement a
safe denial/default-disabled boundary now and document how later permission UI
will activate it. Do not pretend writes are approved.

## Step 11 — mutation test isolation

ALL mutation tests must use temporary Git repositories created specifically for tests.

Never use:

- the H!veAI development repository
- registered real user project repositories
- the user's preserved stash

Mutation tests should verify at least:

- branch creation without data loss
- staging explicit paths only
- commit behavior in temp repo
- rejection of dangerous/unsupported operations
- no shell interpolation

Remote push tests, if implemented, must use a local temporary bare repository,
not GitHub or another network remote.

## Step 12 — Git snapshot persistence

Use the M04 `git_snapshots` table coherently.

Persist snapshots only when explicitly requested/appropriate by M06 APIs.
Do not create an uncontrolled polling loop.

Document what fields are persisted vs returned live.

Do not store raw full diffs in SQLite unless clearly justified.

## Step 13 — frontend integration

Add a restrained Git status surface to the existing Project Cockpit / project UI.

Show useful states such as:

- branch
- HEAD short SHA
- clean/dirty
- staged/unstaged/untracked/conflict counts
- ahead/behind or unavailable
- recent commits summary

Read-only actions such as refresh/open diff are acceptable.

Do not expose active mutation buttons unless they are clearly disabled or routed
through the safe permission boundary.

Follow the Canonical UI Assets section and existing dashboard visual language.
Do not redesign the application.

## Step 14 — security

- sanitize remote URLs containing embedded credentials
- never log tokens/passwords/private-key material
- no shell interpolation
- fixed/bounded child-process execution if Git CLI is used
- timeouts
- stdout/stderr size limits
- path validation
- no arbitrary executable selection
- no implicit network fetch/push

Review Tauri capabilities carefully. Add only narrow command permissions.

## Step 15 — tests

Add meaningful automated coverage for at least:

1. clean repository
2. staged change
3. unstaged change
4. untracked file
5. deleted/renamed file
6. conflict state where practical
7. branch + HEAD detection
8. detached HEAD
9. upstream ahead/behind
10. no-upstream explicit unavailable state
11. sanitized remote URL
12. bounded diff truncation
13. binary diff handling
14. recent commits bounded
15. worktree inspection
16. missing project/repository handling
17. non-Git project behavior
18. explicit-path mutation isolation in temp repo
19. dangerous mutation rejection
20. no network side effects during ordinary status inspection

Preserve all M03-M05 tests.

## Step 16 — documentation

Create:

`H!veAI/docs/migration/M06_LOCAL_GIT_ENGINE.md`

Document:

- technology choice
- read/write separation
- snapshot model
- status semantics
- diff limits
- ahead/behind behavior
- remote sanitization
- mutation permission boundary
- temp-repo test strategy
- SQLite snapshot usage
- Windows limitations
- future M07/M13/M16 integration boundaries

## Step 17 — verification

Frontend:

- typecheck
- tests
- production build

Rust/Tauri:

- format check
- cargo check
- cargo test
- cargo build

M06 Git-specific verification:

- multiple isolated temp repositories
- status matrix
- upstream ahead/behind
- local bare remote if push behavior is tested
- worktree fixture
- bounded diff fixture

Windows bounded smoke:

- launch H!veAI
- verify Project Registry still works
- verify Git status surface for a safe test/registered fixture if available
- verify M01 runtime IPC
- verify M04 database IPC
- verify no legacy port 8765 listener/backend process
- clean shutdown

Do not mutate a real user repository merely to satisfy smoke testing.

## Step 18 — TASKS.md

Update only M06 items.
Use `[x]` only when actually verified.
Do not mark M07 or later complete.

## Step 19 — containment review

Before commit verify:

- no parent source/package changes
- no registered external project folder mutation from tests
- no secret-bearing remote URL staged/logged
- no temp repositories staged
- no production DB staged
- no generated artifacts staged
- M00-M05 logs unchanged
- user stash/files preserved

Run `git diff --check` and staged-diff review.

## Commit and push

If M06 is genuinely complete, create a focused commit:

`feat(H!veAI): add local Git engine`

The commit MUST include:

`H!veAI/docs/H!veAI/codex-logs/M06_LOCAL_GIT_ENGINE_CODEX_LOG.md`

Push normally to `origin/H!veAI`.
Do not force push.

Verify M00-M06 logs separately on GitHub after publication.
A small log-only follow-up commit is acceptable for final remote verification.

## M06 acceptance criteria

M06 is complete only if:

1. Typed local Git inspection engine exists.
2. Registry IDs resolve normal product Git operations.
3. Branch/HEAD/status are accurate.
4. staged/unstaged/untracked/conflict states are distinguishable.
5. ahead/behind has explicit unavailable semantics without upstream.
6. status inspection performs no implicit remote fetch.
7. remotes are sanitized.
8. diff output is bounded and binary-safe.
9. recent commit inspection is bounded.
10. worktree inspection exists.
11. read and mutation boundaries are separate.
12. no unrestricted shell/subcommand interface exists.
13. dangerous Git writes are unavailable/rejected.
14. mutation tests use only temp repositories.
15. Project Cockpit has restrained read-only Git status integration.
16. existing Project Registry remains functional.
17. existing runtime/database regressions pass.
18. no legacy sidecar starts.
19. parent application remains untouched.
20. historical M00-M05 logs remain unchanged.
21. M06 log is committed/pushed/verified.
22. M06 documentation exists.
23. TASKS reflects verified M06 state only.
24. Canonical UI Assets governance remains intact.

## Final response format

Return exactly:

1. M06 RESULT
2. FETCH-BEFORE-PROMPT SYNC
3. VERIFIED GIT ROOT
4. VERIFIED H!veAI ROOT
5. BRANCH / HEAD
6. GIT ENGINE TECHNOLOGY
7. READ API SUMMARY
8. STATUS SEMANTICS
9. AHEAD / BEHIND BEHAVIOR
10. DIFF SAFETY / LIMITS
11. REMOTE SANITIZATION
12. RECENT COMMITS
13. WORKTREE SUPPORT
14. MUTATION BOUNDARY
15. PERMISSION / SAFETY STATUS
16. SNAPSHOT PERSISTENCE
17. PROJECT COCKPIT INTEGRATION
18. CANONICAL UI ASSET STATUS
19. GIT-SPECIFIC TEST RESULTS
20. FRONTEND RESULTS
21. RUST / TAURI RESULTS
22. WINDOWS SMOKE
23. LEGACY CONTAINMENT
24. FILES ADDED
25. FILES MODIFIED
26. PARENT FILES MODIFIED
27. CODEX LOG LOCAL PATH
28. CODEX LOG GITHUB PATH / VERIFICATION
29. PRESERVED HISTORICAL LOG STATUS
30. PRESERVED STASH / USER FILE STATUS
31. COMMIT / PUSH STATUS
32. BLOCKERS / OPEN DECISIONS
33. EXACT NEXT MILESTONE

The exact next milestone is:

`M07 — Filesystem Watcher and Snapshots`

IMPORTANT GOVERNANCE RULE:

Do NOT create, invent, recommend, or claim the existence of an M07 Codex prompt file.
Do NOT include a `RECOMMENDED NEXT CODEX PROMPT` section.
The next prompt is authored only by ChatGPT after independent M06 audit approval.

Do NOT start M07.
Stop after M06.
