# M09 Task Intelligence Parser Codex Log

Date: 2026-08-25
Product: H!veAI
Branch: H!veAI
Synchronized starting HEAD: `69e312d54380b95bcef1cfc915e6e50299a54f62`

## Scope and ownership

M09 was executed as one continuous P01-P10 milestone. No visible H!veAI UI,
route layout, visual asset, installer, or M10 work was introduced.

Changed files:

- `H!veAI/src-tauri/src/task_intelligence.rs`
- `H!veAI/src-tauri/src/lib.rs`
- `H!veAI/src-tauri/permissions/foundation.toml`
- `H!veAI/src-tauri/capabilities/default.json`
- `H!veAI/src/taskIntelligence.ts`
- `H!veAI/tests/m09-task-intelligence-focused.test.ts`
- `H!veAI/tests/fixtures/m09/generic-checklist.md`
- `H!veAI/tests/fixtures/m09/structured-metadata.md`
- `H!veAI/tests/fixtures/m09/handoff.md`
- `H!veAI/tests/fixtures/m09/formulab.md`
- `H!veAI/tests/fixtures/m09/scrubbots.md`
- `H!veAI/tests/fixtures/m09/fmcg-erp-system.md`
- `H!veAI/tests/fixtures/m09/duplicate-identity.md`
- `H!veAI/tests/fixtures/m09/false-positive-prose.md`
- `H!veAI/ARCHITECTURE.md`
- `H!veAI/TASKS.md`
- this log

M09 owns `m09src:<sha256>` task-source rows and `m09task:<sha256>` task rows.
Task rows carry `metadata_json.owner = M09_TASK_INTELLIGENCE_PARSER` and
`schemaVersion = 1`. M09 dependency edges use `SOURCE_EXPLICIT`; M09 never
writes `task_events`. Reconciliation deletes only M09-owned rows and preserves
unrelated/legacy rows, settings, source files, and project bytes.

The neutral storage mapping is exact: explicit DONE -> `TASK_COMPLETE`,
explicit BLOCKED -> `BLOCKED`, all other parsed statuses -> `BACKLOG`.
Richer `parsedStatus` remains in M09 metadata for M10 and no operational M10
state is inferred.

The structured-label grammar is case-insensitive: `Blocker(s)`, `Blocked by`,
`Depends on`, `Dependency/Dependencies`, `Next/Next step`, `Owner/Actor/
Required actor`, `Waiting for`, `External/External wait`, and `Acceptance`,
`Acceptance criteria`, `AC`, `Definition of Done`. Unknown actors remain NULL.

Confidence is deterministic and capped at 1.00: checklist base 0.70, explicit
row base 0.65, explicit ID +0.10, explicit structured status +0.05,
heading context +0.05, structured metadata +0.05, evidenced adapter convention
+0.05. Evidence includes source path/hash, one-based line range, heading path,
and source-evidenced locator text/ID.

## P01

Production symbol(s): `parse`, `is_parser_source`, `read_authoritative_source`,
`read_bounded_text`.

Exact focused test(s): `p01_inventory_boundary_excludes_instruction_bullets`,
`p01_outside_root_source_is_rejected_by_production_reader`,
`p01_source_change_is_retried_once_then_warned`,
`p01_invalid_utf8_isolated_from_valid_source`.

Pre-fix/missing behavior the test would catch: bypassing M08 inventory,
trusting an absolute/traversal path, accepting stale content hashes, or
allowing one invalid source to abort valid sources.

Post-fix behavior proved: parse begins with M08 discovery, accepts only owned
AVAILABLE task-bearing rows, reconstructs and physically contains paths,
bounded-reads UTF-8 bytes, compares SHA-256, retries once on mismatch, and
isolates invalid UTF-8. Instruction bullets do not become tasks.

PASS

## P02

Production symbol(s): `ParsedTask`, `TaskEvidenceLocator`, `TaskConfidence`,
`task_id`, `source_id`.

Exact focused test(s): `p02_ids_are_stable_and_project_scoped`,
`p02_same_source_text_in_two_projects_never_collides`.

Pre-fix/missing behavior the test would catch: line-number identity, duplicate
collision, or cross-project collision.

Post-fix behavior proved: explicit/fallback identities use project, source,
heading/title and deterministic duplicate ordinal; repeated parsing is stable,
duplicate siblings are distinct, and identical text in different projects is
different.

PASS

## P03

Production symbol(s): `task_line`, `heading`, `parse_document`.

