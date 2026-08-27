# M13 Codex Adapter Implementation — Strict Audit

Date: 2026-08-28
Product: H!veAI
Branch: `H!veAI`
Audited implementation commit: `3fc329fca6d97e9cfdb97cbdff796844dee4c4dd`
Audited builder log: `H!veAI/docs/H!veAI/codex-logs/M13_CODEX_ADAPTER_IMPLEMENTATION_LOG.md`
Authoritative prompt: `H!veAI/docs/H!veAI/prompts/M13_CODEX_ADAPTER_IMPLEMENTATION_PROMPT.md`

## Verdict

**FAIL / M13 NOT CLOSED**

- BLOCKER: 0
- MAJOR: 3
- MINOR: 0
- NOTE: 2
- Confidence: HIGH

The implementation is substantial and many boundaries are correct: direct native process launch, fixed executable discovery, project/task scoping, bounded prompt size, output byte caps, persisted session evidence, no arbitrary PID API, narrow Tauri capability, and stale-session reconciliation are all directionally sound. However three milestone-level requirements are not actually implemented as claimed.

## MAJOR M13-R27 — No provider-neutral native adapter contract

The authoritative prompt requires a provider-neutral native adapter contract for availability/readiness, start, status, resume, stop, and recovery/reconciliation, with Codex implementing that contract so provider-specific assumptions do not leak into every caller.

The production source instead exposes Codex-specific free functions and a Codex-specific `CodexAdapter` process map directly. There is no common provider trait/interface/enum-backed dispatch contract that later providers can implement without duplicating lifecycle callers.

This is not a naming concern. M13.02 is a functional architecture requirement and is currently marked complete in canonical tracking even though the contract does not exist.

Required closure:

- add a bounded provider-neutral native adapter contract;
- make Codex implement/use it;
- keep the currently narrow Codex Tauri API if desired, but route lifecycle semantics through the common contract;
- add direct tests proving common status/lifecycle semantics remain provider-neutral and Codex maps correctly.

## MAJOR M13-R28 — Output is not streamed as bounded structured events during execution

The authoritative prompt requires capture of stdout/stderr plus **bounded structured output events suitable for later M14 display** and explicitly says M13 must capture real process evidence without blocking the main thread.

Current production behavior reads stdout/stderr into in-memory `Capture` buffers and only after the process exits does `monitor_process()` join both reader threads and persist one final `STDOUT` event and one final `STDERR` event.

Therefore a long-running Codex session has no persisted incremental output/event stream while it is running. The implementation is bounded, but it is batch-at-exit rather than streamed structured evidence.

Required closure:

- persist bounded incremental stdout/stderr output events while the process is running;
- keep deterministic byte/event caps and truthful truncation;
- do not implement the M14 PTY/xterm UI;
- preserve secret suppression/redaction before persistence;
- ensure event retention does not grow without bound;
- add a controlled long-running fixture test proving intermediate output becomes observable before process exit.

## MAJOR M13-R29 — Stop is a hard kill, not a clean-stop-first lifecycle

The authoritative prompt requires:

- stop only H!veAI-owned sessions;
- **terminate cleanly first when supported**;
- use bounded escalation only for the known owned child process/tree;
- persist resulting state accurately.

Current `stop()` immediately calls `Child::kill()` on the owned process. On Windows this is a forceful termination path, not a clean-stop-first attempt. The implementation also does not establish an owned process-tree/job boundary for escalation, so descendants spawned by Codex are not explicitly governed by the same lifecycle.

The builder log claims the adapter “stops only owned child processes” and the task ledger marks “Stop process cleanly” complete, but the production source does not satisfy that semantic.

Required closure:

- implement a truthful clean-stop-first mechanism where supported by the installed Codex/process model;
- then use bounded escalation only for the owned process or owned process tree;
- if clean stop is impossible on the platform/version, represent that limitation truthfully and implement the safest bounded owned-process-tree termination available;
- prove no arbitrary PID/process termination primitive is introduced;
- add direct tests for graceful request, bounded escalation, final state, and descendant/process-tree containment where technically applicable.

## Notes

### NOTE N01 — Prompt is passed as a command-line argument

`fixed_exec_args()` places the full bounded prompt on the Codex command line. This avoids shell injection because no shell wrapper is used, but command-line arguments can be observable to local process-inspection tooling. The remediation should evaluate whether the installed Codex CLI safely supports stdin or another non-command-line prompt transport. If so, prefer it. Do not weaken compatibility merely to satisfy this note; document the chosen threat model and transport.

### NOTE N02 — Readiness remains auth-unknown

The readiness model truthfully reports `VERSION_VERIFIED_AUTH_UNKNOWN`, which is acceptable when authentication cannot be determined safely. However failed real starts should continue to distinguish authentication failure from generic process failure when bounded Codex output provides reliable evidence, without exposing credential material.

## Accepted areas

The following areas passed source review and should be preserved during remediation:

- direct `Command::new()` execution rather than `cmd /c` / PowerShell shell wrapping;
- fixed adapter-owned Codex argument construction;
- registered ACTIVE project validation and canonicalized cwd;
- task/project ownership validation;
- bounded prompt size;
- bounded stdout/stderr memory capture;
- redaction before final persisted output;
- session persistence through existing `agent_sessions` / `agent_events` tables;
- stale transient Codex sessions reconciled to `CRASHED` after restart;
- resume truthfully represented as unsupported rather than faked;
- narrow `allow-codex-adapter` Tauri capability;
- no arbitrary PID stop endpoint;
- M14 and M21 remain out of scope.

## Milestone state

M00-M12 remain PASS/CLOSED.

M13 remains **IMPLEMENTATION COMPLETE BUT STRICT AUDIT FAIL** pending bounded remediation of R27-R29 and subsequent independent strict re-audit plus user native/visual acceptance.

Strict completed roadmap progress remains **13/20 = 65%**.

Do not activate M14 and do not start M21.
