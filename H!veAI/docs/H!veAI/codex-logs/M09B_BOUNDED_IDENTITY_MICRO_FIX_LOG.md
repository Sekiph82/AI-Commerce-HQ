# M09B Bounded Identity Micro-Fix Log

Date: 2026-08-25
Product: H!veAI
Branch: H!veAI

## Start evidence

- `git fetch origin H!veAI` completed, followed by safe fast-forward from `fba3f98822678c84a03dfef3c52ffe8095b3f68c` to synchronized HEAD `8b16a4c`.
- Starting worktree was clean for tracked files. Pre-existing untracked user files `start-demo.bat` and `task.md` were preserved.
- No additional worktrees, reset, rebase, force-push, installer, M10 code, visible UI source, or canonical asset change was introduced.

R01
Production symbol(s): `parse` source-path deduplication, `task_id`, and dedicated `normalize_path_identity`.
Exact test(s): `r01_distinct_whitespace_paths_never_collide`.
Why old fba3f988 code fails: prose `normalize_text(path)` collapsed internal filename whitespace; M08-approved `plans/a b.md` and `plans/a  b.md` could share the same identity path component, and duplicate discovery could amplify the result.
Post-fix behavior: separators are canonicalized, harmless dot components are removed, meaningful filename whitespace is preserved, Windows case equivalence is applied, and duplicate discovered paths are parsed once. The production fixture returns exactly two tasks with distinct source paths, IDs, and SQLite rows.
Status: PASS

R02
Production symbol(s): `bounded_field`, source-derived heading/milestone and explicit-ID persistence, locator construction, and handoff parsing.
Exact test(s): `r02_oversized_heading_is_bounded_without_snapshot_amplification`, `r02_oversized_handoff_value_is_bounded`, `r02_oversized_explicit_id_is_bounded_and_deterministic`, `r02_bounded_snapshot_repeat_is_deterministic`.
Why old fba3f988 code fails: source-derived explicit IDs, milestone/heading paths, handoff values, and locator text were not covered by the scalar bound contract.
Post-fix behavior: every source-derived persisted scalar is UTF-8 safely bounded at 4096 bytes, emits deduplicated `FIELD_TRUNCATED` evidence naming only the field kind, and remains deterministic across repeated parses without warning amplification.
Status: PASS

E01/E02/E03/E04
Exact evidence:
- E01: `p01_retry_rechecks_physical_containment` now reaches the retry branch through a private test-only path mutation hook; the production reader re-canonicalizes and fails closed after the target changes. A Windows symlink/junction fixture was not required; no environment-limited PASS is claimed.
- E02: `p06_multiple_handoff_sources_merge_in_source_order` derives the expected order from M08 discovery, removes duplicate relative-path inventory entries, and asserts exact merged order.
- E03: `p07_removed_task_and_source_reconcile_only_stale_m09_rows` uses retained and stale M09 sources/tasks plus legacy source/task rows, removes the configured stale source, and asserts stale deletion with retained/legacy survival.
- E04: truthful adapter inspection is recorded below; generic-safe unverified adapters do not receive special bonuses.
Status: PASS

## Adapter evidence

Adapter: FormuLab
Inspected Registry/M08 source path(s): registered FormuLab `PROGRESS.md` source.
Distinct non-generic convention found: YES.
Implemented convention: task explicit IDs beginning with the evidenced `FVL-` family.
Evidence status: PASS.
Special bonus possible: YES, only for a matching `FVL-` task.

Adapter: ScrubBots
Inspected Registry/M08 source path(s): registered ScrubBots `tasks.md` source.
Distinct non-generic convention found: NO.
Implemented convention: selectable exact Registry identity with generic parser fallback.
Evidence status: UNVERIFIED.
Special bonus possible: NO.

Adapter: FMCG
Inspected Registry/M08 source path(s): registered FMCG `TASKS.md` and `PLANS.md` sources.
Distinct non-generic convention found: NO.
Implemented convention: selectable exact Registry identity with generic parser fallback.
Evidence status: UNVERIFIED.
Special bonus possible: NO.

## Focused tests

All 50 `task_intelligence::tests` passed. The required direct tests passed individually:

- `r01_distinct_whitespace_paths_never_collide`: PASS.
- `r02_oversized_heading_is_bounded_without_snapshot_amplification`: PASS.
- `r02_oversized_handoff_value_is_bounded`: PASS.
- `r02_oversized_explicit_id_is_bounded_and_deterministic`: PASS.
- `r02_bounded_snapshot_repeat_is_deterministic`: PASS.
- `p01_retry_rechecks_physical_containment`: PASS.
- `p06_multiple_handoff_sources_merge_in_source_order`: PASS.
- `p07_removed_task_and_source_reconcile_only_stale_m09_rows`: PASS.

The complete prior M09A focused set also remained green: all `p01` through `p10` parser tests passed.

## Regression and security gates

- `npm run typecheck`: PASS.
- `npm test -- --run`: PASS, 5 files / 70 tests.
- `npm run build`: PASS.
- `npm audit --audit-level=high`: PASS, 0 vulnerabilities.
- `cargo fmt --manifest-path H!veAI/src-tauri/Cargo.toml -- --check`: PASS.
- `cargo check --manifest-path H!veAI/src-tauri/Cargo.toml`: PASS.
- `cargo test --manifest-path H!veAI/src-tauri/Cargo.toml`: PASS, 187 tests passed, 0 failed.
- `cargo build --manifest-path H!veAI/src-tauri/Cargo.toml`: PASS.
- Existing publisher failure harness: PASS, all governed failure/rollback assertions.
- Governed production publisher: PASS, Tauri production `--no-bundle`, candidate/stable smoke, frontend-ready marker, no forbidden dev ports, and no visible console host.

## Scope and publication

- Canonical asset hashes unchanged: background `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`; opening video `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`; H!veAI logo `C773839E949222ED787972964AB3EEF27DF0F7D885AF78E7A0E6E340EF6E726C`.
- No visible H!veAI UI source changed.
- Stable `H!veAI.exe` remains the direct target of `Desktop\H!veAI.lnk`; no installer was created.
- Terminal-popup and startup-audio defects remain intentionally unchanged and separately deferred.
- Stable executable SHA-256: `91E5E511AEEA718CCED7C70A6C0B30B1A45B56DA7D6679C37DB7C761D275E314`; size: `17733120` bytes.
- Shortcut target: `H!veAI/dev-bin/H!veAI.exe`; icon: `H!veAI/dev-bin/H!veAI.ico,0`.

## Commit/equality evidence

- Implementation commit: `f919fb664c8b0f74c9a7c626e80e0db59d34fad3`.
- Final local/origin equality was verified at `f919fb664c8b0f74c9a7c626e80e0db59d34fad3` before this final log-record commit.
- Final log-record commit is pushed after that equality verification.
- Historical M09 and M09A logs remain unchanged.

M09B implementation is complete, PENDING INDEPENDENT STRICT RE-AUDIT. M09 remains open; M10 remains BLOCKED/UNSTARTED.
