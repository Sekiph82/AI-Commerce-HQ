# M12A Project-Wide Workflow History Strict Remediation Log

Date: 2026-08-27  
Branch: `H!veAI`

## Synchronized preflight

- `git fetch origin H!veAI`
- Initial local `HEAD`: `8c1a3d96e1980f1e0b69eac24922777a7b6520cd`
- Initial fetched `origin/H!veAI`: `bed1b613754013792318a601d026cc97b74a88b1`
- Initial `git rev-list --left-right --count HEAD...origin/H!veAI`: `0 2`
- `git merge --ff-only origin/H!veAI`: PASS; fast-forwarded to `bed1b613754013792318a601d026cc97b74a88b1`.
- Synchronized pre-implementation local/origin: `bed1b613754013792318a601d026cc97b74a88b1` / `bed1b613754013792318a601d026cc97b74a88b1`
- Synchronized pre-implementation divergence: `0 0`

## R26 remediation

The prior cockpit implementation iterated selected-project tasks, fetched bounded per-task histories, and stopped when the aggregate reached 200 events. An older, event-heavy task could therefore hide newer events from a later task.

The production fix adds one bounded `workflow::project_history` query. It validates the registered project, joins `task_events` to selected-project tasks, filters workflow events, orders the complete selected-project result by `occurred_at DESC, id DESC`, and applies the 200-event limit only after that project-wide ordering. The cockpit consumes this result directly, so derived Activity inherits the same scoped, deterministic subset. No second workflow store, UI expansion, task-authority change, or cross-project access was added.

## Changed files

- `H!veAI/.hiveai/PROJECT_DASHBOARD.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/README.md`
- `H!veAI/TASKS.md`
- `H!veAI/docs/H!veAI/README.md`
- `H!veAI/src-tauri/src/project_cockpit.rs`
- `H!veAI/src-tauri/src/workflow.rs`

No external registered project or Bulk Edit was modified. Parent-root untracked `start-demo.bat` and `task.md` were preserved and not staged. M13 and M21 were not started.

## Adversarial regression evidence

- `cargo test --manifest-path src-tauri/Cargo.toml --lib project_cockpit::tests -- --nocapture --test-threads=1`: PASS, 7 tests; includes older task starvation prevention, global newest-first ordering, deterministic equal-timestamp tie order, selected-project isolation, and derived Activity inheritance.
- `m12_project_history_prevents_cross_task_starvation_and_orders_globally`: PASS; 205 older A events cannot displace the newer B event, result is capped at 200 and ordered newest-first.
- `m12_project_history_tie_order_is_deterministic`: PASS; equal timestamps use descending event ID deterministically across repeated snapshots.
- `m12_project_history_and_activity_exclude_other_project_events`: PASS; other-project workflow events are absent from both history and Activity.
- `npm.cmd test -- --run tests/m12-project-cockpit-focused.test.tsx`: PASS, 1 file, 5 tests.

## Required verification and publication gates

- `npm.cmd test -- --run`: PASS, 10 files, 92 tests.
- `cargo test --manifest-path src-tauri/Cargo.toml --lib -- --nocapture --test-threads=1`: PASS, 285 tests, 0 failures; assertions executed.
- `npm.cmd run typecheck`: PASS.
- `npm.cmd run build`: PASS; frontend production bundle built.
- `npm.cmd audit -- --audit-level=high`: PASS, 0 vulnerabilities.
- `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`: PASS.
- `cargo check --manifest-path src-tauri/Cargo.toml`: PASS; existing non-blocking warnings only.
- `git diff --check`: PASS.
- `powershell.exe -NoProfile -ExecutionPolicy Bypass -File scripts/tests/publish-dev-qa-failure-harness.ps1`: PASS, 9/9 rollback/failure cases.
- `powershell.exe -NoProfile -ExecutionPolicy Bypass -File scripts/publish-dev-qa.ps1`: PASS. Production Tauri `--no-bundle` build, candidate/stable smoke, rollback-safe publication, stable executable validation, shortcut target/icon checks, forbidden-port check, and console-host suppression check completed.
- Published stable executable: `H!veAI/dev-bin/H!veAI.exe`.
- Native/visual/user acceptance is not claimed; it remains pending.

## Git proof

- Implementation commit SHA: `fe1f6f6bdbcf93580e12dd785863fcc4d5d1fe9f`.
- Post-push fetched local `HEAD`: `fe1f6f6bdbcf93580e12dd785863fcc4d5d1fe9f`.
- Post-push fetched `origin/H!veAI`: `fe1f6f6bdbcf93580e12dd785863fcc4d5d1fe9f`.
- Post-push `git rev-list --left-right --count HEAD...origin/H!veAI`: `0 0`.

## Final builder state

`M12A REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

M00-M11 remain PASS/CLOSED. M12 remains historical strict-audit FAIL with R26 remediated; M12 is not marked PASS/CLOSED by this run. Strict completed roadmap progress remains `12/20 = 60%`. M13 remains blocked/not started. M21 remains planned/not started.
