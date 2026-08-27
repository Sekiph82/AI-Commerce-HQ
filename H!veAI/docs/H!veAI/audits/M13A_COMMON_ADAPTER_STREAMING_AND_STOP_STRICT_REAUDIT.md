# M13A Common Adapter, Streaming, and Stop Strict Re-Audit

Date: 2026-08-28
Product: H!veAI
Branch: `H!veAI`
Audited builder log: `H!veAI/docs/H!veAI/codex-logs/M13A_COMMON_ADAPTER_STREAMING_AND_STOP_STRICT_REMEDIATION_LOG.md`
Audited implementation commit: `4834b3b180c7e780d3fdeaa76641f09b546619be`

## Verdict

**FAIL / M13 NOT CLOSED**

- BLOCKER: 0
- MAJOR: 2
- MINOR: 0
- Confidence: HIGH

R27, R28, and R29 are materially improved and the original findings are substantially addressed. However, two new residual production defects remain in the streaming/security path. Because these defects affect persisted session truth and pre-persistence secret suppression, M13 cannot close yet.

## Accepted remediation areas

### R27 - provider-neutral adapter contract

PASS for the original finding.

The production source now defines `AgentProvider`, `AdapterReadiness`, `AdapterStartRequest`, `AdapterSession`, and a provider-neutral `AgentAdapter` trait exposing provider/readiness/start/list/stop/resume/reconcile. `CodexAdapter` implements that trait while preserving the existing Codex compatibility surface. This is sufficient to close the original R27 structural finding.

### R28 - incremental bounded structured output

PARTIAL PASS for the original finding, with new residual findings below.

stdout and stderr are read concurrently while the process is running and emitted incrementally as persisted `STREAM_OUTPUT` events. Per-channel retained bytes and event counts are bounded, and session loading consumes structured stream events.

### R29 - clean-stop-first / owned process tree

PASS for the original finding.

The installed Codex CLI does not expose a stable graceful cancellation mechanism in the inspected bounded operation. Production records that limitation truthfully, waits a bounded grace interval, then escalates only the adapter-owned PID tree through a fixed direct `taskkill.exe /PID <owned-pid> /T /F` invocation. No arbitrary PID API or shell command was introduced.

## Residual finding R30 - MAJOR - chunk-boundary secret redaction is not stream-safe

### Evidence

`Capture::append()` redacts each individual read chunk independently:

```rust
let redacted = redact_output(&String::from_utf8_lossy(bytes));
```

`read_stream()` passes arbitrary `Read::read()` chunks into `Capture::append()` and persists each returned chunk immediately.

`redact_output()` only detects sensitive markers inside the current string. Therefore a recognizable marker can be split across two read boundaries and evade redaction.

Example adversarial stream boundaries:

- chunk 1: `"api_"`
- chunk 2: `"key=super-secret-value\n"`

Neither chunk individually contains `api_key`, so both may be persisted into separate `STREAM_OUTPUT` events. Reconstructed output then contains the secret even though the complete logical stream contains a recognizable sensitive marker.

The same issue applies to split forms of `authorization`, `password`, `secret`, `token`, and `sk-`.

### Why this is MAJOR

The authoritative M13/M13A boundary requires recognizable/suppressible sensitive output to be redacted before persistence. Arbitrary OS pipe read boundaries are not semantic line/token boundaries. A pre-persistence redactor must therefore be stateful across chunk boundaries or operate on complete bounded records/lines with a safe carry buffer.

### Required correction

Implement stream-safe pre-persistence redaction. At minimum:

- carry enough uncommitted suffix state across reads to detect sensitive markers split across chunks, or use a bounded line/record assembler;
- never persist the raw carry buffer before classification/redaction;
- bound the carry buffer explicitly;
- flush final partial content safely at EOF;
- preserve deterministic byte/event caps after redaction;
- add adversarial tests where every sensitive marker is split at multiple character positions across read boundaries.

## Residual finding R31 - MAJOR - incremental event persistence failures are silently discarded

### Evidence

`read_stream()` currently persists each structured stream event with:

```rust
let _ = insert_event(...);
```

The result is ignored. stdout and stderr each hold separate SQLite connections and write concurrently, while the monitor also writes lifecycle events. If SQLite returns a transient lock/busy or another persistence error, the stream event is silently lost.

Meanwhile, `Capture::append()` has already incremented `event_count` and `retained_bytes`, and `SESSION_FINISHED` later reports those in-memory counts as if the retained stream evidence exists durably.

This can produce a contradictory persisted session:

- `SESSION_FINISHED.stdoutEvents = N`
- fewer than N durable `STREAM_OUTPUT` rows actually exist
- reconstructed UI/session output silently omits data
- no diagnostic says persistence degraded or failed.

### Why this is MAJOR

M13's purpose is a truthful durable agent session/event model. Silent loss of incremental output violates that truth boundary and can make the UI claim bounded captured evidence that was never persisted.

### Required correction

Do not ignore incremental persistence failures. Use one of these bounded designs or an equivalent truthful design:

- serialize stream event persistence through one writer channel/thread; or
- use bounded retry/backoff for SQLite busy/locked errors; and
- if persistence still fails, set an explicit session/output diagnostic/degraded flag/event and make final counts distinguish captured vs durably persisted events/bytes.

Requirements:

- no unbounded retry loop;
- no main-thread blocking;
- no silent event loss;
- final session evidence must truthfully distinguish captured, persisted, and truncated/degraded states;
- deterministic concurrent stdout/stderr tests must force persistence contention/failure and verify truthful behavior.

## Test gap

Current focused tests prove a single `read_stream()` call can redact and persist ordinary input, but they do not cover:

1. sensitive tokens split across arbitrary stream chunks;
2. concurrent stdout/stderr persistence contention;
3. an injected/forced persistence failure after `Capture::append()` succeeds;
4. consistency between final session event counts and durable structured event rows.

These adversarial tests are required for closure.

## Scope result

No evidence was found that M14 or M21 was started. The original M13 process containment, direct executable launch, task/project validation, fixed argument construction, prompt-via-stdin change, bounded output caps, resume-unsupported truth, and owned-process stop boundaries should be preserved.

## Closure decision

**M13A STRICT RE-AUDIT: FAIL**

R27: CLOSED.
R28: original streaming requirement materially implemented, but R30/R31 remain.
R29: CLOSED.

M13 remains at **13/20 = 65% completed roadmap progress** and must not be canonically closed until R30 and R31 are remediated, independently re-audited, and user native acceptance is completed where applicable.
