# M11A REV4 - Final Single-Dashboard Integration Closure

## Authority

This is the single authoritative Codex entry prompt for the next H!veAI run.

It is a bounded continuation of M11A and exists only to close the residual findings in:

`H!veAI/docs/H!veAI/audits/M11A_REV3_CONSOLIDATED_STRICT_REAUDIT.md`

Do not split this into a new numbered milestone or separate builder prompts.

Do not start M12.

Current strict completed roadmap count remains **11 / 20 = 55%** until independent M11 closure.

Preserve every REV3 closure not explicitly reopened below.

---

# Mandatory preflight and Task 0

Work from:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Run:

```powershell
git fetch origin H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
git merge --ff-only origin/H!veAI
```

Never reset, rebase, force-push, rewrite user history, create `H!veAI\.git`, or stage unrelated parent-root files.

Preserve user-owned untracked:

- `start-demo.bat`
- `task.md`

Read before editing:

1. `H!veAI/AGENTS.md`
2. `H!veAI/CONSTITUTION.md`
3. `H!veAI/TASKS.md`
4. `H!veAI/CODEX_ROADMAP.md`
5. `H!veAI/docs/H!veAI/prompts/M11A_GLOBAL_COMMAND_CENTER_STRICT_CLOSURE_REV3_CONSOLIDATED_PROMPT.md`
6. `H!veAI/docs/H!veAI/codex-logs/M11A_REV3_CONSOLIDATED_STRICT_CLOSURE_LOG.md`
7. `H!veAI/docs/H!veAI/audits/M11A_REV3_CONSOLIDATED_STRICT_REAUDIT.md`
8. `H!veAI/docs/H!veAI/prompts/CROSS_REPO_SINGLE_DASHBOARD_AKILTA_ATTRIBUTION_PROMPT.md`
9. current `.hiveai/PROJECT_DASHBOARD.md`
10. current watcher, Project Dashboard resolver, Command Center, Task Sources and focused tests
11. this REV4 prompt in full

Before production edits, synchronize prospective tracker truth only in the canonical current-status docs:

- M00-M10 PASS/CLOSED;
- strict completed 11/20 = 55%;
- M11 original historical FAIL;
- M11A REV3 implementation completed but independent REV3 re-audit = FAIL with R15-R18 open;
- M11A REV4 = ACTIVE during this run;
- M11 remains NOT CLOSED;
- M12 remains BLOCKED;
- user native visual acceptance remains pending.

At minimum update prospectively:

- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/README.md`
- `H!veAI/docs/H!veAI/README.md`

Historical prompts/logs/audits are immutable.

---

# Canonical UI Assets

This section is mandatory and authoritative for UI regression protection.

Canonical user-owned asset root:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Preserve without regeneration or substitution:

- sidebar combined H!veAI logo: repo `H!veAI/src/assets/hiveai-logo.png`;
- background: repo `H!veAI/src/assets/hiveai-app-background.png`;
- opening video: repo `H!veAI/src/assets/opening-video.mp4`;
- stable shortcut icon: `H!veAI/dev-bin/H!veAI.ico`;
- existing tracked Akilta wordmark used by the current topbar attribution.

Required unchanged hashes:

- background SHA-256: `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`
- opening video SHA-256: `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`

Preserve accepted behavior:

- no bottom footer band;
- full Akilta attribution stays in the topbar between Workspace breadcrumb/title and Search Workspace;
- exact visible credit remains `Built with ♥ for maximum productivity by Akilta`;
- whole attribution remains one clickable/focusable target;
- title remains `Developed by Akilta`;
- exact destination remains `https://www.akilta.com/`;
- native Chrome-only safe open remains parameterless from frontend;
- H!veAI stays open;
- no Edge fallback;
- no console/terminal flash;
- startup video remains audible and does not replay during same-process navigation;
- accepted sidebar/background/glass identity remains unchanged.

Do not redesign the shell in REV4.

Do not create an installer.

---

# R15 / MAJOR - Reconcile live watcher scope when tracking mode changes

## Defect

REV3 correctly creates narrow watchers for a project that is already in `single-dashboard-watch` when the watcher is attached.

