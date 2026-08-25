# M10 — Workflow State Machine

## Mission

Implement M10 as one whole milestone.

Turn M09 parser truth into explicit, durable, evidence-backed operational workflow truth without starting M11/M12 UI work, agent adapters, Prompt Engine, GPT Audit Engine, GitHub integration, or Project Dashboard runtime ingestion.

M10 owns operational task transitions and durable transition history. M09 remains the source/parser authority.

M10 must close with direct production-path evidence, full regression/security gates, governed no-bundle QA publication, pushed builder log, and independent audit readiness.

---

## Start / repository preflight

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

Before editing, read:

1. `H!veAI/AGENTS.md`
2. `H!veAI/CONSTITUTION.md`
3. `H!veAI/ARCHITECTURE.md`
4. `H!veAI/TASKS.md`
5. `H!veAI/CODEX_ROADMAP.md`
6. `H!veAI/docs/H!veAI/audits/PRE_M10_NATIVE_UX_HOTFIX_X01_X02_MANUAL_ACCEPTANCE_CLOSURE.md`
7. `H!veAI/src-tauri/src/db/migrations.rs`
8. `H!veAI/src-tauri/src/task_intelligence.rs`
9. `H!veAI/src-tauri/src/lib.rs`
10. `H!veAI/src-tauri/permissions/foundation.toml`
11. `H!veAI/src-tauri/capabilities/default.json`
12. existing TypeScript native-contract files such as `taskIntelligence.ts`, `projectRegistry.ts`, `gitEngine.ts`
13. this prompt

Inspect the actual current M09 persistence/reconciliation code before deciding where the M10 integration hooks belong.

---

# Canonical UI Assets

M10 is a native/domain milestone and MUST NOT redesign or visibly change the H!veAI UI.

Canonical user-owned asset root:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Preserve the accepted assets and presentation byte-for-byte unless an unrelated build process legitimately copies unchanged bytes:

- `scene 3 starting point.png`
- `videos and gifs\opening video.mp4`
- `H!veAI logo.png`
- `H!veAI small logo.png`

Known canonical repository asset hashes that must remain unchanged:

- background SHA-256: `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`
- opening video SHA-256: `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`

Preserve:

- accepted sidebar/logo geometry;
- post-sidebar background positioning;
- startup intro lifecycle/audio fixes;
- footer;
- routes/navigation;
- stable launcher/icon;
- current no-scroll accepted shell behavior.

No visible M11/M12 dashboard/cockpit work in M10.

---

# Current architecture truth to preserve

Existing M04 schema already provides:

- `tasks.state`
- `task_events`
- `prompts`
- `agent_sessions`
- `audits`
- `test_runs`
- `decisions`

`task_events` already has:

- `task_id`
- `event_type`
- `from_state`
- `to_state`
- `actor_type`
- `summary`
- `evidence_json`
- `occurred_at`

Do not create a parallel workflow database/table merely because it is easier.

The intended M10 architecture is:

- `tasks.state` = materialized current operational state;
- `task_events` = immutable append-only operational transition history;
- M09 parser metadata = parser/source truth;
- M10 = operational transition authority.

M09 currently seeds storage state as:

- parsed DONE -> `TASK_COMPLETE`
- parsed BLOCKED -> `BLOCKED`
- all other parsed states -> `BACKLOG`

M10 must integrate with this safely rather than replacing M09.

---

# M10.01 — Canonical workflow states

Implement one explicit Rust enum / canonical string set for these states:

```text
BACKLOG
PLANNING_REQUIRED
PROMPT_REQUIRED
PROMPT_READY
READY_FOR_IMPLEMENTATION
BUILDER_RUNNING
IMPLEMENTATION_COMPLETE
AUDIT_REQUIRED
AUDIT_RUNNING
AUDIT_PASSED
VERIFY_REQUIRED
VERIFY_RUNNING
TASK_COMPLETE
AUDIT_FAILED
FIX_REQUIRED
RE_AUDIT_REQUIRED
BLOCKED
WAITING_HUMAN
WAITING_EXTERNAL
DESIGN_GATE
```

