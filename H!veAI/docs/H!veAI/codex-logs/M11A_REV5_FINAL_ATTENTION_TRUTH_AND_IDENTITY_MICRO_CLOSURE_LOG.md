# M11A REV5 Final Attention Truth and Identity Micro-Closure Builder Log

## Run identity

- Product: H!veAI
- Milestone: M11A REV5 final attention truth and identity micro-closure
- Prompt executed: `docs/H!veAI/prompts/M11A_REV5_FINAL_ATTENTION_TRUTH_AND_IDENTITY_MICRO_CLOSURE_PROMPT.md`
- Branch: `H!veAI`
- Starting HEAD after required fetch and fast-forward: `83ee210`
- Starting full SHA: `83ee210d6ed7bff8e6f7fcd802fb76561609cf8b`
- Scope: R19-R22 only, plus required regression evidence and bounded R15 notify-path evidence
- M11 remains pending independent re-audit and user native/visual acceptance. M12 was not started.

## Task 0 tracker synchronization

Task 0 was completed before production edits. Only these prospective H!veAI-owned status documents were synchronized: `TASKS.md`, `CODEX_ROADMAP.md`, `README.md`, and `docs/H!veAI/README.md`.

- M00-M10 remain PASS/CLOSED.
- Strict completed roadmap count remains 11/20 = 55%.
- Original M11 remains historical strict-audit FAIL.
- REV4 is recorded as implementation complete with independent REV4 audit FAIL and R19-R22 open.
- REV5 is recorded as ACTIVE; M11 remains NOT CLOSED and M12 remains BLOCKED.
- User native/visual acceptance remains pending.
- Historical prompts, audits, and logs were not rewritten.

## R19 - WAITING attention truth

Implemented in `src-tauri/src/command_center.rs` in `materialized_operational_evidence`.

- Project status `WAITING` no longer creates attention by itself.
- `WAITING` becomes actionable only through a meaningful `Waiting on` value or meaningful Blockers and waiting content.
- `NONE`, `UNKNOWN`, `NOT_VERIFIED`, empty, and `None verified` remain non-actionable.
- Project status `BLOCKED` and Health `ATTENTION`/`BLOCKED` remain independently actionable.
- Required actor does not manufacture attention.
- Duplicate blocker/wait facts are reduced to one logical item.
- Tests: `m11a_r19_waiting_without_real_wait_fact_stays_out_of_attention`, `m11a_r19_waiting_requires_one_real_fact_and_blocked_is_independent`, and `m11a_r19_health_attention_and_blocked_remain_actionable` passed.

## R20 - Provenance-aware attention identity

Implemented in `src-tauri/src/command_center.rs`.

- `AttentionIdentity` uses project-scoped task identity, explicit evidence class, and bounded normalized source/check identity.
- Materialized quality may suppress a matching persisted TEST_RUN or AUDIT only when the task and normalized check identity are both proven.
- Materialized blocker/wait may suppress matching WORKFLOW or PERMISSION evidence only with a proven task/source identity, or an exact project-scoped source fallback when both task identities are unavailable.
- Unrelated FAIL/BLOCKED/WAITING values are never merged by status wording alone.
- Dashboard duplicate IDs are deduplicated by exact generated identity, while duplicate quality labels use occurrence-qualified deterministic IDs.
- `needs_attention` is computed from the final bounded, deduplicated attention vector.
- Persisted test/audit details now retain command/summary check identity; permission details retain permission identity without speculative mapping.
- Tests: `m11a_r20_matching_test_and_audit_quality_suppress_only_dashboard_duplicates`, `m11a_r20_matching_wait_uses_task_and_source_identity`, and `m11a_r20_unproven_failures_remain_distinct_and_snapshot_ids_repeat` passed.

## R21 - Quality header filtering

Implemented in `src-tauri/src/project_dashboard.rs` in `parse_bounded_facts`.

- The known `Check | Result | Evidence` table header is ignored case-insensitively.
- The two-column `Check | Result` form is also excluded by the existing exact label/value guard.
- Legitimate non-header quality facts remain materialized and bounded.
- The parser test `quality_table_header_is_not_a_materialized_fact` passed, and the Command Center regression asserts no fake Engineering Brief `Check: Result` fact.

## R22 - Stable materialized evidence IDs

Implemented in `src-tauri/src/command_center.rs`.

- `stable_materialized_id` uses a fixed-size SHA-256-derived digest over project identity, evidence class, normalized content/source identity, and duplicate occurrence.
- Blocker and activity IDs no longer depend on list position.
- Quality IDs include normalized label/value and deterministic duplicate occurrence.
- Existing Current work row IDs remain preferred when a real dashboard row ID exists; generated IDs for missing row IDs are bounded and deterministic.
- Unchanged blocker/activity IDs survive insertion of unrelated preceding rows.
- Duplicate quality facts receive distinct deterministic IDs without random UUIDs or unbounded raw text.
- Test: `m11a_r22_materialized_ids_survive_unrelated_preceding_rows_and_duplicate_facts` passed.

## R15 actual notify-path evidence

Implemented as bounded evidence follow-up in `src-tauri/src/watcher.rs`.

