# M11A - Global Command Center + Project Dashboard Strict Closure

## Mission

Perform one bounded M11 remediation run against the independent audit:

`H!veAI/docs/H!veAI/audits/M11_GLOBAL_COMMAND_CENTER_PROJECT_DASHBOARD_STRICT_AUDIT.md`

Fix all open M11 production/evidence findings R01-R08 and E01-E03, plus the user-observed UI/UX acceptance defects UX01-UX04 documented below.

This remains one M11 remediation run. Do not split it into separate milestone prompts. Do not start M12 and do not pull later agent, prompt, audit, GitHub, or AI features forward.

M11 remains FAIL / NOT CLOSED until independent re-audit and required user visual/native acceptance close it.

Current strict completed roadmap count remains **11 / 20 = 55%**.

---

# Task 0 - FIRST TASK: synchronize tracker truth after M11 audit FAIL

Before code changes, read:

- `H!veAI/AGENTS.md`
- `H!veAI/CONSTITUTION.md`
- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/docs/H!veAI/prompts/M11_GLOBAL_COMMAND_CENTER_PROJECT_DASHBOARD_PROMPT.md`
- `H!veAI/docs/H!veAI/codex-logs/M11_GLOBAL_COMMAND_CENTER_PROJECT_DASHBOARD_LOG.md`
- `H!veAI/docs/H!veAI/audits/M11_GLOBAL_COMMAND_CENTER_PROJECT_DASHBOARD_STRICT_AUDIT.md`
- this revised M11A prompt in full

Prospectively update live tracker/status documents so they say:

- M00-M10 = PASS/CLOSED;
- strict completed count = 11/20 = 55%;
- original M11 implementation = historical strict-audit FAIL with 8 MAJOR findings;
- M11A = ACTIVE during this remediation;
- M11 remains NOT CLOSED;
- M12 remains BLOCKED;
- user visual/native acceptance remains pending until after source-level closure;
- the user has reported Command Center layout regressions and Task Sources information-overload that are part of M11A acceptance scope.

Reopen or mark active the M11 package checkboxes contradicted by the audit or the user screenshots. Do not leave workflow-backed portfolio truth, waiting/blocked queue, mixed-source activity, resolver/direct-test closure, no-double-count evidence, no-outer-scroll layout, or user visual acceptance marked validated while their findings are open.

Do not rewrite historical M11 prompt/log/audit files.

---

## Repository preflight

Work from:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Run:

```powershell
git fetch origin H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
```

Fast-forward only if safe:

```powershell
git merge --ff-only origin/H!veAI
```

Never reset, rebase, force-push, overwrite user work, or create `H!veAI\.git`.

Preserve user-owned parent-root untracked `start-demo.bat` and `task.md` if present.

Do not modify tracked files in any registered project repository outside `AI-Commerce-HQ/H!veAI` during M11A. In particular do not touch Bulk Edit while its Etsy process is pending.

---

# Canonical UI Assets

This is a closure/remediation run. Preserve the accepted H!veAI visual identity while correcting the broken Command Center composition.

Canonical user-owned asset root:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Preserve:

- accepted one-piece H!veAI sidebar logo;
- accepted background position after sidebar;
- startup intro video lifecycle and audible playback;
- accepted topbar/sidebar/navigation geometry;
- dark/glass Command Center visual language;
- stable EXE, shortcut and icon;
- exact footer sentence `Built with ♥ for maximum productivity by Akilta`;
- accepted Akilta Chrome link behavior;
- X01 no-terminal-popup behavior;
- X02 startup audio/no-same-process-replay behavior.

Required unchanged canonical hashes:

- background SHA-256: `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`
- opening video SHA-256: `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`

The user screenshots are explicit acceptance evidence that the current M11 layout is not acceptable. M11A may change Command Center sizing/composition and Task Sources presentation as required below. Do not perform an unrelated visual redesign.

---

# UX01 - Restore one-screen Command Center composition

## User-observed defect

At normal desktop size the new M11 Command Center no longer behaves like a clean one-screen command center:

- the Projects rail has its own vertical scrollbar;
- the right rail is vertically overpacked;
- Active Work Queue is pushed/clipped beneath Needs Your Attention/System Status and is not properly visible;
- nested scrollbars make the page look broken;
- the footer/content relationship is cramped;
- the central current-task area has too much dead/empty geometry when data is unavailable.

## Required production behavior

At accepted desktop viewport sizes the Command Center must fit as a deliberate one-screen dashboard above the footer without page-level vertical scrolling and without nested vertical scrollbars in the project rail or right rail.

Use responsive CSS/grid sizing, not hard-coded pixel hacks tied to one screenshot.

Required desktop behavior:

- no `body`/main-page vertical scrollbar for the Command Center at normal supported desktop viewport sizes;
- no vertical scrollbar inside Projects rail;
- no vertical scrollbar inside Needs Your Attention;
- no vertical scrollbar inside Active Work Queue;
- no right-rail card may overlap or hide another card;
- Active Work Queue must always be visibly reachable without scrolling the whole Command Center;
- System Status must be compact and must not consume the space needed by operational panels;
- Current Project/Current Task area should use available space efficiently and must not display fake `0 / 0` when task truth is unavailable;
- footer remains visible and does not cover dashboard content.

For the project rail, show a bounded names-only set that fits. Prefer a maximum visible count with an honest `+N more`/`View all projects` affordance rather than an inner scrollbar. Clicking a visible project still selects it in place. `View all projects` remains the route for the full list.

For right-rail panels, use compact rows and bounded visible counts. If more items exist, show a truthful count plus `View all`/route affordance rather than a nested scrollbar.

## Required visual/layout tests

Add browser/component evidence for at least two representative desktop viewport sizes and prove:

- Command Center root does not create outer vertical overflow;
- project rail does not use vertical scrolling;
- right rail has no overlapping cards;
- Active Work Queue heading/content is visible;
- footer is not overlapped;
- selected-project interaction remains intact.

User native visual acceptance remains required after publication.

---

# UX02 - Remove the giant full-width Recent Activity block from the home dashboard

## Decision

The full-width Recent Activity section at the bottom of Command Center is not necessary on the home dashboard. H!veAI already has a dedicated `Activity` route.

## Required production behavior

- Remove the large full-width `Recent Activity` panel from the Command Center home page.
- Keep only a compact selected-project recent-activity summary inside the Current Project area, maximum 3 to 5 rows.
- Add a small explicit `View activity` navigation affordance if useful.
- Search/filter for the complete activity history belongs on the dedicated Activity page, not the Command Center home.
- Do not delete the underlying bounded M11 activity aggregation. R06 still requires real activity data because Activity and M12 can consume it.

This change should recover substantial vertical space and restore the Command Center's one-screen purpose.

---

# UX03 - Simplify Task Sources presentation without weakening M08/M09

## User-observed defect

The Task Sources page can show 15+ files for one project, including `TASKS.md`, `AGENTS.md`, `.hiveai/audits/*`, `.hiveai/codex-runs/*`, prompts, decisions, handoffs and `PROJECT_DASHBOARD.md`.

Those files are valid internal evidence/source candidates, but presenting all of them as the primary project-tracking surface makes it look as if H!veAI needs 15 task files to understand one project.

That is not the intended mental model.

## Required architecture decision

Preserve M08 source discovery and M09 parsing under the hood. Do not delete source discovery and do not stop collecting bounded evidence.

However, make **one file the user-facing H!veAI entry contract for every project**:

`.hiveai/PROJECT_DASHBOARD.md`

This file already exists in the rollout and remains the single Project Dashboard authority/manifest entrypoint. Do not create a second competing project-status file.

Important: the dashboard file is an index/authority contract, not a duplicate mega-ledger. It can point to the canonical task ledger, handoff, roadmap, history, architecture, decisions, instructions, security and build/test evidence. H!veAI may combine those pointers with Registry/M09/M10/SQLite runtime truth. The user should not need to manage or understand the complete source inventory for ordinary project tracking.

Do not auto-rewrite `.hiveai/PROJECT_DASHBOARD.md` on every watcher event in M11A. That would dirty project repositories and create commit noise. H!veAI reads it as the stable entry contract while dynamic operational state remains in accepted SQLite/M09/M10 truth.

## Task Sources UI behavior

Change the Tasks/Task Sources default presentation so the primary surface shows a compact **Project Intelligence / Dashboard Contract** summary for the selected project:

- `.hiveai/PROJECT_DASHBOARD.md` status;
- manifest status;
- task authority state;
- canonical task source, if any;
- current M09 refresh status/time;
- number of discovered internal evidence sources;
- warnings/degraded state;
- a clear `Advanced source inventory` disclosure/action.

The raw 15+ source table becomes an **advanced diagnostics inventory**, collapsed/hidden by default or placed behind an explicit advanced action. It must not dominate the normal Tasks page.

When advanced inventory is opened, keep existing M08 truth, kinds, authority classes, modified time, and status. Long inventories may use a deliberately bounded table viewport because the user explicitly opened diagnostics. The normal page itself should not require an outer page scrollbar merely to understand project status.

Do not falsely relabel all M08 candidates as task authorities. Clearly distinguish:

- Project Dashboard entry contract;
- canonical task authority;
- context/provenance evidence;
- instruction/config files;
- advanced discovered sources.

## Required tests

- selected project with 15 discovered sources still shows one Project Dashboard entry contract as the primary normal surface;
- advanced source count says 15 without rendering the entire inventory by default;
- canonical task authority is separately identified;
- expanding/opening Advanced source inventory reveals the real M08 rows without data loss;
- no duplicate task metrics are created from context sources.

---

# UX04 - Single-entry Project Dashboard contract for current and future projects

This is the long-term rule M11A must preserve for M12 and future projects:

```text
<project root>/.hiveai/PROJECT_DASHBOARD.md
                  |
                  +-- canonical task source
                  +-- handoff/context
                  +-- roadmap/plan
                  +-- history/audits/logs
                  +-- architecture/decisions
                  +-- instructions/security/build metadata

H!veAI runtime
  Registry + M08 + M09 + M10 + SQLite
                  |
                  v
       truthful Project Dashboard snapshot
```

From the user's perspective there is one H!veAI project contract file per repository. Internally H!veAI may use many evidence files, but they remain implementation detail unless the user opens Advanced diagnostics.

Do not introduce `.hiveai/PROJECT_STATUS.md`, `.hiveai/HIVEAI.md`, another JSON manifest, or any other competing entry file in M11A.

Do not rewrite external repositories in this run. Existing rollout manifests remain compatible.

---

# R01 - Fix M10 workflow integration

## Defect

M11 currently calls M10 `workflow::project_list()` with `limit: Some(4096)`, while M10 rejects values above 500. M11 then swallows that error and substitutes an empty workflow list.

## Required production behavior

- M11 must request workflow tasks within the accepted M10 bound.
- Prefer reusing `workflow::MAX_HISTORY_LIMIT` or a safe public/internal constant rather than duplicating an unexplained number.
- Do not silently convert workflow read failure into empty workflow truth.
- A workflow read error must produce explicit bounded unknown/warning state while preserving whatever independent task/registry truth remains available.
- Current state, last action, allowed actors, attention, running queue and workflow-based health must consume real M10 rows when available.

## Required direct test

Create a real temp DB/project/M09 task and M10 events, then call the actual M11 snapshot path and prove:

- current workflow state is returned;
- latest action is returned;
- allowed actor data is returned;
- an attention state appears in attention;
- a running/verification state appears in queue where applicable.

**PASS only if this test would fail on the pre-fix `limit: Some(4096)` implementation.**

---

# R02 - Preserve UNKNOWN as unknown, never as fake zero

## Defect

Missing M09 snapshot can become task totals of zero for canonical/fallback projects. Fallback authority is also counted as if it were canonical. Frontend registry fallback reports zero attention/running even though snapshot truth is unavailable.

## Required production behavior

Introduce explicit known/unknown semantics without fabricating metrics.

- If M09 intelligence is missing/unreadable, authoritative task totals must be `None`/null/unavailable, not 0.
- A real empty parsed authoritative source may be numeric zero only when M09 actually parsed and produced a valid empty snapshot.
- `CANONICAL`, `FALLBACK_M08_M09`, and `NOT_CANONICALIZED` must remain distinct in authority coverage.
- `authorityDetail` must count true canonical authority truthfully.
- Registry-only snapshot failure must not show `Needs attention = 0` or `Running = 0` as if known. Make those fields nullable/known-state-aware or display `—`/Unavailable.
- Keep browser/native types aligned.

## Required tests

1. canonical manifest + no persisted M09 snapshot -> task counts unavailable;
2. fallback manifest state + no persisted M09 snapshot -> task counts unavailable;
3. canonical manifest + successfully parsed empty task source -> numeric zero is allowed;
4. registry-only frontend fallback -> attention/running displayed unavailable, not false zero;
5. authority detail distinguishes canonical vs fallback vs not-canonicalized.

---

# R03 - Exclude M10-complete tasks from current-task selection

## Defect

Current-task candidate filtering checks parser completion before matching workflow, so a source-active task with parser status active but M10 state `TASK_COMPLETE` can be selected.

## Required production behavior

Use the matching workflow task when determining completion.

Required precedence remains:

1. authoritative/source-active workflow-managed non-complete task requiring attention;
2. otherwise authoritative/source-active workflow-managed non-complete task with newest real latest event, stable task-ID tie-break;
3. otherwise first deterministic non-complete authoritative M09 task;
4. otherwise none.

`TASK_COMPLETE` must never be selected as current when its M10 workflow row exists, regardless of stale parser status.

## Required test

Seed two authoritative tasks:

- task A parser says active, M10 says TASK_COMPLETE with newest event;
- task B is genuinely active.

Prove M11 selects B and never A.

**PASS only if the test fails on the current implementation.**

---

# R04 - Support contained directory provenance pointers without reading them

## Defect

Resolver uses `candidate.is_file()` for every authority role. Accepted rollout manifests contain directory provenance pointers such as:

- `H!veAI/docs/H!veAI/audits/`
- `H!veAI/docs/H!veAI/codex-logs/`

Those are incorrectly marked missing.

## Required production behavior

Make pointer validation role-aware.

- `canonicalTask` must be a contained regular file.
- File-oriented roles should remain file-oriented unless an accepted rollout example proves otherwise.
- `progressHistory` and `buildTest` may accept a physically contained file OR directory pointer when the manifest declares one.
- Do not recursively read a declared directory.
- Do not enumerate directory contents merely to make it available.
- Preserve symlink/junction escape rejection.
- A valid contained history directory must not downgrade the manifest to PARTIAL.
- A missing/rejected secondary pointer may make a valid canonical manifest PARTIAL but must not destroy canonical task authority.

## Required tests

- H!veAI-style manifest with two contained history directories -> VALID if every declared role is available;
- missing history directory -> PARTIAL but canonical remains CANONICAL;
- canonical task path pointing to a directory -> rejected/stale;
- directory symlink/junction escape -> rejected where supported by test platform, otherwise prove through containment helper with platform-safe fixture.

---

# R05 - Surface watcher/M09 refresh failure and recovery

## Defect

Watcher emits `success=false` on M09 parse failure, but the product effectively ignores it. Last-good truth remains, which is good, but failure is invisible.

## Required production behavior

Preserve last-good M09 snapshot and add bounded refresh health truth.

Use an existing safe status/persistence mechanism where possible. Do not create a second parser or unbounded event log.

At minimum expose per project:

- last task/dashboard refresh status;
- last refresh timestamp;
- bounded error code/message or degraded flag when parse failed;
- successful later refresh clears the active failure/degraded state while historical persistence may remain if the existing model supports it.

Command Center snapshot must carry the degraded refresh warning/state.

Frontend event handling must not ignore `success=false`. It may simply refresh the native snapshot and render its warning, but the failure must become visible/truthful.

## Required production-path tests

1. valid source refresh success;
2. force/fixture a parse failure after last-good snapshot;
3. prove last-good task truth remains;
4. prove M11 snapshot exposes degraded refresh warning;
5. next successful parse clears active degraded status and updates M11 truth.

Do not add infinite retry/polling.

---

# R06 - Complete factual portfolio aggregation promised by M11

## Defect

Current M11 activity reads only `task_events`, queue excludes waiting/blocked work, and attention omits real failed test/permission evidence.

## Required production behavior

Use the already accepted SQLite tables and finite bounds. Do not create fake events.

### Needs Your Attention

Include, when real rows exist:

- WAITING_HUMAN;
- WAITING_EXTERNAL;
- DESIGN_GATE;
- BLOCKED;
- AUDIT_FAILED;
- FIX_REQUIRED;
- relevant AUDIT_REQUIRED/VERIFY_REQUIRED if the established contract treats them as attention;
- failed completed verification/test rows for the project/task;
- pending/open permission requests only for explicit states actually represented by existing schema/data;
- missing Registry roots;
- Project Dashboard malformed/stale/refresh-degraded configuration warnings, clearly categorized apart from workflow failure.

### Active Work Queue

Include bounded real items for:

- BUILDER_RUNNING;
- AUDIT_RUNNING;
- VERIFY_RUNNING;
- pending verification where appropriate;
- WAITING_HUMAN / WAITING_EXTERNAL / DESIGN_GATE / BLOCKED as waiting/blocked queue items if this is the accepted M11 queue contract.

Actor/provider must come only from real workflow/session evidence.

### Recent Activity data contract

Build one deterministic bounded merged timeline from real timestamped rows already available now, where applicable:

- workflow/task events;
- agent sessions/events;
- audits;
- test runs;
- Git snapshots;
- project/watcher snapshots.

The home Command Center does not render the giant full-width history panel after UX02. The aggregation remains available for the compact selected-project summary and dedicated Activity route/future consumers.

Requirements:

- stable cross-table kind + ID identity;
- order by timestamp DESC, then deterministic kind/ID tie-break;
- hard global activity bound;
- no N x unbounded query explosion;
- no source bodies/secrets in activity;
- do not manufacture rows when a table is empty.

If a table's actual schema does not support a claimed field/state, document that and omit only that unsupported slice. Do not silently mark the package complete without evidence.

## Required tests

Seed mixed real rows in a temp database and prove:

- each supported activity class appears exactly once;
- deterministic ordering/tie-break;
- queue contains running and waiting/blocked examples;
- failed test appears in attention;
- pending permission appears only for actual pending/open state;
- global bounds truncate deterministically.

Update `TASKS.md` M11.04/M11.05/M11.07 only according to real implemented/tested truth.

---

# R07 - Make native direct evidence actually execute

## Defect

Original M11 Rust tests compiled but did not execute because Windows launched the generated test executable with `STATUS_ENTRYPOINT_NOT_FOUND (0xc0000139)`. Several `command_center.rs` tests are also helper-only despite production-path names.

## Required process

First diagnose the test-launch environment narrowly.

- Record exact Rust toolchain, target triple, generated test executable path, and failure.
- Inspect PATH/runtime-DLL collision only as needed.
- Do not uninstall/reinstall unrelated software or change global machine state destructively.
- Prefer repository-local/shell-local environment correction.
- Do not edit production code merely to hide an environment loader problem.

M11A cannot claim native direct evidence PASS until Rust assertions actually execute.

## Strengthen tests

Replace helper-only proofs with real production-path tests covering:

- resolver present/absent/none-verified/malformed/stale repository identity;
- traversal/absolute/drive/UNC/control/path bounds;
- contained directory roles;
- cross-project/symlink containment;
- canonical task filtering and no-double-count;
- missing M09 -> unknown metrics;
- deterministic current task with M10 completion;
- M10 workflow snapshot integration;
- attention/queue/mixed activity;
- watcher actual task-source change -> M09 parse -> M11 snapshot updated;
- root `.hiveai/PROJECT_DASHBOARD.md` classification;
- warning bound overflow;
- UX03 primary Dashboard Contract presentation with advanced source inventory hidden by default.

For the watcher chain, test the production helper/path rather than merely `category_hint()`.

If the native executable still cannot run after bounded environment diagnosis, STOP and report the milestone as still blocked. Do not publish a PASS claim.

---

# R08 - Bound warning cardinality and snapshot output

## Defect

Manifest checkbox/path warnings have no count cap, and Command Center aggregates project warnings with no global warning bound.

## Required production behavior

Add explicit constants and deterministic behavior, for example:

- per-manifest warnings <= 64;
- per-project snapshot warnings <= 64;
- portfolio warnings <= 256;
- warning scalar <= 1024 or existing safer scalar bound.

Exact values may differ if repository conventions already define a tighter safe bound.

Requirements:

- deduplicate identical warnings where possible;
- truncate deterministically;
- emit one final bounded `WARNING_LIMIT_REACHED` warning when truncation occurs;
- do not build the full unbounded warning vector and truncate only at the IPC edge;
- keep event/error messages free of source bodies/secrets.

## Required tests

Generate a bounded manifest with many checkbox-warning lines and prove warning count and scalar sizes stay within contract. Also prove 128-project aggregation cannot exceed portfolio warning bounds.

---

# E01 - Remove fake named browser-preview identity

Browser preview must not present `FormuLab`, `Scrubbots`, or any other named project as current when native Registry evidence is unavailable.

Required preview state:

- zero projects or neutral Preview/Native data unavailable identity;
- no fake current task/provider/project;
- no named placeholder action that looks project-derived.

Keep unrelated fixture routes outside native Command Center scope untouched if still needed elsewhere.

Add a focused frontend test.

---

# E02 - Strengthen Engineering Brief factual provenance

Recommendation remains `null` in M11.

For factual brief inputs, add bounded structured provenance sufficient for later citation, such as:

- source class;
- project ID where project-specific;
- canonical task source path / manifest provenance where relevant;
- evidence row ID/type where appropriate.

Do not expose full source contents.

Update TypeScript contract and focused rendering tests if provenance becomes visible.

---

# E03 - Final repository equality evidence

The M11A log must contain the concrete final post-log-commit proof, not a placeholder.

After the final evidence/log commit is pushed:

```powershell
git fetch origin H!veAI
git rev-parse HEAD
git rev-parse origin/H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
```

Persist the actual final SHA values and `0 0` divergence into the M11A log. If adding that proof requires another documentation commit, repeat until the persisted final log and final response truthfully identify the actual remote HEAD and prove equality. Avoid an infinite self-referential wording loop: it is acceptable for the final response to name the last log-doc commit, but the persisted log must at least contain equality proof for the implementation/evidence commit it describes and no `pending` placeholders.

---

# Native IPC / permission boundary

Preserve the narrow read-only M11 surface:

- `hiveai_command_center_snapshot`
- `hiveai_project_dashboard_resolve`

Do not add:

- arbitrary filesystem path reads;
- arbitrary SQL;
- shell/process execution for M11 features;
- network/GitHub calls;
- project-file mutations.

If refresh-health state needs persistence, use bounded existing SQLite structures/settings/snapshot metadata or one minimal migration only if truly necessary and justified. Do not create an architectural shadow database.

---

# M08 / M09 / M10 / M11 ownership protection

Do not regress accepted ownership boundaries:

- `.hiveai/PROJECT_DASHBOARD.md` is the single user-facing project entry contract;
- M08 discovers bounded source/evidence candidates behind that contract;
- M09 parses/persists source task intelligence;
- M10 owns operational state and task events;
- M11 aggregates/resolves/read-displays truth.

M11 must not rewrite task state, fabricate M10 events, or parse project task bodies itself.

Watcher-driven refresh must call the existing M09 production path.

Do not confuse `source inventory` with `task authority` in either code or UI.

---

# Frontend race and selection protection

Preserve and directly test:

- generation guard against stale snapshot response;
- listener cleanup;
- selected project remains stable when valid;
- missing selected project deterministically falls back to a real registered project;
- rail click does not navigate;
- `Open cockpit` remains explicit navigation;
- no stale project data appears after rapid project switching/refresh;
- no Command Center nested vertical scrollbars at accepted desktop sizes;
- compact activity summary remains project-scoped.

---

# Required full verification

After focused tests pass, run the complete repository-defined gates.

At minimum:

```text
cargo fmt -- --check
cargo check
cargo test
npm run typecheck
npm test
npm run build
npm audit --audit-level=high
git diff --check
```

Use the Windows command spelling required by the local shell where necessary.

Native Rust tests MUST actually execute. `cargo test --no-run` alone is insufficient for M11A closure.

Run the governed publisher/failure harness required by `AGENTS.md` and produce the stable no-bundle QA executable. Do not create an installer.

Verify canonical background/video hashes remain unchanged and preserve X01/X02/Akilta behavior by source/config diff.

---

# User visual/native acceptance

Do not self-accept UI/native behavior.

After successful M11A implementation/publication, leave these pending for the user:

- Command Center fits as a clean one-screen desktop dashboard with no outer vertical scroll;
- no nested scrollbars in Projects, Needs Your Attention, or Active Work Queue;
- Active Work Queue is clearly visible and not hidden beneath System Status;
- giant full-width Recent Activity block is gone from home;
- compact selected-project Recent Activity is useful and does not dominate layout;
- live project/task/authority metrics look truthful and unknown values do not appear as false zero;
- Task Sources normal view presents one `.hiveai/PROJECT_DASHBOARD.md` Project Dashboard contract as the primary project intelligence surface;
- 15+ discovered files are available only through explicit Advanced source inventory rather than dominating normal project tracking;
- FormuLab/no-authority state appears correctly if registered and available;
- project rail selection updates in place;
- Open cockpit navigates explicitly;
- watcher-driven source change visibly refreshes without restart if practical to demonstrate;
- startup intro/audio/background/footer/Akilta remain correct;
- no terminal popup regression.

Independent source audit occurs before final M11 closure. User acceptance must remain recorded separately.

---

# Tracker state at builder exit

Only after UX01-UX04, R01-R08, E01-E03 implementation, actual native test execution, full gates and publication succeed:

- M00-M10 remain PASS/CLOSED;
- M11A = IMPLEMENTATION COMPLETE / PENDING INDEPENDENT RE-AUDIT + USER VISUAL/NATIVE ACCEPTANCE;
- M11 = NOT YET PASS/CLOSED;
- strict completed remains 11/20 = 55%;
- M12 remains BLOCKED.

Do not mark M11 PASS/CLOSED yourself.

---

# Builder log

Create immutable:

`H!veAI/docs/H!veAI/codex-logs/M11A_GLOBAL_COMMAND_CENTER_STRICT_CLOSURE_LOG.md`

Record:

- starting branch/local/remote SHA/divergence;
- Task 0 tracker changes;
- UX01-UX04, R01-R08 and E01-E03 changes by file/symbol;
- pre-fix failing test evidence for each production defect where practical;
- exact native test-launch diagnosis and resolution;
- actual executed Rust test counts/results;
- frontend test counts/results;
- viewport/layout evidence for Command Center and simplified Task Sources normal view;
- full gate outputs;
- publisher first/failed attempts and final success truthfully;
- canonical asset hashes;
- security/IPC review;
- explicit proof that no external tracked project repository was changed;
- implementation/evidence commit SHAs;
- concrete final local/origin SHA/divergence proof;
- user acceptance still pending;
- independent re-audit still pending.

Historical M11 log remains immutable.

---

# Stop condition

STOP after:

1. bounded M11A remediation including UX01-UX04;
2. actual native direct tests + frontend/layout tests;
3. full regression/security/build gates;
4. governed no-bundle QA publication;
5. pushed immutable M11A log;
6. concrete final repository equality proof;
7. tracker state remains pending independent re-audit/user acceptance.

Do not start M12.