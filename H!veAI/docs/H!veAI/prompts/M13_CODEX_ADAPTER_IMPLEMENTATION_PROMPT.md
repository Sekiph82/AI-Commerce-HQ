# M13 Codex Adapter — Authoritative Implementation Prompt

Date: 2026-08-27
Product: H!veAI
Branch: `H!veAI`
Milestone: M13 Codex Adapter

## Authority

This prompt is the sole implementation authority for M13.

Before coding, safely synchronize the `H!veAI` branch with `origin/H!veAI` using fetch plus fast-forward-only merge. Do not reset, rebase, force-push, or discard user-owned work.

Read in full before implementation:

- `H!veAI/AGENTS.md`
- `H!veAI/CONSTITUTION.md`
- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/.hiveai/PROJECT_DASHBOARD.md`
- `H!veAI/docs/H!veAI/prompts/M12_CLOSURE_AND_M13_ACTIVATION_PROMPT.md`
- `H!veAI/docs/H!veAI/codex-logs/M12_CLOSURE_AND_M13_ACTIVATION_LOG.md`
- `H!veAI/docs/H!veAI/audits/M12_CLOSURE_AND_M13_ACTIVATION_STRICT_AUDIT.md`
- all existing M10/M12 workflow, project-registry, session, permission, activity, audit, and Project Cockpit production/test source needed to preserve current contracts.

M12 is closed. M13 may now be implemented. Do not start M14 or M21.

## M13 purpose

Implement the real project-scoped Codex provider adapter so H!veAI can safely detect, launch, observe, stop, and recover Codex sessions while preserving project/task/session provenance and native security boundaries.

Exit criterion:

> Codex can be safely started, stopped, resumed where supported, and observed from H!veAI as a real project-scoped agent session with bounded output, truthful state, and strict command/process containment.

## Non-negotiable principles

1. Native process ownership belongs in Rust/Tauri, not browser-side shell execution.
2. Do not expose credentials, auth tokens, environment secrets, or raw sensitive command lines to the UI/logs.
3. Do not create an arbitrary shell/terminal execution primitive.
4. Do not accept user-controlled freeform executable paths or arbitrary command argument arrays that bypass the adapter contract.
5. Every launched session must be attached to exactly one registered H!veAI project.
6. A session may additionally attach to one task, or be explicitly marked as a bounded freeform project operation.
7. cwd must resolve to the registered project root or an explicitly allowed contained worktree path. No path escape.
8. Preserve M10 workflow semantics and existing persisted session/permission tables. Do not create a competing workflow/session truth store.
9. M14 owns the rich PTY/live terminal experience. M13 must not prematurely implement M14's xterm/PTy UI.
10. M21 standalone repository migration remains out of scope.

## M13.01 — Codex availability/readiness

Implement native Codex readiness inspection.

The adapter must truthfully detect at minimum:

- whether the `codex` executable is available through the expected environment/path resolution;
- a bounded version result;
- whether the executable can respond to a safe readiness/version probe;
- auth/readiness state where it can be determined safely without revealing credentials;
- unavailable, unsupported, malformed, timed-out, or misconfigured conditions.

Do not scrape or persist secrets.

Expose a provider readiness model suitable for later common adapters, for example conceptually:

- provider
- installed/available
- version
- readiness state
- bounded diagnostic code/message
- checkedAt

Do not invent READY when the evidence is unknown.

## M13.02 — Common agent adapter contract

Create or extend a provider-neutral native adapter contract for:

- availability/readiness
- start
- status
- resume if Codex supports a safe stable mechanism
- stop
- recovery/reconciliation

Codex must implement this contract without baking Codex-specific assumptions into every caller.

The common model must map to existing H!veAI concepts for:

- provider
- project
- optional task
- session ID
- lifecycle state
- timestamps
- permission/wait/crash/exit status where available
- bounded output/events

Do not implement Claude or another provider in this milestone.

## M13.03 — Project-scoped process start and containment

Implement safe native Codex process launch.

Requirements:

- selected project must exist in the Project Registry;
- project status/path must be valid for launch;
- cwd must be the registered project root or an explicitly validated contained worktree;
- canonicalize/normalize containment before launch;
- reject missing paths, archived/missing projects, path traversal, sibling paths, UNC/alternate path confusion where relevant, symlink/junction escape where safely detectable, and malformed input;
- no `cmd /c`, `powershell -Command`, shell string interpolation, or equivalent arbitrary-shell wrapper;
- launch the known Codex executable directly with an adapter-owned allowlisted argument construction;
- reject unsupported/freeform argument injection.

If Codex needs a prompt/instruction input, pass it through the safest supported mechanism and bound its size. Do not turn prompt text into shell syntax.

## M13.04 — stdout/stderr/exit and bounded event capture

Capture real process evidence:

- stdout
- stderr
- exit code / termination result
- process start/end timestamps
- bounded structured output events suitable for later M14 display

Requirements:

- prevent unbounded in-memory growth;
- use explicit byte/event caps;
- preserve recent/bounded evidence deterministically;
- represent truncation truthfully;
- do not claim output that was not captured;
- avoid logging secrets where recognizable/suppressible;
- do not block the Tauri main thread.

Persist only the bounded session/event evidence needed by existing H!veAI contracts.

## M13.05 — Project/task/session provenance

Every session must preserve:

- H!veAI session ID
- provider = CODEX
- project ID
- task ID when attached
- explicit freeform-operation marker when no task is attached
- cwd / contained worktree identity in a non-secret bounded form
- start/end/state
- exit result
- prompt/version provenance when available under current schemas

If a task ID is supplied, prove it belongs to the selected project before launch.

Never silently attach a task from another project.

## M13.06 — Resume, stop, crash, orphan, restart recovery

Implement truthful lifecycle management.

### Stop

- stop only H!veAI-owned/known Codex sessions;
- do not allow arbitrary PID termination;
- terminate cleanly first when supported;
- bounded escalation may be used only for the known owned child process/tree;
- persist the resulting state accurately.

### Resume

If the installed Codex version exposes a documented, stable, safe resume/session mechanism, support it behind the adapter contract with tests.

If it does not, represent resume as unsupported. Do not fake resume by launching a new unrelated session under the old identity.

### Recovery

On H!veAI restart or adapter reconciliation:

- detect sessions persisted as running whose owned process no longer exists;
- mark crash/orphan/stale states truthfully;
- do not attach to unrelated same-name processes;
- do not resurrect sessions without evidence;
- preserve historical ended/crashed sessions.

## M13.07 — Permission/process security boundary

Define and enforce the allowed Codex launch surface.

At minimum:

- provider fixed to CODEX for this adapter;
- no arbitrary executable override;
- no arbitrary shell invocation;
- bounded prompt/instruction input;
- bounded optional task/session selectors;
- validated project/worktree cwd;
- adapter-owned command flags only;
- permission-sensitive operations recorded through existing H!veAI permission/event mechanisms where applicable.

Create narrowly scoped Tauri capabilities/permissions for any new commands. Do not use broad shell/process permissions.

Do not expose a generic `execute command` API.

## M13.08 — Required direct process/security tests

Add adversarial and real/local process tests as appropriate.

At minimum cover:

### Availability/readiness

- Codex present and version probe succeeds;
- executable unavailable;
- readiness probe timeout/failure;
- malformed output remains truthful.

### Project containment

- registered project root launch accepted;
- contained allowed worktree accepted where supported;
- sibling/outside directory rejected;
- `..` escape rejected;
- missing/archived project rejected;
- cross-project task attachment rejected.

### Argument/process security

- shell metacharacters in prompt do not become command syntax;
- executable/flag injection attempts rejected;
- arbitrary PID stop rejected;
- only adapter-owned process may be stopped.

### Output/lifecycle

- stdout capture;
- stderr capture;
- exit code capture;
- bounded/truncated output behavior;
- clean stop;
- crash/non-zero exit;
- stale persisted running session reconciliation after process loss;
- deterministic state after restart/recovery.

Use a controlled test helper process where invoking real Codex would make deterministic assertions impossible, but also perform a safe real Codex availability/version probe against the user's environment where available. Clearly separate simulated process-fixture tests from real Codex evidence.

## M13.09 — UI integration boundary

M13 needs enough H!veAI UI exposure to prove and operate the adapter, but must not implement the M14 Agent Session Center.

Integrate a compact, existing-style Codex adapter surface in the appropriate current Agents and/or Project Cockpit Agents area that can truthfully show:

- Codex availability/readiness;
- provider/version;
- selected project identity;
- existing/persisted Codex sessions;
- safe Start action when sufficient project/task inputs exist;
- Stop for an owned running Codex session;
- unsupported resume state where applicable;
- status/exit/crash evidence;
- bounded recent output or diagnostic summary only as needed to validate M13.

Do not build an xterm terminal, PTY console, terminal resizing, rich streaming terminal viewport, or M14 session-center redesign.

Maintain the established H!veAI visual system and native behavior.

## M13.10 — Preserve existing systems

Do not regress:

- M10 workflow transitions/evidence rules;
- M11 Command Center;
- M12 Project Cockpit;
- M12A project-wide workflow-history ordering;
- M12B cockpit Tauri capability;
- Project Registry read-only semantics toward registered repositories except explicit user actions already permitted;
- Task Source / Project Dashboard authority model;
- startup video/audio;
- H!veAI native icon;
- Akilta attribution/link;
- dev QA publisher and console suppression.

No external registered project may be modified merely to implement/test the adapter.

## M13.11 — Verification gates

Run all relevant focused tests plus the complete regression suite.

Required minimum gates:

- M13 native focused adapter/process/security tests;
- M13 frontend focused tests;
- full Rust library tests;
- full frontend tests;
- `npm.cmd run typecheck`;
- `npm.cmd run build`;
- `npm.cmd audit -- --audit-level=high`;
- `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`;
- `cargo check --manifest-path src-tauri/Cargo.toml`;
- `git diff --check`;
- existing governed publication failure harness;
- existing governed `publish-dev-qa.ps1` production Tauri `--no-bundle` publication.

Where Codex is installed locally, record safe real readiness/version evidence and, if feasible without uncontrolled side effects, a bounded project-scoped smoke session. Do not claim a real Codex session test if only a fixture process was used.

## M13.12 — Governance/status

Update canonical H!veAI status files truthfully during the implementation run.

M13 must remain `IMPLEMENTATION COMPLETE / PENDING INDEPENDENT STRICT AUDIT + USER NATIVE/VISUAL ACCEPTANCE` after builder completion. Do not mark M13 PASS/CLOSED yourself.

Strict completed roadmap progress remains `13 / 20 = 65%` until independent audit and any required user-native acceptance close M13.

Do not activate or implement M14 in this run.

Do not start M21.

## Immutable builder log

Create:

`H!veAI/docs/H!veAI/codex-logs/M13_CODEX_ADAPTER_IMPLEMENTATION_LOG.md`

The log must include at minimum:

- synchronized starting HEAD/origin/divergence;
- exact Codex executable discovery method;
- real Codex version/readiness result, clearly separated from fixtures;
- adapter architecture and contract;
- exact command construction/security model;
- cwd/worktree containment model;
- session/task/project provenance model;
- output/event bounds;
- stop/resume/recovery behavior;
- Tauri capability changes;
- UI surface added;
- exact files changed;
- focused tests and adversarial cases;
- full regression results;
- publication/failure-harness results;
- implementation commit SHA;
- final fetched local HEAD;
- final fetched `origin/H!veAI`;
- final `HEAD...origin/H!veAI` divergence.

## Commit and push

Commit and push all scoped M13 implementation changes to `origin/H!veAI`.

No force push.

## Final builder state

End with exactly this milestone meaning:

**M13 CODEX ADAPTER IMPLEMENTATION COMPLETE / PENDING INDEPENDENT STRICT AUDIT + USER NATIVE/VISUAL ACCEPTANCE**

Stop. Do not start M14 or M21.
