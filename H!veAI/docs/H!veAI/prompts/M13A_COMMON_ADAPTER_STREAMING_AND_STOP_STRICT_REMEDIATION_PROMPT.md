# M13A Common Adapter, Streaming, and Stop Strict Remediation — Authoritative Prompt

Date: 2026-08-28
Product: H!veAI
Branch: `H!veAI`
Milestone: M13A bounded remediation

## Authority

This prompt is the sole implementation authority for the M13 strict-audit remediation findings R27-R29.

Safely synchronize `H!veAI` with `origin/H!veAI` using fetch plus fast-forward-only merge before editing. Do not reset, rebase, force-push, or discard user-owned work.

Read in full before coding:

- `H!veAI/AGENTS.md`
- `H!veAI/CONSTITUTION.md`
- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/.hiveai/PROJECT_DASHBOARD.md`
- `H!veAI/docs/H!veAI/prompts/M13_CODEX_ADAPTER_IMPLEMENTATION_PROMPT.md`
- `H!veAI/docs/H!veAI/codex-logs/M13_CODEX_ADAPTER_IMPLEMENTATION_LOG.md`
- `H!veAI/docs/H!veAI/audits/M13_CODEX_ADAPTER_IMPLEMENTATION_STRICT_AUDIT.md`
- all relevant M10/M12/M13 production/test source required to preserve workflow, project, session, capability, and Project Cockpit contracts.

Do not start M14 or M21.

## Scope

Close exactly these strict-audit findings:

- R27: missing provider-neutral native adapter contract;
- R28: output is batch-at-exit rather than bounded incremental structured events;
- R29: stop immediately force-kills the child instead of clean-stop-first with bounded owned-process escalation.

Preserve all accepted M13 behavior unless a change is required to close one of these findings.

## R27 — Provider-neutral adapter contract

Create a real native common adapter contract suitable for later providers without implementing Claude or M14.

The common contract must cover at minimum:

- readiness/availability;
- start;
- status/list/read session;
- stop;
- resume capability/result;
- recovery/reconciliation;
- provider identity;
- common lifecycle/state model.

Implementation requirements:

1. Do not create a generic arbitrary-process executor.
2. Provider selection must remain bounded/allowlisted.
3. Codex must implement or be dispatched through the common contract.
4. Existing Codex-specific Tauri commands may remain as a compatibility façade if useful, but they must use the common adapter lifecycle underneath rather than becoming a parallel truth path.
5. Keep existing `agent_sessions` and `agent_events` as the durable evidence stores.
6. Do not implement another provider in this run.

Add direct native tests proving:

- provider-neutral readiness/start/stop/status semantics;
- unsupported provider/provider mismatch is rejected truthfully;
- Codex maps correctly into the common session/state model;
- no arbitrary executable/argument surface is introduced.

## R28 — Bounded incremental structured output events

The running session must expose/persist bounded output evidence before exit.

Requirements:

1. Continue capturing stdout and stderr off the Tauri main thread.
2. Convert stream chunks/lines into bounded structured `agent_events` during process execution.
3. Redact/suppress recognizable sensitive content **before persistence**.
4. Enforce both byte and event-count limits.
5. Retention behavior must be deterministic and truthful when truncation occurs.
6. A running session must be able to show at least bounded recent output/event evidence before process termination.
7. Final session completion must still preserve exit code/end state and final truncation metadata.
8. Do not implement PTY/xterm/live terminal UX. That belongs to M14.

Prefer an event model that is suitable for later M14 consumption, e.g. bounded stream events with channel, text, timestamp/sequence, and truncation state, while staying compatible with current persistence schema.

Add adversarial/controlled tests proving:

- a helper process emits output, waits, emits more output, and the first output is persisted/readable before exit;
- stdout and stderr are distinguished;
- sensitive lines are redacted before persistence;
- byte cap and event cap cannot grow unbounded;
- truncation is surfaced truthfully;
- final exit evidence remains correct.

## R29 — Clean-stop-first and owned-process-tree lifecycle

Replace immediate hard-kill semantics with a truthful bounded lifecycle.

First inspect the installed Codex CLI and Windows process model to determine what safe graceful stop mechanism is actually available. Do not invent unsupported behavior.

Requirements:

1. Stop only sessions/processes owned by this H!veAI adapter instance.
2. Attempt a graceful/clean termination signal or supported Codex cancellation path first where technically supported.
3. Wait for a bounded grace period.
4. If still running, escalate only against the owned process or owned process tree.
5. Do not expose arbitrary PID termination.
6. Prevent known child/descendant leakage where the platform supports owned process-tree grouping, such as an appropriate Windows job/process-group ownership mechanism.
7. Persist whether termination was graceful, escalated, failed, or crashed.
8. Do not report `STOPPED` merely because stop was requested if the process did not actually terminate accordingly.

If Windows/Codex provides no reliable graceful stop mechanism, document that finding precisely and implement the safest bounded owned-process-tree termination path, with state names/messages that do not falsely call it graceful.

Add direct process tests proving:

- only owned sessions can be stopped;
- graceful path is attempted when supported;
- bounded escalation occurs after timeout when required;
- final persisted state matches actual termination outcome;
- unrelated processes are unaffected;
- process-tree/descendant handling is bounded and owned where technically supported.

## Prompt transport security note

Audit NOTE N01 observed that the full prompt is currently passed as a command-line argument.

Inspect the installed `codex.exe` help/behavior and determine whether a safe stdin or equivalent prompt-input mechanism exists for `codex exec`.

- If a stable supported stdin mechanism exists, prefer it so prompt contents are not exposed in the process command line.
- If no stable supported mechanism exists, retain the current direct argument transport only with a documented threat-model note and tests proving no shell interpretation occurs.
- Never introduce shell wrappers to solve this.

This note is secondary to R27-R29 and must not cause an unsafe compatibility regression.

## Preserve these accepted boundaries

Do not regress:

- direct native process ownership;
- no `cmd /c` / PowerShell command execution for production Codex launch;
- fixed executable discovery;
- registered ACTIVE project validation;
- canonicalized project cwd;
- cross-project task rejection;
- bounded prompt size;
- secret suppression/redaction;
- existing session/event persistence;
- truthful unsupported resume if no safe stable Codex resume exists;
- stale-session restart reconciliation;
- narrow Tauri capability;
- M12 Project Cockpit behavior;
- startup video/icon behavior;
- Akilta attribution;
- external project repositories;
- Bulk Edit;
- M14 and M21.

## Verification gates

Run at minimum:

1. focused common-adapter tests;
2. focused streaming/output-event tests;
3. focused stop/process-tree lifecycle tests;
4. existing M13 focused native tests;
5. existing M13 frontend tests;
6. full Rust test suite;
7. full frontend test suite;
8. TypeScript typecheck;
9. frontend production build;
10. npm high-severity audit;
11. cargo fmt check;
12. cargo check;
13. `git diff --check`;
14. governed dev-QA failure harness;
15. governed dev-QA publication / production Tauri `--no-bundle` build.

Perform a safe real local Codex readiness/version probe. If a harmless real Codex start/stop smoke can be executed inside a disposable registered test fixture without mutating user projects, do so and document it. Do not run destructive prompts against user repositories.

## Governance and status

Update only truthful M13/M13A status documents required by the existing governance pattern.

Do not mark M13 PASS/CLOSED. Final closure requires independent strict re-audit and user native/visual acceptance.

Strict completed roadmap progress remains `13/20 = 65%` throughout this remediation run.

M14 remains blocked/not started. M21 remains planned/not started.

## Immutable builder log

Create:

`H!veAI/docs/H!veAI/codex-logs/M13A_COMMON_ADAPTER_STREAMING_AND_STOP_STRICT_REMEDIATION_LOG.md`

The log must include:

- synchronized preflight HEAD/origin/divergence;
- exact R27/R28/R29 implementation changes;
- common adapter contract design;
- incremental event retention/cap/redaction behavior;
- graceful-stop/escalation/process-tree design and platform limitations;
- prompt transport decision and evidence;
- exact tests and counts;
- publication results;
- exact files changed;
- implementation commit SHA;
- final fetched local HEAD and `origin/H!veAI`;
- final `HEAD...origin/H!veAI` divergence.

Commit and push all scoped changes to `origin/H!veAI`.

Final builder state:

`M13A REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

Stop after this remediation. Do not start M14 or M21.
