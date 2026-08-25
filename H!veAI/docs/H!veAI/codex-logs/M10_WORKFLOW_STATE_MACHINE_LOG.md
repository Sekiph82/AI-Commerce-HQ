# M10 Workflow State Machine Builder Log

## START STATE

- Branch: `H!veAI`
- Starting HEAD after `git fetch origin H!veAI` and fast-forward: `baa8c000f7405a0aae6bd3353b1c7dcc5bed63fe`
- Starting origin equality: `HEAD == origin/H!veAI`; `0 0` divergence.
- Starting worktree: only pre-existing untracked user files `start-demo.bat` and `task.md`; both were preserved and not staged.
- Dashboard manifest documentation was present, but M10 did not implement Project Dashboard runtime ingestion or authority resolution.

## CONTRACT

- Canonical states: `BACKLOG`, `PLANNING_REQUIRED`, `PROMPT_REQUIRED`, `PROMPT_READY`, `READY_FOR_IMPLEMENTATION`, `BUILDER_RUNNING`, `IMPLEMENTATION_COMPLETE`, `AUDIT_REQUIRED`, `AUDIT_RUNNING`, `AUDIT_PASSED`, `VERIFY_REQUIRED`, `VERIFY_RUNNING`, `TASK_COMPLETE`, `AUDIT_FAILED`, `FIX_REQUIRED`, `RE_AUDIT_REQUIRED`, `BLOCKED`, `WAITING_HUMAN`, `WAITING_EXTERNAL`, `DESIGN_GATE`.
- Canonical actors: `HUMAN`, `CODEX`, `CLAUDE`, `GPT_AUDIT`, `CI`, `EXTERNAL`, `SYSTEM`.
- Normal matrix: backlog -> planning -> prompt required -> prompt ready -> ready for implementation -> builder running -> implementation complete -> audit required/re-audit required -> audit running -> audit passed -> verify required -> verify running -> task complete.
- Failure loop: audit running -> audit failed -> fix required -> ready for implementation -> builder running -> implementation complete -> re-audit required -> audit running.
- Suspension rules: bounded reason is required; `BUILDER_RUNNING`, `AUDIT_RUNNING`, and `VERIFY_RUNNING` resume to safe prerequisites; other states resume exactly; `WAITING_HUMAN` and `DESIGN_GATE` require HUMAN; `WAITING_EXTERNAL` requires EXTERNAL or HUMAN; parser-seeded `BLOCKED` defaults to `BACKLOG`.
- Evidence gates validate finite typed refs and real same-task/same-project rows. Prompt, approval decision, live/completed builder session, live audit session, audit result, started test run, and finished PASS test run gates are enforced. External waits require a bounded external reference.
- Bounds: summaries/rationales 4096 bytes, request IDs 128 bytes, evidence refs 32, evidence scalars 512 bytes, history/list limits 1..500 with default 100.

## PRODUCTION IMPLEMENTATION

- `src-tauri/src/workflow.rs` owns typed state/actor/evidence contracts, the matrix, read model, transition mutation, override mutation, history reads, and restart recovery.
- Existing M04 tables are reused; no parallel workflow table was created. `task_events` remains immutable append-only history and `tasks.state` remains the materialized operational state.
- Normal mutation opens one SQLite transaction, loads the task/project state inside it, checks active/missing ownership, checks expected state, validates matrix/actor/evidence, inserts one event, updates `tasks.state` and `updated_at`, then commits.
- Idempotency uses `m10evt:<sha256(task_id|request_id)>`; same semantic retry returns the original event, while conflicting reuse returns `WORKFLOW_REQUEST_CONFLICT`.
- Human override is separate from normal transition, always records actor HUMAN, writes a `WORKFLOW_OVERRIDE` decision and event in the same transaction, requires rationale, and permits reopening terminal state only through this path.
- Startup calls `workflow::recover_stale` after database initialization. M10-history tasks in transient states are demoted with one SYSTEM `WORKFLOW_RECOVERY` event; a second pass is idempotent.
- IPC surface is narrow: `hiveai_workflow_task_get`, `hiveai_workflow_project_list`, `hiveai_workflow_history`, `hiveai_workflow_transition`, `hiveai_workflow_override`.

## M09 <-> M10 INTEGRATION

