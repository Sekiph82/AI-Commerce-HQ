# M11A REV3 Consolidated Strict Re-Audit

Date: 2026-08-26
Product: H!veAI
Branch: `H!veAI`
Audited builder log: `H!veAI/docs/H!veAI/codex-logs/M11A_REV3_CONSOLIDATED_STRICT_CLOSURE_LOG.md`
Audited implementation commit: `e4958d69acb09b4cb70fea560f49eeb515c84dd9`
Builder-log commit / remote HEAD at audit start: `c9f44a63a265732fa588ad8a749ab4a36d15b65c`
Authoritative builder prompt: `H!veAI/docs/H!veAI/prompts/M11A_GLOBAL_COMMAND_CENTER_STRICT_CLOSURE_REV3_CONSOLIDATED_PROMPT.md`

## Verdict

**FAIL / M11 NOT CLOSED**

- BLOCKER: 0
- MAJOR: 3
- MINOR: 1
- NOTE: 2
- Confidence: HIGH
- Regression risk: HIGH, because the remaining defects sit in the new single-dashboard runtime contract that M12 will depend on.

The REV3 run closed the previously reported footer/topbar, materialized parser foundation, false ABSENT-attention, actor-provenance, and many M11 truthfulness defects. However, the current source still has three production defects that prevent the single-dashboard architecture from being a reliable source of portfolio truth.

## Acceptance matrix

| Area | Result | Independent finding |
| --- | --- | --- |
| P0 topbar Akilta / footer removal | PASS source-level, user visual acceptance pending | Bottom footer is gone; one topbar attribution target wraps wordmark + exact credit and keeps the native Akilta action. |
| P1 single-dashboard watcher scope | PARTIAL / FAIL | Initial migrated-project attachment is narrow, but watcher scope does not reconcile when tracking mode changes while H!veAI is running. |
| P2 materialized Project Dashboard parsing | PARTIAL / FAIL | Typed materialized status is parsed, but important project-status sections are not yet operationally consumed by M11 and parser section lines can exhaust the front-matter field bound. |
| P3 H!veAI dogfood dashboard | PASS source-level | H!veAI dashboard declares single-dashboard-watch and materialized current truth. |
| P4 legacy attention / health semantics | PASS | ABSENT fallback is informational; actionable malformed/stale/unavailable/conflict conditions can require attention; unknown health remains unknown. |
| P5 audit/test actor provenance | PASS | Unproved GPT Audit / CI actor labels were removed. |
| Prior M11A R01-R08 | PASS source-level for previously reopened defects | Workflow bound, unknown semantics, TASK_COMPLETE exclusion, directory pointers, refresh degradation, mixed DB evidence, bounds, and direct tests are present. |
| Task Sources UX | PASS source-level | Project Intelligence exposes the dashboard contract and SINGLE_DASHBOARD mode; raw discovery remains advanced/internal evidence. |
| Command Center one-screen UX | SOURCE IMPLEMENTED / USER ACCEPTANCE PENDING | Full-width Recent Activity is removed and desktop right-rail bounds exist; final native visual geometry remains user-owned acceptance. |

---

# Open findings

## R15 / MAJOR - Watch scope is captured once and does not reconcile when dashboard tracking mode changes

`WatcherManager::configure_project()` resolves whether a project is `single-dashboard-watch`, but after that it returns early whenever an existing watcher is already present and the root path is unchanged. It does not compare the existing `watch_scopes` value with the newly desired scope.

The event callback also captures the `single_dashboard` boolean at watcher creation time.

This creates two stale-runtime modes:

1. A legacy recursive project can be migrated by writing `trackingMode: single-dashboard-watch`, yet its existing watcher remains recursive and continues accepting ordinary TASKS/source events until the watcher is recreated or the app restarts.
2. A single-dashboard project whose dashboard becomes absent, malformed, stale, or no longer declares the mode can retain the old single-dashboard callback and continue ignoring legacy TASKS/source events instead of restoring the required safe fallback.

Calling explicit rescan does not guarantee correction because `rescan_project()` calls `configure_project()` and the same existing-watcher early return applies.

This violates the core runtime promise that current dashboard contract state determines the active watch mode continuously, not only at process startup.

### Required closure

- Derive a desired scope for every configure/reconcile pass.
- Compare desired scope with the currently attached scope.
- If root identity is unchanged but scope changed, tear down and recreate the watcher safely with the new callback/scope.
- Re-evaluate scope after a dashboard signal and after explicit rescan.
- Preserve no-duplicate-watcher behavior.
- Recover correctly from dashboard delete/recreate and `.hiveai` lifecycle changes without requiring an application restart.
- Add production-path tests for legacy -> single, single -> legacy fallback, and same-scope no-op behavior while the manager remains alive.

## R16 / MAJOR - Materialized dashboard sections are parsed but not fully connected to M11 operational aggregation

The new resolver exposes `current_work`, `blockers_waiting`, `quality_verification`, `recent_meaningful_activity`, `waiting_on`, and provenance. `CommandCenter` currently consumes materialized current task, state fallback, next action, required actor, milestone/waiting metadata, health and progress.

However, the actual M11 portfolio collections still derive primarily from M10/SQLite evidence:

- `Needs Your Attention` does not consume verified materialized `Blockers and waiting` items or the corresponding waiting state from a single-dashboard external project.
- `Active Work Queue` does not consume bounded materialized `Current work` rows.
- materialized `Quality and verification` is not used for factual attention/brief signals.
- materialized `Recent meaningful activity` is not surfaced as dashboard-origin recent project evidence when no timestamped SQLite activity exists.

This is especially important under the new architecture because external project agents now write project truth into one file and H!veAI intentionally stops live-watching all the underlying source files. A Project Dashboard can therefore truthfully say that the project is blocked or waiting while the global Command Center still fails to surface that fact in its operational panels.

### Required closure

For valid single-dashboard projects, consume materialized sections conservatively and without fabricating M10 events:

- bounded verified blocker/waiting items -> `Needs Your Attention` with deterministic project-dashboard provenance;
- bounded current-work rows with clearly active/running/waiting/blocked/verification statuses -> Work Queue only when not already represented by stronger M10/agent evidence;
- failed/blocked verification facts -> attention only when the value explicitly proves a failing/actionable result; otherwise keep as factual brief/project evidence;
- `waiting_on` should be represented when it is a real non-NONE/non-UNKNOWN wait and not duplicated by an existing materialized blocker;
- recent meaningful activity may appear in selected-project compact activity or the Activity surface, but do not invent timestamps merely to merge undated text into a chronological timeline;
- preserve M10 as stronger operational truth and prevent duplicate items by stable provenance-aware identities.

Add direct tests using a project that has no M10 workflow/session rows so the dashboard alone proves blocker, current work and waiting truth.

## R17 / MAJOR - Front-matter field bound is incorrectly charged by colon-containing materialized section lines

`parse_manifest()` scans the entire document. Whenever it is outside `## Source authorities`, any line containing `:` increments `front_matter_fields`, even after materialized `##` sections have begun.

This means valid dashboard content such as milestone, activity, provenance or table values containing colons can consume the 32-field front-matter budget. The cross-repository standard intentionally allows multiple bounded materialized sections whose combined item count can exceed 32. A sufficiently informative but otherwise valid dashboard can therefore be rejected as malformed with `front-matter field limit reached (32)`.

### Required closure

- Restrict front-matter scalar parsing/counting to the actual header region before the first top-level `##` section, while preserving the existing recognized scalar fields and real front-matter bound.
- Continue parsing `## Source authorities` and materialized sections with their own dedicated bounds.
- Add a test proving that more than 32 colon-containing lines across otherwise valid bounded materialized sections do not consume the front-matter field budget.
- Add/retain a separate test proving genuinely excessive header/front-matter fields are still rejected.

## R18 / MINOR - Materialized enum-like status values are not validated before becoming runtime state

The shared project contract defines bounded vocabularies for Project status, Health, and Required actor. The parser currently stores arbitrary strings. `project_health()` can return an arbitrary materialized health value uppercased, while the frontend contract expects a closed set of project health values.

### Required closure

Validate or normalize at the parser boundary:

- Project status: `ACTIVE`, `PAUSED`, `WAITING`, `BLOCKED`, `COMPLETE`, `UNKNOWN`.
- Health: `HEALTHY`, `ATTENTION`, `BLOCKED`, `UNKNOWN`.
- Required actor: `HUMAN`, `CODEX`, `CLAUDE`, `GPT_AUDIT`, `CI`, `EXTERNAL`, `NONE`, `UNKNOWN`.

Invalid values must not become authoritative runtime state. Preserve them only as bounded warning/provenance if useful, and expose UNKNOWN/None instead of inventing a new state.

---

# Evidence and acceptance notes

## E07 / NOTE - Layout tests are DOM assertions, not native geometry proof

The focused topbar test verifies the link is in `.topbar`, the footer role is absent, and breadcrumb/Search are present. The Command Center test sets representative widths and verifies key surfaces exist. These are useful component checks but they do not physically measure native WebView overlap, clipping or scrollbars. This remains acceptable only because user native visual acceptance is explicitly still pending.

## E08 / NOTE - Post-log local/origin equality remains builder-local evidence

The remote branch is currently at the immutable log commit `c9f44a63a265732fa588ad8a749ab4a36d15b65c`. The log gives concrete `0 0` equality for the implementation commit before creating the log, then states that equality for the log commit will be verified afterward. GitHub confirms the remote log commit, but the auditor cannot prove the builder's local checkout after that final push. The next builder log should persist final local HEAD, origin HEAD, and `0 0` after all implementation/evidence commits and before exit.

---

# Confirmed REV3 closures

The following previous findings are not reopened by this audit:

- M10 workflow list bound/error integration.
- Unknown task metrics versus real parsed zero.
- `TASK_COMPLETE` exclusion from current task.
- Directory-backed provenance roles.
- Refresh failure/degraded state visibility and last-good M09 preservation.
- Mixed SQLite workflow/agent/audit/test/Git/snapshot evidence aggregation.
- Bounded warning collectors.
- False operational attention from an ABSENT legacy manifest.
- Hard-coded `GPT Audit` and `CI` activity actors.
- Bottom footer removal and topbar attribution source implementation.
- Advanced source inventory separation.

Do not redesign or reimplement these areas except where required to integrate R15-R18.

# Required next action

Do not start M12.

Run one bounded M11A REV4 closure pass for R15-R18 plus direct regression evidence. Preserve all accepted REV3 behavior. M11 may close only after independent re-audit finds no BLOCKER/MAJOR defect and the user performs native visual acceptance of the current published shell.
