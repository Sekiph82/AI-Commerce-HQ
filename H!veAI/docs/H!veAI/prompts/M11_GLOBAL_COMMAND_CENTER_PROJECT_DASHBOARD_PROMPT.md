# M11 — Global Command Center + Project Dashboard Authority Runtime

## Mission

Implement **M11 as one whole milestone**.

Turn the existing H!veAI Command Center from a partially fixture-backed shell into a truthful, bounded, live portfolio operations dashboard backed by the already accepted Registry, Git Engine, watcher, M08 source discovery, M09 task intelligence, and M10 workflow truth.

M11 is also the milestone where the accepted cross-repository `.hiveai/PROJECT_DASHBOARD.md` design becomes a **real runtime authority resolver**.

Do not start M12. Do not implement Codex/Claude adapters, PTY/session execution, Prompt Engine, GPT Audit Engine, GitHub API integration, Next Best Task AI, Project Chat, or an installer.

M11 must close with direct production-path evidence, full regression/security gates, governed no-bundle QA publication, a pushed builder log, user visual acceptance pending where required, and independent strict-audit readiness.

---

# Task 0 — FIRST TASK: synchronize live tracker truth

Before production code changes, repair the stale prospective tracker status left after M10 final manual acceptance.

Read:

- `H!veAI/docs/H!veAI/audits/M10A_WORKFLOW_STATE_MACHINE_STRICT_REAUDIT.md`
- `H!veAI/docs/H!veAI/audits/M10A_MANUAL_ACCEPTANCE_FINAL_CLOSURE.md`

Prospectively update the live tracking/status documents so they say:

- M00-M10 = PASS/CLOSED;
- M10 original strict audit remains historical FAIL;
- M10A remediation + independent re-audit + Akilta native click acceptance are complete;
- Akilta footer link = PASS/ACCEPTED;
- strict completed count = **11 / 20 = 55%**;
- M11 = ACTIVE during this run, later `IMPLEMENTATION COMPLETE / PENDING INDEPENDENT AUDIT + USER VISUAL ACCEPTANCE` only after all builder gates pass;
- M12 remains blocked behind M11;
- M11/M12 are the runtime consumers of the Project Dashboard authority system, but only M11 scope may be implemented in this run.

At minimum synchronize:

- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/README.md`
- `H!veAI/docs/H!veAI/README.md`

In `TASKS.md`, close the remaining M10.10 independent-audit/closure checkboxes and remove the stale M10 pending/manual-Akilta wording prospectively. Preserve all historical audit/log files unchanged.

Do not mark M11 PASS/CLOSED yourself.

---

## Repository preflight

Work from:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Run first:

```powershell
git fetch origin H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
```

Fast-forward only if safe:

```powershell
git merge --ff-only origin/H!veAI
```

Never reset, rebase, force-push, overwrite user work, or create `H!veAI\.git`.

Read before editing:

1. `H!veAI/AGENTS.md`
2. `H!veAI/CONSTITUTION.md`
3. `H!veAI/ARCHITECTURE.md`
4. `H!veAI/TASKS.md`
5. `H!veAI/CODEX_ROADMAP.md`
6. `H!veAI/docs/H!veAI/PROJECT_DASHBOARD_SYSTEM.md`
7. `.hiveai/PROJECT_DASHBOARD.md`
8. M10 prompt, strict audit, M10A prompt, M10A re-audit, final manual closure
9. `H!veAI/src-tauri/src/projects.rs`
10. `H!veAI/src-tauri/src/watcher.rs`
11. `H!veAI/src-tauri/src/task_sources.rs`
12. `H!veAI/src-tauri/src/task_intelligence.rs`
13. `H!veAI/src-tauri/src/workflow.rs`
14. `H!veAI/src-tauri/src/git_engine.rs`
15. current SQLite migrations/schema
16. `H!veAI/src/pages.tsx`
17. `H!veAI/src/registryContext.tsx`
18. `H!veAI/src/projectRegistry.ts`
19. `H!veAI/src/taskIntelligence.ts`
20. `H!veAI/src/workflow.ts`
21. `H!veAI/src/components/Shell.tsx`
22. `H!veAI/src/command-center.css`
23. current tests for Registry/M07/M08/M09/M10/UI
24. this prompt

Preserve user-owned untracked `start-demo.bat` and `task.md` if still present.

---

# Canonical UI Assets

M11 materially replaces placeholder/live-data content, but it must preserve the accepted H!veAI visual identity and shell geometry rather than redesigning the application.

Canonical user-owned asset root:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Preserve:

- one-piece H!veAI sidebar logo geometry;
- accepted background after the sidebar;
- startup intro lifecycle and audible playback;
- footer text and accepted Akilta external link behavior;
- current topbar/sidebar navigation structure;
- stable EXE/shortcut/icon behavior;
- current dark/glass visual language;
- accepted desktop no-outer-scroll behavior.

Required unchanged hashes:

- background SHA-256: `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`
- opening video SHA-256: `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`

Preserve X01 terminal-popup suppression and X02 startup-audio/replay behavior.

Preserve the accepted Akilta footer behavior:

- exact visible sentence remains `Built with ♥ for maximum productivity by Akilta`;
- final `Akilta` remains clickable;
- opens exactly `https://www.akilta.com/` in Google Chrome;
- H!veAI stays open;
- no terminal flash;
- no Edge fallback.