Exact focused test(s): `p03_checkbox_and_neutral_storage_mapping`,
`p10_locator_and_confidence_are_bounded_and_deterministic`.

Pre-fix/missing behavior the test would catch: prose bullets becoming tasks,
missing checkbox states, or inferred operational workflow states.

Post-fix behavior proved: all four checkbox markers, explicit task rows and
heading context are parsed deterministically; prose is ignored; storage stays
within the neutral M09 mapping.

PASS

## P04

Production symbol(s): `fields_for`, `normalize_actor`, `resolve_dependencies`,
`persist`.

Exact focused test(s): `p04_structured_metadata_and_unknown_actor`,
`p04_dependency_resolves_only_to_an_unambiguous_explicit_id`,
`p04_ambiguous_dependency_stays_metadata_only_with_warning`.

Pre-fix/missing behavior the test would catch: free-prose actor/dependency
guessing, wrong child association, or false dependency edges.

Post-fix behavior proved: explicit metadata attaches to the nearest task,
known actors normalize, unknown actors remain NULL, exactly one explicit ID
resolves to `SOURCE_EXPLICIT`, and unresolved/ambiguous references remain
metadata with structured warnings and no false edge.

PASS

## P05

Production symbol(s): `HandoffSummary`, handoff extraction in `parse_document`,
persisted snapshot `list` path.

Exact focused test(s): `p05_handoff_narrative_and_checklist_are_separate`.

Pre-fix/missing behavior the test would catch: handoff narrative becoming fake
tasks or list rereading source files.

Post-fix behavior proved: Current and Waiting sections are retained as line
evidence, narrative is summary-only, and a checklist remains a task. The
persisted snapshot returned by `list` contains the handoff without file reads.

PASS

## P06

Production symbol(s): `adapter_for`, `ParserAdapterIdentity`.

Exact focused test(s): `p06_adapter_selection_is_explicit`,
`p06_registered_adapter_fixtures_use_evidenced_conventions`.

Pre-fix/missing behavior the test would catch: name-only aliases, accidental
selection for similarly named projects, or invented project evidence.

Post-fix behavior proved: exact Registry names select `formulab`, `scrubbots`,
and `fmcg-erp-system`; all other names use generic. Sanitized fixtures prove
FVL notation, TASK-XXX notation, and FMCG module/phase checklist conventions.

Inspected read-only Registry/inventory evidence:

- FormuLab: `C:\Users\sekip\Desktop\FormuLab\PROGRESS.md`; FVL-numbered work notation.
- ScrubBots: `C:\Users\sekip\Desktop\ScrubBots\tasks.md`; TASK-XXX task IDs.
- FMCG ERP: `C:\Users\sekip\Desktop\fmcg-erp-system\TASKS.md` and `PLANS.md`; module/phase headings with checklist tasks.

No full private document content was copied into the repository.

PASS

## P07

Production symbol(s): `persist`, `list`, M09-owned SQL reconciliation.

Exact focused test(s): `p07_owned_sql_reconciliation_preserves_events_and_is_idempotent`,
`p07_unrelated_rows_and_project_bytes_are_preserved`.

Pre-fix/missing behavior the test would catch: blanket project deletion,
duplicate unchanged rows, task-event writes, or managed-project mutation.

Post-fix behavior proved: repeated parses keep IDs/counts stable, owned rows
reconcile transactionally, direct SQL confirms owner/schema and zero task events,
legacy task/source/settings rows survive byte-for-byte, and source bytes remain
unchanged.

PASS

## P08

Production symbol(s): `hiveai_task_intelligence_parse`,
`hiveai_task_intelligence_list`, `src/taskIntelligence.ts`,
`allow-task-intelligence`.

Exact focused test(s): TypeScript `invokes the bounded parse command with the
project id`; TypeScript `invokes the persisted list command with the project
id`; Rust `p08_list_reads_persisted_snapshot_only`.

Pre-fix/missing behavior the test would catch: incorrect command names/args,
broad permissions, route-driven hidden parsing, or list crawling managed files.

Post-fix behavior proved: only two narrow commands and one narrow permission
were added; parse/list are explicit; list returns persisted state only. No UI
route imports or invokes the new module.

PASS

## P09

Production symbol(s): project status boundary, `ParserWarning`,
`trim_warnings`.

Exact focused test(s): `p09_archived_project_is_rejected`,
`p09_missing_project_is_rejected`, `p09_warning_bound_is_structured`,
`p01_invalid_utf8_isolated_from_valid_source`.

