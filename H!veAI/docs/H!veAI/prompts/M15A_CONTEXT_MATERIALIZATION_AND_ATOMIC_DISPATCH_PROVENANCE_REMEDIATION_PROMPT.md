# M15A Context Materialization and Atomic Dispatch Provenance Remediation Prompt

Date: 2026-09-05
Product: H!veAI
Branch: `H!veAI`
Milestone: M15A remediation
Authority: authoritative remediation prompt

## 0. Scope authority

The independent M15 strict audit is FAIL with exactly two MAJOR findings:

- M15-R54: dispatch is not atomically reserved before provider/session start.
- M15-R55: collected context is mostly not materialized into generated implementation/audit-support prompts.

Fix R54 and R55 only.

M14 remains PASS/CLOSED.
M15 remains OPEN.
M16-M20 remain blocked/planned.
M21 remains planned/not started.
Strict completed roadmap progress remains `15 / 20 = 75%`.

Do not rewrite historical M15 implementation evidence.

## 1. Non-negotiable inherited boundaries

Preserve all accepted M04-M14/M14E boundaries, including:

- ACTIVE registered-project confinement;
- task/project ownership validation;
- no arbitrary executable, shell, raw args, or PID control;
- prompt text never in provider argv;
- bounded stdin transport;
- secret-safe persistence and stream sanitization;
- dedicated final assistant response capture;
- no visible background console windows;
- immutable prompt versions once approved/used;
- governed stable EXE publication;
- M21 not started.

## 2. R54 required remediation — atomic/single-use dispatch claim

Redesign M15 dispatch so an approved prompt version cannot launch more than one provider session and cannot launch without durable provenance reservation.

### Required behavior

Before native provider/session start:

1. validate ACTIVE project, task ownership, prompt ownership, provider allowlist, exact version, approval state, and approved-body hash;
2. atomically claim that exact approved prompt version for one dispatch attempt;
3. persist immutable dispatch intent/provenance including exact prompt ID, version ID, version number, body SHA-256, provider, project ID, optional task ID, and a unique dispatch attempt/reservation ID;
4. make concurrent/replayed dispatch of the same version fail before provider start.

After claim:

- launch only through the accepted Agent Session Center provider path;
- bind the created session to the reserved exact prompt/version/hash/provider;
- finalize the reservation exactly once;
- if provider start fails, persist truthful failed dispatch state without making the version silently dispatchable again unless an explicit new prompt version or explicit audited retry model is used;
- if session-provenance persistence cannot complete, stop/contain the owned launch where technically possible and return a durable degraded/failure state rather than a provenance-less successful session.

Do not solve this with an in-memory mutex alone. The single-use claim must be durable in SQLite and race-safe.

Use a narrow additive migration if required.

### Required adversarial tests

Prove all of the following:

- two concurrent dispatch calls for the same approved version result in at most one provider start;
- replay after successful dispatch is rejected before provider launch;
- tampered content/hash is rejected before claim/launch;
- DB failure during claim causes zero provider starts;
- failure after claim but before/at provider start remains durably truthful;
- a session cannot end up with missing exact prompt/version/hash provenance after successful M15 dispatch;
- cross-project/task/provider mismatch remains rejected.

## 3. R55 required remediation — materialized bounded context

The collector may continue to gather only bounded, approved evidence. Do not add recursive filesystem scraping.

Create a deterministic prompt-body renderer that uses the already-collected safe context values.

### Implementation prompts

The generated body must include, when available and INCLUDED:

- project identity/status/branch/head reference;
- selected task title/state/required actor/milestone;
- task requirements/acceptance criteria and dependency references;
- approved M08 source references with path/kind/hash metadata only;
- Project Dashboard authority/provenance/warnings relevant to execution;
- relevant bounded test evidence;
- relevant bounded audit evidence only when appropriate;
- explicit omitted/truncated/stale/unavailable/excluded summary.

Do not include values from EXCLUDED/OMITTED items.

### Remediation prompts

Preserve selected-finding behavior, but also include the bounded project/task/context constraints needed to fix those findings safely.

### Audit-support prompts

Implement a distinct `AUDIT_SUPPORT` renderer with explicit audit-support semantics. It must not silently reuse the implementation body.

At minimum it should identify:

- audit target/scope;
- exact project/task context;
- evidence references available for inspection;
- expected verification/closure outputs;
- prohibition against mutation unless explicitly authorized by the prompt.

### Determinism and bounds

- same request + same context manifest must generate identical body bytes/hash;
- body must remain within `MAX_PROMPT_BODY_BYTES`;
- truncation must be UTF-8 safe and explicit;
- materialization must never read additional arbitrary files;
- secret/excluded entries must not leak into the body;
- context manifest/hash and rendered context must remain provenance-linked.

## 4. UI requirements

Keep the existing Prompt Engine workflow:

`Context -> Generate draft -> Review/Edit -> Approve -> Provider -> Dispatch`

No redesign of unrelated pages.

The user must be able to inspect the context manifest and the generated body must visibly contain meaningful task/context evidence rather than only a manifest hash/count.

