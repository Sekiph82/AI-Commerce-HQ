# M11 Global Command Center + Project Dashboard Runtime - Independent Strict Audit

Date: 2026-08-26
Branch: `H!veAI`
Prompt commit: `146bd634b02db18dc809e41b2a8ea0409ce3c973`
Implementation commit: `010906876a0e0be774cbf61f4e0ce2b59ed410ca`
Builder log / audited remote HEAD before this audit: `1f833de5e90a2405599fa55f670924dbb97c4de8`

## VERDICT

**FAIL**

- BLOCKER: 0
- MAJOR: 8
- MINOR: 3
- NOTE: 1
- Confidence: HIGH
- Regression risk: HIGH for portfolio truth until R01-R08 are closed.
- M11 is NOT CLOSED.
- M12 remains BLOCKED.
- Strict completed roadmap count remains **11 / 20 = 55%**.

The M11 implementation adds a substantial and directionally correct Project Dashboard resolver, a read-only Command Center snapshot service, watcher-triggered M09 refresh plumbing, and a native Command Center view. However, several production defects currently make the dashboard's workflow/task/authority truth incomplete or incorrect. In particular, the M11 aggregator always requests an invalid M10 workflow list limit and silently converts that failure to an empty workflow list, so the Command Center cannot currently consume M10 workflow truth as claimed.

The builder also truthfully reports that the Rust test executable could not run in the local Windows environment. This matters because the core M11 resolver/watcher/aggregation behavior is native Rust, and the required end-to-end production-path proof is therefore still missing.

---

## 1. Scope / branch / builder-log recovery

Audited change range:

`146bd634b02db18dc809e41b2a8ea0409ce3c973..1f833de5e90a2405599fa55f670924dbb97c4de8`

The range is two commits and is bounded to M11 implementation, tracker synchronization, native IPC/permissions, watcher integration, live Command Center UI, focused frontend tests, and the M11 builder log.

The builder log remains an evidence claim, not acceptance. It correctly records:

- M11 implementation complete but pending independent audit and user visual/native acceptance;
- frontend suites claimed PASS;
- `cargo test --no-run` compiled native tests;
- actual Rust test execution failed before assertions with Windows `STATUS_ENTRYPOINT_NOT_FOUND (0xc0000139)`;
- QA publication passed on a later bounded attempt;
- visual/native acceptance remains pending.

No M12 implementation, agent adapter, Prompt Engine, GPT Audit Engine, GitHub integration, or installer was found in the audited diff.

---

## 2. Acceptance matrix