M11 may change Command Center text/data widgets necessary to make them truthful. Do not perform a broad visual redesign.

---

# Current production defects / gaps M11 must close

The current Command Center still contains production-desktop placeholders or derived pseudo-live truth:

- `pages.tsx` imports fixture project/activity/attention/queue data;
- KPI values such as task totals/health are unavailable or fixture-backed;
- the Registry context manufactures pseudo operational values such as `READY_FOR_IMPLEMENTATION`, `Codex`, zero progress, and generic next actions from Registry status alone;
- Current Task, workflow, recent activity and project metrics do not yet consume M09/M10 truth;
- the `AI Assistant` surface claims `GPT-4o` even though no such M11 provider integration exists;
- System Status contains hard-coded summary strings alongside the real detailed native panels;
- the accepted `.hiveai/PROJECT_DASHBOARD.md` manifests are documentation-only and not runtime-consumed;
- M07 watcher refresh currently persists filesystem/Git snapshot truth but does not automatically run M09 parse for task-candidate changes;
- root `.hiveai/...` paths must be classified correctly, not only nested `/.hiveai/...` paths.

M11 must remove these false or incomplete operational claims from the native Command Center.

---

# M11.01 — Native Project Dashboard authority resolver

Create a reusable native Rust service/module, preferably a dedicated module such as:

`H!veAI/src-tauri/src/project_dashboard.rs`

The resolver is project-scoped and reads **only** the fixed manifest location beneath a registered project root:

`.hiveai/PROJECT_DASHBOARD.md`

Do not add a second recursive crawler. M08 remains the source-discovery authority. The M11 resolver reads one fixed manifest and maps its declared authority roles onto existing M08/M09 evidence.

## Supported v1 manifest identity

Require exact schema:

`hiveai-project-dashboard/v1`

Recognize the existing v1 front-matter fields:

- `hiveaiDashboardSchema`
- `projectKey`
- `repository`
- `branchPolicy`
- `dashboardMode`
- `refreshPolicy`

Require `dashboardMode: source-map` for a fully valid v1 manifest.

## Supported authority labels

Parse the existing rollout format under `## Source authorities` without requiring any repository rewrite in M11.

Support these exact logical labels, allowing the singular/plural and slash variants already present in the rollout:

- `Canonical task source:`
- `Handoff source:` / `Handoff sources:`
- `Roadmap source:` / `Roadmap/plan source:` / `Roadmap / plan source:`
- `Progress/history source:` / `Progress/history sources:`
- `Architecture source:` / `Architecture/design source:`
- `Decision source:` / `Decision/governance source:`
- `Agent instruction source:` / `Agent instruction sources:`
- `Security source:`
- `Build/test metadata:`

Extract actual candidate paths only from bounded backtick-delimited path tokens. Free prose such as `workspace manifests where present` is context, not a path.

Recognize phrases such as `none verified`, `none verified yet`, and `none verified at repository root` as **no declared authority**, not as filenames.

Do not parse arbitrary prose into filesystem paths.

## Resolver output

Create a bounded typed model reusable by M12 later. At minimum expose:

- project ID;
- manifest status;
- schema;
- project key;
- declared repository identity;
- branch policy text;
- refresh policy text;
- task-authority state;
- resolved role map;
- normalized relative source paths;
- per-source existence/containment status;
- provenance mode;
- warnings.

Use a finite manifest status set, for example:

```text
VALID
PARTIAL
ABSENT
MALFORMED
STALE
UNAVAILABLE
```

Use a finite task-authority state set, for example:

```text
CANONICAL
NOT_CANONICALIZED
FALLBACK_M08_M09
```

Exact Rust enum names may differ, but serialized strings must be explicit and tested.

## Important authority semantics

### Valid manifest + valid canonical task source

- The declared canonical task source is the task authority.
- M11 task counts/current-task selection must consume M09 tasks whose normalized `sourcePath` matches that canonical source.
- Other roadmap/history/handoff/architecture/instruction roles are context/provenance, not second task ledgers.
- If the same relative file appears in multiple roles, it may appear in provenance roles but must contribute task identity/count **once**.

