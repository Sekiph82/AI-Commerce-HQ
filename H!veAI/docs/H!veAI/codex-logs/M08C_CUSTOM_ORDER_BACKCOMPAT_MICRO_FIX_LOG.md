# M08C Custom Order Backward-Compatibility Micro Fix Codex Log

Date: 2026-08-25
Product: H!veAI
Branch: H!veAI
Synchronized base HEAD: `b1dcc8cf0fbcb723c281ac5b1f73d8dc97ceff81`

## Scope

This was the single M08C compatibility micro-fix. No UI, presentation, IPC,
permissions, migration schema, publisher, installer, or M09 work was started.

Changed files:

- `H!veAI/src-tauri/src/task_sources.rs`
- `H!veAI/TASKS.md`
- `H!veAI/docs/H!veAI/codex-logs/M08C_CUSTOM_ORDER_BACKCOMPAT_MICRO_FIX_LOG.md`

Historical M08, M08A, and M08B logs remain separate and unchanged.

## Self-audit

### DEFECT 1

Production function changed: `load_custom_paths()` now deserializes optional
order metadata and passes it through one normalization boundary. Valid explicit
non-negative unique contiguous orders are honored and sorted; missing, duplicate,
negative, non-contiguous, or invalid metadata preserves persisted vector order
and assigns contiguous in-memory orders. H!veAI-owned mutation writes the
normalized vector with explicit orders.

Exact test: `task_sources::tests::legacy_custom_settings_without_order_normalize_and_preserve_position`.

Pre-fix behavior that would fail the test: the historical three-entry JSON has
no order fields, so the old `#[serde(default)] order: i64` implementation gave
all entries order 0; list/discovery and a middle path-only rename therefore
lost the persisted relative position and the mutation did not prove repaired
contiguous metadata.

Post-fix behavior proved: direct old-shape SQLite settings containing vector
order `z.md, A.md, m.md` list as orders `0,1,2`; production `discover()` keeps
that custom order before `TASKS.md`; renaming only `A.md` with `order=None`
returns `z.md, renamed.md, m.md` at `0,1,2`; direct SQLite JSON inspection
proves explicit persisted orders `0,1,2` after the mutation.

PASS

### DEFECT 2

Exact test: `task_sources::tests::custom_sources_order_before_standard_authority_order_in_persisted_inventory`.

Why it has >=3 CUSTOM + multiple STANDARD: it creates `custom-a.md`,
`custom-b.md`, and `custom-c.md`, explicitly reorders `custom-c.md` to position
zero, and also creates `TASKS.md`, `PLANS.md`, and `ROADMAP.md`.

Exact asserted order: `custom-c.md`, `custom-a.md`, `custom-b.md`, `TASKS.md`,
`PLANS.md`, `ROADMAP.md`.

PASS

## Test evidence

Focused Rust task-source suite: 35/35 passed. Every focused test passed:

- `archived_project_rejects_all_discovery_mutations`
- `bounded_tasks_and_handoffs_discovery`
- `candidate_file_limit_is_enforced`
- `case_insensitive_standard_filename_has_no_duplicate`
- `custom_available_to_missing_reconciliation_is_persisted`
- `custom_directory_and_remove_are_production_backed`
- `custom_path_equivalent_inputs_dedupe`
- `custom_path_listing_reports_available_and_missing`
- `custom_sources_order_before_standard_authority_order_in_persisted_inventory`
- `custom_update_order_remove_equivalence_and_containment_are_safe`
- `deleted_standard_is_reconciled_and_custom_remains_missing`
- `deleted_standard_row_is_removed_while_legacy_row_is_unchanged`
- `depth_limit_warning_rejects_first_source_beyond_boundary`
- `discovery_does_not_mutate_registered_project_tree`
- `ignored_trees_are_not_traversed`
- `legacy_custom_settings_without_order_normalize_and_preserve_position`
- `metadata_contains_authority_priority_depth_and_hash`
- `missing_project_is_bounded_error`
- `nested_source_depth_is_bounded`
- `non_git_discovery_does_not_write_tasks`
- `outside_and_parent_escape_are_rejected`
- `oversized_source_is_bounded`
- `persisted_content_hash_changes_without_changing_m08_identity`
- `preversion_shape_requires_deterministic_identity_and_rich_fields`
- `project_sources_persist_owner_schema_hash_and_idempotent_identity`
- `repeated_discovery_is_idempotent_and_hash_changes`
- `root_handoff_variant_is_classified_as_handoff`
- `root_tasks_discovery`
- `safe_custom_file_and_missing_path_are_persisted`
- `source_order_is_deterministic_by_priority_then_path`
- `symlink_escape_is_rejected_or_records_environment_limit`
- `unavailable_registered_root_is_bounded_error`
- `unreadable_failpoint_preserves_other_valid_source`
- `unrelated_legacy_project_source_survives_reconciliation`
- `visited_entry_limit_is_structured`

Focused frontend M08 suite: `tests/m08-task-sources-focused.test.tsx`, 20/20
passed unchanged.

Full frontend regression: 68/68 passed.

Full automated/security/native gates passed:

- `npm run typecheck`
- `npm run build`
- `npm audit --audit-level=high` (0 vulnerabilities)
- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`
- `cargo check --manifest-path src-tauri/Cargo.toml`
- `cargo test --manifest-path src-tauri/Cargo.toml` (137/137)
- `cargo build --manifest-path src-tauri/Cargo.toml`
- `scripts/tests/publish-dev-qa-failure-harness.ps1` (all 9 scenarios PASS)
- `scripts/publish-dev-qa.ps1` (Tauri production `--no-bundle`, smoke-tested)

## Publication evidence

Stable executable: `H!veAI/dev-bin/H!veAI.exe`

- SHA-256: `A972FCDC1D15FF9DA2027C6C4A50EAC4E7BD84E55660F312AE9F96C56AD58DD2`
- Size: `17333760` bytes
- Desktop shortcut target: `H!veAI/dev-bin/H!veAI.exe`
- Desktop shortcut icon: `H!veAI/dev-bin/H!veAI.ico,0`
- No Windows installer created.

Canonical repository asset hashes remain unchanged:

- `src/assets/hiveai-app-background.png`: `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`
- `src/assets/opening-video.mp4`: `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`
- `src/assets/hiveai-logo.png`: `C773839E949222ED787972964AB3EEF27DF0F7D885AF78E7A0E6E340EF6E726C`

The Windows physical symlink/junction environment limitation remains the
previously documented OS error 1314 and is not part of this micro-fix.

## Final status

M08C automated implementation and evidence: PASS.

Independent strict re-audit remains open. Native `/tasks` visual status remains
`PENDING USER VISUAL ACCEPTANCE`. M09 remains BLOCKED/UNSTARTED. No installer
was created and work stops after M08C.

Publication commit equality after push will be recorded in this same immutable
log before final closure.
