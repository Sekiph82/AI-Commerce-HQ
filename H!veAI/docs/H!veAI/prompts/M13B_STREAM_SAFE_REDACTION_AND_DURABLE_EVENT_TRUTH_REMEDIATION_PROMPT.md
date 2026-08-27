# M13B Stream-Safe Redaction and Durable Event Truth — Authoritative Remediation Prompt

Date: 2026-08-28
Product: H!veAI
Branch: `H!veAI`
Milestone: M13 Codex Adapter
Remediation findings: R30, R31

## Authority

This prompt is the sole implementation authority for this bounded M13B remediation.

Before coding, safely synchronize `H!veAI` with `origin/H!veAI` using fetch plus fast-forward-only merge. Do not reset, rebase, force-push, discard user-owned work, or modify unrelated parent-root files.

Read in full before coding:

- `H!veAI/AGENTS.md`
- `H!veAI/CONSTITUTION.md`
- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/.hiveai/PROJECT_DASHBOARD.md`
- `H!veAI/docs/H!veAI/prompts/M13_CODEX_ADAPTER_IMPLEMENTATION_PROMPT.md`
- `H!veAI/docs/H!veAI/codex-logs/M13_CODEX_ADAPTER_IMPLEMENTATION_LOG.md`
- `H!veAI/docs/H!veAI/audits/M13_CODEX_ADAPTER_IMPLEMENTATION_STRICT_AUDIT.md`
- `H!veAI/docs/H!veAI/prompts/M13A_COMMON_ADAPTER_STREAMING_AND_STOP_STRICT_REMEDIATION_PROMPT.md`
- `H!veAI/docs/H!veAI/codex-logs/M13A_COMMON_ADAPTER_STREAMING_AND_STOP_STRICT_REMEDIATION_LOG.md`
- `H!veAI/docs/H!veAI/audits/M13A_COMMON_ADAPTER_STREAMING_AND_STOP_STRICT_REAUDIT.md`
- the exact M13/M13A production/test files needed to close R30 and R31.

Do not start M14 or M21.

## Current accepted state

Preserve these accepted M13/M13A boundaries:

- provider-neutral `AgentAdapter` contract exists and Codex implements it;
- Codex launch remains direct/native with fixed adapter-owned arguments;
- prompt transport remains bounded stdin, not command-line prompt text;
- registered ACTIVE project/task containment remains enforced;
- stdout/stderr are incrementally read while the process is running;
- stream byte/event caps remain bounded;
- resume remains truthfully unsupported where no safe stable mechanism exists;
- stop remains owned-process-only with truthful graceful-unavailable evidence, bounded grace, and owned process-tree escalation;
- no generic shell/command/PID primitive may be introduced.

M13 remains open only because R30 and R31 remain.

---

# R30 — Stream-safe pre-persistence sensitive-output redaction

The current implementation redacts arbitrary read chunks independently. That is not safe because sensitive markers may cross OS pipe read boundaries.

## Required behavior

Redaction must be stateful across arbitrary input chunk boundaries.

A complete logical stream containing recognizable sensitive material must not leak merely because the marker/value is split between two or more reads.

At minimum protect the existing sensitive marker classes:

- `api_key`
- `apikey`
- `token`
- `password`
- `secret`
- `authorization`
- `sk-`

You may strengthen the detector, but do not weaken current suppression.

## Required design properties

Implement a bounded stream-safe redaction layer, for example a bounded line/record assembler or another stateful streaming redactor.

It must:

1. keep an explicitly bounded uncommitted carry buffer across reads;
2. never persist raw carry content before it has been classified/redacted;
3. correctly detect sensitive markers split at arbitrary character positions across reads;
4. handle markers/values split across more than two reads;
5. safely flush final unterminated partial content at EOF;
6. handle UTF-8 boundaries truthfully without panics or unbounded buffering;
7. apply output byte/event caps to the safely redacted retained representation;
8. never write the original sensitive bytes into `agent_events` before redaction;
9. preserve independent stdout/stderr channel identity and sequence behavior;
10. preserve existing user-facing redacted marker semantics or improve them without exposing the secret.

Do not solve this by buffering the entire unbounded process output until exit.

## Mandatory R30 tests

Add direct adversarial tests that feed the stream through deliberately controlled tiny chunks.

For each protected sensitive marker, test multiple split positions, including splits inside the marker itself. Examples must include at least:

- `api_` + `key=super-secret-value`
- `author` + `ization: Bearer ...`
- `pass` + `word=...`
- `to` + `ken=...`
- `s` + `k-...`

Also test:

- marker split over three or more chunks;
- secret-bearing final line with no newline;
- normal non-sensitive content crossing chunks remains reconstructable;
- redacted bytes are absent from every persisted `STREAM_OUTPUT` payload, not only from the final reconstructed session string;
- event/byte caps still work after stateful redaction.

Tests must inspect durable DB event payloads directly for absence of the secret.

---

# R31 — Durable event truth under concurrent persistence failure/contention

The current incremental path ignores `insert_event()` failures after capture accounting has already advanced. Silent durable event loss is forbidden.

## Required behavior

Incremental stdout/stderr event persistence must be truthful even when SQLite temporarily returns busy/locked or another write failure.

No `STREAM_OUTPUT` persistence failure may be silently discarded.

## Preferred architecture

Prefer a single bounded persistence writer/channel for lifecycle + stream output where practical, or an equivalent deterministic design that avoids competing stdout/stderr SQLite writers.

A bounded retry/backoff strategy for transient SQLite `BUSY`/`LOCKED` is acceptable if it remains deterministic and bounded.

## Required truth model

Final session evidence must distinguish, directly or through equivalent explicit fields/events:

- bytes/events safely captured after redaction;
- bytes/events durably persisted;
- truncation due configured caps;
- degradation/loss due persistence failure, if any remains after bounded retry.

If persistence ultimately fails:

- record an explicit diagnostic/degraded state/event through a path that does not silently claim success;
- do not report captured event counts as durable event counts;
- do not fabricate reconstructed output that is absent from durable evidence;
- preserve truthful lifecycle completion even if output evidence is degraded.

Do not use unbounded retries and do not block the Tauri main thread.

## Mandatory R31 tests

Add deterministic tests for:

1. simultaneous stdout and stderr incremental output;
2. ordered per-channel sequences under concurrent reading;
3. forced transient SQLite busy/locked behavior with bounded retry recovery;
4. forced terminal persistence failure after bounded retries;
5. explicit degraded diagnostic/evidence when durable output cannot be persisted;
6. consistency between durable `STREAM_OUTPUT` row counts and final persisted-output counts;
7. captured counts never masquerade as persisted counts;
8. normal no-contention execution remains unchanged;
9. final state remains truthful for COMPLETED, FAILED, STOPPED, and CRASHED cases.

Use controlled fixtures/injection points where necessary to force deterministic persistence failures. Do not rely on probabilistic timing races.

---

# Security and regression requirements

Preserve all accepted M13 security properties:

- no arbitrary executable override;
- no shell wrapper;
- no raw command string interpolation;
- no arbitrary PID stop;
- no prompt in process command line;
- no credentials/auth tokens intentionally queried or persisted;
- project/task scoping remains strict;
- existing narrow Tauri capability remains narrow.

Run at minimum:

- focused native M13B tests for R30/R31;
- all existing `codex_adapter` tests;
- focused M13 frontend tests;
- full Rust library tests;
- full frontend tests;
- TypeScript typecheck;
- frontend production build;
- dependency audit at high severity;
- `cargo fmt --check`;
- `cargo check`;
- `git diff --check`;
- governed publisher failure harness;
- governed dev QA publication including production Tauri `--no-bundle` build and existing smoke/console/shortcut gates.

A harmless Codex readiness/version probe is allowed. Do not run an unsupervised real Codex coding operation against a user project merely to close R30/R31; deterministic local fixtures are preferred for the streaming/persistence tests.

## Governance updates

Update canonical H!veAI tracking truthfully:

- M13 remains not closed until independent re-audit + user native acceptance where applicable;
- record M13B as remediation complete pending audit after implementation;
- keep strict completed roadmap count at `13 / 20 = 65%`;
- M14 remains blocked/not started;
- M21 remains planned/not started.

Do not rewrite historical failed audits/logs/prompts.

## Immutable builder log

Create:

`H!veAI/docs/H!veAI/codex-logs/M13B_STREAM_SAFE_REDACTION_AND_DURABLE_EVENT_TRUTH_REMEDIATION_LOG.md`

Include:

- synchronized preflight proof;
- exact R30 root cause and final streaming-redaction architecture;
- exact R31 root cause and durable persistence architecture;
- capture-vs-persisted truth model;
- bounded retry/backoff parameters if used;
- explicit degradation semantics;
- adversarial split-marker test matrix;
- forced persistence contention/failure test evidence;
- exact files changed;
- every verification/publication command and result;
- exact implementation commit SHA;
- fetched final `HEAD`, `origin/H!veAI`, and divergence proof.

Commit and push all scoped changes to `origin/H!veAI`.

## Final builder state

`M13B REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

Stop after M13B. Do not start M14 or M21.