### Valid manifest + `Canonical task source: none verified...`

This is an intentional state, not a failure.

Expose:

`TASK AUTHORITY NOT YET CANONICALIZED`

Do **not** promote tasks parsed from PROGRESS/ROADMAP/history prose into authoritative portfolio task metrics merely because M09 found candidates.

This rule is required for projects such as FormuLab and future projects that have not yet established a canonical task ledger.

### Manifest ABSENT

Fall back to accepted M08/M09 merged task intelligence. Mark provenance clearly as fallback.

### Manifest MALFORMED / STALE

Do not crash the portfolio.

- emit bounded warning(s);
- do not trust the invalid/stale declared task authority;
- fall back to accepted M08/M09 task intelligence;
- expose the fallback provenance so UI does not imply manifest authority.

### PARTIAL

If the canonical task authority is valid but non-task context sources such as architecture/history/security are missing, keep canonical task authority and mark the missing roles as warnings/partial. Do not downgrade a valid canonical task source merely because a secondary context file is absent.

## Repository identity

When Registry/Git identity provides an unambiguous GitHub `owner/repo`, compare it to the manifest `repository` field case-insensitively after bounded normalization.

A conflicting repository identity is STALE/invalid authority and must not allow cross-project path trust.

If Registry has no GitHub identity, retain the manifest identity as declared provenance but do not invent verification.

## Safety / containment

Manifest and authority resolution must be bounded and physically contained.

At minimum:

- manifest max bytes: 64 KiB;
- max manifest line length: 4096 bytes;
- max front-matter fields: 32;
- max extracted source paths: 128 total;
- max source paths per role: 32;
- max relative path scalar: 512 UTF-8 bytes;
- reject absolute paths;
- reject drive-qualified paths;
- reject UNC paths;
- reject `..` traversal;
- reject NUL/control-character path tricks;
- canonicalize/physically verify existing candidates beneath the registered project root;
- reject symlink/junction escape;
- preserve exact source filename casing in output;
- do not recursively read directories listed as history/build metadata;
- do not read `.env`, credentials, secrets, or arbitrary authority-file contents in M11 merely because the manifest names them;
- do not write, rename, normalize, or auto-fix project files;
- do not commit generated dashboard status back into repositories.

The only project file body M11 itself needs to parse is the fixed manifest. Task content continues through M08/M09.

## No checkbox/task leakage from manifest

The manifest is pointer metadata and must not become a task ledger.

If the manifest itself contains task checkbox syntax such as `[ ]`, `[x]`, `[~]`, `[!]`, emit a warning. Do not turn those manifest checkboxes into portfolio tasks.

---

# M11.01B — Watcher-driven live intelligence refresh

The accepted target flow is:

```text
watcher event
  -> M08 source refresh
  -> M09 task intelligence refresh
  -> manifest authority resolution
  -> Command Center snapshot refresh
```

Implement the narrowest safe integration needed to make this true for registered ACTIVE projects.

## Required behavior

For debounced task/source candidate changes:

- reuse the existing M09 `task_intelligence::parse()` path, which already invokes M08 discovery;
- do not duplicate M08 discovery or M09 parsing logic inside watcher code;
- do not create a second parser;
- keep current watcher debounce/bounds;
- a parse failure must not crash the watcher thread or erase the last good M09 snapshot;
- surface bounded refresh warning/degraded evidence rather than retrying forever;
- do not write any project files.

Correct root `.hiveai` classification so a change at:

`.hiveai/PROJECT_DASHBOARD.md`

is recognized as task/dashboard-relevant. The existing nested `/.hiveai/` recognition is not sufficient for a root `.hiveai/` path.

After a successful relevant refresh, notify the frontend through a bounded native event or an equivalently event-driven generation mechanism so the Command Center can refresh without app restart/manual reload.

Do not implement blind high-frequency polling as the primary refresh architecture.

If using a Tauri event, keep payload minimal, for example:

- project ID;
- refresh category;
- generated timestamp;
- success/warning indicator.

No source body content in event payloads.

## Required direct integration proof

A production-path test must prove:

1. an existing registered project has a canonical task source;
2. task intelligence is parsed;
3. the source file changes;
4. the watcher-relevant refresh path is exercised;
5. M09 task intelligence updates without app restart/manual parse;
6. subsequent M11 snapshot exposes the refreshed task truth.

Also prove root `.hiveai/PROJECT_DASHBOARD.md` receives the correct watcher category.

---

# M11.01C — Live portfolio aggregation service

Create a dedicated read-only portfolio aggregation contract, preferably a Rust module such as:

`H!veAI/src-tauri/src/command_center.rs`

Expose one bounded snapshot command, for example:

`hiveai_command_center_snapshot`

