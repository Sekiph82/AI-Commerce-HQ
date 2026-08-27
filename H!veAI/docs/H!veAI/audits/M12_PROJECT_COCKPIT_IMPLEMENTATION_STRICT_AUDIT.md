# M12 Project Cockpit Implementation Strict Audit

Date: 2026-08-27
Branch: `H!veAI`
Audited builder log: `H!veAI/docs/H!veAI/codex-logs/M12_PROJECT_COCKPIT_IMPLEMENTATION_LOG.md`
Audited implementation commit: `3eadf3c8ec254db1bf61a550c6716f299ac9ff07`

## Verdict

**FAIL / M12 NOT CLOSED**

- BLOCKER: 0
- MAJOR: 1
- MINOR: 0
- NOTE: 2
- Confidence: HIGH

The implementation is broad, well-scoped, and passes the reported regression/publication gates, but one production correctness defect remains in the Project Cockpit workflow-history aggregation. User native/visual acceptance also remains pending.

## R26 / MAJOR - Project-wide workflow history is not globally bounded by recency

`project_cockpit::snapshot()` builds `workflow_history` by iterating the project task list and calling `workflow::history()` separately for each task with a per-task limit of `MAX_COCKPIT_HISTORY` (200). It appends events task-by-task, and as soon as the aggregate length reaches 200 it truncates and breaks out of the task loop. Only after that early break does it sort the retained subset.

This means a single early task with 200 historical events can consume the entire cockpit history budget and prevent newer events belonging to later tasks from ever entering the candidate set. Sorting after truncation cannot recover events that were never fetched.

For an M12 Project Cockpit, Workflow and Activity surfaces are project-scoped. Their bounded history must therefore be selected by project-wide truth, not by task iteration order.

### Required correction

Use one project-wide bounded workflow-history read, or otherwise merge bounded per-task candidates before applying the final project-wide limit.

The final contract must be:

1. only events owned by the selected project;
2. deterministic global ordering by `occurred_at` descending with a stable tie-breaker;
3. final cap applied after global ordering;
4. no task can starve newer events from another task merely because it appears earlier in the task list;
5. existing M10 workflow source of truth remains authoritative;
6. no second workflow store or rewritten history is introduced.

### Required direct regression test

Create at least two tasks in the same project.

- Seed more than the cockpit history limit on task A with older timestamps.
- Seed one or more newer events on task B.
- Assert that the returned bounded project cockpit history contains the newer task-B events.
- Assert ordering is globally newest-first and deterministic.
- Assert no event from another project can enter the result.

The test must fail against the current M12 implementation and pass only after the production fix.

## Reviewed strengths

- Snapshot ownership is explicitly project-scoped through the registered project ID.
- Project Dashboard authority is resolved per selected project.
- Git cockpit reads use `persist: false`; the direct native test confirms no `git_snapshots` row is written by cockpit loading.
- Missing/non-Git states are explicit rather than fabricated.
- Dashboard activity without verified timestamps remains `UNDATED`.
- Runtime status correctly remains `IMPLEMENTATION_COMPLETE_PENDING_AUDIT`; M13 and M21 were not started.
- Frontend route loading contains stale initial request responses using request identity.

## Evidence boundary notes

### NOTE E01

The builder log reports 282 Rust tests, 92 frontend tests, full typecheck/build/audit, failure harness, governed publication, PE validation, shortcut validation, and console suppression as PASS. Those are accepted as builder evidence, but they do not cover R26 because the current focused native tests do not construct a project-wide multi-task history starvation case.

### NOTE E02

User native/visual acceptance is still required after the production defect is corrected. M12 must not be marked PASS/CLOSED until both strict audit and user acceptance pass.

## Closure state

**M12 = FAIL / REMEDIATION REQUIRED**

Strict roadmap progress remains **12 / 20 = 60%**.

M13 remains BLOCKED.
M21 remains PLANNED / NOT STARTED.