| Area | Result | Audit note |
|---|---|---|
| Task 0 tracker synchronization | PASS/PARTIAL | M10 closure and 11/20 truth were synchronized. M11 package checkboxes overstate several incomplete areas. |
| Native manifest resolver service | PARTIAL | Core fixed-path/bounds/containment design exists, but real rollout directory roles are misclassified and warning output is not bounded. |
| Schema / mode validation | PASS | Exact v1 schema and `source-map` mode are checked. |
| Repository identity mismatch fallback | PASS by source | Mismatched registered Git identity becomes STALE/fallback. Direct executed Rust evidence is unavailable. |
| Canonical task authority filtering | PASS by source | Canonical source path filtering and task-ID dedup exist. |
| NOT_CANONICALIZED semantics | PASS/PARTIAL | Tasks are suppressed, but broader portfolio known/unknown accounting is inconsistent. |
| ABSENT/MALFORMED/STALE fallback | PARTIAL | Resolver fallback exists, but fallback is later counted as if canonical in authority summary. |
| Watcher root `.hiveai` classification | PASS by source | Root `.hiveai/...` is now TaskCandidate. |
| Watcher -> M09 refresh | PARTIAL | Parse is invoked, but failure is not surfaced as durable/degraded evidence and required end-to-end test was not executed/proven. |
| Live Registry-backed projects | PASS | Snapshot is Registry-backed. |
| M10 workflow-backed current state/attention/queue | FAIL | Workflow list request is always outside M10's allowed limit and the error is swallowed. See R01. |
| Task KPI truthfulness | FAIL | Missing M09 snapshot can become authoritative zero counts. See R02. |
| Deterministic current task | FAIL | A parser-active but M10-complete task can still be chosen. See R03. |
| Project Dashboard role provenance | FAIL | Directory-valued rollout roles are marked missing. See R04. |
| Needs Your Attention | PARTIAL | Workflow categories exist, but failed test/permission evidence is not aggregated and workflow inputs are currently lost by R01. |
| Active Work Queue | FAIL/PARTIAL | Running/VerifyRequired logic exists, but waiting/blocked work is absent and workflow inputs are currently lost by R01. |
| Engineering Brief factual-only | PASS/PARTIAL | Recommendation remains null. Provenance strings are too generic for strong traceability. |
| Recent Activity | PARTIAL | Only `task_events` are aggregated, not the broader accepted evidence set. |
| Selected-project interaction | PASS by frontend source/tests | Names-only rail, in-place selection and explicit cockpit navigation exist. |
| Browser preview truthfulness | PARTIAL | Native metrics are unavailable, but a FormuLab identity and Scrubbots placeholder survive. |
| Narrow IPC / capability | PASS | Two read-only M11 commands, no arbitrary path/SQL/network surface. |
| Output bounds | FAIL | Manifest warning amplification and portfolio warning aggregation have no explicit count bound. See R08. |
| Frontend mounted tests | PASS as builder evidence/source | Focused frontend tests exist and builder claims full frontend PASS. |
| Required native direct tests | FAIL/UNVERIFIED | Rust tests compiled but did not execute; several named tests are shallow and required E2E proof is absent. See R07. |
| User visual/native acceptance | UNVERIFIED | Explicitly pending and not used to derive verdict. |
| Final local/origin equality after final log commit | UNVERIFIED/MINOR | Remote HEAD is known; persisted log only proves equality before the final documentation commit. |

---

# 3. Production findings

## R01 - MAJOR - M11 always discards M10 workflow truth because it calls the workflow list with an invalid limit

Production `command_center::summarize_project()` calls:

```rust
workflow::project_list(
    database,
    WorkflowProjectListQuery {
        project_id: project.id.clone(),
        limit: Some(4096),
    },
)
.unwrap_or(WorkflowProjectList { tasks: Vec::new(), ... })
```

M10's public workflow contract has `MAX_HISTORY_LIMIT = 500`, and `bounded_limit()` rejects every list/history limit above 500 with `WORKFLOW_BOUNDS`.

Therefore every M11 project workflow-list call with `4096` fails. M11 then silently converts that error to an empty workflow list.

Consequences include:

- `currentState` is usually absent even when M10 has state;
- latest workflow action is lost;
- allowed actor truth is lost;
- M10 attention states disappear;
- running/pending verification queue items disappear;
- health cannot be based on actual M10 state;
- the primary M11 claim of Registry/M09/M10-backed operations is false in production.

Required fix:

- use an allowed bounded M10 list size, or add a dedicated internal bounded query contract if M11 genuinely requires more than 500;
- do not swallow workflow-list errors as empty evidence;
- surface a bounded per-project warning/unknown state when workflow evidence cannot be loaded;
- add a direct production-path test with actual M10 task events proving the M11 snapshot returns current state, latest action, attention and queue truth.

Pre-fix test requirement: the new test must fail because the current M11 snapshot receives no workflow tasks.

---

## R02 - MAJOR - Unknown M09 task truth is converted into authoritative zero counts, and fallback is mislabeled as canonical authority

M11 loads the persisted M09 snapshot with `.ok()`. When the snapshot is absent, it adds a warning but `authoritative_tasks()` returns an empty vector.

Then, for every task-authority state except `NOT_CANONICALIZED`, M11 emits:

```text
totalTasks = 0
activeTasks = 0
completedTasks = 0
```

This turns "M09 has not been parsed / snapshot unavailable" into "there are zero tasks", which violates the no-fake-live-metrics contract.

The same area counts every state other than `NOT_CANONICALIZED` as a known canonical task project when generating `authorityDetail`. `FALLBACK_M08_M09` is explicitly not canonical authority, so the string "projects without canonical task authority" can be wrong.

The frontend registry-only failure snapshot similarly reports `needsAttention=0` and `running=0` when those values are actually unknown.

Required fix:

- distinguish `unknown/unavailable` from numeric zero throughout the native snapshot and frontend fallback;
- only emit numeric task counts when the relevant M09 task snapshot is actually available and the chosen authority mode supports task metrics;
- count `CANONICAL` separately from fallback when describing canonical authority coverage;
- make unavailable attention/running metrics nullable or otherwise explicitly unknown in fallback UI rather than false zero;
- add direct tests for canonical manifest + missing M09 snapshot, fallback + missing M09 snapshot, and registry-only native snapshot failure.

---

## R03 - MAJOR - Current-task selection can select an M10 `TASK_COMPLETE` task

`select_current_workflow()` constructs `active_ids` by calling:

```rust
task_is_complete(task, None)
```

With `workflow=None`, completion is determined only from parser status. A task whose source still looks active but whose durable M10 state is `TASK_COMPLETE` remains in `active_ids` and can then be selected as the current workflow task, especially if it has a recent latest event.

The M11 prompt explicitly requires that a completed task not be selected merely because it is newest.

Required fix:

- current-task candidate filtering must consider the matching M10 workflow state when present;
- `TASK_COMPLETE` workflow-managed tasks must be excluded from active/current selection;
- preserve the required precedence: attention task, newest non-complete workflow task, deterministic active authoritative M09 task, none;
- add a direct test where parser status is still active but workflow state is `TASK_COMPLETE`, alongside another active task, and prove the completed task is never selected.

---

## R04 - MAJOR - Valid rollout directory authority pointers are treated as missing files

The resolver validates every declared authority path with:

```rust
let exists = candidate.is_file();
```

That is correct for canonical task files, but not for pointer roles that are intentionally directory-valued provenance, such as the accepted H!veAI manifest:

```text
Progress/history sources: `H!veAI/docs/H!veAI/audits/`, `H!veAI/docs/H!veAI/codex-logs/`
```

Those valid contained directories become `MISSING`, causing a real accepted manifest to be marked `PARTIAL` and making its history provenance inaccurate.

The M11 contract explicitly says not to recursively read directory-valued history/build metadata. That implies a contained declared directory can be a valid pointer without reading its contents.

Required fix:

- define role-aware pointer type rules;
- canonical task, handoff, roadmap, architecture, decision, instruction and security file roles should require a file unless the accepted contract says otherwise;
- progress/history and build/test metadata may accept a physically contained file or directory pointer where rollout manifests already do so;
- do not recurse/read directory contents in M11;
- preserve symlink/junction physical containment rules;
- test the real H!veAI-style history directory manifest and prove it is not downgraded solely because those paths are directories.

---

## R05 - MAJOR - Watcher-triggered M09 parse failure is emitted but effectively invisible to the product

Watcher integration correctly calls the existing `task_intelligence::parse()` path. On failure it emits a refresh event with `success=false`.

However:

- the watcher status is not degraded for an M09 refresh failure;
- no bounded failure warning/evidence is persisted for later Command Center snapshot consumption;
- the frontend event listener ignores the event payload and simply refreshes the snapshot;
- the snapshot can continue to show last-good M09 truth without telling the user that live refresh failed.

The prompt required preserving last-good evidence **and surfacing bounded refresh warning/degraded evidence**.

Required fix:

- retain last-good M09 snapshot;
- record a bounded per-project refresh failure state/warning in native watcher/project evidence, or another existing safe persistence/status channel;
- include that warning/degraded state in the M11 snapshot;
- have the frontend honor `success=false` by refreshing and visibly presenting the native warning, without inventing source content;
- clear the degraded refresh warning only after a later successful relevant refresh;
- add direct success/failure/recovery tests.

---

## R06 - MAJOR - Portfolio aggregation is materially incomplete while tracker items are marked complete

The accepted M11 contract requires real portfolio attention, work queue and activity evidence.

Current production source:

- Recent Activity reads only `task_events`;
- Active Work Queue includes only M10 RUNNING states and `VERIFY_REQUIRED`;
- waiting/blocked workflow states are not included in the queue despite the tracker marking waiting/blocked work complete;
- failed `test_runs` are not surfaced as attention;
- real pending/open permission requests are not queried even though the schema includes `permission_requests`;
- agent/audit/test/Git/watcher evidence is not aggregated into Recent Activity;
- the tracker itself still leaves `Show real task/workflow/agent/audit/Git activity` unchecked, while other related M11 packages are marked complete.

