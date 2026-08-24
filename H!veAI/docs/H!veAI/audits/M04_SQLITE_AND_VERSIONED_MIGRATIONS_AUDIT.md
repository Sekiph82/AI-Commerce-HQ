# M04 — SQLite and Versioned Migrations Audit

Date: 2026-08-24
Product: H!veAI
Result: APPROVED WITH NON-BLOCKING FOLLOW-UP

## Scope audited

Reviewed the M04 Codex log, Rust persistence modules, migration framework, startup integration, database-status IPC, schema coverage, test evidence, containment, and GitHub publication state on branch `H!veAI`.

## Verdict

M04 is approved. The persistence foundation is suitable for M05 Project Registry work.

## Verified findings

### PASS — Rust-native persistence

H!veAI owns a Rust-native SQLite layer using `rusqlite` with bundled SQLite. No Python sidecar or external database service is required.

### PASS — application-data location

The production database is resolved from the Tauri app-data directory for `ai.hiveai.desktop` and uses the stable filename `hiveai.db`. The frontend status reports only the sanitized relative filename rather than exposing the full user path.

### PASS — explicit versioned migrations

Migration versions are ordered and contiguous. Migration history is stored in a `migrations` table with version, name, and applied timestamp. Migration application is transactional and a failed migration does not silently continue.

### PASS — migration safety

The implementation validates previously applied migration history against the expected ordered migration list and fails safely on mismatches or unknown versions.

### PASS — schema foundation

The initial schema covers the architecture's persistence domains including projects, repositories, project sources, git snapshots, tasks, dependencies, task sources/events, prompts/versions, agent sessions/events/tool calls, permission requests, audits/findings, test runs, alerts, decisions, GitHub sync state, settings, and migrations.

### PASS — foreign keys and indexes

Foreign keys are explicitly enabled. The schema defines project/task/prompt/audit/session relationships and representative lookup indexes.

### PASS — tests

M04 evidence reports 15 Rust tests total, including 10 persistence-focused tests plus the preserved M03 runtime tests. Coverage includes fresh migration, idempotent rerun, migration history, foreign keys, required tables/indexes, rollback on failure, invalid history handling, representative relationship constraints, and isolated temporary databases.

### PASS — startup integration

Persistence initialization occurs during Tauri startup. App-data resolution, directory creation, DB open, migration execution, state registration, and status exposure are all child H!veAI responsibilities. Initialization failure is surfaced and does not silently claim healthy persistence.

### PASS — narrow IPC

`hiveai_database_status` is read-only and reports only non-sensitive metadata. No arbitrary SQL execution surface was introduced.

### PASS — legacy containment

No legacy Python/FastAPI runtime is started. No parent DB is reused or migrated implicitly. M00-M03 historical logs remain unchanged.

## Non-blocking follow-up

1. CSP localhost development origins remain deferred production-hardening debt and must not be broadened casually.
2. M05 must use the new H!veAI persistence layer rather than introducing a second data store.
3. Project registration must be explicit and user-driven. No automatic scan of arbitrary user folders or repositories.
4. Registration must be read-only with respect to the selected repository during M05. Detect and persist metadata, but do not mutate branches, files, remotes, worktrees, or repository configuration.
5. Store normalized canonical paths carefully and preserve the original user-selected path for display/audit where useful.
6. Any UI added or modified from M05 onward must obey the Canonical UI Assets rules in `H!veAI/AGENTS.md` and the milestone prompt.

## Audit result

APPROVED WITH NON-BLOCKING FOLLOW-UP

M05 — Project Registry may begin.
