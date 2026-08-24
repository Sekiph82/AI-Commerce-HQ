# M04 — H!veAI SQLite and Versioned Migrations

You are continuing H!veAI development after independent M03 audit approval.

Do NOT start M05.

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

## Read first

Read completely before coding:

- `H!veAI/AGENTS.md`
- `H!veAI/CONSTITUTION.md`
- `H!veAI/ARCHITECTURE.md`
- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/docs/H!veAI/README.md`
- `H!veAI/docs/H!veAI/audits/M03_RUNTIME_ARCHITECTURE_REFACTOR_AUDIT.md`
- `H!veAI/docs/H!veAI/codex-logs/README.md`
- historical M00/M01/M02/M03 Codex logs
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
- historical M00/M01/M02/M03 logs

## Durable M04 Codex log

Create a NEW log before implementation:

`H!veAI/docs/H!veAI/codex-logs/M04_SQLITE_AND_VERSIONED_MIGRATIONS_CODEX_LOG.md`

Record chronologically:

- sync/preflight
- design decisions
- schema choices
- migrations
- commands
- failures/fixes
- tests
- security decisions
- git state
- commit/push
- GitHub verification

Never rewrite historical logs.
Never record secrets or sensitive values.

## M04 objective

Create the first production-grade H!veAI-owned local persistence layer using SQLite and explicit versioned migrations.

M04 is persistence infrastructure only.

Do NOT implement M05 Project Registry product workflows yet.
Do NOT scan/register real user repositories automatically.
Do NOT start legacy Python/FastAPI runtime.
Do NOT reuse the legacy parent's ad-hoc SQLite migration approach.

Preserve the M03 architecture decision:

- Rust-native H!veAI core
- no always-on Python sidecar
- legacy commerce runtime disabled

## Step 1 — inspect persistence requirements

Inspect the authoritative architecture and current Rust app.

Map the target persistence entities listed in architecture, including:

- projects
- repositories
- project_sources
- git_snapshots
- tasks
- task_dependencies
- task_sources
- task_events
- prompts
- prompt_versions
- agent_sessions
- agent_events
- agent_tool_calls
- permission_requests
- audits
- audit_findings
- test_runs
- alerts
- decisions
- github_sync_state
- settings
- migrations

For M04, implement a coherent schema foundation for these tables, even if later milestones do not yet populate most of them.

Do not over-model speculative fields. Prefer stable IDs, timestamps, status fields, JSON metadata only where justified, foreign keys, and explicit indexes.

## Step 2 — SQLite technology choice

Use a Rust-owned SQLite solution compatible with Tauri 2 and local-first desktop operation.

Choose and document one approach, preferably a mature Rust crate such as `rusqlite` or `sqlx` with SQLite support.

Selection criteria:

- deterministic local migrations
- transactions
- foreign-key support
- testability with temporary databases
- no external database service
- no Python dependency
- minimal runtime complexity

Do not add a second persistence stack unless strictly necessary.

## Step 3 — application data path

The production database must live in an H!veAI-owned application data directory, not inside the Git repository.

Use the Tauri app-data path associated with identifier:

`ai.hiveai.desktop`

Define a stable database filename, for example:

`hiveai.db`

Document exact path behavior on Windows.

Tests must use temporary isolated databases and must never touch the user's production DB.

## Step 4 — migration framework

Create a versioned migration mechanism with:

- monotonically increasing migration version
- human-readable migration name
- deterministic ordered execution
- migration history table
- applied timestamp
- transaction per migration where safe
- rollback-on-failure semantics for a failed migration transaction
- clear startup failure when migration fails
- no swallowed migration exceptions
- idempotent startup after migrations are already applied

Migration history must be inspectable.

Do not silently continue after a schema migration error.

## Step 5 — initial schema

Create the initial migration set for the H!veAI domain.

At minimum define appropriate primary keys and relationships for:

### Project layer
- projects
- repositories
- project_sources
- git_snapshots

### Task layer
- tasks
- task_dependencies
- task_sources
- task_events

### Prompt / agent layer
- prompts
- prompt_versions
- agent_sessions
- agent_events
- agent_tool_calls
- permission_requests

### Audit / test layer
- audits
- audit_findings
- test_runs

### Coordination layer
- alerts
- decisions
- github_sync_state
- settings

Use foreign keys and indexes for likely lookup paths.

Enable SQLite foreign keys explicitly.

## Step 6 — timestamps and identifiers

Adopt consistent conventions:

- durable string/UUID-style IDs or another well-justified stable ID strategy
- UTC timestamps
- `created_at`
- `updated_at` where appropriate

Document the convention.

Do not make filesystem path or GitHub URL the primary key.

## Step 7 — repository/path safety

For future project data, define path fields as data only.

M04 must NOT automatically open, mutate, scan, or normalize user repositories beyond test fixtures.

Do not persist secrets, tokens, credential values, raw `.env` contents, or private keys.

## Step 8 — Rust persistence module

Create clean Rust modules under the child Tauri source for:

- DB path resolution
- connection initialization
- migration execution
- schema-version inspection
- health/status

Suggested shape, adapt as needed:

- `src-tauri/src/db/mod.rs`
- `src-tauri/src/db/migrations.rs`
- `src-tauri/src/db/schema.rs` or embedded SQL files

Do not put all persistence code into `lib.rs`.

## Step 9 — database status IPC

Expose a narrow read-only command such as:

`hiveai_database_status`

It should safely report only non-sensitive metadata such as:

- initialized yes/no
- database engine
- schema version
- migration count
- database path or sanitized app-data-relative path
- foreign keys enabled
- last migration status

Do not expose arbitrary SQL execution to the frontend.

## Step 10 — frontend status surface

Add only a minimal non-invasive M04 status surface to the existing H!veAI shell showing database readiness.

Do NOT redesign the M02 dashboard in M04.
Do NOT implement Project Registry UI.

Browser preview/mock mode must clearly indicate native database status is unavailable rather than fabricating production state.

## Step 11 — migration tests

Add meaningful Rust tests using isolated temporary databases.

At minimum verify:

1. empty DB migrates to latest version
2. re-running migrations is idempotent
3. migration history is correctly recorded
4. foreign keys are enabled
5. required tables exist
6. required indexes exist where declared
7. migration failure rolls back cleanly
8. a partially/incorrectly versioned DB fails safely
9. production path resolver is not used by tests
10. no legacy parent DB is touched

If rollback testing requires an intentionally failing test migration, keep it test-only.

## Step 12 — schema integrity tests

Add tests for representative constraints:

- repository belongs to project
- task belongs to project
- task dependency references valid tasks
- prompt version references prompt
- agent session references project/task where designed
- audit findings reference audits

Do not overfill tables with fake production records.

## Step 13 — startup integration

Initialize H!veAI persistence during Tauri startup in a controlled order:

1. resolve app-data directory
2. create H!veAI-owned directory if necessary
3. open SQLite
4. enable safety pragmas/foreign keys as justified
5. run migrations
6. register DB state
7. continue app startup only if persistence is healthy

If initialization fails, surface a clear safe error and do not silently claim a healthy DB.

Do not start any legacy sidecar.

## Step 14 — migration documentation

Create:

`H!veAI/docs/migration/M04_SQLITE_AND_VERSIONED_MIGRATIONS.md`

Document:

- crate/technology choice
- database location
- schema overview
- table relationships
- migration mechanism
- failure semantics
- backup/restore considerations
- security/privacy constraints
- future M05 usage boundary
- known limitations

## Step 15 — M03 carry-forward review

Preserve the M03 Rust-native runtime boundary.

Do not weaken Tauri permissions.

CSP localhost development origins may remain if necessary for verified dev flow; do not broaden them. Record CSP as deferred production-hardening debt unless M04 legitimately needs a safe change.

## Step 16 — verification

Run from the H!veAI workspace:

Frontend:

- typecheck
- tests
- production build

Rust/Tauri:

- `cargo fmt --manifest-path H!veAI/src-tauri/Cargo.toml -- --check`
- `cargo check --manifest-path H!veAI/src-tauri/Cargo.toml`
- `cargo test --manifest-path H!veAI/src-tauri/Cargo.toml`
- `cargo build --manifest-path H!veAI/src-tauri/Cargo.toml`

Database-specific:

- fresh temp DB migration
- idempotent re-run
- failure/rollback test
- schema-version inspection
- required-table/index checks

Windows bounded smoke:

- launch H!veAI
- verify H!veAI window renders
- verify runtime IPC still works
- verify database-status IPC works
- verify DB is created only in H!veAI app-data location
- verify no legacy port 8765 listener
- verify no legacy backend process
- clean shutdown

## Step 17 — TASKS.md

Update only M04 items.

Use `[x]` only for completed and verified items.
Do not mark M05 or later complete.

## Step 18 — containment review

Before commit verify:

- no parent application source/package modifications
- no production `.db` file staged
- no temp DB staged
- no node_modules/dist/target/log artifacts staged
- no secrets staged
- M00/M01/M02/M03 logs unchanged
- pre-M00 stash and user files preserved

Run:

- `git diff --check`
- staged diff review

## Commit and push

If M04 is genuinely complete, create a focused commit:

`feat(H!veAI): add SQLite persistence and migrations`

The commit MUST include:

`H!veAI/docs/H!veAI/codex-logs/M04_SQLITE_AND_VERSIONED_MIGRATIONS_CODEX_LOG.md`

Push normally to:

`origin/H!veAI`

Do not force push.

After push verify on GitHub that M00, M01, M02, M03 and M04 logs all exist as separate files under:

`H!veAI/docs/H!veAI/codex-logs/`

If needed, use a small log-only follow-up commit to record remote verification.

## M04 acceptance criteria

M04 is complete only if:

1. H!veAI owns a Rust-native SQLite persistence layer.
2. Database lives in H!veAI app-data, not the repository.
3. Versioned migrations exist.
4. Migration history is persisted.
5. Migration failures are not swallowed.
6. Fresh DB migration passes.
7. Re-run is idempotent.
8. Failure rollback is tested.
9. Foreign keys are enabled.
10. Initial schema covers the architecture's core persistence entities.
11. Required relationships/indexes exist.
12. Tests use isolated temporary DBs.
13. No arbitrary SQL IPC exists.
14. Read-only database status IPC works.
15. Existing M01/M02/M03 regressions pass.
16. No legacy Python sidecar/runtime starts.
17. Parent app remains untouched.
18. M00-M03 logs remain unchanged.
19. M04 log is committed/pushed/verified.
20. Migration documentation exists.
21. TASKS reflects verified M04 state only.

## Final response format

Return exactly:

1. M04 RESULT
2. FETCH-BEFORE-PROMPT SYNC
3. VERIFIED GIT ROOT
4. VERIFIED H!veAI APPLICATION ROOT
5. BRANCH / HEAD
6. SQLITE TECHNOLOGY
7. DATABASE LOCATION
8. SCHEMA VERSION
9. MIGRATION FRAMEWORK SUMMARY
10. TABLES CREATED
11. INDEX / FK SUMMARY
12. DATABASE STATUS IPC
13. STARTUP INTEGRATION
14. MIGRATION TEST RESULTS
15. FRONTEND TEST / BUILD RESULTS
16. RUST / TAURI RESULTS
17. WINDOWS SMOKE RESULT
18. LEGACY RUNTIME CONTAINMENT
19. CSP STATUS
20. FILES ADDED
21. FILES MODIFIED
22. PARENT FILES MODIFIED
23. CODEX LOG LOCAL PATH
24. CODEX LOG GITHUB PATH / VERIFICATION
25. PRESERVED HISTORICAL LOG STATUS
26. PRESERVED STASH / USER FILE STATUS
27. COMMIT / PUSH STATUS
28. BLOCKERS / OPEN DECISIONS
29. EXACT NEXT MILESTONE

The exact next milestone is:

`M05 — Project Registry`

IMPORTANT GOVERNANCE RULE:

Do NOT create, invent, recommend, or claim the existence of an M05 Codex prompt file.
Do NOT include a `RECOMMENDED NEXT CODEX PROMPT` section.
The next prompt is authored only by ChatGPT after independent M04 audit approval.

Do NOT start M05.
Stop after M04.
