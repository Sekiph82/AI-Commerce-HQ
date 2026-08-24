# M07 — H!veAI Filesystem Watcher and Snapshots

You are continuing H!veAI development after independent M06 audit approval.

Do NOT start M08.

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
and overall dark visual language as closely as practical whenever M07 adds or
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
- `H!veAI/docs/H!veAI/audits/M06_LOCAL_GIT_ENGINE_AUDIT.md`
- `H!veAI/docs/H!veAI/codex-logs/README.md`
- historical M00-M06 Codex logs
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
- historical M00-M06 logs

## Durable M07 Codex log

Create a NEW log before implementation:

`H!veAI/docs/H!veAI/codex-logs/M07_FILESYSTEM_WATCHER_AND_SNAPSHOTS_CODEX_LOG.md`

Record chronologically:

- fetch/sync/preflight
- watcher technology choice
- debounce/coalescing rules
- path-scope decisions
- event normalization
- snapshot refresh behavior
- failures and corrections
- tests
- Windows smoke
- git state
- commit/push
- GitHub verification

Never rewrite historical logs.
Never record secrets, tokens, private-key material, `.env` contents, or sensitive file contents.

## M07 objective

Build H!veAI's safe local filesystem watcher and project snapshot refresh layer.

M07 must detect meaningful local project changes and trigger bounded refreshes of project/Git/task-source evidence without mutating registered projects.

M07 is infrastructure for M08 Task Source Discovery and later workflow intelligence.

Do NOT implement M08 task-source discovery/parser logic.
Do NOT parse task Markdown into normalized tasks.
Do NOT implement Codex/Claude agents.
Do NOT implement GitHub API integration.
Do NOT execute user project code.
Do NOT expose unrestricted filesystem access to the frontend.

## Step 1 — inspect M05/M06 boundaries

Use registered project roots from M05 Project Registry.
Use the M06 Git Engine for Git refresh/snapshot generation where appropriate.

The watcher must never accept arbitrary frontend paths for normal product operations.

Missing or archived projects must be handled safely.

## Step 2 — watcher technology

Choose a Rust-owned watcher suitable for Windows desktop use, preferably a mature crate such as `notify`.

Document the choice and why it fits:

- recursive project watching
- Windows path semantics
- rename/create/modify/remove events
- debounce/coalescing
- restart/recovery
- multiple project roots

Do not poll the whole machine.
Do not watch outside registered project roots plus narrowly justified H!veAI-owned metadata paths.

## Step 3 — watcher manager

Create a dedicated Rust watcher manager/service, separate from `lib.rs`.

It should support at least:

- start watching a registered active project
- stop watching a project
- refresh watcher set from registry
- report watcher state/health
- recover from missing/moved project roots
- avoid duplicate watchers for the same normalized path

Use project IDs as the public product identity.

## Step 4 — scope and exclusions

Watch only meaningful paths under registered project roots.

Define exclusions for noisy/generated directories such as, where appropriate:

- `.git/objects`
- `.git/logs` unless specific refs/status metadata is needed
- `node_modules`
- `target`
- `dist`
- `build`
- `.next`
- caches/temp directories
- large generated binary/output directories

Do not blindly hardcode one stack's exclusions if repository evidence suggests otherwise. Implement a sensible default exclusion policy with bounded configurability deferred unless required.

Never read secret-bearing file contents merely because they changed.

## Step 5 — event normalization

Normalize raw watcher events into typed internal events with fields such as:

- project ID
- event ID
- event kind: CREATE / MODIFY / REMOVE / RENAME / RESCAN_REQUIRED
- path relative to project root
- old path for rename when available
- timestamp
- source: WATCHER
- category hint: GIT_METADATA / TASK_CANDIDATE / SOURCE / CONFIG / OTHER

Do not store raw absolute paths unnecessarily in frontend-facing payloads.

## Step 6 — debounce and coalescing

Implement bounded debounce/coalescing to prevent event storms.

Requirements:

- rapid repeated modifications to the same path coalesce
- rename/create/remove sequences are normalized where practical
- refresh work is rate-limited per project
- event buffers are bounded
- overflow/error produces `RESCAN_REQUIRED` rather than silently dropping correctness

Document debounce windows and limits.

## Step 7 — snapshot refresh

When a meaningful change is accepted after debounce:

- refresh registered-path availability
- refresh M06 Git snapshot when Git-relevant paths changed
- record an evidence timestamp
- persist only bounded structured snapshot metadata