Also expose a narrow reusable per-project resolver command for M12 later, for example:

`hiveai_project_dashboard_resolve`

Exact names may follow repository conventions, but keep the IPC surface narrow and read-only.

Do not expose arbitrary paths, arbitrary SQL, arbitrary manifest text, or generic filesystem reads.

## Portfolio snapshot model

Return bounded factual data sufficient to drive the current Command Center without N frontend calls per project.

At minimum include:

- generated timestamp;
- visible registered projects;
- portfolio KPI data;
- project operation summaries;
- needs-attention items;
- active work queue items;
- recent activity;
- deterministic Engineering Brief facts;
- per-project dashboard authority/provenance summary;
- bounded warnings.

Do not return full source documents or full unbounded task histories.

Recommended portfolio project bound: 128 visible projects per snapshot. If the registry contains more, fail or truncate truthfully with an explicit warning and deterministic ordering. Do not silently ignore overflow.

Recommended recent activity bound: default 50, hard maximum 200.

Recommended attention/work queue bound: 100 each.

Exact constants may be adjusted if existing repository bounds already provide safer values. Record final values in the M11 log.

---

# M11.02 — Truthful KPI strip

Keep the existing six-card KPI geometry, but replace placeholder/fake operational values.

Recommended factual cards:

1. **Projects** — visible registered portfolio projects.
2. **Active tasks** — authoritative/fallback active tasks only; projects with `NOT_CANONICALIZED` task authority must not contribute speculative task counts.
3. **Needs attention** — unique live attention items/projects based on real state.
4. **Running** — real M10 transient running work only (`BUILDER_RUNNING`, `AUDIT_RUNNING`, `VERIFY_RUNNING`) with source-active truth.
5. **Completed tasks** — authoritative/fallback completed tasks.
6. **Portfolio health** — categorical/factual, e.g. `6 / 8 healthy`, not an invented percentage.

Do not retain fake values such as `312`, `87%`, `01`, or browser fixture numbers in the native production dashboard.

If some project task authority is intentionally not canonicalized, show that coverage truth in detail text, for example `2 projects without canonical task authority`, rather than counting candidates as tasks.

## Project health contract

Do not invent a pseudo-scientific score.

Use deterministic categorical health with documented precedence, for example:

```text
MISSING
BLOCKED
ATTENTION
RUNNING
HEALTHY
UNKNOWN
```

At minimum:

- missing/unavailable registered root -> MISSING;
- real BLOCKED/AUDIT_FAILED/FIX_REQUIRED operational state -> BLOCKED or ATTENTION according to the final documented contract;
- WAITING_HUMAN / WAITING_EXTERNAL / DESIGN_GATE -> ATTENTION;
- real RUNNING workflow -> RUNNING;
- valid active project with no attention state -> HEALTHY unless truth is genuinely unknown;
- malformed/stale manifest can degrade to ATTENTION/UNKNOWN but must not fabricate task health.

Document the exact final precedence in source and log.

## Progress

Only compute task progress when a task-authority denominator is real:

`completed authoritative tasks / total authoritative tasks`

If task authority is NOT_CANONICALIZED or total is unknown, show `—`/unknown rather than `0%`.

---

# M11.03 — Current Project operational panel

Preserve the current interaction pattern:

- project rail stays names-only;
- clicking a project selects it in place;
- the center panel updates;
- `Open cockpit` remains the explicit navigation action;
- do not navigate automatically on rail click.

Replace the current placeholder Current Task/Workflow data with real selected-project snapshot truth.

At minimum show:

- project name;
- Registry status / categorical health;
- dashboard authority status/provenance;
- current task title or truthful no-authority/no-active-task state;
- current workflow state where real;
- last workflow action summary/time where real;
- next allowed workflow state/action where real;
- required/allowed actor information where real;
- canonical task source path or fallback/not-canonicalized provenance;
- real task progress when computable.

## Deterministic current-task selection

Define one stable current-task algorithm and test it.

Recommended precedence:

1. source-active workflow-managed non-complete task requiring attention;
2. otherwise source-active workflow-managed non-complete task with newest `latestEvent.occurredAt`, stable ID tie-break;
3. otherwise first active authoritative M09 task in deterministic source/order identity;
4. otherwise no current task.

Do not select a completed task merely because it is newest.

If a valid manifest explicitly says task authority is NOT_CANONICALIZED, do not promote a PROGRESS/ROADMAP candidate into Current Task.

## Existing panel geometry

Use the existing central panel rather than inventing a second competing project-card layout.

The existing `Project metrics` mini-area may be repurposed for a compact live Active Work Queue if that preserves the accepted layout better.

---

# M11.04 — Needs Your Attention

Use real evidence only.

Include bounded items for states such as:

- `WAITING_HUMAN`;
- `DESIGN_GATE`;
- `WAITING_EXTERNAL`;
- `BLOCKED`;
- `AUDIT_FAILED`;
- `FIX_REQUIRED`;
- failed verification/CI evidence where a real failed test row exists;
- pending permission requests only if the existing schema contains a real pending/open state that can be queried truthfully;
- missing registered project roots where appropriate.

Do not label a project as needing attention because of fixture data.

Manifest malformed/stale warnings may be shown as project configuration attention, but keep them distinct from workflow failure.

Each attention item must contain enough factual identity to navigate/select the correct project, but no arbitrary action mutation is required in M11.

Use a compact right-rail surface. The current fake `AI Assistant / GPT-4o` panel is a good candidate to replace with `Needs Your Attention` while preserving overall geometry.

Remove the unsupported `GPT-4o` production claim from M11 UI.

---

# M11.05 — Active Work Queue

Populate from real operational truth only.

Include:

- builder running;
- audit running;
- verify running;
- pending verification state where appropriate;
- blocked/waiting work as a clearly distinct queue state if included.

Each item should expose bounded factual fields such as:

- project;
- task;
- workflow stage/state;
- actor/provider when proved;
- updated/latest-event time;
- attention state.

Do not claim a Codex/Claude/GPT/CI actor from Registry preference alone.

Prefer the current `Project metrics` mini-area or another bounded existing Command Center region rather than adding a large new scrolling table that breaks the accepted one-screen composition.

---

# M11.06 — AI Engineering Brief surface/data contract

M11 does **not** have an AI recommendation engine yet.

Keep the `AI Engineering Brief` surface, but populate it with deterministic factual brief inputs/facts only.

Examples of acceptable factual lines:

- number of active registered projects;
- number of authoritative active tasks;
- number of real attention items;
- number of running workflow tasks;
- selected project current task/state;
- selected project authority provenance;
- recent failed audit/test evidence if real.

Expose provenance/source identity in the data contract so later AI-generated narrative can cite where facts came from.

Do not generate model-authored recommendations in M11.

Do not claim `GPT-4o`, GPT, Codex, Claude, or any provider is backing the brief unless a later real adapter/session proves it.

Clearly separate factual brief items from any future AI recommendation field. Future AI recommendation should be null/unavailable in M11.

---

# M11.07 — Recent Activity

Replace fixture activity with bounded real activity.

Aggregate only real timestamped evidence available in the accepted schema, such as:

- `task_events` workflow transitions/recovery/overrides;
- agent session/events where real rows exist;
- audit results;
- test-run results;
- Git snapshots;
- relevant project/watcher snapshots if useful.

Use deterministic ordering:

- timestamp DESC;
- stable type/id tie-break.

Do not synthesize events that do not exist.

Implement bounded search/filter over the loaded recent-activity set. At minimum support:

- free-text search over bounded display fields;
- event/type filter;
- selected project filtering or a clear portfolio/selected-project mode.

Do not issue arbitrary SQL search strings from the frontend.

---

# M11.08 — Selected-project session memory and race safety

The selected project must remain stable during same-session navigation/refresh.

Use a bounded session-scoped persistence mechanism such as `sessionStorage`, or an equally narrow existing application session mechanism.

Rules:

- remember only the selected project ID;
- never store project source bodies/secrets in browser storage;
- if the saved project ID is no longer present, fall back deterministically after current Registry/snapshot truth is known;
- project rail click updates in place;
- `Open cockpit` is explicit navigation.

## Race safety

Late async refreshes must not replace newer snapshot/selection truth.

Use request-generation identity, cancellation, or an equivalent deterministic guard.

Required test:

- start refresh A;
- start newer refresh B;
- B resolves first and becomes visible;
- A resolves later;
- A must not overwrite B or revert the selected project.

If the final architecture uses a single portfolio snapshot and therefore removes per-project fetch races, still add a direct stale-refresh regression test for the actual refresh mechanism.

---

# M11.09 — System Status truthfulness

Do not regress the accepted native Runtime/Database/Watcher health panels.

The compact System Status area currently contains hard-coded summary strings such as `Operational`, `Schema v7`, `Watching`.

Either:

- derive compact values from the existing real native status contracts; or
- remove the hard-coded summary rows and rely on the already real status components.

Do not maintain two disagreeing system-status truths.

Git Engine may be described as read-only only if the current mutation-status/native contract still proves that fact.

---

# M11.10 — Frontend/native contracts

Add typed TypeScript contracts rather than scattering raw `invoke()` calls through `pages.tsx`.

Recommended files:

- `H!veAI/src/projectDashboard.ts`
- `H!veAI/src/commandCenter.ts`

Exact names may follow current conventions.

The Command Center page should consume one coherent live snapshot contract.

