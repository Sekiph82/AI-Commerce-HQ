# M15 Prompt Engine Implementation Strict Audit

Date: 2026-09-05
Repository: Sekiph82/AI-Commerce-HQ
Branch: `H!veAI`
Scope: M15 Prompt Engine implementation commit `a09f5e5c3c15184d189165566d035df7e7505fc5`

## Verdict

STRICT AUDIT: FAIL

- BLOCKER: 0
- MAJOR: 2
- MINOR: 0

M15 MUST remain OPEN.
M16 MUST NOT activate.
M21 MUST NOT start.

## Evidence reviewed

- Authoritative M15 implementation prompt.
- Builder log `M15_PROMPT_ENGINE_IMPLEMENTATION_LOG.md`.
- Implementation commit `a09f5e5c3c15184d189165566d035df7e7505fc5`.
- `src-tauri/src/prompt_engine.rs`.
- `src-tauri/src/db/migrations.rs`.
- `src/PromptEnginePage.tsx`.
- `tests/m15-prompt-engine-focused.test.tsx`.
- Accepted M14E provenance and M14 closure boundary.

## Positive findings

The implementation adds a real Prompt Engine route, additive migration v11, prompt kinds, immutable prompt version records, body/context hashes, explicit review and approval, project/task confinement, provider selection, and reuse of the accepted Agent Session Center path. The builder log also records successful focused/full Rust and frontend regressions, governed publication, stable EXE equality, and truthful disclosure that no real M15-created provider dispatch was performed.

Those strengths do not close the two findings below.

## M15-R54 MAJOR — dispatch is not atomically reserved before provider start

### Evidence

In `prompt_engine::dispatch`, the code:

1. loads the version and verifies `APPROVED`;
2. validates the approved body hash;
3. calls `agent_session_center::start(...)`;
4. only after the provider session has already started, updates `prompt_versions` to `DISPATCHED`;
5. then separately updates `agent_sessions` with prompt/version provenance.

The prompt-version update and agent-session provenance update are independent SQLite writes after process/session start. The returned affected-row counts are not checked.

Migration v11 adds ordinary columns and indexes but no single-use reservation/dispatch uniqueness mechanism that would prevent two concurrent dispatches of the same approved version.

### Impact

This breaks the required immutable chain:

`prompt version -> explicit approval -> selected provider -> exact agent session`

A persistence failure after provider start can leave a live/terminal session without complete M15 provenance. Two concurrent dispatch requests can both observe `APPROVED`, both start provider sessions, and race to overwrite the one `dispatched_session_id` field on the prompt version.

The builder's mocked frontend happy path does not reproduce this failure mode, and the log does not contain a real M15 dispatch proving the route under native conditions.

### Required closure

Dispatch must use a durable single-use reservation/claim before provider launch, with deterministic rollback/failure truth. A prompt version must not be launchable twice. Exact prompt/version/hash/provider provenance must be reserved before native process creation and finalized to exactly one session, or the dispatch must fail before launch. Persistence errors must never create an unprovenanced live session.

Add focused race/failure tests that fail on the current implementation.

## M15-R55 MAJOR — collected context is mostly not materialized into generated builder prompts

### Evidence

`collect_context` gathers project identity, task requirements/dependencies, M08 source references, Project Dashboard authority, audit findings, test evidence, dispositions, and a deterministic context hash.

But `implementation_body` renders only:

- user title/summary,
- project ID,
- task ID/freeform label,
- context manifest hash,
- included byte/item counts,
- a generic execution contract.

It does not include the collected task requirements, dependencies, source references, dashboard constraints/warnings, test evidence, or other included context values.

`AUDIT_SUPPORT` is also routed through the same generic implementation body because generation matches only `REMEDIATION` specially and sends all other kinds to `implementation_body`.

### Impact

The Prompt Engine stores an explainable context snapshot, but the generated implementation/audit-support prompt sent to Codex/Claude does not actually carry most of that evidence. A provider receives a manifest hash it cannot dereference through the provider prompt. Therefore the generated prompt is not materially reproducible from its body and is not reliably builder-ready from the selected task/context.

This is contrary to the M15 product goal that prompts be generated from current project/task/context and that implementation/audit-support prompts be first-class supported prompt types.

### Required closure

Generate prompt bodies from a deterministic, bounded materialized projection of the included context manifest. Include only safe selected values already admitted by the collector, with explicit sectioning and truncation/disposition markers. Do not recursively read arbitrary project files.

Add a dedicated `AUDIT_SUPPORT` body contract rather than silently treating it as implementation.

Add tests proving:

- task acceptance/dependency evidence appears in an implementation prompt when collected;
- included source/dashboard/test references appear deterministically;
- omitted/excluded entries are represented truthfully without leaking values;
- audit-support generation has distinct, testable semantics;
- prompt body remains within the existing bound;
- same input/context yields the same generated body/hash.

## Regression note

The builder log records one earlier timing-sensitive concurrent Rust test failure that later passed in serialized regression. This remains disclosed evidence but is not independently classified here because the final required serialized suite passed.

## Closure boundary

M15 is NOT accepted.

Required next step: M15A remediation for R54 and R55 only, followed by independent strict re-audit and native/user acceptance of the published Prompt Engine.

M14 remains PASS/CLOSED.
M16-M20 remain blocked/planned.
M21 remains planned/not started.
Strict completed roadmap progress remains `15 / 20 = 75%`.