But `configure_project()` currently returns early when an existing watcher already exists and the root path did not change. It does not compare the current attached `watch_scopes` value with the newly resolved desired scope. The callback has already captured its old `single_dashboard` boolean.

Therefore runtime migrations can remain stale until restart.

## Required production behavior

For every watcher configure/reconcile pass derive a desired scope from the current Project Dashboard resolution:

- valid/partial + `trackingMode: single-dashboard-watch` -> `SINGLE_DASHBOARD`;
- otherwise supported legacy mode -> `LEGACY_RECURSIVE`.

Compare:

- registered root identity;
- desired watch scope;
- currently attached root;
- currently attached scope.

If root is unchanged but desired scope changed, safely remove the existing watcher and attach a new watcher whose callback reflects the new mode.

Do not leak or duplicate watcher handles.

### Required runtime transitions without app restart

Prove all of these on a live `WatcherManager`:

1. legacy project starts recursive;
2. dashboard is created/updated to declare `single-dashboard-watch`;
3. dashboard signal is observed;
4. manager reconciles to `SINGLE_DASHBOARD`;
5. later TASKS/src changes no longer trigger routine project-status refresh;
6. dashboard is then removed, malformed, stale, or mode is removed;
7. manager reconciles back to `LEGACY_RECURSIVE` safely;
8. legacy TASKS/source behavior works again;
9. no app restart is needed.

Explicit `rescan_project()` must also re-evaluate and correct scope.

A dashboard event that changes its own tracking contract must cause bounded scope reconciliation after the event is processed. Avoid deadlocks between watcher locks, callbacks and reconfiguration.

### `.hiveai` lifecycle

The narrow root-lifecycle watch exists to keep the dashboard watch recoverable. Handle at minimum:

- dashboard file atomic replace/rename;
- dashboard file delete/recreate;
- `.hiveai` directory delete/recreate when the platform emits only parent/root lifecycle events.

Do not broaden routine single-dashboard status watching back to recursive project-source watching merely to solve this lifecycle.

### Git evidence

Keep the REV3 narrow Git strategy. Dashboard signal and explicit rescan may capture bounded Git evidence. Do not restore full-root recursion for Git status.

### Required tests

Add real production-path tests that fail against REV3 behavior and prove:

- legacy -> single transition while manager lives;
- single -> legacy fallback while manager lives;
- same root + same scope does not duplicate watcher;
- explicit rescan reconciles scope;
- dashboard atomic replacement works;
- dashboard and `.hiveai` delete/recreate recovery is deterministic within platform constraints;
- no cross-project leakage;
- TASKS/src remain ignored only while current desired scope is SINGLE_DASHBOARD.

---

# R16 / MAJOR - Connect materialized dashboard truth to M11 operational surfaces

## Product contract

For migrated projects, `.hiveai/PROJECT_DASHBOARD.md` is the single H!veAI-facing materialized status contract.

M10 remains stronger when H!veAI owns a matching workflow task. Materialized dashboard status must fill external/project-agent truth when M10/agent-session evidence does not exist.

Do not fabricate M10 events or write SQLite workflow history merely to display external project status.

## Needs Your Attention

For a valid single-dashboard project, create deterministic bounded attention evidence from materialized facts when they are explicitly actionable.

At minimum:

- every non-empty verified item in `## Blockers and waiting`, excluding `NONE`, `UNKNOWN`, `NOT_VERIFIED`, and `None verified`, may become a Project Dashboard attention item;
- non-empty `Waiting on` may become one attention item when it is not already represented by the blockers list;
- `Project status = BLOCKED` or `Health = BLOCKED/ATTENTION` supports attention severity;
- `Project status = WAITING` plus a real `Waiting on` value supports attention;
- `Required actor = HUMAN` or `EXTERNAL` may qualify a real wait for owner/external attention, but do not create an item from actor alone without an actual wait/task/blocker fact;
- failed verification evidence may create attention only when the Quality/verification value explicitly proves `FAIL`, `FAILED`, `ERROR`, `BLOCKED`, or an equivalent existing accepted failure token.

Use stable deterministic IDs based on project identity + materialized source identity/content. Do not duplicate an equivalent stronger M10/audit/test/permission item.

All such items must carry clear `PROJECT_DASHBOARD` provenance/category.

## Active Work Queue

