# M11A Post-Log Strict Re-Audit + Current Product Delta

Date: 2026-08-26
Product: H!veAI
Branch: `H!veAI`
Audited builder log: `H!veAI/docs/H!veAI/codex-logs/M11A_GLOBAL_COMMAND_CENTER_STRICT_CLOSURE_LOG.md`
Audited implementation commits reported by builder: `5b15a1b0cbdd06f82eb7f7fe378aee991e3b7805`, `76ffc951956d902743b88090de0661fed42672be`
Current pre-audit branch state inspected after prompt merge: `623c87b6c51cadc7b2c699b0233defb308c60f4a`

## Verdict

**FAIL / NOT CLOSED under the current product contract**

- BLOCKER: 0
- MAJOR: 4
- MINOR: 1
- NOTE: 2
- Confidence: HIGH
- Regression risk: HIGH because the next change alters watcher scope and the Project Dashboard contract.

This verdict does not mean the latest builder run failed to implement the prompt it had received. Source review shows that the prior M11A R01-R08/E01-E02/UX01-UX04 remediation is substantially present. The branch is not closable because two explicit user product decisions were made after that run, plus two residual truthfulness defects remain in the current source.

## Source-level closure status from the latest M11A run

### Prior R01 workflow integration: PASS in source
`command_center.rs` now calls `workflow::project_list` with `workflow::MAX_HISTORY_LIMIT` and no longer uses the invalid 4096 limit. Workflow read failure is exposed as unknown/warning instead of silently becoming empty truth.

### Prior R02 unknown metrics: PASS in source
Missing M09 task intelligence yields nullable project task totals; a genuinely parsed empty source may yield zero. Portfolio KPI types are nullable.

### Prior R03 completed-task selection: PASS in source
Current-task selection checks the matching M10 workflow and excludes `TASK_COMPLETE` tasks.

### Prior R04 directory provenance: PASS in source
`project_dashboard.rs` accepts physically contained directories for the bounded `progressHistory` / `buildTest` roles while keeping canonical task authority file-only.

### Prior R05 refresh failure visibility: PASS in source
Watcher refresh health is persisted in bounded settings state, last-good M09 truth is preserved, and degraded refresh status is exposed to Command Center.

### Prior R06 mixed evidence: PARTIAL/PASS with residual R10 below
The current implementation merges workflow, agent, audit, test, Git and project-snapshot rows into one bounded deterministic activity timeline and adds real failed audit/test/permission evidence. A residual actor-provenance defect remains.

### Prior R07 native direct evidence: SOURCE PRESENT / EXECUTION CLAIMED
Production-path tests are now present, including a real watcher-to-M09-to-M11 test. The builder log claims 251 Rust tests actually executed after a shell-local Windows common-controls manifest workaround. Builder logs remain claims under governance, so the next run must execute the suite again and persist fresh evidence.

### Prior R08 bounds: PASS in source
Manifest/project/portfolio warning collectors are bounded and deterministic.

### Prior UX01-UX03: SOURCE IMPLEMENTED / USER ACCEPTANCE PENDING
The large full-width Recent Activity home panel is removed from the routed Command Center, project rail and right-rail visible rows are bounded, and raw Task Sources inventory is moved behind an Advanced disclosure. Native visual acceptance of the published result has not yet been recorded.

### Prior UX04: PARTIAL
`.hiveai/PROJECT_DASHBOARD.md` is presented as the single user-facing entry contract, but the runtime watcher still recursively watches the whole registered project root and treats ordinary task/source changes as independent refresh triggers. This conflicts with the user's newer single-dashboard-watch decision.

---

# Open findings under the current product contract

## R11 / MAJOR — Bottom footer still consumes workspace; topbar attribution decision is not implemented

Current `src/components/Shell.tsx` still renders a dedicated `<footer className="global-footer">` after the content area. The current topbar contains breadcrumb + Search Workspace + controls but no Akilta attribution in the requested center/flexible slot.

Required current product behavior:

- remove the dedicated bottom footer band and its reserved height;
- move the existing Akilta wordmark + exact visible sentence `Built with ♥ for maximum productivity by Akilta` into the top command bar between breadcrumb and Search Workspace;
- make the whole attribution one accessible clickable target;
- tooltip/title: `Developed by Akilta`;
- use existing safe native `hiveai_open_akilta` behavior and exact `https://www.akilta.com/` destination;
- preserve no-terminal, Chrome-only behavior;
- reclaim the removed footer height for application content.

## R12 / MAJOR — Single-dashboard watch architecture is not implemented