Rules:

- no free-form workflow state strings;
- serialize as exact SCREAMING_SNAKE_CASE values;
- unknown persisted workflow states must produce a structured bounded error, not silently coerce;
- `TASK_COMPLETE` is terminal for normal transitions;
- RUNNING states are transient and must not survive a native restart as if work is still running;
- `BLOCKED`, `WAITING_HUMAN`, `WAITING_EXTERNAL`, `DESIGN_GATE` are suspension/attention states, not shortcuts around the main pipeline.

Do not add speculative states unless direct implementation evidence proves they are required. If a new state is truly necessary, document it in the builder log and update `ARCHITECTURE.md` prospectively.

---

# M10.02 — Actor model

Canonical actor types:

```text
HUMAN
CODEX
CLAUDE
GPT_AUDIT
CI
EXTERNAL
SYSTEM
```

`SYSTEM` is allowed only for bounded bootstrap/recovery/internal bookkeeping. It must not impersonate a human, builder, auditor, CI system, or external actor.

The workflow read model should expose the current state plus allowed/required actor information derived from the state/transition contract. Do not overwrite M09 parser `required_actor` merely to drive M10 UI semantics.

---

# M10.03 — Allowed transition matrix

Implement the normal happy path exactly:

```text
BACKLOG
  -> PLANNING_REQUIRED
  -> PROMPT_REQUIRED
  -> PROMPT_READY
  -> READY_FOR_IMPLEMENTATION
  -> BUILDER_RUNNING
  -> IMPLEMENTATION_COMPLETE
  -> AUDIT_REQUIRED
  -> AUDIT_RUNNING
  -> AUDIT_PASSED
  -> VERIFY_REQUIRED
  -> VERIFY_RUNNING
  -> TASK_COMPLETE
```

Implement the audit failure/remediation loop:

```text
AUDIT_RUNNING
  -> AUDIT_FAILED
  -> FIX_REQUIRED
  -> READY_FOR_IMPLEMENTATION
  -> BUILDER_RUNNING
  -> IMPLEMENTATION_COMPLETE
  -> RE_AUDIT_REQUIRED
  -> AUDIT_RUNNING
```

After a task has a prior failed audit/remediation cycle, `IMPLEMENTATION_COMPLETE` must route to `RE_AUDIT_REQUIRED`, not silently return to first-pass `AUDIT_REQUIRED`.

Normal direct jumps outside the matrix must fail with a structured error such as:

`WORKFLOW_INVALID_TRANSITION`

Do not silently auto-skip states.

Human override is a separate explicit operation described below, not an escape hatch hidden in normal transition code.

---

# Suspension / waiting / design-gate semantics

Normal nonterminal states may be suspended into:

```text
BLOCKED
WAITING_HUMAN
WAITING_EXTERNAL
DESIGN_GATE
```

When entering a suspension state, persist in the event evidence:

- suspended/current prior state;
- deterministic resume state;
- bounded reason/summary;
- any evidence refs.

If the suspended state was a RUNNING state, the resume state must be the safe prerequisite state rather than claiming the external process is still running:

- `BUILDER_RUNNING` -> resume to `READY_FOR_IMPLEMENTATION`
- `AUDIT_RUNNING` -> resume to `AUDIT_REQUIRED` or `RE_AUDIT_REQUIRED` according to history
- `VERIFY_RUNNING` -> resume to `VERIFY_REQUIRED`

Otherwise resume to the original state.

A suspension state may normally exit only to its stored resume state.

Required resume actors:

- `WAITING_HUMAN` -> HUMAN
- `DESIGN_GATE` -> HUMAN
- `WAITING_EXTERNAL` -> EXTERNAL or HUMAN
- `BLOCKED` -> HUMAN, SYSTEM, or the actor explicitly allowed by the blocker evidence

Do not permit arbitrary hold-to-hold chains through the normal transition API. Use a new explicit event/override only when there is a real reason.

A parser-seeded `BLOCKED` task with no M10 history may safely default its resume target to `BACKLOG`.

