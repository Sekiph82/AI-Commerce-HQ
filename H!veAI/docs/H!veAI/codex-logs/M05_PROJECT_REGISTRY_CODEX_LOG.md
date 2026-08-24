# M05 Project Registry Codex Log

Product: H!veAI

## Milestone start

- Timestamp: 2026-08-24 Europe/Istanbul
- Milestone: M05 - H!veAI Project Registry
- Status: IN PROGRESS
- Repository root: `C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`
- Application root: `C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`
- Branch: `H!veAI`
- Synchronized starting HEAD: `12a73507277e8dd4fe08c66ed859749cf14d2f7d`
- Remote: `origin https://github.com/Sekiph82/AI-Commerce-HQ.git`
- Preserved user state: `stash@{0}`, untracked `start-demo.bat`, untracked `task.md`

## Fetch-before-prompt preflight

- Read `H!veAI/AGENTS.md` before prompt access, including the Canonical UI Assets rules.
- Opened the official GitHub branch URL: `https://github.com/Sekiph82/AI-Commerce-HQ/tree/H!veAI`.
- Ran `git fetch origin H!veAI`; remote advanced from `9287a5d` to `12a7350`.
- Ran `git rev-list --left-right --count HEAD...origin/H!veAI`: `0 2`.
- No conflicting tracked changes existed, so `git merge --ff-only origin/H!veAI` safely advanced the checkout to `12a7350`.
- Read the authoritative M04 audit and M05 prompt from the synchronized checkout.

## Repository preflight

- `git rev-parse --show-toplevel`: canonical parent Git root confirmed.
- `git branch --show-current`: `H!veAI`.
- `git rev-parse HEAD`: `12a73507277e8dd4fe08c66ed859749cf14d2f7d`.
- `git remote -v`: canonical HTTPS origin confirmed.
- `git status --short`: only preserved untracked `start-demo.bat` and `task.md`.
- `git stash list`: preserved pre-M00 user package-change stash confirmed.
- Tags and worktrees inspected; `H!veAI` contains no `.git` directory.

Historical M00, M01, M02, M03 and M04 Codex logs will remain unchanged.

## Baseline

- Frontend typecheck, 9 tests, and production build passed.
- Rust format check, check, 15 tests, and build passed.
- M04 audit is approved with non-blocking follow-up requiring explicit, read-only project registration and Canonical UI Assets compliance.

## Design decisions

- Registry uses the M04 Rust SQLite layer and adds migration version 3 for project identity/settings and repository detection fields.
- Registration is explicit and path-driven. No automatic machine-wide scanning, project execution, package installation, Git mutation, `.hiveai` creation, or external repository writes are allowed.
- Paths preserve the user-selected display string and store a canonical normalized duplicate-detection value. Missing paths are derived as `MISSING` during reads without silently mutating registry records.
- Git detection reads `.git` metadata files only, sanitizes HTTP credentials, supports common GitHub remote forms, and stores no secrets.
- Archive hides registry records without touching the folder. Remove deletes only H!veAI registry rows. Repair validates a new path and repository identity before updating rows.
- Canonical dashboard assets were inspected. The source H!veAI logo and Akilta wordmark were copied unchanged into `src/assets/`; the existing dark dashboard density was extended for the registry surface.

## Implementation log

Implementation and verification entries will be appended chronologically. Failures will remain recorded when corrected.

### 2026-08-24 - registry implementation

- Added migration version 3 `project_registry_fields` to extend the M04 schema without introducing a second datastore.
- Added `src-tauri/src/projects/paths.rs`, `detection.rs`, and `registry.rs` with typed project records, safe path validation/normalization, duplicate handling, Git metadata detection, remote credential sanitization, archive/remove/repair semantics, and list/search/filter/sort behavior.
- Added typed, allowlisted IPC commands for project list/register/get/settings/archive/remove/repair. No shell, generic Git command, arbitrary filesystem browser, or arbitrary SQL surface was added.
- Registration writes only H!veAI SQLite rows in a transaction. Non-Git folders are valid; Git folders store sanitized remote, branch, HEAD, default branch, and GitHub identity metadata.
- Added real `/projects` registry UI with explicit Add Project form, persisted records, search/sort/status controls, priority, Git/non-Git indicators, archive/remove/repair actions, and cached-metadata Project Cockpit integration.
- Copied canonical `H!veAI logo.png` unchanged to `src/assets/hiveai-logo.png` and canonical `akilta-wordmark-a1.svg` unchanged to `src/assets/akilta-wordmark.svg`. Shell branding is H!veAI and footer text is exactly `Built with ♥ for maximum productivity by Akilta`.
- Added `docs/migration/M05_PROJECT_REGISTRY.md`.

### 2026-08-24 - implementation failure and correction

- First registry Rust test run exposed a shared SELECT missing its single-project `WHERE` clause; registration, archive/repair, and list-state tests reported a SQLite parameter-count error.
- Correction: `fetch_project` now appends `WHERE p.id = ?1` to the shared query. Follow-up registry tests passed.
- First child-code build was preceded by one procedural parent-root `npm run build`, which failed on the old parent’s missing `framer-motion` dependency. No parent files changed; the correct child-workspace build passed afterward.

### 2026-08-24 - verification

- Frontend baseline before implementation: typecheck, 9 tests, and build passed.
- Final frontend verification: typecheck passed, 10 tests passed, and production build passed with canonical logo/wordmark assets bundled.
- Rust project-registry suite passed: 22 tests total, including 7 new path/detection/registry tests plus preserved M04 and M03 coverage.
- Final Rust format check, check, and build passed.
- Registry tests covered explicit non-Git registration, deterministic duplicate rejection, isolated Git metadata fixture, credential sanitization, missing state, search/filter, archive/remove folder safety, repair safety, and no automatic scan behavior.

### 2026-08-24 - bounded Windows smoke

- `npm run tauri:dev` launched the child Vite server and H!veAI Tauri executable.
- Window title: `H!veAI`; Vite response: `200` at `http://127.0.0.1:5173`.
- Existing app-data database migrated and startup logged `schema_version=3`; runtime and database status IPC remained active.
- Legacy process count: `0`; port `8765` listeners: `0`.
- Ctrl+C stopped Tauri/Vite; follow-up found zero desktop processes, zero port-5173 listeners, and zero legacy processes.

### 2026-08-24 - containment and diff review

- `git diff --check`: PASS with line-ending warnings only.
- Changes are confined to `H!veAI/`; parent application code and managed external project folders were not modified.
- No user project `.git` metadata, production DB, temp repo, `node_modules`, `dist`, `target`, runtime log, secret, or `.env` artifact was staged.
- M00, M01, M02, M03 and M04 Codex logs remain unchanged.
- The pre-M00 stash and untracked parent `start-demo.bat` and `task.md` remain preserved.

### 2026-08-24 - publication verification

- Implementation commit: `85c4002` (`feat(H!veAI): add Project Registry`).
- Implementation commit pushed successfully to `origin/H!veAI`.
- M00-M05 logs verified as separate files on GitHub under `H!veAI/docs/H!veAI/codex-logs/`.
- No M06 prompt was created or recommended, and M06 was not started.

PHASE STATUS: COMPLETE
EXACT NEXT MILESTONE: M06 — Local Git Engine