- `task_intelligence::persist` now checks for M10 history before parser UPSERT state changes. Managed `tasks.state` is preserved while parser title/source/metadata fields refresh.
- Stale parser tasks without M10 history retain existing M09 deletion behavior. Stale parser tasks with M10 history are retained, have `sourceActive=false` / `sourceRetired=true`, lose only the stale source reference, and keep the task row and all events.
- Reappearance uses the same stable M09 task ID, sets `sourceActive=true`, refreshes parser metadata/title, preserves operational state, preserves `created_at`, and preserves all events.
- Direct proofs: `m10_m09_reparse_preserves_workflow_state_and_events`, `m10_m09_stale_source_preserves_managed_history`, `m10_m09_reappearance_reactivates_same_task_without_history_loss`, and `m10_no_history_stale_task_keeps_m09_cleanup`.

## IPC / ACL / TYPESCRIPT

- Added `allow-workflow-state-machine` to `permissions/foundation.toml` and only that permission to the main-window capability.
- Added typed `src/workflow.ts` enums, interfaces, bounded limit validation, and wrappers. No visible UI route or component was changed.
- TypeScript contract tests prove exact command names, canonical enum strings, expected-state/request-ID payloads, bounded limits, and no browser-preview fake workflow state.

## DIRECT TESTS

- `m10_happy_path_requires_each_canonical_step`: catches skipped or reordered canonical states.
- `m10_invalid_direct_jump_is_rejected`: catches normal matrix bypass.
- `m10_audit_failure_routes_to_reaudit_after_fix`: catches returning to first-pass audit after remediation.
- `m10_prompt_ready_requires_same_task_prompt`: catches prompt existence without task ownership.
- `m10_builder_running_requires_matching_live_builder_session`: catches missing live builder evidence.
- `m10_audit_pass_requires_matching_pass_audit`: catches incompatible audit result.
- `m10_verify_complete_requires_finished_pass_test_run`: catches unfinished/failed verification completion.
- `m10_cross_project_evidence_is_rejected`: catches cross-project evidence laundering.
- `m10_expected_state_prevents_stale_double_transition`: catches stale-client overwrite.
- `m10_request_id_is_idempotent` and `m10_request_id_conflicting_reuse_is_rejected`: catch duplicate and conflicting retry behavior.
- `m10_waiting_human_round_trip_resumes_exact_prior_state`, `m10_running_state_suspension_resumes_to_safe_prerequisite`, and `m10_parser_seeded_blocked_defaults_resume_to_backlog`: catch unsafe suspension recovery.
- `m10_override_requires_nonempty_rationale`, `m10_override_records_decision_and_event_atomically`, and `m10_task_complete_reopen_requires_override`: catch invisible or unsupported human corrections.
- `m10_restart_recovery_demotes_stale_running_states`: covers all transient recovery semantics and idempotent second pass.
- History bounds/order, archived-project mutation rejection/history read, and all M09 reparse/stale/reappearance tests are included in the same production-path suite.

## FAILED ATTEMPTS

1. Initial workflow test compile/run exposed a fixture SQL bug: Rust `Option::None` was interpolated as SQL `None` for an ended session. Fixed the helper to emit SQL `NULL` or a quoted timestamp.
2. The cross-task prompt fixture initially violated the task foreign key before reaching the workflow gate. Fixed it to use a valid nullable task reference so the production ownership rejection is exercised.
3. The first TypeScript contract test used a top-level mock variable with hoisted `vi.mock`; Vitest rejected it. Fixed the test with `vi.hoisted`.
4. The first audit-remediation assertion expected builder completion itself to jump to re-audit. The contract requires the explicit `IMPLEMENTATION_COMPLETE -> RE_AUDIT_REQUIRED` transition; the test now proves that exact step.
5. No failed production gate remains. These attempts are retained here as chronology rather than erased.

## FULL REGRESSION

- `npm run typecheck`: PASS.
- `npm test -- --run`: PASS, 7 files / 79 tests.
- `npm run build`: PASS, 1987 modules transformed.
- `npm audit --audit-level=high`: PASS, 0 vulnerabilities.
- `cargo fmt --manifest-path H!veAI/src-tauri/Cargo.toml -- --check`: PASS.
- `cargo check --manifest-path H!veAI/src-tauri/Cargo.toml`: PASS.
- `cargo test --manifest-path H!veAI/src-tauri/Cargo.toml`: PASS, 216 tests, 0 failed.
- `cargo build --manifest-path H!veAI/src-tauri/Cargo.toml`: PASS.
- Focused M10 Rust suite: PASS, 24 tests.
- Focused TypeScript/native UX suite: PASS, 9 tests.
- M08 source discovery subset: PASS, 35 tests.
- M09 task intelligence subset: PASS, 53 tests.
- Watcher subset: PASS, 26 tests.
- Git Engine subset: PASS, 21 tests.
- Database subset: PASS, 9 tests.
- Existing registry fixtures emitted two non-failing `fatal: Not a valid commit name ...` lines while all tests passed; this is existing fixture behavior, not an M10 failure.
- Existing watcher symlink tests reported environment privilege `UNVERIFIED` messages while passing; no M10 code depends on those environment-limited operations.

