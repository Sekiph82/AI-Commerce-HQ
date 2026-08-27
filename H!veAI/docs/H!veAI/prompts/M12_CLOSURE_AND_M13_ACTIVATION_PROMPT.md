# M12 Closure and M13 Activation Prompt

Date: 2026-08-27
Product: H!veAI
Branch: `H!veAI`

## Authority

This prompt is the sole authority for this run.

Work only on the `H!veAI` branch. Safely synchronize with `origin/H!veAI` before making changes.

Do not implement M13 in this run. Do not start M21.

## Accepted M12 closure evidence

Treat the following as accepted immutable evidence for canonical M12 closure:

- `H!veAI/docs/H!veAI/codex-logs/M12_PROJECT_COCKPIT_IMPLEMENTATION_LOG.md`
- `H!veAI/docs/H!veAI/audits/M12_PROJECT_COCKPIT_IMPLEMENTATION_STRICT_AUDIT.md`
- `H!veAI/docs/H!veAI/codex-logs/M12A_PROJECT_WIDE_WORKFLOW_HISTORY_STRICT_REMEDIATION_LOG.md`
- `H!veAI/docs/H!veAI/audits/M12A_PROJECT_WIDE_WORKFLOW_HISTORY_STRICT_REAUDIT.md`
- `H!veAI/docs/H!veAI/codex-logs/M12B_NATIVE_OPEN_COCKPIT_ROUTE_LOADING_REMEDIATION_LOG.md`
- `H!veAI/docs/H!veAI/audits/M12B_NATIVE_OPEN_COCKPIT_ROUTE_LOADING_STRICT_REAUDIT.md`
- User native/visual acceptance on 2026-08-27 confirming that the Project Cockpit opens correctly and that the Overview, Tasks, Workflow, Agents, Audit, Git, Tests, Activity, Files, and Settings tabs render and are navigable in the governed native H!veAI executable.

Historical M12 failures and remediation records must remain immutable history. Do not rewrite or delete prior prompts, logs, or audits.

## Required canonical transition

Update canonical H!veAI status so that:

- M00-M12 = `PASS/CLOSED`.
- M12A R26 remediation = `PASS/CLOSED`.
- M12B native Open Cockpit remediation = `PASS/CLOSED`.
- Strict completed roadmap progress changes from `12 / 20 = 60%` to `13 / 20 = 65%`.
- M13 becomes `READY / ACTIVE FOR NEXT IMPLEMENTATION RUN`.
- M13 implementation has **not** started in this run.
- M14-M20 remain planned/blocked behind their normal dependencies.
- M21 remains planned and must not be started.

Update, at minimum where applicable:

- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/README.md`
- `H!veAI/docs/H!veAI/README.md`
- `H!veAI/.hiveai/PROJECT_DASHBOARD.md`

Do not modify production source code, frontend implementation, native runtime code, external registered projects, or Bulk Edit in this status-transition run.

## M13 activation truth

M13 is the next roadmap milestone and is the Codex Adapter milestone.

Its canonical packages are:

### M13.01 - Codex availability/readiness
- Detect Codex installation/version.
- Detect auth/readiness without exposing credentials.
- Surface unavailable/misconfigured state truthfully.

### M13.02 - Common agent adapter contract
- Implement provider-neutral availability/start/resume/stop/status contract.
- Map Codex to common session/event model.

### M13.03 - Project-scoped process start
- Start Codex in registered project/worktree cwd.
- Validate cwd containment.
- Avoid arbitrary shell execution.

### M13.04 - Session output capture
- Capture stdout.
- Capture stderr.
- Capture exit code.
- Stream bounded structured events.

### M13.05 - Task/session mapping
- Attach session to one project.
- Attach session to one task or explicit freeform operation.
- Preserve prompt/version provenance when available.

### M13.06 - Resume/stop/recovery
- Resume supported Codex session where safe.
- Stop process cleanly.
- Detect crashed/orphaned process.
- Recover truthful state after H!veAI restart.

### M13.07 - Permission boundary
- Define allowed process launch arguments.
- Block arbitrary command injection.
- Record permission-sensitive operations.

### M13.08 - Direct process tests
- Availability tests.
- cwd/containment tests.
- stdout/stderr/exit tests.
- stop/crash/recovery tests.
- malformed/injection input tests.

### M13.09 - Regression/audit/closure
- Full security/process regression.
- Production QA publication.
- Independent strict audit.

For this run, only activate M13 canonically. Do not implement any M13 package.

If no separate authoritative M13 implementation prompt exists yet, record that fact truthfully and state that a separate run must prepare it before M13 coding begins.

## Verification

Before committing:

1. Fetch `origin/H!veAI` and confirm safe synchronization.
2. Confirm M00-M12 are recorded as PASS/CLOSED in canonical trackers.
3. Confirm strict completed progress is exactly `13 / 20 = 65%`.
4. Confirm M13 is READY / ACTIVE FOR NEXT IMPLEMENTATION RUN and implementation is not started.
5. Confirm M14-M20 remain planned/blocked as appropriate.
6. Confirm M21 remains planned/not started.
7. Confirm no production files under `H!veAI/src/` or `H!veAI/src-tauri/` changed in this run.
8. Run `git diff --check`.

## Immutable log

Create:

`H!veAI/docs/H!veAI/codex-logs/M12_CLOSURE_AND_M13_ACTIVATION_LOG.md`

The log must include:

- accepted M12 evidence;
- exact canonical status transition;
- exact files changed;
- verification results;
- whether an authoritative M13 implementation prompt already exists;
- exact implementation/status-transition commit SHA;
- exact final local HEAD;
- exact fetched `origin/H!veAI`;
- exact `HEAD...origin/H!veAI` divergence count.

## Commit and push

Commit and push all scoped changes to `origin/H!veAI`.

Final state must be:

`M12 PASS/CLOSED / ROADMAP 13 OF 20 = 65% / M13 READY FOR NEXT IMPLEMENTATION RUN`

Stop after this status-transition run. Do not implement M13. Do not start M21.