- Test: `actual_notify_path_reconciles_dashboard_scope_without_restart`.
- The test starts a live manager in `LEGACY_RECURSIVE`, physically creates `.hiveai/PROJECT_DASHBOARD.md`, waits with an 8-second deadline for actual notify delivery and `SINGLE_DASHBOARD` transition, physically removes the dashboard, and waits for `LEGACY_RECURSIVE` recovery.
- The test does not inject `manager.sender` and does not broaden watcher scope.
- The test passed on the available Windows notify backend.
- Existing live scope, exact filtering, last-good snapshot, and production watcher-to-M09/M11 tests also passed.

## Executed test and gate evidence

All Rust assertions executed; no `cargo test --no-run` command was used as acceptance.

- R19 focus: `cargo test --lib command_center::tests::m11a_r19 -- --nocapture --test-threads=1` -> **3 passed**.
- R20 focus: `cargo test --lib command_center::tests::m11a_r20 -- --nocapture --test-threads=1` -> **3 passed**.
- R21 focus: `cargo test --lib project_dashboard::tests::quality_table_header_is_not_a_materialized_fact -- --nocapture --test-threads=1` -> **1 passed**.
- R22 focus: `cargo test --lib command_center::tests::m11a_r22_materialized_ids_survive_unrelated_preceding_rows_and_duplicate_facts -- --nocapture --test-threads=1` -> **1 passed**.
- Notify focus: `cargo test --lib watcher::tests::actual_notify_path_reconciles_dashboard_scope_without_restart -- --nocapture --test-threads=1` -> **1 passed**.
- Full native: `cargo test --lib -- --nocapture --test-threads=1` -> **273 passed, 0 failed**.
- Focused frontend: `npm.cmd test -- --run --reporter=dot tests/m07.06-focused.test.tsx tests/m08-task-sources-focused.test.tsx tests/m09.02-focused.test.tsx` -> **2 files passed, 49 tests passed**.
- Full frontend: `npm.cmd test -- --run --reporter=dot` -> **9 files passed, 86 tests passed**.
- TypeScript: `npm.cmd run typecheck` -> **PASS**.
- Production frontend build: `npm.cmd run build` -> **PASS**.
- Dependency audit: `npm.cmd audit --audit-level=high` -> **0 vulnerabilities**.
- Rust formatting: `cargo fmt --all -- --check` -> **PASS**.
- Native check: `cargo check` -> **PASS**.
- Patch hygiene: `git diff --check` -> **PASS**.
- The Windows Rust commands used the established local common-controls manifest workaround via `C:\tmp\hiveai-common-controls.manifest`; no global machine change was made.

## Governed QA publication

- Command: `powershell.exe -ExecutionPolicy Bypass -File .\scripts\publish-dev-qa.ps1`
- Result: production Tauri `--no-bundle` build, smoke test, staged publication, rollback retention, and shortcut validation **PASS**.
- Command: `powershell.exe -ExecutionPolicy Bypass -File .\scripts\tests\publish-dev-qa-failure-harness.ps1`
- Result: all 9 failure-harness cases **PASS**, including stable-byte preservation, exact rollback, locked-target failure, no spawned test process, and no build bypass.
- Stable executable: `H!veAI/dev-bin/H!veAI.exe`
- Stable executable SHA-256: `96EB40FD337100BB71BA1BC450D420898E8978D1AC83D0B5260B46EA32E40745`
- Desktop shortcut target: `H!veAI/dev-bin/H!veAI.exe`
- Desktop shortcut icon: `H!veAI/dev-bin/H!veAI.ico,0`
- Canonical background SHA-256: `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`
- Canonical opening video SHA-256: `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`
- The current topbar Akilta attribution, footer removal/reclaimed workspace, startup audio/replay, terminal suppression, Advanced source inventory, and Chrome-only external opening behavior were preserved.
- No installer was created. Bulk Edit was not touched. Codex/Claude adapters, Prompt Engine, GPT Audit Engine, GitHub integration, and AI recommendation generation were not started.

## Repository and tracker boundaries

- Only H!veAI tracked files were staged.
- Only H!veAI's own `.hiveai/PROJECT_DASHBOARD.md` was modified.
- No external registered project repository was modified.
- Parent-root untracked `start-demo.bat` and `task.md` were preserved and not staged.
- The dashboard remains the H!veAI single-dashboard watch contract and keeps Health `UNKNOWN`, no invented timestamps, and independent audit/user acceptance as the next gate.

## Commit and remote evidence

- Implementation commit: `5162341`
- Implementation full SHA: `5162341d4a343c11ad9f57d9493f3aa9aa8fb1df`
- The implementation commit was pushed to `origin/H!veAI` and fetched back.
- Post-implementation equality: local `5162341d4a343c11ad9f57d9493f3aa9aa8fb1df`, origin `5162341d4a343c11ad9f57d9493f3aa9aa8fb1df`, `git rev-list --left-right --count HEAD...origin/H!veAI` = `0 0`.
- This historical log is immutable. The final post-log equality is verified after this log is pushed and reported in the closure response; prior logs remain unchanged.

## Final builder state

`IMPLEMENTATION COMPLETE / PENDING INDEPENDENT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

M11 remains NOT CLOSED. M12 remains BLOCKED and was not started.