Consume bounded `## Current work` rows for valid single-dashboard projects when they represent genuinely current work and there is no stronger equivalent M10 workflow or live agent-session item.

Use conservative explicit status mapping. Examples that may qualify:

- ACTIVE
- IN_PROGRESS / IN PROGRESS
- RUNNING
- IMPLEMENTING
- AUDITING
- VERIFYING / VERIFICATION
- WAITING
- BLOCKED
- COMPLETE_PENDING_AUDIT / IMPLEMENTATION_COMPLETE_PENDING_AUDIT when they still require a next actor/gate

Do not put clearly completed/closed historical rows in the active queue.

Do not interpret arbitrary unknown status prose as active. Preserve unknown rows only in project materialized detail.

Stable identity must prefer the dashboard row ID when present plus project identity.

## Project compact recent activity

`## Recent meaningful activity` in the cross-repo contract does not require a timestamp on every item.

Do not invent timestamps.

Required behavior:

- make these bounded dashboard-origin facts available in the selected project's compact Recent activity surface when native timestamped DB activity is absent or does not already represent the same fact;
- if an item contains a verified parseable ISO timestamp under an explicitly supported format, it may join the chronological Activity timeline;
- otherwise preserve it as undated dashboard activity/evidence and do not pretend it occurred at `now`.

The giant full-width home Recent Activity panel stays removed.

## Quality and verification

Use bounded materialized Quality/verification facts in factual project/Engineering Brief provenance.

Only explicit failure tokens create attention. PASS or descriptive evidence remains factual context, not a warning.

## Materialized provenance

Every materialized-derived surface must be traceable to:

`.hiveai/PROJECT_DASHBOARD.md`

Do not expose source bodies.

Do not recursively read provenance paths because they are named in the dashboard.

## No double counting

Add deterministic de-duplication across:

- M10 workflow items;
- agent sessions;
- persisted audit/test/permission evidence;
- materialized Current work;
- materialized blockers/waits;
- M09 task identities.

A single logical piece of work must not inflate KPI/attention/queue counts merely because it appears in multiple provenance layers.

## Required direct tests

Create project fixtures with a valid single-dashboard manifest but no M10 workflow or agent sessions and prove:

1. verified dashboard blocker appears in Needs Your Attention;
2. dashboard Waiting on appears once, not duplicated;
3. active Current work appears in Work Queue;
4. completed Current work does not appear;
5. explicit failed Quality fact appears as attention;
6. PASS Quality fact does not appear as attention;
7. dashboard compact recent activity is visible without invented timestamps;
8. later matching stronger M10 evidence suppresses/replaces the weaker duplicate;
9. materialized rows do not change authoritative task-count totals;
10. bounded lists remain bounded.

---

# R17 / MAJOR - Fix Project Dashboard header/front-matter field accounting

## Defect

The current parser increments `front_matter_fields` for colon-containing lines throughout the document whenever it is outside Source authorities. Materialized sections therefore consume the header field budget.

## Required parser boundary

Treat the header/front-matter region as the initial scalar region before the first top-level `##` heading.

Only this header region may contribute to `MAX_FRONT_MATTER_FIELDS`.

After the first `##` section begins:

- do not parse/count generic colon lines as front-matter fields;
- parse `## Source authorities` using source-role logic and source-path bounds;
- parse materialized sections using their dedicated section/item/scalar bounds.

Preserve existing recognized header keys:

- `hiveaiDashboardSchema`
- `projectKey`
- `repository`
- `branchPolicy`
- `dashboardMode`
- `trackingMode`
- `refreshPolicy`

Unknown header lines may remain bounded according to existing policy but must not let materialized section content consume the same counter.

### Required tests

- A valid dashboard with more than 32 colon-containing lines distributed across bounded materialized sections parses successfully.
- A genuinely excessive header/front-matter region still fails closed.
- Existing v1 dashboards remain compatible.
- Source authorities remain independently bounded.

---

# R18 / MINOR - Validate standardized materialized enums

At the Project Dashboard parse boundary normalize case and validate the values defined by the shared cross-repository contract.

## Project status allowed values

- ACTIVE
- PAUSED
- WAITING
- BLOCKED
- COMPLETE
- UNKNOWN

