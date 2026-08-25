# M08B Task Source Discovery Final Strict Re-audit

Date: 2026-08-25

## Verdict

`FAIL`

M08B closes the five findings from the M08A re-audit for newly-written M08A/M08B custom-path settings and substantially improves the direct evidence. However, source-level review found one remaining production compatibility defect in the custom-order contract and one smaller evidence mismatch. M09 remains blocked.

Builder logs were treated as claims only. This re-audit checked the authoritative M08B prompt, branch history, M08B diff, current Rust production source, historical original-M08 persistence shape, mounted frontend tests, tracker truth, and current branch state.

## Audited state

- M08B prompt commit: `d51bfad93b4a29b5284fa403ddb89cb5ee952227`
- M08B implementation/publication commit: `8d8327e4a210d896fccc809270b836b04305cf6d`
- M08B publication-equality follow-up / branch HEAD at audit start: `dbd7b7beb568babacd0ca614fd263a56dcfac100`
- No M09 implementation or installer was introduced.
- Canonical PNG/MP4/logo assets are absent from the M08B diff.

## Acceptance matrix

### B01 true positional custom reorder

`FAIL`

The new insertion algorithm is correct for settings already carrying contiguous explicit `order` values: it removes the selected item, inserts at the bounded requested index, and renumbers the vector. The three-item production test proves last-to-first, first-to-last, rename-without-order, containment rejection, duplicate-target rejection and normalized removal.

The remaining defect is backward compatibility with custom paths persisted by the original M08 implementation.

Original M08 persisted `StoredCustomPath` with only `id`, `display_path`, and `normalized_path`; there was no `order` field. Current M08B deserializes the new `order: i64` using `#[serde(default)]`, so every historical custom path without an order becomes `order = 0`. `load_custom_paths()` returns that vector without migration or normalization.

Consequences for a project that configured more than one custom path before order metadata was introduced:

1. multiple historical paths all report order 0;
2. discovery no longer preserves their stored relative sequence as explicit custom order and can fall through to freshness/path tie-breaks;
3. a path-only update on a historical second/third item captures `original_order = 0`, removes the item, then inserts it at 0, violating the required rule that path-only rename preserve the current relative position;
4. the frontend disables `Move earlier` whenever `customPath.order === 0`, so every historical entry can appear to be first.

This is a real production compatibility defect, not only a test gap.

Severity: `MAJOR`.

Required closure: normalize legacy custom-path settings that lack valid explicit contiguous order using their persisted vector sequence before list/discovery/update semantics are applied, and persist/repair contiguous explicit order on the next H!veAI-owned settings write without mutating project files. Add a direct legacy-settings fixture with at least three paths lacking `order` and prove 0/1/2 normalization plus path-only rename preserving the middle position.

### B02 safe pre-version `project_sources` adoption

`PASS`

The compatibility predicate is now materially narrow. It rejects declared owner/schema rows, requires matching project identity, normalized `relativePath == source_path`, an allowed old origin, deterministic legacy M08 row id, and a rich old-M08 field shape. Deceptive partial and foreign-owner rows are directly preserved in tests.

### B03 persisted SQL and custom/standard ordering evidence

`PARTIAL`

Persisted hash-change and deleted-standard/legacy-preservation SQL evidence are now direct and meaningful.

The custom/standard ordering test proves production ordering with CUSTOM before TASKS/PLAN/ROADMAP and proves an explicit reorder changes the result. However, the M08B prompt explicitly required **at least three CUSTOM paths plus multiple STANDARD classes** in this combined ordering test. The implemented test uses only two CUSTOM paths (`custom-a.md`, `custom-b.md`) plus three STANDARD classes.

A separate B01 test does exercise three custom paths, so this is not treated as a second production MAJOR. It remains a strict evidence mismatch that should be closed in the same tiny compatibility fix by extending the combined ordering test to three CUSTOM paths.

Severity: `MINOR evidence gap`.

### B04 mounted frontend transition evidence

`PASS`

The add test now completes the native add, performs the production refresh path, and asserts the new custom path becomes visible without remount. The reorder test uses two configured paths, invokes B -> order 0, receives refreshed B/A state, and asserts B appears before A in DOM order. The table test asserts path, kind, origin, authority/priority, modified evidence and status. Existing stale list/add/remove, error/empty, rescan replacement and browser isolation evidence remain present.

### B05 tracker/log truth

`PASS WITH NOTE`

The tracker correctly leaves independent audit and user visual acceptance open and keeps M09 blocked. The M08B log is separate and records publication equality for `8d8327e4...`; the branch then advanced to the documentation-only equality follow-up `dbd7b7be...`. This is acceptable scoped bookkeeping.

## Severity summary

- `BLOCKER`: 0
- `MAJOR`: 1
  - historical original-M08 custom-path settings without `order` are not normalized/migrated, breaking the positional/order contract.
- `MINOR`: 1
  - combined custom+standard ordering evidence uses two CUSTOM paths instead of the explicitly required three.
- `UNVERIFIED`: Windows physical symlink/junction creation remains OS error 1314 as previously recorded.

## Required next step

Do not start M09 and do not ask for final visual acceptance yet.

Use one micro-fix only:

1. normalize historical custom settings lacking/invalid/duplicate explicit order by persisted vector position before runtime ordering semantics;
2. ensure path-only rename preserves that normalized relative position;
3. ensure the UI receives contiguous 0..n-1 order values for those historical settings;
4. add a direct three-path legacy-settings fixture proving normalization and rename preservation;
5. extend the combined CUSTOM + STANDARD ordering test to at least three CUSTOM paths;
6. rerun focused/full gates and publish without changing accepted UI/presentation behavior.

M08 remains `FAIL / one micro-fix required`. M09 remains blocked.