This is not merely an optional future enhancement because the prompt and M11 task ledger explicitly include these factual surfaces.

Required fix:

- implement bounded, deterministic aggregation from the real accepted schema for the evidence classes that exist now;
- include workflow waiting/blocked items in the active/waiting queue according to the M11 contract;
- include real failed verification/test evidence in attention;
- include real pending permission requests only for explicit pending/open states supported by existing rows;
- expand Recent Activity to real task/workflow, agent/session, audit, test, Git snapshot, and watcher/project snapshot evidence where timestamped rows exist;
- deterministic cross-table ordering must use timestamp plus stable ID/source tie-break;
- never fabricate provider/activity if a table has no real rows;
- add direct mixed-evidence tests and update tracker checkboxes only when source + tests support them.

---

## R07 - MAJOR - Required native production-path test evidence is not executed, and several named native tests do not test their names

The builder log explicitly records that Rust tests did not execute because the generated test executable failed to launch with Windows `STATUS_ENTRYPOINT_NOT_FOUND (0xc0000139)`. `cargo test --no-run` proves compilation only.

This is especially important for M11 because the authority resolver, task aggregation and watcher refresh chain are native Rust.

Additionally, the native `command_center.rs` tests named:

- `m11_current_task_selection_is_deterministic`
- `m11_project_health_is_categorical_and_evidence_based`
- `m11_portfolio_counts_use_authoritative_tasks_only`

currently assert only tiny helper facts such as `is_running_state`, `progress_percent`, `attention_state`, and case-insensitive `same_path`. They do not exercise the actual snapshot/current-task/count production paths.

The prompt required direct proof of:

- manifest present/absent/malformed/stale/cross-project containment;
- no-double-count authority filtering;
- deterministic current task;
- no-authority null metrics;
- watcher source change -> M09 update -> M11 snapshot update;
- root `.hiveai` classification;
- real workflow/attention/queue/activity behavior.

Required closure:

- fix the local Rust test runtime environment or run the native test binaries in an environment where they actually execute;
- add/strengthen direct production-path tests so their bodies exercise `project_dashboard::resolve`, `command_center::snapshot`, M09/M10 persisted truth, and the real watcher refresh integration;
- provide command output proving assertions executed, not only binaries compiled;
- do not mark native direct evidence PASS until actual tests run.

This is an evidence finding and also blocks confidence in the production defects above until execution proof exists.

---

## R08 - MAJOR - M11 warning/output cardinality is not bounded despite an explicit bounded-warning contract

The manifest parser adds one warning for every line containing task-checkbox syntax. There is no warning-count cap.

A single manifest is byte-bounded to 64 KiB, but short checkbox-bearing lines can still produce thousands of warnings. Portfolio aggregation then appends project warnings across up to 128 projects without a global warning-count bound.

This allows a small bounded input per project to amplify into a very large IPC snapshot and UI warning payload. M11's contract explicitly requires bounded warnings and bounded snapshot output.

Required fix:

- add a finite per-manifest warning cap with deterministic truncation and a final `WARNING_LIMIT_REACHED` style warning;
- add a finite per-project and/or portfolio warning cap in Command Center aggregation;
- bound warning scalar bytes;
- ensure repeated identical checkbox/path warnings can be deduplicated where useful;
- test overflow behavior deterministically.

---

# 4. MINOR findings

## E01 - MINOR - Browser preview still presents named fake project identity

`CommandCenterLive` falls back to `FormuLab` for `currentName` in non-desktop mode even though `previewSnapshot()` has no native projects, and retains an `Open Scrubbots placeholder` action.

The prompt required browser preview to represent native evidence as unavailable/empty rather than inventing project identity.

Required fix: show a neutral unavailable/preview label and remove named project placeholders from the M11 Command Center preview path.

## E02 - MINOR - Engineering Brief provenance is too generic for later citation-grade traceability

Brief facts use generic source strings such as `Project Registry`, `M09 + Project Dashboard authority`, and `M10 workflow + Registry`. The project summary contains a canonical task source, but the brief facts do not carry project/source identifiers sufficient for strong later provenance.

