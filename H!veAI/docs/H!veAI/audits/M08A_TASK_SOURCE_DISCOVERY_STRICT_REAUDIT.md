# M08A Task Source Discovery Strict Re-audit

Date: 2026-08-25

## Verdict

`FAIL`

M08A closes several important production defects from the original M08 audit, but direct source/evidence review still finds unresolved MAJOR items. M09 remains blocked.

Builder logs were treated as claims only. This audit checked the authoritative M08A prompt, branch history, implementation diff, Rust production source, mounted frontend source/tests, Tauri IPC/ACL, tracker truth, and final branch state.

## Audited state

- M08A prompt commit: `c4190ac149b002ca34d397db727f2fcccc7ba1ad`
- M08A implementation/publication commit: `a6474a2fb585829e88a84f0c9384d4be5ed30caa`
- M08A log follow-up commit / branch HEAD at audit start: `09abd3b15059afc798397567ff1a147073691522`
- Historical M08 log/audit remain unchanged.
- No M09 implementation or installer was introduced.
- Canonical PNG/MP4 assets are absent from the M08A diff.

## Acceptance matrix

### C01 filesystem work bounds

`PASS`

Production now counts visited filesystem entries with `MAX_VISITED_ENTRIES = 4096`, caps accepted candidates at 512, keeps `MAX_DISCOVERY_DEPTH = 4`, rejects the first source beyond the depth boundary, and persists a synthetic `DISCOVERY_WARNING` / `LIMIT_REACHED` record with structured warning text. Root enumeration and bounded-directory enumeration both consume the shared work budget.

### C02 custom update/order/remove

`FAIL`

The update IPC and persistence fields exist, containment/dedupe are reused, and remove-by-path now normalizes both sides. However, actual reordering is not reliable.

`custom_path_update()` sets the requested item to the requested numeric order, then sorts all entries by `(order, normalized_path)`, then renumbers sequentially. If two entries collide at the same order, lexical path ordering decides the winner. Therefore a request to move item B from order 1 to order 0 can leave B second when A and B both temporarily have order 0 and A sorts first.

The current Rust test avoids exposing this by simultaneously renaming `b.md` to `a-renamed.md`, whose lexical order happens to win the collision. The mounted frontend reorder test uses only one configured custom path, so it cannot prove movement relative to another item.

This violates the required deterministic custom reordering contract.

Severity: `MAJOR`.

### C03 M08-owned non-destructive reconciliation

`FAIL`

Explicit `schemaVersion = 1` and `owner = M08_TASK_SOURCE_DISCOVERY` metadata are now written and unrelated minimal legacy rows survive.

However, pre-version adoption is too broad. Reconciliation treats any row as compatible M08 inventory when its JSON merely contains `relativePath` and `origin` in `STANDARD|CUSTOM|SYSTEM`. Such a row is then deleted during reconciliation even without a deterministic M08 identity/shape proof.

Because `project_sources` is a shared persistence table, this does not meet the prompt requirement to adopt pre-version rows only when identity/shape proves they are M08 inventory.

Severity: `MAJOR`.

### C04 stale frontend races

`PASS`

A request generation plus selected-project ref now prevents delayed project-A list/add/remove completions from refreshing project A after project B becomes current. Mounted same-instance tests exercise delayed A list and stale A add/remove completion after B selection.

### C05 truthful mounted frontend evidence

`PARTIAL / FAIL`

The new suite materially improves stale-list, stale-mutation, error, empty, rescan replacement, remove and browser-preview evidence.

Two required transitions are still not directly proven:

1. `custom_add_command_uses_native_boundary` proves the add IPC call, but does not prove the add completion triggers a refreshed visible custom/source inventory.
2. `custom_update_reorder_executes_and_refreshes_visible_inventory` proves only the update IPC call. It does not assert a refresh call or changed visible ordering/inventory.

The source table tests also do not directly assert the complete required metadata presentation as one production-backed transition: path, kind, origin, authority, priority, modified evidence and status.

Severity: `MAJOR evidence gap` because M08A was specifically an evidence-integrity closure.

### C06 direct Rust persistence/evidence matrix

`PARTIAL / FAIL`

Direct SQL evidence now proves owner/schema metadata, one-row idempotency and legacy-row preservation. AVAILABLE-to-MISSING and unreadable-source isolation are also materially improved.

Still missing from direct SQL evidence required by the M08A prompt:

- persisted `content_hash` actually changes after the source file content changes;
- deleted STANDARD source removes its owned persisted row while unrelated rows remain;
- discovered output ordering across multiple CUSTOM entries plus STANDARD authority classes proves configured custom reorder semantics.

Existing tests cover returned-model hash changes and standard ordering, but those are not substitutes for the required persisted SQL/change and custom-order evidence.

Severity: `MAJOR evidence gap`.

### C07 root handoff family

`PASS`

Root matching now accepts any case-insensitive filename containing `handoff` and ending in `.md`, while nested arbitrary repository Markdown remains outside the approved discovery boundary.

### C08 registered-project status boundary

`PASS WITH MINOR COVERAGE NOTE`

Production centralizes status enforcement through `discovery_project()`: ACTIVE allowed, MISSING unavailable, ARCHIVED rejected. The archived test directly proves discovery and add rejection; list/remove/update share the same production boundary but are not each independently asserted.

### C09 containment-aware custom status

`PASS WITH WINDOWS LINK CASE UNVERIFIED`

`custom_paths_list` now canonicalizes existing targets and reports `OUTSIDE_ROOT`/`UNREADABLE` rather than blindly returning `CONFIGURED`. Physical link creation remains legitimately UNVERIFIED under Windows OS error 1314.

### C10 tracker/log truth

`PARTIAL`

The remediation log is separate from the historical M08 log and lists the focused test names. It records publication commit equality at `a6474a2...`.

The branch HEAD after the documentation follow-up is `09abd3b...`, so the log's equality statement is accurate for the implementation/publication commit but is not the final branch HEAD after the log follow-up itself. This is a documentation note, not a production blocker.

The larger issue is that the log claims C02/C05/C06 closure more strongly than the actual tests and source support.

## Severity summary

- `BLOCKER`: 0
- `MAJOR`: 5
  - custom reorder collision semantics are incorrect;
  - pre-version reconciliation compatibility predicate is too broad/destructive;
  - add visible-refresh transition evidence missing;
  - update/reorder visible-refresh/order transition evidence missing;
  - direct SQL/custom-order evidence remains incomplete.
- `MINOR/NOTE`: archived per-operation test breadth; final log HEAD bookkeeping nuance; Windows symlink remains exact UNVERIFIED.

## Required next step

Do not start M09.

Use one bounded M08B closure pass only. It must:

1. implement true positional reorder semantics without order-collision lexical fallback;
2. narrow pre-version adoption to a deterministic legacy-M08 identity/shape predicate and prove a deceptive unrelated `relativePath + origin` row survives;
3. add mounted add-refresh and multi-item reorder-visible-order tests;
4. add direct SQL persisted-hash-change and deleted-standard-row tests;
5. add direct discovery ordering evidence with at least two CUSTOM paths plus STANDARD sources;
6. keep all accepted M08/M08A production behavior and canonical presentation unchanged.

M08 remains `FAIL / remediation required`. M09 remains blocked.