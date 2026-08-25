# M08B Task Source Discovery Final Strict Closure Log

Date: 2026-08-25
Branch: `H!veAI`
Synchronized base HEAD: `d51bfad` (`H!veAI: add M08B final strict closure prompt`)
Active prompt: `docs/H!veAI/prompts/M08B_TASK_SOURCE_DISCOVERY_FINAL_STRICT_CLOSURE_PROMPT.md`

## Scope

M08B is the only active remediation. Original M08 and M08A strict audits remain
historical FAIL gates. Historical M08 and M08A logs were not modified. M09 was
not started and no installer was created.

## Changed files

- `H!veAI/TASKS.md`
- `H!veAI/src-tauri/src/task_sources.rs`
- `H!veAI/tests/m08-task-sources-focused.test.tsx`
- `H!veAI/docs/H!veAI/codex-logs/M08B_TASK_SOURCE_DISCOVERY_FINAL_STRICT_CLOSURE_LOG.md`

## B01-B05 closure evidence

### B01 positional reorder

`custom_path_update()` identifies the item, validates a renamed target with
the existing physical containment and dedupe rules, removes the item from its
ordered vector, inserts it at `clamp(requestedOrder, 0, lenAfterRemoval)`, and
renumbers every configured path contiguously. No lexical tie-break can override
an explicit insertion position. A path-only update preserves the current
relative position. Deterministic ids follow the normalized target path.

Rust test `custom_update_order_remove_equivalence_and_containment_are_safe`
proves three custom paths: last-to-first, first-to-last, middle-to-middle,
reorder without rename, rename without explicit reorder, containment rejection,
duplicate-target rejection, and equivalent-case removal.

The mounted UI test `custom_update_reorder_executes_and_refreshes_visible_inventory`
uses A and B, requests B order 0, receives refreshed B-then-A custom state, and
asserts the DOM order is B before A.

### B02 safe pre-version adoption

Pre-version adoption now requires all of: no declared owner/schema; matching
`projectId`; `relativePath` equal to persisted `source_path` under normalized
Windows-insensitive comparison; old origin; deterministic legacy id derived
from `projectId|origin|normalizedRelativePath`; string `sourceKind`, `status`,
`authorityClass`, `absolutePath`; numeric `priority` and `depth`; and a string
warnings array. Rows with deceptive partial shape or another owner survive.

`preversion_shape_requires_deterministic_identity_and_rich_fields` directly
proves the real rich legacy row is reconciled while deceptive and foreign-rich
rows remain.

### B03 persisted SQL evidence

`persisted_content_hash_changes_without_changing_m08_identity` reads both
hashes directly from `project_sources`, proving same deterministic id, one
owned row, and changed persisted hash after file mutation.

`deleted_standard_row_is_removed_while_legacy_row_is_unchanged` proves the
owned STANDARD row disappears after physical deletion while the unrelated row's
hash and metadata JSON remain byte-for-byte unchanged.

`custom_sources_order_before_standard_authority_order_in_persisted_inventory`
proves production discovery order is custom B, custom A, TASKS, PLANS, ROADMAP:
custom order first, then documented standard authority priority.

### B04 mounted transitions

`custom_add_command_uses_native_boundary` enters a new path, invokes add for
the selected project, returns a refreshed custom list, and asserts the new
path becomes visible without remounting.

`custom_update_reorder_executes_and_refreshes_visible_inventory` proves the
update command, refreshed two-item state, and visible B-before-A DOM order.

`renders_source_kind_and_origin_columns` directly asserts path, kind, origin,
authority plus numeric priority, modified timestamp, and status in the real
rendered row. Existing stale list/add/remove, error, empty, rescan, and browser
preview tests remain green.

## Focused Rust tests: 34/34 PASS

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
`persisted_content_hash_changes_without_changing_m08_identity`;
`deleted_standard_row_is_removed_while_legacy_row_is_unchanged`;
`preversion_shape_requires_deterministic_identity_and_rich_fields`;
`custom_sources_order_before_standard_authority_order_in_persisted_inventory`;
`unrelated_legacy_project_source_survives_reconciliation`;
`custom_available_to_missing_reconciliation_is_persisted`;
`unreadable_failpoint_preserves_other_valid_source`;
`custom_update_order_remove_equivalence_and_containment_are_safe`;
`archived_project_rejects_all_discovery_mutations`.

Every listed Rust test result is PASS. The symlink/junction case remains exact
`UNVERIFIED`: Windows link creation returned `A required privilege is not held
by the client. (os error 1314)`. Unreadable evidence is
`REAL_PRODUCTION_PATH_WITH_TEST_FAILPOINT`.

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

Every listed frontend test result is PASS.

## Full gates and publication

PASS: `npm run typecheck`.

PASS: full frontend regression, `68/68`.

PASS: `npm run build`.

PASS: `npm audit --audit-level=high`, 0 vulnerabilities.

PASS: `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`.

PASS: `cargo check --manifest-path src-tauri/Cargo.toml`.

PASS: full Rust regression, `136/136`.

PASS: `cargo build --manifest-path src-tauri/Cargo.toml`.

PASS: publisher failure harness, all nine cases.

PASS: production publisher `tauri build --no-bundle`, candidate smoke/readiness,
rollback, shortcut, no-localhost, and no-visible-console checks.

Stable executable: `H!veAI/dev-bin/H!veAI.exe`

Stable executable SHA-256: `1F74CD17BB1313E16D03A5467FC6A546A70A1B2D4F5B17C5C6BD4C7F4D95BE3A`

Stable executable size: `17,354,240` bytes.

Desktop shortcut targets `H!veAI/dev-bin/H!veAI.exe` and uses
`H!veAI/dev-bin/H!veAI.ico,0` derived from the canonical small logo.

Canonical background SHA-256 remains
`7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`.
Canonical opening-video SHA-256 remains
`A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`.
No canonical PNG/MP4/logo bytes changed.

## Final state

M08B automated closure is
`PENDING INDEPENDENT STRICT RE-AUDIT + USER VISUAL ACCEPTANCE`.
Native `/tasks` status remains `PENDING USER VISUAL ACCEPTANCE`.
No installer was created. No M09 work was started.

Publication commit equality verified after push: local HEAD and
`origin/H!veAI` were both `8d8327e4a210d896fccc809270b836b04305cf6d`.
