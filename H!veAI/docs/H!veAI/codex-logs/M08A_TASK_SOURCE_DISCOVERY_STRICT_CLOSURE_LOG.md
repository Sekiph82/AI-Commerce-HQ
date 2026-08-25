# M08A Task Source Discovery Strict Closure Codex Log

Date: 2026-08-25
Branch: `H!veAI`
Synchronized base HEAD: `c4190ac` (`H!veAI: add M08 strict closure remediation prompt`)
Active prompt: `docs/H!veAI/prompts/M08A_TASK_SOURCE_DISCOVERY_STRICT_CLOSURE_PROMPT.md`

## Scope and historical boundary

M08A is the single active remediation for the independent M08 strict-audit
FAIL. The historical M08 builder log and strict audit were preserved unchanged.
M09 was not started and no installer was created.

## Changed files

`TASKS.md`; `src-tauri/permissions/foundation.toml`;
`src-tauri/src/lib.rs`; `src-tauri/src/task_sources.rs`; `src/pages.tsx`;
`src/taskSources.ts`; `tests/m08-task-sources-focused.test.tsx`; and this new
log.

## Finding closure

- F01: Discovery now uses `MAX_DISCOVERY_DEPTH = 4`,
  `MAX_CANDIDATE_FILES = 512`, `MAX_VISITED_ENTRIES = 4096`, and
  `MAX_SOURCE_BYTES = 2097152`. Root and approved-directory enumeration count
  visited entries; depth, candidate, and visited-entry limits produce a
  persisted synthetic `DISCOVERY_WARNING` record with `LIMIT_REACHED` status
  and structured warnings. The first source beyond depth four is rejected.
- F02: Added `hiveai_task_source_custom_path_update` with safe target update,
  explicit persisted order, deterministic custom-first ordering, and
  normalized case/slash-equivalent remove-by-path behavior. Update reuses
  containment and dedupe validation.
- F03: `project_sources` reconciliation deletes and replaces only rows owned by
  `owner = M08_TASK_SOURCE_DISCOVERY` or proven compatible M08 source metadata.
  Unrelated legacy rows survive. Every M08 row has `schemaVersion = 1` and the
  explicit owner marker in `metadata_json`; reconciliation remains transactional.
- F04: Task Sources list and custom mutation completions carry the current
  project/request generation. Delayed project-A list, add, and remove results
  cannot reclaim project-B UI.
- F05: Mounted frontend tests exercise real add, remove, update/reorder, error,
  empty, rescan replacement, stale list, stale mutation, table metadata, and
  browser-preview transitions. The UI does not claim parsed task/workflow data.
- F06: Rust tests directly inspect SQLite row counts, deterministic ids,
  persisted hashes, ownership metadata, legacy preservation, custom
  AVAILABLE-to-MISSING transition, warnings, exact depth, update/order/remove,
  archived policy, and unreadable-source failpoint isolation.
- F07: This remediation log records every focused test name and result and is
  the new immutable evidence file; the historical M08 log was not rewritten.
- N01: ACTIVE projects are allowed; MISSING roots return bounded unavailable
  errors; ARCHIVED projects reject discovery and custom mutation.
- N02: Custom status listing canonicalizes existing targets and returns
  `OUTSIDE_ROOT` or `UNREADABLE` instead of reporting an escaped target as
  ordinary `CONFIGURED`.

## Contracts

Custom paths are stored in H!veAI-owned settings under
`task_sources.custom_paths.<projectId>`. Each entry stores display path,
normalized path, deterministic id, and integer order. Ordering is custom first,
then configured order, then freshness evidence, then normalized relative path;
standard sources retain authority priority. No source body is returned and no
project files, `tasks`, or `task_sources` rows are written by M08A.

Native commands are:

- `hiveai_task_sources_discover`
- `hiveai_task_sources_list`
- `hiveai_task_source_custom_paths_list`
- `hiveai_task_source_custom_path_add`
- `hiveai_task_source_custom_path_remove`
- `hiveai_task_source_custom_path_update`

The dedicated `allow-task-source-discovery` permission contains exactly these
task-source commands and remains attached through the existing default
capability. No unrestricted filesystem, shell, network, or public test bypass
was added. The unreadable evidence uses a private `cfg(test)` failpoint at the
production hash/read boundary.

## Focused Rust tests: 30/30 PASS