---

# M10.04 — Evidence model and transition gates

Create a small bounded typed evidence-reference model, for example:

```text
PROMPT
AGENT_SESSION
AUDIT
TEST_RUN
DECISION
GIT_SNAPSHOT
TASK_SOURCE
EXTERNAL_REFERENCE
```

Exact naming may follow existing repository conventions, but the model must remain explicit and finite.

Each evidence ref must be bounded and, when it references an existing H!veAI SQLite table, must be validated against real rows.

Where applicable validate:

- referenced row exists;
- project/task ownership matches the workflow task;
- provider/result/state is compatible with the requested transition;
- no cross-project evidence laundering.

Minimum gates:

### `PROMPT_REQUIRED -> PROMPT_READY`
Require a real `prompts` row associated with the same task/project.

### `PROMPT_READY -> READY_FOR_IMPLEMENTATION`
Require explicit human approval evidence, preferably a matching `decisions` row. Do not treat prompt existence alone as approval.

### `READY_FOR_IMPLEMENTATION -> BUILDER_RUNNING`
Require a matching `agent_sessions` row whose provider is CODEX or CLAUDE, task/project matches, `started_at` exists, and it is not already ended.

### `BUILDER_RUNNING -> IMPLEMENTATION_COMPLETE`
Require the matching builder session to be completed/ended truthfully. Do not mark implementation complete merely because a Git diff exists.

### `AUDIT_REQUIRED / RE_AUDIT_REQUIRED -> AUDIT_RUNNING`
Require evidence that an audit execution/session has actually started. Until M16 exists, tests may seed the existing session/evidence tables, but production logic must validate rather than invent.

### `AUDIT_RUNNING -> AUDIT_PASSED`
Require a real matching `audits` row with result `PASS`.

### `AUDIT_RUNNING -> AUDIT_FAILED`
Require a real matching `audits` row whose result is not PASS and requires follow-up, including FAIL or CONDITIONAL where represented.

### `VERIFY_REQUIRED -> VERIFY_RUNNING`
Require a matching `test_runs` execution record started for the same task/project.

### `VERIFY_RUNNING -> TASK_COMPLETE`
Require completed matching test evidence with PASS result. Every cited required verification run must be finished and passing.

### suspension states
Require a non-empty bounded reason. `WAITING_EXTERNAL` also requires a bounded external reference/description rather than an invented external system state.

Do not perform network access, run tests, run agents, or run audits inside the workflow state machine. M10 verifies/persists evidence; later milestones produce that evidence.

---

# Evidence bounds

At minimum enforce:

- summary/reason/rationale: max 4096 UTF-8 bytes, truncate only where the contract explicitly allows truncation; mutation requests should preferably reject oversize input with a structured error rather than silently rewrite intent;
- request ID: max 128 bytes;
- evidence refs per operation: max 32;
- evidence kind/id/locator scalars: max 512 bytes;
- history read default: 100 events;
- history read max: 500 events.

No unbounded JSON or unbounded event-history IPC payloads.

---

# M10.05 — Atomic transitions and stale-client protection

Create a transition request that includes at least:

- `taskId`
- `expectedFromState`
- `toState`
- `actorType`
- `requestId`
- `summary`
- bounded evidence refs

The native transition must:

1. open one SQLite transaction;
2. load the current task state inside that transaction;
3. reject if task/project is missing or not mutable;
4. reject if `expectedFromState` does not equal current state using `WORKFLOW_CONFLICT` or equivalent;
5. validate the transition matrix;
6. validate actor/evidence requirements;
7. insert exactly one immutable `task_events` row;
8. update `tasks.state` and `tasks.updated_at` atomically;
9. commit;
10. return the resulting workflow state/event.

No event without state update and no state update without event.

Normal workflow mutation must be allowed only for ACTIVE registered projects. Read/history may remain available for missing/archived projects.

---

# Request idempotency

Mutation retries must not create duplicate task events.

Use deterministic M10 event identity based on task + request ID, for example:

`m10evt:<sha256(task_id | request_id)>`

Required behavior:

- same request ID + same semantic operation -> return the already-recorded result idempotently;
- same request ID reused with conflicting semantic payload -> structured conflict error;
- different request IDs -> separate events;
- event ordering remains deterministic using `occurred_at` plus stable ID tie-break.

Do not add a new idempotency table unless the existing task-event schema is genuinely insufficient.

---

# M10.06 — Human override

Implement a separate explicit native operation for human override. Do not overload the normal transition command.

Required request fields:

- `taskId`
- `expectedFromState`
- `toState`
- `requestId`
- non-empty `rationale`
- optional bounded evidence refs

Rules:

- actor is always HUMAN;
- rationale is mandatory and bounded;
- stale expected state still conflicts;
- write a `decisions` row with a clear `WORKFLOW_OVERRIDE` decision kind or equivalent;
- write a `task_events` event type `WORKFLOW_OVERRIDE` or equivalent;
- event evidence must reference the decision/rationale;
- update `tasks.state` atomically in the same transaction;
- never erase/rewrite earlier task events;
- do not permit an override to claim a RUNNING state unless compatible live execution evidence exists;
- reopening `TASK_COMPLETE` is permitted only through this explicit override path and must remain visible in history.

Human override is exceptional correction, not the default transition mechanism.

---

# M10.07 — Restart / interruption recovery

On native H!veAI startup, M10 must truthfully recover stale workflow RUNNING states.

After the database is initialized, before the app presents operational workflow truth, run one bounded recovery pass over tasks that have M10 workflow history and are currently in:

```text
BUILDER_RUNNING
AUDIT_RUNNING
VERIFY_RUNNING
```

Recovery targets:

- `BUILDER_RUNNING` -> `READY_FOR_IMPLEMENTATION`
- `AUDIT_RUNNING` -> `AUDIT_REQUIRED` or `RE_AUDIT_REQUIRED` based on actual prior workflow history
- `VERIFY_RUNNING` -> `VERIFY_REQUIRED`

For each recovered task:

- append a `WORKFLOW_RECOVERY` event;
- actor = SYSTEM;
- record the interrupted state and reason;
- update task state atomically;
- never claim the external builder/auditor/test process is still running when H!veAI cannot prove ownership/liveness.

Recovery must be idempotent. A second restart with no new transient state must not append another recovery event.

Do not attempt to recover actual Codex/Claude processes in M10. That belongs to M13/M14.

---

# Critical M09 <-> M10 ownership integration

This is a blocking M10 contract.

M09 reparsing must not destroy M10 operational truth.

## A. Prevent parser reparse from clobbering workflow state

Once a task has at least one M10-owned workflow event, subsequent M09 UPSERT/reparse must preserve the existing `tasks.state` value.

M09 may continue updating parser-owned fields such as title/source/milestone/parser metadata where appropriate, but it MUST NOT reset an M10-managed task back to parser seed `BACKLOG`, `BLOCKED`, or `TASK_COMPLETE`.

Tasks with no M10 workflow event may continue receiving the existing M09 seed mapping.

Do not solve this by duplicating the task row.

## B. Preserve workflow history when a parser source/task disappears

The current `task_events.task_id` foreign key cascades on task deletion. After M10, blindly deleting a task with workflow history would destroy the operational audit trail.

Required behavior:

- if an M09 task becomes stale and has NO M10 workflow history, existing M09 stale cleanup may continue to delete it according to the accepted M09 contract;
- if an M09 task becomes stale and HAS M10 workflow history, do not delete the task/event history;
- mark it deterministically as source-retired/inactive using a bounded M09-owned metadata flag or equally narrow existing-model mechanism;
- normal M09 active snapshot/list output must not present the retired source task as if it is still active;
- explicit M10 history lookup by task ID must remain available;
- if the same stable task identity later reappears, reactivate the existing row, refresh parser metadata, preserve its M10 state, preserve `created_at`, and preserve all task events.

Prefer the existing M09 metadata model over a new schema/table if it can represent `sourceActive`/retired truth safely.

