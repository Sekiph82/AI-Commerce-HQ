# M05 — H!veAI Project Registry

You are continuing H!veAI development after independent M04 audit approval.

Do NOT start M06.

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

Use the dashboard reference to reproduce the layout, spacing, visual hierarchy, panels, cards, navigation, right-side assistant/status columns, typography density, and overall dark visual language as closely as practical whenever M05 touches UI surfaces.

Use the H!veAI logo in product branding.

Use the Akilta logo in the footer with exactly:

`Built with ♥ for maximum productivity by Akilta`

The top-left product brand must say `H!veAI`, not `AI Command Center`.

This milestone must not invent a competing visual direction. Existing M02/M03/M04 UI must be adapted toward the canonical dashboard reference only where M05 UI work requires changes.

## Read first

Read completely before coding:

- `H!veAI/AGENTS.md`
- `H!veAI/CONSTITUTION.md`
- `H!veAI/ARCHITECTURE.md`
- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/docs/H!veAI/README.md`
- `H!veAI/docs/H!veAI/audits/M04_SQLITE_AND_VERSIONED_MIGRATIONS_AUDIT.md`
- `H!veAI/docs/H!veAI/codex-logs/README.md`
- historical M00-M04 Codex logs
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
- historical M00-M04 logs

## Durable M05 Codex log

Create a NEW log before implementation:

`H!veAI/docs/H!veAI/codex-logs/M05_PROJECT_REGISTRY_CODEX_LOG.md`

Record chronologically:

- sync/preflight
- design decisions
- project-path safety decisions
- metadata detection
- DB reads/writes
- commands
- failures/fixes
- tests
- UI asset usage
- git state
- commit/push
- GitHub verification

Never rewrite historical logs.
Never record secrets or sensitive values.

## M05 objective

Build the first real H!veAI Project Registry on top of the M04 SQLite layer.

The registry must let the user explicitly add an existing local project folder without mutating that project.

M05 owns:

- project registration
- project identity
- canonical/local path storage
- Git repository detection
- remote/default branch/GitHub identity detection
- priority
- preferred builder
- preferred auditor
- task-source policy settings
- archive/remove-from-registry operations
- path repair when a registered project moves
- search/sort/filter
- Project Registry UI integration

M05 does NOT own:

- Git mutation
- branch creation
- commit/push
- filesystem watching
- task parsing
- Codex/Claude execution
- GPT audit execution
- automatic project discovery across the user's machine

Those belong to later milestones.

## Step 1 — inspect M04 persistence and current UI

Inspect the actual M04 DB schema and persistence APIs before adding registry services.

Use the existing `projects` and `repositories` tables where appropriate.

Do not introduce a second datastore.

Inspect current `/projects` and Project Cockpit UI and align M05 integration with the Canonical UI Assets section.

## Step 2 — explicit user-driven registration only

Registration must begin only from an explicit user action such as `Add Project`.

Do NOT recursively scan arbitrary disks, Desktop, Documents, home directories, or known development folders for projects.

The user selects or provides a folder path.

Registration is read-only toward the selected project folder.

M05 must NOT:

- edit project files
- edit `.git/config`
- change remotes
- checkout branches
- create worktrees
- run package installs
- run project code
- commit/push
- create `.hiveai/` inside the managed project

## Step 3 — project identity model

Persist at minimum:

- stable project ID
- display name
- original selected path
- normalized/canonical path
- registration timestamp
- last validated timestamp
- status: ACTIVE / MISSING / ARCHIVED as appropriate
- priority
- preferred builder
- preferred auditor
- task-source policy

Use stable IDs independent of filesystem path.

Do not use path as the primary key.

## Step 4 — Git metadata detection

For a selected folder, safely detect read-only:

- whether it is a Git repository
- repository root
- current branch if available
- HEAD SHA if available
- remotes
- preferred origin URL
- default branch where determinable without mutation
- GitHub owner/repo if the remote matches a supported GitHub URL form

Detection must tolerate:

- non-Git folders
- detached HEAD
- no remote
- multiple remotes
- missing origin
- malformed remote URLs
- inaccessible/moved folders

Do not mutate the repository to improve detection.

## Step 5 — path safety and normalization

Implement Windows-safe canonical path handling.

Requirements:

- preserve a user-display path
- normalize for duplicate detection
- handle casing and separators safely
- avoid creating false duplicates where possible
- reject obvious empty/invalid selections
- do not resolve into unrelated directories through unsafe path concatenation

Define duplicate-registration behavior clearly.

## Step 6 — registry Rust service

Create a clean Rust project-registry domain/service layer under the child Tauri source.

Suggested organization:

- `src-tauri/src/projects/mod.rs`
- `src-tauri/src/projects/registry.rs`
- `src-tauri/src/projects/detection.rs`

Adapt as needed but keep registry logic out of `lib.rs`.

## Step 7 — narrow Project Registry IPC

Expose narrow commands such as:

- `hiveai_projects_list`
- `hiveai_project_register`
- `hiveai_project_get`
- `hiveai_project_update_settings`
- `hiveai_project_archive`
- `hiveai_project_remove_from_registry`
- `hiveai_project_repair_path`

Exact names may vary, but the API must remain typed and bounded.

Do NOT expose arbitrary filesystem browsing, arbitrary shell execution, arbitrary SQL, or generic Git command execution.

If folder selection requires a Tauri dialog plugin, add the narrowest capability necessary and document it.

## Step 8 — persistence behavior

Registration must write project and repository metadata through the M04 persistence layer.

Use transactions for logically grouped writes.

Do not persist secrets or credential-bearing remote URLs with embedded tokens/passwords. Sanitize before storage/logging.

Define archive vs remove semantics:

- Archive keeps historical registry metadata but hides from active default views.
- Remove-from-registry deletes H!veAI registry records as designed but NEVER deletes the user's project folder or repository.

## Step 9 — path repair

Support repairing a project path after a folder moves.

Repair must:

- require explicit user action
- validate the new folder
- detect identity/remote metadata read-only
- show enough metadata to avoid accidental reassignment
- update registry path only after validation

Do not move filesystem content.

## Step 10 — Project Registry UI

Implement the real `/projects` registry surface using the canonical dashboard reference.

At minimum support:

- Add Project
- project list/cards/table matching the canonical visual language
- search
- sort
- filters
- priority display/control
- project status
- Git/non-Git indicator
- branch/remote summary where available
- preferred builder/auditor
- archived/missing states
- open Project Cockpit
- archive
- remove-from-registry
- repair path

Do not fake live data. Registered project information must come from the M05 persistence layer.

## Step 11 — Project Cockpit integration

For a real registered project, Project Cockpit should use registry identity/metadata where available.

Do not implement M06 Git Engine features beyond the read-only metadata captured at registration time.

Clearly distinguish cached registration metadata from live Git status that belongs to M06.

## Step 12 — branding/footer integration

Where the shell is touched in M05:

- use the H!veAI logo from the Canonical UI Assets directory
- ensure top-left branding says `H!veAI`
- use the Akilta logo in the footer
- render exactly: `Built with ♥ for maximum productivity by Akilta`

Do not redraw either logo.

Copy/import assets into an H!veAI-owned application assets directory only if needed for runtime packaging, preserving the source artwork without redesign.

## Step 13 — tests

Add meaningful Rust/frontend tests.

At minimum verify:

1. explicit folder registration succeeds
2. no automatic scanning occurs
3. duplicate registration is handled deterministically
4. Git repo metadata detection works on an isolated temp repo
5. non-Git folder registration works
6. missing path state is handled
7. remote URL sanitization prevents credential leakage
8. archive does not delete filesystem content
9. remove-from-registry does not delete filesystem content
10. path repair updates registry records only
11. list/search/filter behavior works
12. project UI renders real registry records
13. historical M00-M04 logs remain unchanged

Use temporary folders/repos for tests only. Never mutate real user projects during automated tests.

## Step 14 — verification

Run from the H!veAI workspace:

Frontend:

- typecheck
- tests
- production build

Rust/Tauri:

- cargo fmt check
- cargo check
- cargo test
- cargo build

M05-specific:

- register isolated temp non-Git project
- register isolated temp Git project
- detect metadata
- duplicate handling
- archive/remove safety
- repair-path safety
- persistence reload

Windows bounded smoke:

- launch H!veAI
- verify Project Registry renders
- register a disposable test folder only
- verify it persists after app restart if practical
- verify no selected project files are modified
- verify no legacy runtime starts
- verify clean shutdown

## Step 15 — migration documentation

Create:

`H!veAI/docs/migration/M05_PROJECT_REGISTRY.md`

Document:

- registration flow
- registry schema usage
- path normalization
- Git metadata detection
- duplicate policy
- archive/remove semantics
- path repair
- security/containment
- UI integration
- Canonical UI Assets usage
- M06 boundary

## Step 16 — TASKS.md

Update only M05 items.

Use `[x]` only for verified items.
Do not mark M06 or later complete.

## Step 17 — containment review

Before commit verify:

- no parent application source/package modifications
- no managed external project files modified
- no user project `.git` metadata modified
- no secrets staged
- no test temp repos staged
- no production DB staged
- no generated artifacts staged
- M00-M04 logs unchanged
- stash/user files preserved

Run `git diff --check` and review staged diff.

## Commit and push

If M05 is genuinely complete, create a focused commit:

`feat(H!veAI): add Project Registry`

The commit MUST include:

`H!veAI/docs/H!veAI/codex-logs/M05_PROJECT_REGISTRY_CODEX_LOG.md`

Push normally to `origin/H!veAI`.

Do not force push.

After push verify M00-M05 logs all exist separately under:

`H!veAI/docs/H!veAI/codex-logs/`

If needed, use a small log-only follow-up commit for final remote verification.

## M05 acceptance criteria

M05 is complete only if:

1. User can explicitly register an existing folder.
2. No automatic machine-wide project scanning occurs.
3. Registration is read-only toward managed project folders.
4. Registry data persists in the M04 SQLite layer.
5. Git/non-Git detection works safely.
6. Remote/default branch/GitHub identity metadata is detected where possible.
7. Paths are normalized safely and duplicates handled.
8. Project priority/builder/auditor/task-source settings persist.
9. Missing/moved project paths are represented safely.
10. Archive works without deleting project files.
11. Remove-from-registry works without deleting project files.
12. Path repair requires explicit action and does not move files.
13. Project Registry UI uses real persisted records.
14. UI follows Canonical UI Assets rules.
15. H!veAI branding/logo is used where shell is touched.
16. Akilta footer branding is present where shell is touched.
17. Frontend checks pass.
18. Rust/Tauri checks pass.
19. Temp-repo/path safety tests pass.
20. Legacy runtime remains disabled.
21. Parent app remains untouched.
22. M00-M04 logs remain unchanged.
23. M05 log is committed/pushed/verified.
24. M05 migration documentation exists.
25. TASKS reflects verified M05 state only.

## Final response format

Return exactly:

1. M05 RESULT
2. FETCH-BEFORE-PROMPT SYNC
3. VERIFIED GIT ROOT
4. VERIFIED H!veAI APPLICATION ROOT
5. BRANCH / HEAD
6. CANONICAL UI ASSETS STATUS
7. REGISTRY ARCHITECTURE
8. PROJECT IDENTITY MODEL
9. PATH NORMALIZATION / DUPLICATE POLICY
10. GIT METADATA DETECTION
11. REGISTRY IPC
12. PERSISTENCE INTEGRATION
13. PROJECT REGISTRY UI
14. PROJECT COCKPIT INTEGRATION
15. H!veAI / AKILTA BRANDING STATUS
16. ARCHIVE / REMOVE SAFETY
17. PATH REPAIR
18. TEST RESULTS
19. FRONTEND BUILD RESULTS
20. RUST / TAURI RESULTS
21. WINDOWS SMOKE RESULT
22. LEGACY RUNTIME CONTAINMENT
23. FILES ADDED
24. FILES MODIFIED
25. PARENT / EXTERNAL PROJECT FILES MODIFIED
26. CODEX LOG LOCAL PATH
27. CODEX LOG GITHUB PATH / VERIFICATION
28. PRESERVED HISTORICAL LOG STATUS
29. PRESERVED STASH / USER FILE STATUS
30. COMMIT / PUSH STATUS
31. BLOCKERS / OPEN DECISIONS
32. EXACT NEXT MILESTONE

The exact next milestone is:

`M06 — Local Git Engine`

IMPORTANT GOVERNANCE RULE:

Do NOT create, invent, recommend, or claim the existence of an M06 Codex prompt file.
Do NOT include a `RECOMMENDED NEXT CODEX PROMPT` section.
The next prompt is authored only by ChatGPT after independent M05 audit approval.

Do NOT start M06.
Stop after M05.
