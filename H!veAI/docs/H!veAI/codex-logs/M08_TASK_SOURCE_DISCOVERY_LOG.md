# M08 Task Source Discovery Codex Log

Date: 2026-08-25
Branch: `H!veAI`
Synchronized base HEAD before implementation: `ca92320`
Prompt: `docs/H!veAI/prompts/M08_TASK_SOURCE_DISCOVERY_PROMPT.md`

## Scope and boundary

M08 discovers bounded task/planning/handoff source documents for explicitly
registered projects. It does not parse task entities, infer workflow state,
create prompts or sessions, call network services, mutate project files, write
`.hiveai` configuration, create an installer, or start M09.

## Discovery contract

Standard root source names are matched case-insensitively while preserving the
on-disk spelling: `TASKS.md`, `tasks.md`, `TASK.md`, `PLANS.md`, `PLAN.md`,
`PROGRESS.md`, `ROADMAP.md`, `AGENTS.md`, `CLAUDE.md`, `HANDOFF.md`,
`SESSION_HANDOFF.md`, and root `*handoff*.md` variants.

Approved bounded directories are `tasks/`, `plans/`, `handoffs/`, and
`.hiveai/`. Discovery is recursive only within those directories. Ignored
trees include `.git`, `node_modules`, `dist`, `build`, `target`, `.next`,
`coverage`, `.cache`, `.venv`, `venv`, and `vendor`.

Source kinds are `TASKS`, `HANDOFF`, `PROGRESS`, `PLAN`, `ROADMAP`, `AGENTS`,
`CLAUDE`, `HIVEAI_CONFIG`, `CUSTOM`, and `OTHER_TASK_SOURCE`. Metadata includes
relative/absolute path, origin, status, authority class, priority, size,
filesystem modified time, discovery time, SHA-256 content hash, depth, and
warnings. No source body is returned to the frontend.

Performance bounds are enforced at depth 4, 512 candidate files, 2 MiB per
hash/read, and 64 configured custom paths per project.

## Containment and persistence

Every request resolves identity through Project Registry. Existing registered
roots and custom targets are physically canonicalized and checked with
`starts_with` containment before opening. Lexical `..` escapes and absolute
outside-root paths are rejected. Bounded directory entries are physically
resolved before traversal, so outside symlink/junction directories are not
followed. Missing but syntactically safe custom paths remain configured and
are reported `MISSING`.

Custom paths are stored in H!veAI SQLite `settings` under the deterministic
project-scoped key `task_sources.custom_paths.<projectId>`, with display and
normalized forms. `project_sources` is reconciled in one bounded transaction:
standard rows are replaced by the current inventory, custom missing rows remain
visible, unchanged identity is deterministic, and repeated scans do not create
duplicates. M08 never writes `tasks` or `task_sources`.

The symlink/junction test is `UNVERIFIED` in this Windows environment because
creation failed with the exact OS error: `A required privilege is not held by
the client. (os error 1314)`. Ordinary outside-root and traversal rejection
tests pass directly.

## Native API and UI

Native IPC commands:

- `hiveai_task_sources_discover(projectId)`
- `hiveai_task_sources_list(projectId)`
- `hiveai_task_source_custom_paths_list(projectId)`
- `hiveai_task_source_custom_path_add(request)`
- `hiveai_task_source_custom_path_remove(projectId, pathOrId)`

The dedicated `allow-task-source-discovery` permission and main-window
capability entry allow only those five commands. No arbitrary filesystem read
API or shell command was added.

The existing `/tasks` route is now the live Task Sources workspace. Native mode
uses the selected Registry project, shows loading/error/empty states, source
metadata, rescan, and custom add/remove controls. Browser preview explicitly
states that filesystem discovery is unavailable and invokes no native source
commands. The Command Center and accepted presentation composition were not
redesigned.

## Changed files

- `ARCHITECTURE.md`
- `TASKS.md`
- `docs/H!veAI/UI_LAYOUT_GOVERNANCE.md`
- `src-tauri/Cargo.lock`
- `src-tauri/Cargo.toml`
- `src-tauri/capabilities/default.json`
- `src-tauri/permissions/foundation.toml`
- `src-tauri/src/lib.rs`
- `src-tauri/src/task_sources.rs`
- `src/pages.tsx`
- `src/styles.css`
- `src/taskSources.ts`
- `tests/m08-task-sources-focused.test.tsx`
- `docs/H!veAI/codex-logs/M08_TASK_SOURCE_DISCOVERY_LOG.md`

Historical M08.00/M08.00B logs and audits were not modified.

## Focused evidence

Rust production discovery tests: 17/17 PASS, including root and
case-insensitive source handling, bounded task/plans/handoff directories,
ignored trees, outside and traversal rejection, safe custom and missing paths,
Windows-normalized dedupe, idempotency, hash change, standard/custom
reconciliation, oversized sources, deterministic ordering, missing roots,
non-Git projects, project-tree immutability, no task-row writes, and the
symlink limitation record.

Frontend Task Sources tests: 7/7 PASS. They cover live Registry project ID,
loading, real metadata rows, rescan IPC and refresh, custom add/remove
boundary, truthful empty/browser behavior, and project selection refresh.

M08.00 presentation regression tests remain green in the full frontend suite.

## Full verification

PASS: `npm run typecheck`

PASS: `npm test -- --reporter=dot` (55/55)

PASS: `npm run build`

PASS: `npm audit --audit-level=high` (0 vulnerabilities)

PASS: `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`

PASS: `cargo check --manifest-path src-tauri/Cargo.toml`

PASS: `cargo test --manifest-path src-tauri/Cargo.toml` (119/119)

PASS: `cargo build --manifest-path src-tauri/Cargo.toml`

PASS: `scripts/tests/publish-dev-qa-failure-harness.ps1` (all nine isolated
failure, cleanup, rollback, and no-bypass cases)

PASS: `scripts/publish-dev-qa.ps1`, including production Tauri `--no-bundle`,
candidate smoke, readiness, no forbidden development port, no visible console
host, SHA-256 swap/rollback checks, and shortcut validation.

## Publication

Stable executable: `dev-bin/H!veAI.exe`

Stable executable SHA-256:
`F4B286E32AD57866CFC4F1FC86537DAF664A43BC4A1B8C8432FCD0F36D9647D2`

Stable executable size: 17,282,048 bytes

Shortcut target: `Desktop/H!veAI.lnk` -> `dev-bin/H!veAI.exe`

Shortcut icon: `dev-bin/H!veAI.ico`, derived from the canonical small-logo
source and unchanged by this milestone.

Canonical asset hashes remain unchanged:

- `src/assets/hiveai-app-background.png`:
  `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`
- `src/assets/opening-video.mp4`:
  `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`

## Final repository state

The implementation was based on synchronized `ca92320`. The final local and
remote HEAD values will be recorded after commit and push in this log's
immutable follow-up commit metadata.

No installer was created. No M09 work was started.

Manual visual status for the native `/tasks` Task Sources workspace:
`PENDING USER VISUAL ACCEPTANCE`.