## PUBLICATION

- Publisher failure/rollback harness: PASS, all 9 protections.
- Governed production publisher: `powershell -ExecutionPolicy Bypass -File .\H!veAI\scripts\publish-dev-qa.ps1`; PASS with Tauri `--no-bundle`, frontend build, release build, smoke test, stable swap, and shortcut verification.
- Stable executable: `H!veAI/dev-bin/H!veAI.exe`; SHA-256 `7C8305D5C5EFFAA6B987597C41E42637532644C8CD8FC1BE1FECD20164BAF1F9`; size `18009088` bytes.
- Stable icon: `H!veAI/dev-bin/H!veAI.ico`; SHA-256 `D83ED52300040617D1DA2502E35DC25FEC66AF030CDF444DD52B491716B0940E`; size `143206` bytes.
- Desktop shortcut `C:\Users\sekip\Desktop\H!veAI.lnk` targets the stable EXE directly, uses the stable icon with index 0, has empty arguments, and works from `H!veAI/dev-bin`.
- Canonical opening-video SHA-256 remains `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686` in both repository and canonical source. Background remains `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`.
- X01 source fix remains present (`CREATE_NO_WINDOW` / `0x08000000` production Git path); X02 source/config fixes remain present (audible playback setup and WebView2 autoplay policy).
- Installer scan: no `.msi`, `.msix`, `.appx`, `.appxbundle`, `.msixbundle`, or `.wixpdb` files under `H!veAI`.

## TRACKER TRUTH

- M10 state: IMPLEMENTATION COMPLETE / PENDING INDEPENDENT AUDIT.
- Strict completed count: `10 / 20 = 50%`; M10 is not counted closed by the builder.
- M11/M12: PLANNED/BLOCKED. No Project Dashboard manifest runtime ingestion, visible UI, M11, or M12 work was implemented.
- M09 parser behavior and the fixed X01/X02 native behavior remain preserved except the explicit ownership/history integration required by M10.

## COMMITS / REMOTE

- Implementation commit: `b4ca040ac9df94e2f2d7f13eaaa77f644327f5ae` (`Implement M10 workflow state machine`).
- Log/docs commit: SELF / verified after final push in this session.
- Final local HEAD: SELF / verified after final push in this session.
- `origin/H!veAI` HEAD: SELF / verified after final push in this session.
- Final equality proof: `HEAD == origin/H!veAI`; `0 0` divergence, verified after the final pushed log commit.

## PRE-PUSH SELF-AUDIT

1. Can a normal transition bypass the canonical matrix? **No.**
2. Can stale frontend state cause a double transition? **No; expected state is read and checked inside the SQLite transaction.**
3. Can one retried request create duplicate events? **No; deterministic task/request event identity returns the original result.**
4. Can evidence from another task/project satisfy a gate? **No; every table-backed ref is checked for task/project ownership.**
5. Can a human override happen without rationale/history? **No; rationale, decision, and override event are atomic and bounded.**
6. Can restart leave a transient state falsely active? **No; startup recovery demotes M10-history transient states and is idempotent.**
7. Can M09 reparse reset an M10-managed state? **No; parser UPSERT preserves state when workflow history exists.**
8. Can M09 stale cleanup delete a task with M10 history and cascade events? **No; managed stale tasks are retained and source-retired.**
9. Can retired reappearance lose identity/history? **No; stable task ID, created time, state, and events are preserved.**
10. Did visible UI/M11/M12/Project Dashboard runtime ingestion slip into scope? **No.**
11. Did canonical assets or native UX hotfix regress? **No; hashes and source/config checks pass.**
12. Are tracker/log claims truthful and will final local HEAD equal origin after the final pushed log? **Yes; final proof is recorded after push below.**

M10 remains pending independent strict audit. Stop after the final pushed log/equality proof.
