# M12A Project-Wide Workflow History Strict Re-Audit

Date: 2026-08-27
Product: H!veAI
Branch: `H!veAI`
Audited remediation log: `H!veAI/docs/H!veAI/codex-logs/M12A_PROJECT_WIDE_WORKFLOW_HISTORY_STRICT_REMEDIATION_LOG.md`
Audited implementation commit: `fe1f6f6bdbcf93580e12dd785863fcc4d5d1fe9f`
Historical finding: M12 R26

## Verdict

**PASS / R26 CLOSED / M12 SOURCE + REGRESSION AUDIT PASSED / USER NATIVE-VISUAL ACCEPTANCE STILL REQUIRED**

- BLOCKER: 0
- MAJOR: 0
- MINOR: 0
- NOTE: 1
- Confidence: HIGH
- Regression risk: LOW

The M12A remediation correctly closes R26. Selected-project workflow history is now queried as one project-wide ordered dataset and bounded only after global ordering. The prior task-iteration starvation path has been removed from the cockpit snapshot.

## R26 closure

### Historical defect

The original M12 cockpit iterated project tasks, fetched per-task histories, and stopped once the aggregate reached the cockpit cap. An event-heavy earlier task could therefore prevent newer workflow events belonging to a later task from entering the snapshot.

### Production correction

`project_cockpit::snapshot()` now obtains workflow history through:

```rust
workflow::project_history(database, project_id, MAX_COCKPIT_HISTORY)?
```

The old per-task accumulation, early break, truncation, and post-hoc sort were removed.

`workflow::project_history()` validates the registered project and executes one selected-project query joining `task_events` to `tasks` with:

```sql
WHERE t.project_id=?1
  AND e.event_type LIKE 'WORKFLOW_%'
ORDER BY e.occurred_at DESC, e.id DESC
LIMIT ?2
```

Therefore:

1. events are scoped to the selected project before limiting;
2. all selected-project tasks compete in one global ordering;
3. newest workflow events cannot be starved merely because another task has many older events;
4. equal timestamps have deterministic descending event-ID tie ordering;
5. the 200-event bound remains explicit and database-enforced;
6. no second workflow store or alternate authority path was introduced.

## Adversarial coverage

The remediation adds direct tests for the exact failure class and adjacent invariants:

- an older task with 205 events cannot hide a newer event from another task;
- returned history remains capped at 200;
- global newest-first ordering is asserted;
- equal timestamp ordering is deterministic across repeated snapshots;
- other-project events are excluded from workflow history;
- derived cockpit Activity also contains the surviving newer selected-project event and excludes other-project workflow evidence.

These are materially stronger than the original M12 tests because they directly exercise the cross-task starvation boundary that caused R26.

## Regression / publication evidence

Builder evidence reports:

- M12 cockpit Rust focused tests: 7 PASS;
- frontend M12 focused tests: 5 PASS;
- full frontend: 92 PASS;
- full Rust: 285 PASS, 0 failures;
- typecheck/build/npm audit/fmt/check/diff-check: PASS;
- publisher failure harness: 9/9 PASS;
- governed production Tauri `--no-bundle` publication: PASS;
- stable executable and shortcut/icon/port/console-host checks: PASS.

No external registered project, Bulk Edit, M13, or M21 scope was modified.

## Independent source conclusion

No remaining production defect was found in the bounded R26 remediation. The new project-wide history function is simpler, more correct, and directly aligned with the M12 cockpit contract.

### NOTE V01 - User visual/native acceptance remains outstanding

This re-audit closes the source/regression finding only. M12 still requires the user's native visual acceptance of the Project Cockpit before the milestone can be canonically marked PASS/CLOSED and roadmap progress can advance from 12/20 to 13/20.

## Final state

**R26: PASS/CLOSED**

**M12 TECHNICAL STRICT RE-AUDIT: PASS**

**M12 MILESTONE: PENDING USER NATIVE/VISUAL ACCEPTANCE ONLY**

Do not start M13 or M21 until M12 is canonically closed after user acceptance.