If dispatch is already claimed/dispatched, the UI must make the state truthful and disable/reject duplicate dispatch.

## 5. Required native acceptance scenario

Use a disposable/explicitly safe registered fixture if possible.

Required acceptance:

1. create/select an ACTIVE fixture project;
2. select a task with known acceptance criteria/dependency evidence;
3. generate an IMPLEMENTATION prompt;
4. verify the prompt body visibly contains the expected bounded task/context evidence;
5. edit if desired;
6. approve exact version;
7. dispatch once to an installed provider through M15;
8. verify the resulting Agent Session Center session carries the exact prompt/version/hash/provider provenance;
9. attempt a second dispatch of the same version and prove it is rejected before another provider process starts;
10. verify final assistant response remains readable using accepted M14E chat-first behavior.

If real provider execution is unsafe/unavailable, do not fake PASS. Record the exact boundary and require user native acceptance.

## 6. Explicit execution gates

Execute and record each gate individually:

1. `git fetch origin H!veAI`.
2. fast-forward-only synchronization.
3. confirm exact branch `H!veAI`.
4. record starting HEAD/worktree.
5. preserve unrelated user files.
6. read M15 authoritative prompt.
7. read M15 implementation log.
8. read M15 strict audit.
9. confirm M14 remains PASS/CLOSED.
10. confirm M15 remains OPEN.
11. confirm M16 not activated.
12. confirm M21 not started.
13. inspect current prompt v11 schema.
14. inspect `prompt_engine::dispatch`.
15. inspect Agent Session Center start/provenance behavior.
16. reproduce current duplicate-dispatch race with a controlled fixture/test.
17. reproduce post-start provenance failure hazard with controlled injection/fixture.
18. design durable single-use dispatch claim.
19. add migration only if needed.
20. prove migration apply/reapply/history compatibility.
21. implement atomic claim before provider start.
22. implement replay/duplicate rejection before launch.
23. implement exact provider/project/task/hash reservation.
24. implement truthful failed-claim/start/finalization states.
25. ensure no arbitrary process-control surface is added.
26. add concurrent dispatch test.
27. add replay dispatch test.
28. add claim DB-failure test.
29. add post-claim provider-start failure test.
30. add exact session provenance test.
31. inspect current context collector output.
32. build deterministic context rendering model.
33. materialize task requirements/acceptance/dependencies.
34. materialize approved source references.
35. materialize dashboard authority/provenance/warnings.
36. materialize relevant bounded test evidence.
37. preserve selected remediation findings.
38. add distinct AUDIT_SUPPORT renderer.
39. prove excluded values do not enter body.
40. prove deterministic same-input body/hash.
41. prove UTF-8-safe body bound/truncation.
42. add implementation prompt context-materialization test.
43. add remediation context-materialization test.
44. add audit-support focused test.
45. add frontend duplicate-dispatch state/denial test.
46. add frontend materialized-context visibility test.
47. run focused M15/M15A Rust tests.
48. run full serialized Rust regression.
49. run focused Prompt Engine frontend tests.
50. run full frontend regression.
51. run TypeScript typecheck.
52. run frontend production build.
53. run `npm audit --audit-level=high`.
54. run Rust fmt check.
55. run Rust all-targets check.
56. run Rust `pty-support` check.
57. run `git diff --check`.
58. run security review for project/task/provider/process confinement.
59. run secret/context leakage adversarial tests.
60. run publisher failure/rollback harness.
61. governed production Tauri `--no-bundle` publication.
62. verify candidate/stable SHA equality and PE/startup/shortcut/icon.
63. verify no visible console popup.
64. run disposable native M15 dispatch acceptance if safely feasible.
65. verify duplicate second dispatch launches no second provider session.
66. verify exact prompt/version/hash appears on created session.
67. verify M14E final-response/chat-first behavior remains green.
68. create immutable M15A remediation log.
69. commit only scoped files.
70. push normally, no force.
71. confirm local HEAD equals `origin/H!veAI`.
72. leave M15 implementation complete pending independent re-audit + user acceptance.
73. do not close M15.
74. do not activate M16.
75. do not start M21.

## 7. Required remediation log

Create:

`H!veAI/docs/H!veAI/codex-logs/M15A_CONTEXT_MATERIALIZATION_AND_ATOMIC_DISPATCH_PROVENANCE_REMEDIATION_LOG.md`

The log must include:

- implementation commit SHA;
- changed files;
- exact migration behavior if any;
- atomic dispatch design;
- race/failure reproduction evidence;
- context materialization examples without secrets;
- test counts;
- publication hashes;
- native acceptance evidence or explicit limitation;
- final milestone state.

## 8. Completion boundary

Do not claim M15 PASS/CLOSED.

Expected builder final state:

- M15-R54 remediation complete/pending independent re-audit;
- M15-R55 remediation complete/pending independent re-audit;
- M15 remains OPEN;
- M16 remains blocked;
- M21 not started;
- progress remains `15 / 20 = 75%`.
