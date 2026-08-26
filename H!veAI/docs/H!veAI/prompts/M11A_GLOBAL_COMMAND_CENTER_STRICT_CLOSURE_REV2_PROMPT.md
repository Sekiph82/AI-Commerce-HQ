# M11A REV2 - Strict Closure + User Acceptance Priority Fixes

## Authority

This file is the authoritative M11A entry prompt for the next Codex run and supersedes the execution order in:

`H!veAI/docs/H!veAI/prompts/M11A_GLOBAL_COMMAND_CENTER_STRICT_CLOSURE_PROMPT.md`

Read that prior M11A prompt in full because R01-R08, E01-E03, UX01-UX04 and all safety/test/closure requirements remain mandatory unless explicitly superseded below.

Do not split the milestone. This remains one bounded M11A remediation run.

Do not start M12.

Current strict completed milestone count remains 11/20 = 55%.

---

# FIRST PRIORITY AFTER TRACKER PREFLIGHT - P0 Move the bottom Akilta credit into the top command bar

The user has explicitly rejected the current bottom footer geometry because it consumes vertical workspace and contributes to the Command Center overflow problem.

The user screenshot shows the desired conceptual location: the existing credit should live elegantly in the open topbar space BETWEEN the left `Workspace / Command Center` breadcrumb area and the `Search workspace` control.

## Required behavior

Remove the dedicated bottom footer band from the main application shell so its vertical height becomes usable application workspace.

Move the existing exact credit:

`Built with ♥ for maximum productivity by Akilta`

into the top command bar.

Preferred desktop placement:

```text
[ Workspace / Command Center ]        [ Built with ♥ for maximum productivity by Akilta ]   [ Search workspace ] [ icons... ]
```

The credit must visually belong to the topbar, not float randomly over page content.

Preserve:

- the exact visible sentence;
- final `Akilta` clickable behavior;
- exact destination `https://www.akilta.com/`;
- Google Chrome external-open behavior already accepted for H!veAI;
- H!veAI remains open;
- no terminal/console flash;
- no Edge fallback;
- keyboard accessibility/focus behavior.

## Layout requirements

- The moved credit must not overlap the breadcrumb or Search Workspace control.
- The topbar should use available flexible space with responsive constraints.
- On normal desktop widths the full sentence should be visible and balanced.
- At narrower supported widths, degrade gracefully without causing horizontal overflow. Prefer responsive text compression/hiding of nonessential prefix text before hiding the Akilta identity or breaking core navigation/search.
- Do not increase topbar height materially.
- Removing the old footer must actually release its height to the page. Do not leave an empty footer spacer.
- The Command Center and Tasks page should gain usable vertical area from the removed footer.
- No page-level or nested scrollbar should be introduced by the topbar change.

## Required tests

Add direct frontend/layout evidence that:

1. the bottom footer band is absent from the routed application shell;
2. the exact credit appears in the topbar;
3. final Akilta remains the existing native external-link action;
4. breadcrumb and Search Workspace remain visible and non-overlapping at representative desktop widths;
5. no new outer vertical/horizontal overflow is introduced;
6. footer removal increases usable content height rather than leaving dead spacing.

This P0 task must be implemented and tested before the remaining M11A production fixes.

---

# P1 Single-dashboard watch architecture

The user has made a product decision: H!veAI should actively watch only one project-status file per registered project:

`.hiveai/PROJECT_DASHBOARD.md`

The existing M08/M09 source inventory remains valuable for diagnostics, evidence provenance, audits and deep inspection, but H!veAI should not treat all discovered TASKS/AGENTS/audits/logs/prompts/handoffs/roadmaps as independent live-watch triggers for routine dashboard refresh when a Project Dashboard contract exists.

## Backward-compatible project contract

Keep current v1 compatibility:

```text
hiveaiDashboardSchema: hiveai-project-dashboard/v1
dashboardMode: source-map
```

Recognize the extended project-side contract:

```text
trackingMode: single-dashboard-watch
refreshPolicy: project-agent-maintained; H!veAI watches only .hiveai/PROJECT_DASHBOARD.md
```

A project may also contain materialized status sections such as:

- `## H!veAI live status`
- `## Current work`
- `## Blockers and waiting`
- `## Milestone summary`
- `## Quality and verification`
- `## Recent meaningful activity`
- `## Provenance`

The cross-repository authoring standard is documented in:

`H!veAI/docs/H!veAI/prompts/CROSS_REPO_SINGLE_DASHBOARD_AKILTA_ATTRIBUTION_PROMPT.md`

Read it before implementing this P1 behavior.

## Watcher semantics

For a registered ACTIVE project with a valid `.hiveai/PROJECT_DASHBOARD.md` that declares `trackingMode: single-dashboard-watch`:

- `.hiveai/PROJECT_DASHBOARD.md` is the only project-status live-watch trigger;
- ordinary changes to `TASKS.md`, `AGENTS.md`, audit files, codex-run files, prompts, handoffs, roadmap, changelog, architecture, decisions, etc. must NOT independently trigger routine M09/M11 dashboard refresh;
- those files remain discoverable/internal evidence if the user opens Advanced source inventory or a deep diagnostic path;
- when PROJECT_DASHBOARD changes, refresh the project's dashboard truth through the narrow existing production path;
- no polling loop;
- no background project-file writes from H!veAI;
- no generated commit noise.

For a legacy project without a valid Project Dashboard or without `single-dashboard-watch`, preserve the existing safe legacy fallback behavior until migration is complete. Do not brick old projects.

## Materialized dashboard state

Where the extended v1 materialized sections exist, expose their verified project-status fields in M11 as primary project-summary evidence while preserving M10 workflow truth for H!veAI-owned workflow state.

Rules:

- never invent `0` when status is unknown;
- `UNKNOWN` remains unknown;
- source-authority pointers remain provenance, not independent live-watch subscriptions;
- do not double-count task/status truth from both PROJECT_DASHBOARD and TASKS/roadmap/history;
- if materialized dashboard status conflicts with stronger local M10 workflow truth, surface the conflict/warning rather than silently overwriting M10;
- if dashboard declares a canonical task source, retain it as provenance but do not require H!veAI watcher subscription to that source in single-watch mode;
- keep malformed/stale/absent manifest fallback explicit.

## Task Sources UI semantics

Do not delete the raw source inventory.

Default Task Sources presentation should distinguish:

- `H!veAI live contract` -> `.hiveai/PROJECT_DASHBOARD.md`
- `Live tracking` -> `SINGLE_DASHBOARD`
- `Internal evidence sources` -> discovered count

The full discovered file table belongs under an explicit `Advanced source inventory` disclosure/action.

When advanced inventory is opened, clearly label the rows as discovered/internal evidence and NOT independent live-watch targets for a single-dashboard project.

This explains why move-in-range can have 15 discovered sources while H!veAI actively watches only one status contract.

## Required tests

Prove with real production-path fixtures:

1. single-dashboard project + TASKS.md change alone does not trigger routine project-status refresh;
2. same project + `.hiveai/PROJECT_DASHBOARD.md` change does trigger refresh;
3. raw source inventory is still available for diagnostics;
4. advanced inventory labels sources as internal evidence, not live watch targets;
5. legacy project without single-watch declaration retains safe fallback behavior;
6. no task/status double-count occurs;
7. malformed/stale dashboard does not crash or silently fabricate truth.

---

# P2 Continue all existing M11A strict-remediation requirements

After P0 and P1, execute every still-open requirement from:

`H!veAI/docs/H!veAI/prompts/M11A_GLOBAL_COMMAND_CENTER_STRICT_CLOSURE_PROMPT.md`

including, without omission:

- R01 workflow integration bound/error handling;
- R02 known/unknown metric semantics;
- R03 TASK_COMPLETE exclusion from current task;
- R04 contained directory provenance pointers;
- R05 watcher/M09 refresh failure visibility/recovery;
- R06 factual attention/work queue/mixed activity aggregation;
- R07 actual native Rust assertion execution and production-path tests;
- R08 warning/output cardinality bounds;
- E01 neutral browser preview identity;
- E02 structured factual provenance;
- E03 final local/origin equality evidence;
- UX01 one-screen Command Center composition;
- UX02 remove giant full-width Recent Activity from home;
- UX03 simplified Task Sources default surface;
- UX04 one H!veAI Project Dashboard entry contract.

Where P1 changes the watcher architecture, P1 supersedes older wording that requires every task/source candidate file to act as an independent live-watch trigger for projects already migrated to single-dashboard-watch.

The required activity aggregation still exists as data and on the dedicated Activity route even though the giant home-page activity panel is removed.

---

# Canonical UI / regression protection

Preserve unchanged:

- sidebar H!veAI logo and geometry;
- canonical background asset and position;
- startup intro asset/audio/lifecycle;
- X01 terminal-popup suppression;
- X02 startup audio/no same-process replay;
- stable EXE/shortcut/icon;
- Akilta link destination/native Chrome behavior;
- accepted dark/glass visual language.

Required unchanged hashes:

- background SHA-256: `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`
- opening video SHA-256: `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`

Do not create an installer.

Do not modify tracked files in other registered project repositories during this H!veAI M11A run.

Bulk Edit remains untouched while Etsy approval is pending.

---

# Exit state

Only after P0, P1, R01-R08, E01-E03, UX01-UX04, actual Rust test execution, frontend tests, full gates and governed QA publication succeed:

- M11A = IMPLEMENTATION COMPLETE / PENDING INDEPENDENT RE-AUDIT + USER VISUAL/NATIVE ACCEPTANCE;
- M11 remains NOT PASS/CLOSED until independent audit and user acceptance;
- strict completed stays 11/20 = 55%;
- M12 remains BLOCKED.

Create/update the immutable M11A builder log required by the original M11A prompt and include P0/P1 evidence explicitly.

Stop. Do not start M12.