Do not weaken the accepted M09 stale cleanup for tasks that never acquired workflow history.

## C. Direct cross-milestone proof

This integration is not complete without tests proving both state preservation and event-history preservation across real M09 parse/reparse/stale/reappearance flows.

---

# M10.08 — Native service / IPC / TypeScript contract

Add a dedicated Rust module such as:

`H!veAI/src-tauri/src/workflow.rs`

Register narrow commands, for example:

```text
hiveai_workflow_task_get
hiveai_workflow_project_list
hiveai_workflow_history
hiveai_workflow_transition
hiveai_workflow_override
```

Exact names may vary slightly if repository naming convention demands it, but keep the surface narrow.

Read operations:

- task current workflow state;
- bounded project task workflow list;
- bounded task event history.

Mutation operations:

- normal transition;
- human override.

Do not expose arbitrary SQL, arbitrary event insertion, raw state mutation, filesystem access, shell access, process launch, or network access.

Add a dedicated permission entry such as:

`allow-workflow-state-machine`

and add only that permission to the main-window capability.

Add a TypeScript native contract file, preferably:

`H!veAI/src/workflow.ts`

with typed enums/interfaces/wrappers.

Do NOT wire a new visible M10 UI into `pages.tsx` or redesign Command Center/Project Cockpit. M11/M12 consume this service later.

---

# M10 read model

A task workflow read result should provide enough factual state for M11/M12 later without implementing those screens now:

- task ID / project ID;
- current workflow state;
- workflow-managed boolean;
- source-active/retired truth;
- allowed next normal states;
- allowed actor(s) for those transitions;
- suspension resume state if applicable;
- latest workflow event summary/time;
- whether recovery/attention is required;
- bounded source/parser identifiers where already available.

Do not add AI recommendations in M10.

---

# M10.09 — Required direct tests

Add direct Rust production-path tests. Test names may be adjusted, but the behaviors below are mandatory.

### State matrix

`m10_happy_path_requires_each_canonical_step`

Prove the exact happy path with seeded valid evidence.

`m10_invalid_direct_jump_is_rejected`

Example: `BACKLOG -> TASK_COMPLETE` through normal transition must fail.

`m10_audit_failure_routes_to_reaudit_after_fix`

Prove failed audit -> fix -> new implementation -> `RE_AUDIT_REQUIRED`.

### Actor/evidence gates

`m10_prompt_ready_requires_same_task_prompt`

`m10_builder_running_requires_matching_live_builder_session`

`m10_audit_pass_requires_matching_pass_audit`

`m10_verify_complete_requires_finished_pass_test_run`

Also include at least one cross-project evidence rejection test.

### Concurrency / idempotency

`m10_expected_state_prevents_stale_double_transition`

Two operations using the same old expected state must not both mutate.

`m10_request_id_is_idempotent`

Same request repeated produces one event.

`m10_request_id_conflicting_reuse_is_rejected`

### Suspension states

`m10_waiting_human_round_trip_resumes_exact_prior_state`

`m10_running_state_suspension_resumes_to_safe_prerequisite`

`m10_parser_seeded_blocked_defaults_resume_to_backlog`

### Human override

`m10_override_requires_nonempty_rationale`

`m10_override_records_decision_and_event_atomically`

`m10_task_complete_reopen_requires_override`

### Restart recovery

`m10_restart_recovery_demotes_stale_running_states`

Cover builder/audit/verify running states and prove second recovery pass is idempotent.

### M09 integration

`m10_m09_reparse_preserves_workflow_state_and_events`

Required sequence:

1. M09 parse task -> parser seed state;
2. perform one or more M10 transitions;
3. change source metadata/content without changing task identity;
4. M09 reparse;
5. parser fields update;
6. M10 workflow state remains unchanged;
7. event count/history remains unchanged.

`m10_m09_stale_source_preserves_managed_history`

Required sequence:

1. parse task;
2. create M10 history;
3. remove task/source from parser input;
4. M09 reparse;
5. task/event rows survive;
6. task is marked source-inactive/retired;
7. M09 active list no longer returns it.