Current `watcher.rs` still attaches `RecommendedWatcher` to the entire registered root with `RecursiveMode::Recursive` and marks `TASKS.md`, plans, `.hiveai/*`, and `src/*` events as task/source relevant. Any relevant ordinary source event can run the existing M09 parser.

The current `project_dashboard.rs` also does not parse `trackingMode`; its typed parsed model contains schema/project/repository/branch/dashboardMode/refreshPolicy/roles only.

Required current product behavior for a migrated project declaring:

`trackingMode: single-dashboard-watch`

- `.hiveai/PROJECT_DASHBOARD.md` becomes the sole project-status live trigger;
- do not recursively subscribe to the whole project tree for routine project-status tracking;
- use a narrow watch attachment suitable for atomic file replacement, preferably the contained `.hiveai` directory non-recursively with exact filename filtering, plus only the minimum additional root-lifecycle mechanism genuinely required;
- ordinary TASKS/AGENTS/audit/log/prompt/handoff/roadmap/src changes do not independently refresh M09/M11;
- a dashboard change may trigger one bounded M08/M09 refresh to re-read internal evidence and then refresh M11;
- explicit user rescan may still refresh deep inventory;
- legacy projects without the extended contract keep safe legacy fallback behavior.

## R13 / MAJOR — Project Dashboard is still pointer-only; materialized status contract is not runtime-consumed

The cross-repository project contract now defines `trackingMode: single-dashboard-watch` plus bounded materialized sections (`H!veAI live status`, Current work, Blockers and waiting, Milestone summary, Quality and verification, Recent meaningful activity, Provenance). Current `project_dashboard.rs` parses only source-authority pointers and does not expose those materialized status fields to M11.

Required behavior:

- parse the extended v1 fields/sections with strict bounds;
- expose materialized project status to Command Center;
- preserve M10 as stronger H!veAI-owned workflow truth where applicable;
- conflicts are explicit warnings, never silent overwrite;
- no status/task double-count between materialized dashboard and M09 source data;
- internal source paths remain provenance, not independent watch subscriptions in single-dashboard mode.

## R14 / MAJOR — Legacy absent manifests and benign manifest warnings can create false attention/health truth

`project_dashboard::resolve` represents an absent manifest with fallback provenance plus an informational warning. `command_center::project_health` currently treats any non-empty `dashboard.warnings` as ATTENTION, and `summarize_project` creates Needs Your Attention rows for dashboard warnings generally.

That means a supported legacy project with no manifest can be classified as needing attention solely because the manifest is absent, even though ABSENT is an accepted fallback state. Benign parser warnings may also inflate operational attention.

Required behavior:

- ABSENT + supported legacy fallback is informational provenance, not an operational attention failure by itself;
- only MALFORMED/STALE/UNAVAILABLE or explicitly severity-qualified actionable dashboard problems should create configuration attention;
- PARTIAL secondary provenance should not automatically become workflow failure;
- project health/portfolio health must distinguish task-health unknown from actual HEALTHY rather than fabricating confidence.

## E04 / MINOR — Activity actor fields fabricate GPT Audit / CI identity where the schema does not prove it

`read_activity` hard-codes `actor: Some("GPT Audit")` for every `audits` row and `actor: Some("CI")` for every `test_runs` row. The accepted schema stores audit result/summary and test command/result but does not store an actor/provider on those tables.

Required behavior:

- actor/provider is `null` unless the queried evidence actually proves it;
- do not infer GPT Audit from table name or CI from a test row;
- UI already supports optional actors, so omit unproved labels.

---

# Evidence / acceptance gaps

## E05 / NOTE — Latest builder log does not itself contain final post-log-commit local/origin equality
The log records equality for the implementation/evidence commit and says final log-document commit is verified separately. Current branch later moved again when prompt updates were merged. The next builder log must contain a concrete final implementation/evidence equality proof with no `pending` placeholders.

## E06 / NOTE — User native visual acceptance remains pending
The user has not yet accepted the newly published UX01-UX03 result. The next publication must be visually checked after R11-R14/E04 and the single-dashboard changes.

---

# Required next action

Do not start M12.

Run one consolidated M11A continuation prompt that:

1. preserves the source-level fixes already present from the latest M11A run;
2. implements R11-R14 and E04;
3. implements the current topbar attribution and single-dashboard/materialized-status product decisions;
4. re-runs actual Rust/frontend/full regression and governed QA publication;
5. leaves M11 pending independent re-audit + user native acceptance.
