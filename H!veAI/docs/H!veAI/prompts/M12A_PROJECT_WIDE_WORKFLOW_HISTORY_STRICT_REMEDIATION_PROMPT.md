# M12A Project-Wide Workflow History Strict Remediation

## Authority

This is the single authoritative prompt for the next H!veAI remediation run.

It exists only to close R26 from:

`H!veAI/docs/H!veAI/audits/M12_PROJECT_COCKPIT_IMPLEMENTATION_STRICT_AUDIT.md`

Do not start M13.
Do not start M21.
Do not redesign the Project Cockpit.
Do not reopen closed M00-M11 work.

Strict roadmap progress remains **12 / 20 = 60%** until M12 is independently closed and user native/visual acceptance passes.

## Mandatory preflight

Work only on branch `H!veAI`.

Synchronize safely with `origin/H!veAI` using fetch + fast-forward only. Never reset, rebase, force-push, rewrite history, delete user work, or stage unrelated parent-root files.

Read in full before editing:

1. `H!veAI/AGENTS.md`
2. `H!veAI/CONSTITUTION.md`
3. `H!veAI/TASKS.md`
4. `H!veAI/CODEX_ROADMAP.md`
5. `H!veAI/docs/H!veAI/prompts/M12_PROJECT_COCKPIT_IMPLEMENTATION_PROMPT.md`
6. `H!veAI/docs/H!veAI/codex-logs/M12_PROJECT_COCKPIT_IMPLEMENTATION_LOG.md`
7. `H!veAI/docs/H!veAI/audits/M12_PROJECT_COCKPIT_IMPLEMENTATION_STRICT_AUDIT.md`
8. `H!veAI/src-tauri/src/project_cockpit.rs`
9. `H!veAI/src-tauri/src/workflow.rs`
10. relevant M12 native/frontend tests

Historical prompts, audits, and builder logs are immutable. Do not edit them.

## R26 / MAJOR - Fix project-wide workflow history starvation

### Current defect

The current M12 snapshot builds `workflow_history` by iterating tasks and fetching per-task history. It truncates/breaks as soon as the aggregate reaches `MAX_COCKPIT_HISTORY`, then sorts only the retained subset.

This allows an early task with many old events to consume the entire budget and hide newer events from later tasks.

### Required production contract

Project Cockpit workflow history must be a truthful project-wide bounded history.

Implement the smallest safe correction using the existing M10 workflow/event store.

The result must satisfy all of the following:

1. include only `task_events` belonging to tasks owned by the selected project;
2. order globally by `occurred_at` descending;
3. use a deterministic stable tie-breaker, preferably event ID descending or the existing canonical event ordering contract;
4. apply `MAX_COCKPIT_HISTORY` only after project-wide ordering;
5. no task may starve newer events from another task due to task iteration order;
6. preserve existing event fields, actor/state parsing, evidence semantics, and M10 truth;
7. do not introduce a second workflow store;
8. do not rewrite task history;
9. do not weaken project containment;
10. do not expand the final response beyond the established bounded limit.

Prefer a direct project-scoped workflow history query/helper over N per-task reads if it can reuse existing workflow parsing safely.

If a new internal workflow helper is added, keep it non-public unless required by the existing architecture, and cover it directly.

## Direct adversarial regression tests

Add tests that fail against the current implementation.

At minimum:

### Test A - cross-task starvation prevention

Create project P with tasks A and B.

- Seed more than `MAX_COCKPIT_HISTORY` older events for task A.
- Seed one or more strictly newer events for task B.
- Load the Project Cockpit snapshot.
- Assert the newer task-B event(s) appear in `workflow_history`.
- Assert total returned events do not exceed the cockpit limit.
- Assert global order is newest-first.

### Test B - deterministic tie ordering

Seed events from different tasks with equal `occurred_at` values.
Assert repeated snapshot loads return the same deterministic ordering.

### Test C - project containment

Seed a newer event in another registered project.
Assert it never appears in the selected project's cockpit history or derived activity.

Preserve all existing M12 tests.

## Derived Activity

`build_activity()` consumes workflow history. After R26 is fixed, confirm the Activity tab inherits the corrected project-wide recent workflow subset and still:

- remains selected-project scoped;
- keeps dashboard-only timestamp-less activity as `UNDATED`;
- remains bounded and deterministic.

Do not fabricate timestamps for Project Dashboard activity.

## Status truth

During this remediation run, canonical status files must remain truthful:

- M00-M11 = PASS/CLOSED;
- M12 implementation exists but strict audit = FAIL with R26 open;
- M12A remediation = ACTIVE while working;
- strict completed roadmap progress = 12/20 = 60%;
- M13 = BLOCKED;
- M21 = PLANNED / NOT STARTED;
- user native/visual acceptance remains pending.

At completion, mark only the remediation implementation complete pending independent re-audit. Do not mark M12 PASS/CLOSED yourself.

## Verification gates

Run and record at minimum:

- direct R26 focused native tests;
- all existing `project_cockpit` native tests;
- complete Rust library test suite;
- M12 focused frontend tests;
- complete frontend suite;
- `npm.cmd run typecheck`;
- `npm.cmd run build`;
- `npm.cmd audit -- --audit-level=high`;
- `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`;
- `cargo check --manifest-path src-tauri/Cargo.toml`;
- `git diff --check`;
- established publisher failure harness;
- governed `publish-dev-qa.ps1` publication.

Preserve startup video, native icon, Akilta attribution, terminal suppression, shortcut behavior, Command Center, Projects, Tasks, and existing M12 UI unless R26 requires a strictly internal data correction.

## Scope protection

Do not modify external registered projects.
Do not touch Bulk Edit.
Do not start Codex Adapter/M13.
Do not start standalone migration/M21.
Do not add provider process launching.
Do not add new UI features.

## Immutable builder log

Create:

`H!veAI/docs/H!veAI/codex-logs/M12A_PROJECT_WIDE_WORKFLOW_HISTORY_STRICT_REMEDIATION_LOG.md`

Record:

- exact synchronized baseline;
- root cause;
- production design chosen;
- exact files changed;
- direct adversarial tests and results;
- full regression results;
- publication results;
- implementation commit SHA;
- final local HEAD;
- final fetched `origin/H!veAI`;
- exact `HEAD...origin/H!veAI` divergence.

Commit and push all scoped changes to `origin/H!veAI`.

Final builder state:

`M12A REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

Stop. Do not start M13 or M21.