`m10_m09_reappearance_reactivates_same_task_without_history_loss`

Reintroduce the same stable task identity and prove:

- same task row identity;
- source active again;
- parser metadata refreshed;
- M10 state preserved;
- original `created_at` preserved;
- all task events preserved.

Also prove an M09 stale task with NO M10 history still follows existing M09 cleanup behavior.

### History/read bounds

`m10_history_is_bounded_and_deterministically_ordered`

### Project lifecycle

`m10_archived_or_missing_project_rejects_mutation_but_allows_history_read`

If the existing Registry status model distinguishes path MISSING from archived status differently, align the test with actual repository semantics and document it.

---

# Focused TypeScript evidence

Add lightweight tests for the TypeScript workflow wrapper proving:

- exact native command names;
- enum/string contract alignment;
- mutation payload includes `expectedFromState` + `requestId`;
- history/list limit validation is bounded;
- no browser-preview fake workflow state is invented.

No visual acceptance is required for M10 unless Codex unexpectedly touches visible UI, which it should not.

---

# Scope protection

M10 MUST NOT implement:

- M11 Global Command Center live portfolio UI;
- M12 Project Cockpit workflow tab;
- `.hiveai/PROJECT_DASHBOARD.md` runtime ingestion/authority resolution;
- Codex adapter;
- Claude adapter;
- PTY/session center;
- Prompt Engine;
- GPT Audit Engine;
- GitHub API integration;
- Next Best Task AI;
- Engineering Brief AI;
- Project Chat;
- installer/release packaging.

The Project Dashboard manifest system is intentionally reserved for M11/M12. Do not pull it forward into M10.

Do not change visible H!veAI UI merely to demonstrate workflow state.

---

# Security / safety requirements

- SQLite transaction boundaries must prevent partial transition writes.
- Never trust frontend-provided current state without checking DB state inside the mutation transaction.
- Validate task/project identity and evidence ownership.
- No arbitrary SQL from frontend.
- No shell/process launch.
- No network access.
- No arbitrary filesystem reads.
- No secret-bearing evidence bodies; store references/bounded metadata only.
- Reject oversized mutation intent rather than allocating unbounded strings.
- Never fabricate agent/audit/test execution evidence.
- Builder logs are not workflow evidence by themselves.
- Preserve M09 event-history compatibility and existing foreign-key safety.
- No blanket task/task_event deletion.

---

# Tracker truth during builder run

At start of implementation:

- M00-M09 remain PASS/CLOSED;
- pre-M10 X01/X02 remain PASS/CLOSED;
- strict completed milestone count remains `10 / 20 = 50%` until independent M10 audit closes;
- M10 may be marked `IMPLEMENTATION COMPLETE / PENDING INDEPENDENT AUDIT` only after implementation/regression/publication succeeds;
- do NOT mark M10 PASS/CLOSED yourself;
- M11/M12 remain BLOCKED.

Update prospectively as needed:

- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/README.md`
- `H!veAI/docs/H!veAI/README.md` only if its live-status section requires synchronization
- `H!veAI/ARCHITECTURE.md` only if the actual final M10 contract adds a real architectural detail not already represented.

Do not rewrite historical prompts/logs/audits.

---

# Regression / verification gates

Run focused M10 tests first, then full repository gates.

At minimum:

```powershell
npm run typecheck
npm test -- --run
npm run build
npm audit --audit-level=high
cargo fmt --manifest-path H!veAI/src-tauri/Cargo.toml -- --check
cargo check --manifest-path H!veAI/src-tauri/Cargo.toml
cargo test --manifest-path H!veAI/src-tauri/Cargo.toml
cargo build --manifest-path H!veAI/src-tauri/Cargo.toml
```

Also explicitly run relevant regression subsets for:

- M08 source discovery;
- M09 task intelligence;
- watcher/Git Engine where M09 source/persistence integration is touched;
- database migrations/integrity;
- Tauri permissions/capabilities;
- pre-M10 native UX focused tests.

Run the existing publisher failure/rollback harness.

Run the governed production Tauri `--no-bundle` QA publisher.

Verify:

- stable `H!veAI/dev-bin/H!veAI.exe` exists;
- Desktop `H!veAI.lnk` still targets the stable EXE directly;
- shortcut/icon unchanged;
- canonical background/video hashes unchanged;
- startup audio/console-window source fixes remain present;
- no installer artifacts;
- no M11/M12 runtime code accidentally introduced.

---

# Required builder log

Create:

`H!veAI/docs/H!veAI/codex-logs/M10_WORKFLOW_STATE_MACHINE_LOG.md`

Required sections:

```text
START STATE
branch / starting HEAD / origin equality / worktree status