## Health allowed values

- HEALTHY
- ATTENTION
- BLOCKED
- UNKNOWN

## Required actor allowed values

- HUMAN
- CODEX
- CLAUDE
- GPT_AUDIT
- CI
- EXTERNAL
- NONE
- UNKNOWN

Do not let arbitrary values such as `SUPER_HEALTHY`, `BROKENISH`, or unknown actor names become operational runtime enums/state labels.

On invalid input:

- preserve a bounded warning/provenance note if useful;
- operational typed value becomes UNKNOWN or absent according to current optional semantics;
- do not crash portfolio snapshot generation.

`Current workflow state` remains an open string because it may legitimately carry project-specific workflow states. Do not over-constrain it.

Add direct parser and Command Center tests for valid lowercase/case-normalized input and invalid values.

---

# Regression protection - Do not reopen REV3 closures

The following must remain correct:

- M10 workflow list uses the accepted bound and errors do not silently become empty truth;
- unknown task truth remains null/unknown, not fake zero;
- M10 `TASK_COMPLETE` is never selected as current task;
- contained directory-backed provenance stays supported;
- last-good M09 truth survives refresh failures and degraded state is visible;
- audit/test actors are null unless schema evidence proves them;
- legacy ABSENT dashboard alone does not create operational attention;
- malformed/stale/unavailable/current actionable conflict states remain truthful;
- browser preview identity remains neutral;
- raw M08 source inventory remains available under Advanced source inventory;
- Task Sources shows the dashboard as the live contract and SINGLE_DASHBOARD when applicable;
- no second H!veAI project status/manifest file is created;
- no external registered project repository is modified;
- Bulk Edit is untouched;
- no installer is created.

---

# Testing and verification gates

Rust native tests must actually execute assertions. `cargo test --no-run` is not acceptance.

Use the already established narrow shell-local Windows common-controls workaround if the same `STATUS_ENTRYPOINT_NOT_FOUND` condition occurs. Do not mutate Windows globally.

At minimum execute and persist exact results for:

1. focused Project Dashboard parser tests;
2. focused watcher lifecycle/scope tests;
3. focused Command Center materialized aggregation tests;
4. full Rust native suite with assertions actually executed;
5. focused frontend Command Center/Task Sources/Akilta shell tests;
6. full frontend suite;
7. typecheck;
8. production frontend build;
9. dependency security audit;
10. `cargo fmt --all -- --check`;
11. `cargo check`;
12. `git diff --check`;
13. canonical background/video SHA verification;
14. X01 no-visible-console regression path;
15. X02 opening-audio/replay regression path;
16. governed QA publication and publisher failure harness.

Do not mark user native visual acceptance PASS yourself.

---

# Publication and closure evidence

Use the governed QA publisher already established by the repository. Do not create an installer.

Create a new immutable builder log:

`H!veAI/docs/H!veAI/codex-logs/M11A_REV4_FINAL_SINGLE_DASHBOARD_INTEGRATION_CLOSURE_LOG.md`

The log must include:

- exact starting HEAD;
- exact implementation commit(s);
- R15-R18 implementation evidence;
- exact focused/full test counts and commands;
- real watcher live-transition test evidence;
- materialized blocker/queue/activity evidence;
- parser bound regression evidence;
- enum validation evidence;
- final published stable EXE SHA-256;
- canonical asset hashes;
- scope proof that no external project repository and no Bulk Edit path was modified;
- final local HEAD SHA;
- final `origin/H!veAI` SHA after fetch;
- exact `git rev-list --left-right --count HEAD...origin/H!veAI` result with `0 0` required before builder exit.

Do not write `pending` placeholders for final remote equality.

If publishing the log itself creates one final commit, push it, fetch again, and persist the post-log equality in a separate immutable closure evidence file or in another already-authorized non-historical current evidence artifact without rewriting the log. Never edit an immutable log after publication.

---

# Exit state

If and only if R15-R18 and all required regression/security/publication gates succeed:

- M11A REV4 = IMPLEMENTATION COMPLETE / PENDING INDEPENDENT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE;
- M11 remains NOT PASS/CLOSED;
- strict completed remains 11/20 = 55%;
- M12 remains BLOCKED.

Stop.

Do not start M12.