Required fix: extend factual brief provenance with bounded project/source IDs or paths where the fact is project/source-derived, while keeping recommendation null in M11.

## E03 - MINOR - Final local/origin equality after the documentation-only log commit is unverified

The persisted builder log proves local/origin equality at implementation commit `010906876a0e0be774cbf61f4e0ce2b59ed410ca`, then states that a final documentation-only commit will follow. The remote branch is now `1f833de5e90a2405599fa55f670924dbb97c4de8`, but GitHub cannot independently prove the builder's local checkout equality after that final push.

Future M11A closure log must record concrete final local SHA, remote SHA and `0 0` divergence after its final evidence commit.

---

# 5. NOTE

## N01 - Front-matter field limit counts recognized fields, not all physical front-matter-like key/value lines

The parser's 32-field limit applies to recognized keys inserted into its field map. Unknown `key: value` lines do not consume that counter. The 64 KiB total manifest cap still limits input size, so this is not elevated above NOTE in this audit. M11A may tighten the parser if it can do so without breaking rollout compatibility.

---

# 6. Positive findings retained

The following implementation choices are sound and should be preserved during remediation:

- fixed manifest location `.hiveai/PROJECT_DASHBOARD.md` beneath registered roots;
- no second recursive crawler for authority ingestion;
- exact v1 schema and `source-map` mode;
- relative path normalization and physical containment checks;
- registered Git identity mismatch -> STALE/fallback behavior;
- canonical source filtering of M09 tasks;
- task-ID deduplication;
- explicit NOT_CANONICALIZED state rather than task invention;
- root `.hiveai/...` watcher classification;
- read-only M11 IPC commands and dedicated Tauri permission;
- factual Engineering Brief recommendation remains null;
- names-only project rail and explicit `Open cockpit` action;
- session-scoped project selection;
- frontend generation guard against stale async snapshot responses;
- browser/native fallback does not launch adapters or mutate project files;
- X01 terminal suppression, X02 intro audio/replay behavior and accepted Akilta external-link path are not regressed by the inspected M11 diff.

---

# 7. Security / safety

PASS on the primary privilege boundary:

- no generic filesystem read IPC;
- no arbitrary SQL IPC;
- no shell/process launch added by M11;
- no network/GitHub API integration;
- no project-file mutation;
- no generated status commit back into tracked project repositories;
- manifest referenced paths are physically containment-checked when existing.

Open safety/performance concern: R08 warning/output amplification must be bounded before M11 closure.

---

# 8. Tracker / documentation truth

Task 0 correctly moved M10 to PASS/CLOSED and strict completed progress to 11/20 = 55%.

M11 must remain FAILED / NOT CLOSED after this audit. M12 remains blocked.

Several M11 package checkboxes in `TASKS.md` are too optimistic after direct source inspection, especially workflow-backed portfolio data, waiting/blocked queue, real mixed-source activity, manifest/direct evidence, and no-double-count/direct production evidence.

Historical M11 builder log must remain immutable. Corrective truth belongs in this audit, the M11A prompt/log, and prospective tracker updates.

---

# 9. Closure gate for M11A

M11A may close only if all of the following are true:

1. R01-R08 are fixed in production source.
2. E01-E03 are corrected or explicitly accepted as non-blocking with truthful evidence.
3. Required native Rust tests actually execute and pass, not only compile.
4. Direct end-to-end watcher -> M09 -> M11 snapshot proof executes.
5. Canonical/non-canonical/fallback/malformed/stale/directory-role/cross-project authority tests execute.
6. Workflow/current-task/attention/queue/activity mixed-evidence tests execute.
7. Full frontend + Rust regression/security/build gates pass.
8. Governed no-bundle QA publication passes with no installer.
9. Final M11A log records concrete final local/origin equality after its final evidence commit.
10. Independent M11A strict re-audit passes.
11. User visual/native acceptance remains separate and must be obtained before final M11 PASS/CLOSED if layout/runtime behavior materially changed.

Until then:

- M00-M10 = PASS/CLOSED.
- M11 = FAIL / remediation required.
- M12 = BLOCKED.
- strict completed roadmap count = **11 / 20 = 55%**.
