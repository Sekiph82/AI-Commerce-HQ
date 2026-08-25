# M08C Custom Order Backward-Compatibility Strict Re-audit

Date: 2026-08-25

## Verdict

`CONDITIONAL`

M08C closes the one remaining M08B production compatibility defect and the three-CUSTOM ordering evidence gap at source level. No BLOCKER or MAJOR production finding remains in the audited M08/M08A/M08B/M08C Task Source Discovery chain.

The milestone is not yet closed because the remediated native `/tasks` Task Sources workspace still requires direct user visual acceptance. M09 remains blocked until that manual gate is recorded.

Builder logs were treated as claims only. This audit checked the M08C prompt, branch history/diff, current Rust production source, direct M08C Rust fixtures, tracker truth, and current branch state.

## Audited state

- M08C prompt commit: `b1dcc8cf0fbcb723c281ac5b1f73d8dc97ceff81`
- M08C implementation/publication commit: `3c1ea19eb647b31f5449e9e32bc4021ac1d14fc8`
- Documentation-only follow-up / branch HEAD at audit start: `6f14c185540779bb106d5c0fa9cd21cc5594bf2a`
- M08C diff from prompt base changes only `src-tauri/src/task_sources.rs`, `TASKS.md`, and the new M08C log.
- No frontend production source, IPC/ACL, migration, publisher, installer, or canonical visual asset file changed in M08C.
- M09 remains unstarted.

## Acceptance matrix

### D01 — Legacy custom settings distinguish missing order from explicit order

`PASS`

Production now deserializes settings through `RawStoredCustomPath { order: Option<i64> }` rather than mapping absent historical `order` directly to numeric zero. This preserves the distinction required for backward compatibility.

### D02 — Normalization policy is correct and deterministic

`PASS`

`normalize_custom_paths()` accepts explicit order metadata only when every entry has a non-negative contiguous unique order set `0..n-1`. In that valid case it sorts by explicit order. If order metadata is missing, duplicated, negative, non-contiguous, or otherwise invalid, it preserves the persisted JSON vector sequence and assigns contiguous in-memory orders by vector position.

This closes the M08B defect where multiple original-M08 entries without `order` all became order zero.

### D03 — Runtime consumers use normalized ordering

`PASS`

`load_custom_paths()` applies the normalization boundary before returning settings. Discovery, custom-path list, add/remove/update and ordering semantics consume the normalized `StoredCustomPath` vector. Existing explicit positional reorder behavior from M08B remains intact.

### D04 — Path-only rename preserves normalized legacy position and repairs persisted metadata

`PASS`

The direct production-path fixture `legacy_custom_settings_without_order_normalize_and_preserve_position` seeds actual old-shape settings JSON with three entries in deliberately non-lexical vector order: `z.md`, `A.md`, `m.md`, with no `order` fields.

The test directly proves:

- list returns `z.md`, `A.md`, `m.md` at orders `0,1,2`;
- discovery returns those CUSTOM sources in that order before `TASKS.md`;
- path-only rename of the middle item with `order=None` yields `z.md`, `renamed.md`, `m.md` at `0,1,2`;
- direct SQLite settings JSON inspection after mutation contains explicit contiguous order values `0,1,2`.

The pre-M08C implementation would fail the order assertions because historical missing order metadata deserialized as zero for every entry.

### D05 — Three-CUSTOM plus STANDARD ordering evidence

`PASS`

`custom_sources_order_before_standard_authority_order_in_persisted_inventory` now uses three CUSTOM files (`custom-a.md`, `custom-b.md`, `custom-c.md`) plus `TASKS.md`, `PLANS.md`, and `ROADMAP.md`. It explicitly reorders `custom-c.md` to position zero and asserts exact production discovery order:

`custom-c.md`, `custom-a.md`, `custom-b.md`, `TASKS.md`, `PLANS.md`, `ROADMAP.md`.

This closes the M08B minor evidence mismatch.

### D06 — Previously accepted M08A/M08B behavior remains isolated from this micro-fix

`PASS`

The M08C implementation diff does not change frontend production code, Tauri IPC/ACL, filesystem bounds, containment, project-source ownership/reconciliation, startup/presentation assets, or publisher scripts. The correction is bounded to custom settings loading/normalization plus Rust evidence and tracker/log bookkeeping.

## Evidence notes

### Automated gate execution

The M08C log reports focused Rust `35/35`, focused frontend `20/20`, full frontend `68/68`, full Rust `137/137`, typecheck/build/audit, publisher failure harness and production `--no-bundle` publication as PASS. These execution results are builder-reported claims; the audit independently verified the relevant production code and the direct test bodies that close D01-D05.

### MINOR — one focused-test name in the M08C log is not exact

The current Rust source names the visited-entry test `visited_entry_limit_warning_is_structured`, while the final M08C log lists `visited_entry_limit_is_enforced`. This is a documentation-name mismatch only. The production visited-entry limit implementation and test remain outside the M08C functional change and were accepted in the prior M08A audit chain. No additional builder remediation is warranted for this typo.

### NOTE — publication equality bookkeeping

The M08C log records local/origin equality for the implementation/publication commit `3c1ea19...`. The branch then advanced to documentation-only follow-up `6f14c185...`, which is the audited branch HEAD. This does not change production bytes.

### UNVERIFIED — Windows physical symlink/junction fixture

The previously documented Windows OS error 1314 remains an accepted environment limitation. M08C does not change containment logic.

## Severity summary

- `BLOCKER`: 0
- `MAJOR`: 0
- `MINOR`: 1 documentation test-name mismatch
- `UNVERIFIED`: Windows physical symlink/junction creation under OS error 1314
- `MANUAL`: native `/tasks` visual acceptance still pending

## Final gate

Automated/source-level M08 Task Source Discovery closure is clean.

`M08C = CONDITIONAL PASS`

The only remaining milestone-closing gate is direct user inspection of the refreshed native Task Sources workspace. Do not start M09 until that acceptance is recorded and the final M08 closure audit/tracker update is committed.