Do NOT persist raw file contents.
Do NOT persist full diffs automatically.
Do NOT create uncontrolled polling loops.

Use the existing M04/M06 SQLite structures coherently; add a versioned migration only if genuinely required.

## Step 8 — project snapshot model

Create a typed project snapshot/evidence model suitable for later M08/M09 use.

At minimum consider:

- project ID
- project availability
- Git snapshot ID / timestamp when available
- last filesystem event timestamp
- last watcher refresh timestamp
- evidence generation timestamp
- changed-path count since prior snapshot
- rescan-required flag
- watcher health

Do not over-model task semantics yet.

## Step 9 — moved/missing repositories

If a watched project root disappears:

- mark watcher/project availability appropriately
- stop or suspend the watcher safely
- do not delete registry data
- do not search the filesystem automatically for a replacement

If the user later repairs the project path through the M05 registry, watcher refresh must be able to attach to the repaired root.

## Step 10 — startup integration

At Tauri startup, after database/project registry are healthy:

- initialize watcher manager
- load active registered projects
- establish safe watchers
- expose watcher health

Startup must remain bounded. A single bad/missing project must not crash all H!veAI startup if it can be represented as a degraded project watcher state.

Do not start legacy sidecars or project processes.

## Step 11 — IPC boundary

Expose narrow read/control IPC only, such as:

- watcher status summary
- project watcher status
- explicit refresh watcher set
- explicit project rescan/refresh request

Do NOT expose arbitrary filesystem watch paths.
Do NOT expose arbitrary file-read IPC.
Do NOT expose generic OS event streams without project scoping.

If frontend event streaming is added, use typed H!veAI events and bounded payloads.

## Step 12 — frontend integration

Add restrained watcher/snapshot health to the existing Project Cockpit and/or system status areas.

Useful UI examples:

- Watching / Paused / Missing / Degraded
- last change time
- last snapshot time
- changed-path count
- rescan required
- safe Refresh action

Follow the Canonical UI Assets section and existing dashboard visual language.
Do not redesign the application.

## Step 13 — security and privacy

- watcher paths come from registered projects only
- no arbitrary frontend path injection
- no automatic external network action
- no user project execution
- no package install
- no file mutation
- no secret-content harvesting
- do not log file contents
- sanitize errors before frontend/log exposure
- bound event queues and payload sizes

Review Tauri capabilities and add only narrow command permissions.

## Step 14 — large-repository protections

Add reasonable protections for large/noisy repositories:

- excluded generated directories
- bounded queue
- debounce/coalescing
- per-project refresh throttling
- overflow => rescan-required semantics
- no full-tree hash on every event

Document limitations on very large repos/network drives/symlink-heavy trees.

## Step 15 — tests

Add meaningful automated coverage for at least:

1. watch create event
2. watch modify event
3. watch remove event
4. rename where supported
5. repeated rapid modification coalescing
6. excluded directory noise ignored
7. project-scoped relative path normalization
8. duplicate watcher prevention
9. missing project transition
10. repaired path reattachment or refresh path
11. bounded queue/overflow => rescan-required
12. Git-relevant event triggers bounded Git refresh
13. non-Git project watcher behavior
14. no automatic project mutation
15. no arbitrary path IPC
16. startup with one missing project remains safely degraded
17. clean shutdown stops watcher resources

Preserve M03-M06 tests.

All filesystem tests must use temporary directories and repositories.
Do not mutate registered real user projects for tests.

## Step 16 — documentation

Create:

`H!veAI/docs/migration/M07_FILESYSTEM_WATCHER_AND_SNAPSHOTS.md`

Document:

- watcher crate/technology
- path scoping
- exclusions
- debounce/coalescing
- event model
- snapshot refresh rules
- persistence behavior
- missing/moved project behavior
- queue limits/overflow semantics
- Windows limitations
- privacy/security boundary
- future M08/M09 integration

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

Watcher-specific:

- isolated temp directory watcher tests
- event storm/debounce test
- missing root test
- watcher shutdown test
- temp Git repository integration test

Windows bounded smoke:

- launch H!veAI
- verify existing Project Registry works
- verify existing Git status surface works
- verify watcher manager initializes
- verify a safe temp/fixture registered project can produce a watcher event if practical
- verify no legacy backend process / port 8765
- verify clean shutdown leaves no watcher process/resource leak

Do not modify a real user project merely to satisfy smoke testing.

## Step 18 — M06 carry-forward