Refactoring the large `pages.tsx` Command Center section into a dedicated component/file is allowed if it reduces complexity, but do not perform unrelated page rewrites.

## Browser preview

Browser preview must not pretend to be live H!veAI.

Do not show fixture portfolio numbers as operational facts.

Show explicit native-data-unavailable/preview states and safe empty values instead of fake `10 projects`, `312 tasks`, `87% health`, fake queues, fake GPT provider, etc.

`fixtures.ts` may remain for unrelated placeholder routes if still needed, but native Command Center production logic must not depend on fixture portfolio/activity/attention/queue truth.

---

# M11.11 — IPC / permission boundary

Add only narrow read permissions required for:

- portfolio snapshot;
- per-project dashboard authority resolution.

No arbitrary filesystem read permission.
No shell/process launch.
No network access.
No GitHub API access.
No arbitrary SQL.
No project file mutation.

Do not broaden an existing permission merely for convenience.

The per-project resolver command accepts a registered `projectId`, not a caller-provided filesystem root or arbitrary path.

---

# Required native production-path tests

Names may vary slightly, but the behaviors are mandatory.

## Manifest resolver

`m11_manifest_valid_canonical_task_authority_filters_task_truth`

- valid v1 manifest;
- canonical task source exists;
- another parser source also contains task-like content;
- only canonical source contributes authoritative task metrics/current-task candidates.

`m11_manifest_not_canonicalized_does_not_promote_progress_tasks`

- valid manifest;
- canonical task source says none verified;
- PROGRESS contains task-like explicit content;
- result says NOT_CANONICALIZED;
- authoritative task count/current task remain unknown/empty.

`m11_manifest_absent_falls_back_to_m08_m09`

`m11_manifest_malformed_falls_back_with_warning`

`m11_manifest_stale_missing_canonical_source_falls_back_with_warning`

`m11_manifest_partial_secondary_missing_preserves_valid_task_authority`

`m11_manifest_duplicate_roles_do_not_double_count_tasks`

`m11_manifest_repository_identity_mismatch_is_not_trusted`

`m11_manifest_rejects_absolute_parent_and_cross_project_paths`

`m11_manifest_rejects_symlink_or_junction_escape`

`m11_manifest_bounds_are_enforced`

`m11_manifest_pointer_checkboxes_never_become_tasks`

Use temporary real project directories and real SQLite Registry/M08/M09 paths. Do not test only a standalone string parser.

Include one fixture modeled on the H!veAI special case where:

- canonical task source is `H!veAI/TASKS.md`;
- a root legacy `TASKS.md` exists;
- root legacy task content must not override/double-count H!veAI child task truth.

Include one fixture modeled on FormuLab's intentional `none verified` canonical task state.

## Watcher integration

`m11_root_hiveai_manifest_is_task_dashboard_relevant`

`m11_watcher_task_change_refreshes_m09_and_dashboard_without_restart`

Preserve all M07 containment/debounce/overflow behavior.

## Portfolio aggregation

`m11_portfolio_counts_use_authoritative_tasks_only`

`m11_current_task_selection_is_deterministic`

`m11_attention_items_come_from_real_workflow_truth`

`m11_running_queue_uses_real_running_states_only`

`m11_recent_activity_is_bounded_and_deterministically_ordered`

`m11_project_health_is_categorical_and_evidence_based`

`m11_missing_project_is_truthful_and_does_not_crash_portfolio`

`m11_archived_project_does_not_leak_into_default_operational_portfolio`

If current Registry semantics intentionally include archived projects somewhere, document the final visible-portfolio rule and test it. Do not silently change M05 archive semantics.

---

# Required mounted frontend tests

At minimum add mounted tests proving:

1. Command Center uses `hiveai_command_center_snapshot` production data rather than fixture portfolio numbers.
2. Six KPI cards render the supplied real snapshot values/details.
3. Project rail remains names-only.
4. Clicking a rail project updates the center Current Project in place and does not auto-navigate.
5. `Open cockpit` navigates explicitly to the selected project.
6. selected project ID survives a same-session remount/route cycle.
7. stale refresh A cannot overwrite newer refresh B.
8. NOT_CANONICALIZED project shows a truthful task-authority message and no invented current task/progress.
9. manifest fallback/partial warnings are represented without crashing the page.
10. Needs Your Attention renders real supplied attention items and a truthful empty state when none exist.
11. Active Work Queue renders real supplied queue items and a truthful empty state when none exist.
12. Recent Activity search/filter works on bounded supplied real activity.
13. Engineering Brief renders factual supplied items and no fake provider claim.
14. native desktop Command Center does not render known fixture values such as `312`, `87%`, or `GPT-4o` as live truth.
15. browser preview does not invent live portfolio metrics.
16. existing footer/Akilta regression remains green.
17. existing startup intro/no-terminal focused regressions remain green.