Pre-fix/missing behavior the test would catch: parsing unavailable/archived
projects, unbounded warnings, or source-body leakage in errors.

Post-fix behavior proved: status errors are bounded, warnings use stable codes,
warning retention is capped at 512 with `WARNING_LIMIT_REACHED`, and messages
contain only bounded path/error-class evidence.

PASS

## P10

Production symbol(s): `TaskEvidenceLocator`, `TaskConfidence`, `evidence`,
confidence scoring in `parse_document`.

Exact focused test(s): `p10_locator_and_confidence_are_bounded_and_deterministic`,
`p06_registered_adapter_fixtures_use_evidenced_conventions`.

Pre-fix/missing behavior the test would catch: sibling lines leaking into a
locator, unbounded/non-repeatable scores, or adapter bonus without a matched
convention.

Post-fix behavior proved: child metadata is included, the sibling is excluded,
score/reasons are repeatable and bounded, and adapter convention evidence is
required before the bonus.

PASS

## Focused test results

Focused Rust task-intelligence suite: 20/20 PASS:

- `p01_invalid_utf8_isolated_from_valid_source` PASS
- `p01_inventory_boundary_excludes_instruction_bullets` PASS
- `p01_outside_root_source_is_rejected_by_production_reader` PASS
- `p01_source_change_is_retried_once_then_warned` PASS
- `p02_ids_are_stable_and_project_scoped` PASS
- `p02_same_source_text_in_two_projects_never_collides` PASS
- `p03_checkbox_and_neutral_storage_mapping` PASS
- `p04_ambiguous_dependency_stays_metadata_only_with_warning` PASS
- `p04_dependency_resolves_only_to_an_unambiguous_explicit_id` PASS
- `p04_structured_metadata_and_unknown_actor` PASS
- `p05_handoff_narrative_and_checklist_are_separate` PASS
- `p06_adapter_selection_is_explicit` PASS
- `p06_registered_adapter_fixtures_use_evidenced_conventions` PASS
- `p07_owned_sql_reconciliation_preserves_events_and_is_idempotent` PASS
- `p07_unrelated_rows_and_project_bytes_are_preserved` PASS
- `p08_list_reads_persisted_snapshot_only` PASS
- `p09_archived_project_is_rejected` PASS
- `p09_missing_project_is_rejected` PASS
- `p09_warning_bound_is_structured` PASS
- `p10_locator_and_confidence_are_bounded_and_deterministic` PASS

Focused TypeScript M09 suite: 2/2 PASS:

- `invokes the bounded parse command with the project id` PASS
- `invokes the persisted list command with the project id` PASS

## Full verification and publication

- Full frontend regression: 70/70 PASS.
- Full Rust regression: 157/157 PASS.
- `npm run typecheck`: PASS.
- `npm run build`: PASS.
- `npm audit --audit-level=high`: PASS, 0 vulnerabilities.
- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`: PASS.
- `cargo check --manifest-path src-tauri/Cargo.toml`: PASS.
- `cargo build --manifest-path src-tauri/Cargo.toml`: PASS.
- `scripts/tests/publish-dev-qa-failure-harness.ps1`: all 9 scenarios PASS.
- `scripts/publish-dev-qa.ps1`: PASS, production Tauri `--no-bundle`, smoke-tested.

Stable executable: `H!veAI/dev-bin/H!veAI.exe`

- SHA-256: `1C18D78BD3EAF6CE4C345769FC6DAEA856A77BC8BFD256F9AAD4524E32F626B9`
- Size: `17622016` bytes.
- Desktop shortcut target: `H!veAI/dev-bin/H!veAI.exe`.
- Shortcut icon: `H!veAI/dev-bin/H!veAI.ico,0`.
- No installer created; no browser-hosted H!veAI shell.

Canonical repository asset hashes unchanged:

- `src/assets/hiveai-app-background.png`: `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`
- `src/assets/opening-video.mp4`: `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`
- `src/assets/hiveai-logo.png`: `C773839E949222ED787972964AB3EEF27DF0F7D885AF78E7A0E6E340EF6E726C`

## Final status

M09 implementation complete, PENDING INDEPENDENT STRICT AUDIT. M10 remains
BLOCKED/UNSTARTED. Historical M00-M08 logs remain separate and unchanged.

Implementation/publication commit: `cdac3774a403a04ae94db707483a8dfad27efa52`.
It was pushed to `origin/H!veAI`, and local/origin equality was verified. This
immediate documentation-only follow-up records the final publication evidence;
its final commit is the branch HEAD.