Preserve the M06 Git mutation default-denied boundary.
M07 must not activate Git mutation UI or authorization.

The internal boolean mutation gate noted in the M06 audit remains non-blocking technical debt for the future permission-engine milestone; do not broaden it in M07 unless necessary for correctness.

## Step 19 — TASKS.md

Update only M07 items.
Use `[x]` only when actually verified.
Do not mark M08 or later complete.

## Step 20 — containment review

Before commit verify:

- no parent application source/package changes
- no registered external project mutation from tests
- no watched real project files staged
- no production DB staged
- no temp repositories/directories staged
- no file-content dumps/logs staged
- no secrets staged
- M00-M06 logs unchanged
- user stash/files preserved

Run `git diff --check` and staged diff review.

## Commit and push

If M07 is genuinely complete, create a focused commit:

`feat(H!veAI): add filesystem watcher and snapshots`

The commit MUST include:

`H!veAI/docs/H!veAI/codex-logs/M07_FILESYSTEM_WATCHER_AND_SNAPSHOTS_CODEX_LOG.md`

Push normally to `origin/H!veAI`.
Do not force push.

Verify M00-M07 logs separately on GitHub after publication.
A small log-only follow-up commit is acceptable for final remote verification.

## M07 acceptance criteria

M07 is complete only if:

1. Rust-owned filesystem watcher manager exists.
2. Watchers are scoped to registered project roots.
3. No arbitrary frontend watch path is exposed.
4. Event normalization exists.
5. Debounce/coalescing exists.
6. Queue/buffer behavior is bounded.
7. Overflow/error produces safe rescan-required semantics.
8. Noisy generated directories are reasonably excluded.
9. Missing/moved projects are handled safely without registry deletion.
10. Repaired project paths can be reattached/refreshed.
11. Project snapshot/evidence timestamps are maintained.
12. Git-relevant events can trigger bounded M06 Git refresh.
13. Raw file contents/full diffs are not automatically persisted.
14. Watcher startup/shutdown lifecycle is controlled.
15. Frontend shows restrained watcher/snapshot health.
16. Existing Project Registry remains functional.
17. Existing Git Engine remains functional and default-denied for writes.
18. Existing runtime/database regressions pass.
19. No legacy sidecar starts.
20. Parent application remains untouched.
21. Historical M00-M06 logs remain unchanged.
22. M07 log is committed/pushed/verified.
23. M07 documentation exists.
24. TASKS reflects verified M07 state only.
25. Canonical UI Assets governance remains intact.

## Final response format

Return exactly:

1. M07 RESULT
2. FETCH-BEFORE-PROMPT SYNC
3. VERIFIED GIT ROOT
4. VERIFIED H!veAI ROOT
5. BRANCH / HEAD
6. WATCHER TECHNOLOGY
7. WATCHER MANAGER SUMMARY
8. WATCH SCOPE / EXCLUSIONS
9. EVENT MODEL
10. DEBOUNCE / COALESCING
11. QUEUE / OVERFLOW BEHAVIOR
12. PROJECT SNAPSHOT MODEL
13. GIT REFRESH INTEGRATION
14. MISSING / MOVED PROJECT BEHAVIOR
15. IPC BOUNDARY
16. FRONTEND INTEGRATION
17. CANONICAL UI ASSET STATUS
18. SECURITY / PRIVACY STATUS
19. LARGE-REPO PROTECTIONS
20. WATCHER-SPECIFIC TEST RESULTS
21. FRONTEND RESULTS
22. RUST / TAURI RESULTS
23. WINDOWS SMOKE
24. M06 MUTATION-BOUNDARY REGRESSION
25. LEGACY CONTAINMENT
26. FILES ADDED
27. FILES MODIFIED
28. PARENT FILES MODIFIED
29. CODEX LOG LOCAL PATH
30. CODEX LOG GITHUB PATH / VERIFICATION
31. PRESERVED HISTORICAL LOG STATUS
32. PRESERVED STASH / USER FILE STATUS
33. COMMIT / PUSH STATUS
34. BLOCKERS / OPEN DECISIONS
35. EXACT NEXT MILESTONE

The exact next milestone is:

`M08 — Task Source Discovery`

IMPORTANT GOVERNANCE RULE:

Do NOT create, invent, recommend, or claim the existence of an M08 Codex prompt file.
Do NOT include a `RECOMMENDED NEXT CODEX PROMPT` section.
The next prompt is authored only by ChatGPT after independent M07 audit approval.

Do NOT start M08.
Stop after M07.