If using native refresh events, add a frontend test proving the event triggers a bounded snapshot refresh and does not register leaking duplicate listeners after remount.

---

# No-double-count invariants

This is a blocking M11 acceptance condition.

A single logical task must not become multiple portfolio tasks merely because:

- the same file is named in more than one manifest role;
- TASKS and ROADMAP both describe related work;
- HANDOFF repeats a current task title;
- history/changelog repeats old tasks;
- root legacy task files coexist with a manifest-declared child canonical task source.

When a valid canonical task source exists, source-path authority filtering is the first line of defense. M09 stable task identity remains the task identity authority inside the accepted source set.

Do not build a second fuzzy title-deduplication engine in M11.

---

# Activity / data truth rules

- Builder logs are not workflow evidence by themselves.
- Historical audits/logs may appear as provenance sources, but do not automatically become current task state.
- Registry preference for builder/auditor does not prove an active actor.
- Git modified files do not prove implementation completion.
- A watcher event does not prove task completion.
- A malformed manifest does not erase the last good M09 data; it changes authority mode to fallback.
- NOT_CANONICALIZED is a valid truthful state, not an error to hide.
- Do not derive coverage/code-quality/performance percentages unless a real accepted source defines them.
- Do not create a generic AI narrative in M11.

---

# Performance / lifecycle requirements

- No N×unbounded filesystem crawl from the Command Center.
- No high-frequency blind polling as primary refresh mechanism.
- One portfolio snapshot must remain bounded.
- Manifest reads are one fixed file per visible registered project at most per necessary refresh/generation, not recursive scans.
- Reuse M08/M09 persisted intelligence rather than reparsing on every React render.
- Event/listener cleanup must be deterministic.
- Avoid React setState after unmount.
- Avoid stale selected-project data after rapid switching.
- Do not block the UI thread with filesystem parsing.
- Keep native work off the React render path.

If caching a resolved manifest, its invalidation must be driven by project identity/path + watcher-relevant change/generation and must never serve one project's authority map to another project.

Do not introduce a new persistence table merely for cache convenience unless direct source inspection proves it is necessary. Prefer bounded recomputation from one manifest plus existing persisted M08/M09 truth.

---

# Layout requirements

Preserve the accepted one-screen Command Center composition at the current primary desktop viewport.

Recommended mapping to existing geometry:

- top: six truthful KPI cards;
- left: names-only project rail;
- center: selected Current Project + current task/workflow truth;
- center bottom left: Recent Activity;
- center bottom right: compact Active Work Queue;
- right top: factual Engineering Brief;
- right middle: Needs Your Attention, replacing the unsupported fake AI Assistant panel;
- right bottom: truthful System Status.

This mapping is recommended because it satisfies M11 without a broad redesign. If source inspection proves a smaller layout change is safer, preserve the same information hierarchy and document why.

Avoid:

- outer body scrollbar at accepted desktop viewport;
- horizontal overflow;
- unnecessary nested scrollbars;
- tiny unreadable text caused by cramming too much data;
- modal-heavy basic project switching;
- changing sidebar/logo/footer geometry.

Preserve keyboard/focus visibility and accessible labels.

---

# Manual native acceptance required after publication

Because M11 materially changes visible Command Center content, builder closure must leave these items **PENDING USER ACCEPTANCE**:

1. H!veAI opens with no terminal popup.
2. Startup intro/audio behavior remains accepted.
3. Footer Akilta link remains working in Chrome with H!veAI staying open.
4. Command Center shows real registered project names, no fake fixture project metrics.
5. Project rail click changes Current Project in place.
6. `Open cockpit` navigates only when explicitly clicked.
7. KPI values/details look truthful and stable.
8. A canonical-manifest project shows its canonical task authority/provenance.
9. A project with no canonical task authority shows `TASK AUTHORITY NOT YET CANONICALIZED` rather than invented tasks.
10. Needs Your Attention and Active Work Queue look correct/empty truthfully.
11. No `GPT-4o` or fake active-agent claim appears without real evidence.
12. Recent Activity is real/bounded and filters correctly.
13. No outer-body/horizontal overflow or broken nested scrollbars at the accepted desktop viewport.
14. Rapidly switching projects does not flash data from the wrong project.
15. Editing a watched task source causes the Command Center to refresh after the bounded watcher pipeline without app restart/manual parse.

Codex must not self-accept these native visual/runtime checks.

---

# Regression preservation

Do not regress M00-M10 accepted behavior, especially:

- project registration/path repair/archive/remove semantics;
- Git Engine read-only/default-denied mutation boundary;
- watcher containment/debounce/overflow and Git refresh;
- M08 discovery bounds/containment/custom source ordering;
- M09 stable IDs/parser bounds/dependency behavior/source retirement/reappearance;
- M10 workflow actor/evidence/idempotency/recovery/read-model behavior;
- M09/M10 state/history ownership integration;
- X01 no-console Git/process behavior;
- X02 audible startup intro + one-per-process behavior;
- stable publisher/EXE/shortcut/icon;
- canonical background/video bytes;
- Akilta Chrome-only footer link;
- no installer.

No project-native tracked repository outside `AI-Commerce-HQ/H!veAI` may be modified by this M11 implementation. Runtime may read registered local project manifests/sources through bounded accepted paths, but builder must not rewrite FormuLab/Scrubbots/FMCG/etc. repositories in this run.

---

# Security self-audit before push

Before final push, explicitly inspect and record:

- every new `Command::new` / process launch: M11 should add none;
- every new filesystem read path: only registered-root-contained fixed manifest resolution is allowed in M11;
- every frontend argument crossing IPC;
- every Tauri permission/capability change;
- absence of generic URL/path/shell/SQL interfaces;
- no secrets/source bodies in logs or IPC warnings;
- no cross-project manifest cache leakage;
- no symlink/junction containment escape;
- no project-file writes;
- no generated Git commits inside tracked application projects;
- no network/GitHub API usage.

---

# Verification gates

Run and record exact commands/results.

At minimum:

1. focused Project Dashboard resolver Rust tests;
2. focused M11 portfolio aggregation Rust tests;
3. focused watcher -> M09 -> M11 refresh integration tests;
4. existing M08 tests relevant to containment/source discovery;
5. existing M09 parser/integration tests;
6. existing M10 workflow tests;
7. focused M11 mounted frontend tests;
8. existing footer/pre-M10 native UX focused frontend tests;
9. full frontend test suite;
10. `npm run typecheck`;
11. `npm run build`;
12. `npm audit --audit-level=high`;
13. `cargo fmt --all -- --check` from the correct manifest/workspace context;
14. `cargo check`;
15. `cargo test`;
16. `cargo build`;
17. publisher failure/rollback harness;
18. governed Tauri production `--no-bundle` QA publication;
19. stable EXE/shortcut/icon validation;
20. canonical background/video hashes;
21. no-installer scan.

If the repository has dedicated security/QA scripts, run the accepted equivalents and record exact names/results.

Do not erase failed attempts from the builder log after fixing them.

---

# Required M11 builder log

Create:

`H!veAI/docs/H!veAI/codex-logs/M11_GLOBAL_COMMAND_CENTER_PROJECT_DASHBOARD_LOG.md`

Record:

- starting branch/local HEAD/origin HEAD/divergence;
- Task 0 tracker synchronization;
- final architecture chosen;
- exact manifest schema/status/authority enums;
- exact manifest/source bounds;
- exact portfolio/activity/queue bounds;
- source precedence and NOT_CANONICALIZED semantics;
- watcher refresh integration and event/invalidation mechanism;
- changed files/symbols;
- pre-fix failure evidence for meaningful regression tests;
- all failed attempts retained chronologically;
- focused test results;
- full regression/security results;
- production publication evidence;
- stable EXE/icon/shortcut evidence;
- canonical asset hashes;
- user manual acceptance checklist as PENDING;
- implementation commit SHA;
- log/tracker commit SHA;
- exact final local HEAD;
- exact final `origin/H!veAI` HEAD;
- exact `git rev-list --left-right --count HEAD...origin/H!veAI` result.

Do not use `SELF` placeholders. Persist concrete final equality evidence inside the log itself.

If the final log commit changes HEAD after an earlier equality check, run the equality commands again and append the concrete final result before stopping.

---

# Tracker truth at builder stop

Only after implementation + regression + publication pass, prospectively set:

- M00-M10 PASS/CLOSED;
- strict completed count remains `11 / 20 = 55%` until independent M11 audit + required user visual/native acceptance close M11;
- M11 = `IMPLEMENTATION COMPLETE / PENDING INDEPENDENT AUDIT + USER VISUAL ACCEPTANCE`;
- M12 = BLOCKED;
- M13+ remain planned/blocked;
- Project Dashboard runtime resolver = implemented in M11, but M12 Project Cockpit consumption remains future work.

Do not mark M11 PASS/CLOSED.

---

# Stop boundary

Stop after:

- Task 0 tracker sync;
- full M11 implementation;
- direct tests;
- full regression/security gates;
- governed no-bundle QA publication;
- pushed M11 builder log/tracker truth;
- concrete final local/origin equality proof.

Do **not** start M12.

Leave M11 pending independent ChatGPT source-level strict audit and the explicit user native/visual acceptance checklist.