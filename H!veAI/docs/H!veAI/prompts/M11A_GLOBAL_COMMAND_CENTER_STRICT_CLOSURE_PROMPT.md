# M11A - Global Command Center + Project Dashboard Strict Closure

## Mission

Perform one **bounded M11 remediation run** against the independent audit:

`H!veAI/docs/H!veAI/audits/M11_GLOBAL_COMMAND_CENTER_PROJECT_DASHBOARD_STRICT_AUDIT.md`

Fix exactly the open M11 production/evidence findings R01-R08 and E01-E03. Do not redesign M11, do not start M12, and do not pull later agent/prompt/audit/GitHub/AI features forward.

M11 remains FAIL / NOT CLOSED until an independent re-audit and required user visual/native acceptance close it.

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

Prospectively update live tracker/status documents so they say:

- M00-M10 = PASS/CLOSED;
- strict completed count = 11/20 = 55%;
- original M11 implementation = historical strict-audit FAIL with 8 MAJOR findings;
- M11A = ACTIVE during this remediation;
- M11 remains NOT CLOSED;
- M12 remains BLOCKED;
- user visual/native acceptance remains pending until after source-level closure.

Reopen or mark active the M11 package checkboxes contradicted by the audit. In particular do not leave workflow-backed portfolio truth, waiting/blocked queue, mixed-source activity, resolver/direct-test closure, or no-double-count evidence marked validated while their findings are open.

Do not rewrite the historical M11 prompt/log/audit.

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

---

# Canonical UI Assets

This is a closure/remediation run. Do not redesign the visual system.

Canonical user-owned asset root:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Preserve:

- accepted one-piece H!veAI sidebar logo;
- accepted background position after sidebar;
- startup intro video lifecycle and audible playback;
- accepted topbar/sidebar/navigation geometry;
- dark/glass Command Center visual language;
- accepted no-outer-scroll desktop behavior;
- stable EXE, shortcut and icon;
- exact footer sentence `Built with ♥ for maximum productivity by Akilta`;
- accepted Akilta Chrome link behavior;
- X01 no-terminal-popup behavior;
- X02 startup audio/no-same-process-replay behavior.

Required unchanged canonical hashes:

- background SHA-256: `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`
- opening video SHA-256: `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`

Only make UI changes necessary to correct truthfulness, warning/error presentation, preview placeholders, and factual provenance.

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

### Recent Activity

Build one deterministic bounded merged timeline from real timestamped rows already available now, where applicable:

- workflow/task events;
- agent sessions/events;
- audits;
- test runs;
- Git snapshots;
- project/watcher snapshots.

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
- warning bound overflow.

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

The M11A log must contain the **concrete final post-log-commit** proof, not a placeholder.

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

# M09 / M10 ownership protection

Do not regress accepted ownership boundaries:

- M08 discovers source candidates;
- M09 parses/persists source task intelligence;
- M10 owns operational state and task events;
- M11 aggregates/resolves/read-displays truth.

M11 must not rewrite task state, fabricate M10 events, or parse project task bodies itself.

Watcher-driven refresh must call the existing M09 production path.

---

# Frontend race and selection protection

Preserve and directly test:

- generation guard against stale snapshot response;
- listener cleanup;
- selected project remains stable when valid;
- missing selected project deterministically falls back to a real registered project;
- rail click does not navigate;
- `Open cockpit` remains explicit navigation;
- no stale project data appears after rapid project switching/refresh.

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

- Command Center layout/visual acceptance at normal desktop size;
- live project/task/authority metrics look truthful;
- FormuLab/no-authority state appears correctly if registered and available;
- project rail selection updates in place;
- Open cockpit navigates explicitly;
- watcher-driven source change visibly refreshes without restart if practical to demonstrate;
- startup intro/audio/background/footer/Akilta remain correct;
- no terminal popup regression.

Independent source audit occurs before final M11 closure. User acceptance must remain recorded separately.

---

# Tracker state at builder exit

Only after R01-R08/E01-E03 implementation, actual native test execution, full gates and publication succeed:

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
- R01-R08 and E01-E03 changes by file/symbol;
- pre-fix failing test evidence for each production defect where practical;
- exact native test-launch diagnosis and resolution;
- actual executed Rust test counts/results;
- frontend test counts/results;
- full gate outputs;
- publisher first/failed attempts and final success truthfully;
- canonical asset hashes;
- security/IPC review;
- no external tracked project repository changes;
- implementation/evidence commit SHAs;
- concrete final local/origin SHA/divergence proof;
- user acceptance still pending;
- independent re-audit still pending.

Historical M11 log remains immutable.

---

# Stop condition

STOP after:

1. bounded M11A remediation;
2. actual native direct tests + frontend tests;
3. full regression/security/build gates;
4. governed no-bundle QA publication;
5. pushed immutable M11A log;
6. concrete final repository equality proof;
7. tracker state remains pending independent re-audit/user acceptance.

Do **not** start M12.