CONTRACT
canonical states
actors
normal transition matrix
failure loop
suspension/resume rules
evidence gates

PRODUCTION IMPLEMENTATION
workflow module/symbols
transaction strategy
idempotency strategy
human override strategy
restart recovery strategy

M09 <-> M10 INTEGRATION
how reparse preserves M10 state
how stale parser tasks with M10 history are retired without deleting history
how reappearance reactivates safely
exact tests

IPC / ACL / TYPESCRIPT
commands
permission/capability
typed wrapper

DIRECT TESTS
name + what pre-fix/incorrect behavior each test would catch

FAILED ATTEMPTS
chronological, never erase

FULL REGRESSION
commands + actual results

PUBLICATION
publisher harness
stable EXE SHA/size
shortcut target/icon
asset hashes
installer scan

TRACKER TRUTH
M10 implementation status
strict completed count
M11/M12 state

COMMITS / REMOTE
implementation commit
log/docs commit(s)
final local HEAD
origin/H!veAI HEAD
final equality proof
```

Builder log is a claim record, not independent acceptance.

---

# Pre-push self-audit

Before the final push, explicitly answer in the log:

1. Can any normal transition bypass the canonical matrix?
2. Can stale frontend state cause a double transition?
3. Can one retried request create duplicate events?
4. Can evidence from another task/project satisfy a gate?
5. Can a human override happen without rationale/history?
6. Can a restart leave BUILDER_RUNNING/AUDIT_RUNNING/VERIFY_RUNNING falsely active?
7. Can an M09 reparse reset an M10-managed `tasks.state`?
8. Can M09 stale cleanup delete a task with M10 history and cascade-delete its events?
9. Can a retired task reappear without preserving identity/history?
10. Did any visible UI/M11/M12/Project Dashboard runtime ingestion slip into scope?
11. Did canonical assets or the native UX hotfix regress?
12. Are TASKS/ROADMAP/log claims truthful and is final local HEAD equal to origin after the final pushed log commit?

Any YES to questions 1-9 or 10-11 in an unsafe sense blocks completion.

---

# Stop condition

Stop after and only after:

1. canonical workflow states/actors/matrix exist in production Rust;
2. evidence gates are real and cross-project safe;
3. transitions are transactional, stale-state protected, and request-id idempotent;
4. suspension/resume behavior is deterministic;
5. human override is explicit, rationale-backed, decision/event recorded;
6. restart recovery truthfully clears stale RUNNING states;
7. M09 reparse cannot clobber M10-managed workflow state;
8. stale M09 tasks with M10 history do not lose task_events and are hidden from active parser output;
9. same stable task reappearance restores source activity without losing workflow state/history;
10. required direct tests pass;
11. full frontend/Rust/security/regression gates pass;
12. governed no-bundle QA publication succeeds;
13. canonical UI/media assets and pre-M10 UX fixes remain unchanged;
14. no M11/M12/agent/prompt/audit-engine/GitHub/AI recommendation scope is implemented;
15. tracker is updated only to implementation-complete/pending-independent-audit truth;
16. builder log is committed and pushed;
17. final local HEAD == `origin/H!veAI` is verified AFTER the final log/docs commit;
18. M10 remains pending independent strict audit and M11/M12 remain blocked.

Then stop and wait for independent audit.