`root_tasks_discovery`; `case_insensitive_standard_filename_has_no_duplicate`;
`bounded_tasks_and_handoffs_discovery`; `ignored_trees_are_not_traversed`;
`outside_and_parent_escape_are_rejected`;
`safe_custom_file_and_missing_path_are_persisted`;
`custom_path_equivalent_inputs_dedupe`;
`repeated_discovery_is_idempotent_and_hash_changes`;
`deleted_standard_is_reconciled_and_custom_remains_missing`;
`oversized_source_is_bounded`; `non_git_discovery_does_not_write_tasks`;
`custom_directory_and_remove_are_production_backed`;
`missing_project_is_bounded_error`; `unavailable_registered_root_is_bounded_error`;
`source_order_is_deterministic_by_priority_then_path`;
`discovery_does_not_mutate_registered_project_tree`;
`metadata_contains_authority_priority_depth_and_hash`;
`nested_source_depth_is_bounded`; `custom_path_listing_reports_available_and_missing`;
`candidate_file_limit_is_enforced`; `root_handoff_variant_is_classified_as_handoff`;
`symlink_escape_is_rejected_or_records_environment_limit`;
`depth_limit_warning_rejects_first_source_beyond_boundary`;
`visited_entry_limit_warning_is_structured`;
`project_sources_persist_owner_schema_hash_and_idempotent_identity`;
`unrelated_legacy_project_source_survives_reconciliation`;
`custom_available_to_missing_reconciliation_is_persisted`;
`unreadable_failpoint_preserves_other_valid_source`;
`custom_update_order_remove_equivalence_and_containment_are_safe`;
`archived_project_rejects_all_discovery_mutations`.

Symlink/junction containment remains `UNVERIFIED`: Windows link creation was
denied with `A required privilege is not held by the client. (os error 1314)`.
Unreadable classification is `REAL_PRODUCTION_PATH_WITH_TEST_FAILPOINT`.

## Focused frontend tests: 20/20 PASS

`native_tasks_uses_selected_live_registry_project`;
`shows_loading_before_source_response`;
`renders_real_source_metadata_and_rescan_refreshes`;
`custom_add_command_uses_native_boundary`;
`empty_response_renders_truthful_empty_ui`;
`project_change_requests_new_selected_project_inventory`;
`browser_preview_does_not_invoke_native_filesystem_commands`;
`shows_custom_path_status`; `does_not_render_task_workflow_columns`;
`rescan_refreshes_inventory_for_selected_project`;
`keeps_custom_path_input_scoped_to_workspace`;
`renders_source_kind_and_origin_columns`;
`renders_selected_project_identity_from_registry`;
`delayed_project_a_response_cannot_replace_project_b_inventory`;
`stale_custom_add_completion_cannot_refresh_project_a_after_project_b_selection`;
`stale_custom_remove_completion_cannot_refresh_project_a_after_project_b_selection`;
`rejected_native_list_renders_truthful_error_ui`;
`rescan_replaces_visible_source_row_with_discover_response`;
`custom_remove_executes_and_refreshes_visible_inventory`;
`custom_update_reorder_executes_and_refreshes_visible_inventory`.

## Verification and publication

PASS: focused Rust task-source tests, 30/30.

PASS: focused frontend Task Sources tests, 20/20.

PASS: `npm run typecheck`.

PASS: full frontend regression, 68/68.

PASS: `npm run build`.

PASS: `npm audit --audit-level=high`, 0 vulnerabilities.

PASS: `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`.

PASS: `cargo check --manifest-path src-tauri/Cargo.toml`.

PASS: full Rust regression, 132/132.

PASS: `cargo build --manifest-path src-tauri/Cargo.toml`.

PASS: `scripts/tests/publish-dev-qa-failure-harness.ps1`, all nine cases.

PASS: `scripts/publish-dev-qa.ps1`, production Tauri `--no-bundle`, smoke,
readiness, no-localhost, no-visible-console, rollback, SHA-256, shortcut, and
canonical-asset checks.

Stable executable: `dev-bin/H!veAI.exe`

Stable executable SHA-256: `DBD5BBA99BEC8F0A2860425F161D8022E2328CC9C944766C29C690407EAC985D`

Stable executable size: `17,375,744` bytes.

Shortcut target: `Desktop/H!veAI.lnk` -> `dev-bin/H!veAI.exe`.

Shortcut icon: `dev-bin/H!veAI.ico,0`, derived from the canonical small logo.

Canonical visual bytes remain unchanged. Background SHA-256 is
`7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23` and
opening-video SHA-256 is
`A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`.

## Final state

M08A automated closure is clean pending independent strict re-audit and user
visual acceptance. Native `/tasks` visual status remains
`PENDING USER VISUAL ACCEPTANCE`. No installer was created and M09 was not
started.

Publication commit verified immediately after push: local HEAD and
`origin/H!veAI` were both `a6474a2fb585829e88a84f0c9384d4be5ed30caa`.
