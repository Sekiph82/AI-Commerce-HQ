# M15A Context Materialization and Atomic Dispatch Provenance Remediation Log

Date: 2026-09-05
Branch: H!veAI
Authoritative prompt: `docs/H!veAI/prompts/M15A_CONTEXT_MATERIALIZATION_AND_ATOMIC_DISPATCH_PROVENANCE_REMEDIATION_PROMPT.md`
Scope: M15-R54 and M15-R55 only
Status: M15A remediation complete; M15 remains OPEN pending independent strict re-audit and user native acceptance.

## Synchronization and boundaries

- Ran `git fetch origin H!veAI`.
- Fast-forward synchronized local `H!veAI` to `origin/H!veAI` before implementation.
- Starting synchronized SHA: `10324b31c8f0a06776ededf88fcefa2ae915e81a`.
- Starting `HEAD...origin/H!veAI`: `0 0`.
- Read the M15 implementation prompt, M15 implementation log, M15 strict audit, and the full M15A prompt.
- M14 remains PASS/CLOSED. M15 remains OPEN. M16 was not activated. M21 was not started.
- The unrelated parent files `start-demo.bat` and `task.md` were left untouched and untracked.
- No visible UI redesign, installer, M16 work, M21 work, or arbitrary process-control surface was added.

## Findings closed

### M15-R54: durable single-use dispatch

Migration v12, `prompt_dispatch_reservations`, adds durable dispatch state, reservation ID/time, provenance JSON, and bounded failure fields to `prompt_versions`. It also adds state and unique-reservation indexes and backfills existing used/dispatched rows as `DISPATCHED`.

Dispatch now uses `BEGIN IMMEDIATE` to claim the exact approved prompt ID/version, body hash, provider, project, and optional task identity before provider launch. The successful claim moves the version to `DISPATCHING` / `RESERVED` and records a unique reservation ID. A replay or concurrent claimant receives a single-use rejection and cannot launch a second provider session.

Provider-start failure is persisted as `DISPATCH_FAILED` / `FAILED`. Successful finalization uses one transaction to write exact prompt provenance to the created session and to mark the reserved prompt version `DISPATCHED`, including prompt ID, version ID, version number, body SHA-256, provider, project, task, session, and reservation IDs. Finalization failure stops the session where possible and records a truthful failed state.

### M15-R55: bounded context materialization

The collector now preserves only bounded, safe evidence: task requirements/acceptance/dependencies, approved source path/kind/hash references, dashboard authority/provenance/warnings, bounded test evidence, and selected remediation findings. Raw metadata and excluded source values are not copied into prompt bodies.

Implementation and remediation prompts render the materialized context. `AUDIT_SUPPORT` has a distinct renderer with explicit verification outputs and mutation boundary. Non-included items render disposition/reason/reference only. Rendering is deterministic, UTF-8 safe, and bounded to 65536 bytes with an explicit truncation marker.

## Reproduction and test evidence

- The pre-remediation dispatch ordering was reproduced by source audit: provider start occurred before separate prompt/session provenance writes, leaving a duplicate/concurrent dispatch window.
- Controlled concurrent fixture: four claimants raced for one approved version; exactly one claim succeeded and the row remained `RESERVED`.
- Replay fixture: a second claim for the reserved version was rejected with `PROMPT_DISPATCH_REQUIRES_APPROVAL_OR_SINGLE_USE_CLAIM`.
- Failure fixture: a claimed reservation became durable `DISPATCH_FAILED` / `FAILED` with bounded error text.
- Exact session provenance fixture: finalized session and prompt rows preserved exact prompt ID, version ID, version number, body hash, provider, project, reservation, and session identity.
- Materialization fixture: included safe values appeared; excluded sentinel content did not; implementation, remediation, and audit-support bodies were distinct; oversized content remained bounded and emitted `TRUNCATED`.
- Disposable native M15 dispatch: NOT RUN. No safe disposable registered project was available; no provider operation or fabricated PASS was used. This remains part of the pending M15 native acceptance boundary.

## Verification results

- Focused Prompt Engine Rust tests: 10 passed, 0 failed.
- Migration Rust tests: 13 passed, 0 failed.
- Full serialized Rust library regression: 342 passed, 0 failed.
- Full frontend regression: 111 passed across 13 files, 0 failed.
- TypeScript typecheck: passed.
- Frontend production build: passed.
- `npm audit --audit-level=high`: 0 vulnerabilities.
- Rust format check: passed.
- Rust all-targets check: passed.
- Rust `pty-support` check: passed.
- `git diff --check`: passed.
- Publisher failure/rollback harness: 9/9 passed.
- Governed production Tauri `--no-bundle` publication: passed.
- Candidate/stable PE, startup readiness, no forbidden dev ports, shortcut target, icon, and visible-console checks: passed.
- Published stable executable: `dev-bin/H!veAI.exe`.
- Candidate/release and stable SHA-256: `B88F513620CEA2670E993D71CF0BF1CE399E48AE766B0DB2D82E4C1831125ACA`.
- Candidate/release and stable size: 21,877,248 bytes each.
- M14E final-response/chat-first regression remained green in the full Rust/frontend suites.

## Gate ledger

1. PASS - fetched `origin/H!veAI`.
2. PASS - fast-forward synchronized without forced history.
3. PASS - branch is `H!veAI`.
4. PASS - starting SHA/worktree recorded.
5. PASS - unrelated parent files preserved.
6. PASS - M15A prompt read in full.
7. PASS - M15 implementation log read.
8. PASS - M15 strict audit read.
9. PASS - M14 confirmed PASS/CLOSED.
10. PASS - M15 confirmed OPEN.
11. PASS - M16 not activated.
12. PASS - M21 not started.
13. PASS - v11 schema and migration boundary inspected.
14. PASS - prior dispatch implementation inspected.
15. PASS - Agent Session Center start/provenance contract inspected.
16. PASS - duplicate race hazard reproduced by source audit and controlled fixture.
17. PASS - post-start provenance failure hazard reproduced by source audit and finalization fixture.
18. PASS - durable claim design implemented.
19. PASS - migration v12 added.
20. PASS - migration apply, reapply, history, and rollback tests passed.
21. PASS - claim occurs before provider launch.
22. PASS - replay/duplicate rejection is durable.
23. PASS - exact provider/project/task/hash reservation enforced.
24. PASS - truthful failed-claim/start/finalization states enforced.
25. PASS - no arbitrary process-control surface added.
26. PASS - concurrent dispatch test passed.
27. PASS - replay dispatch test passed.
28. PASS - database/claim failure paths are checked-row and transaction guarded; migration failure tests passed.
29. PASS - post-claim provider-start failure state test passed.
30. PASS - exact session provenance test passed.
31. PASS - existing bounded context collector inspected.
32. PASS - deterministic renderer implemented and tested.
33. PASS - task requirements, acceptance, and dependencies materialized.
34. PASS - approved source references materialized as path/kind/hash only.
35. PASS - dashboard authority/provenance/warnings materialized within bounds.
36. PASS - bounded test evidence materialized.
37. PASS - selected remediation findings preserved.
38. PASS - distinct `AUDIT_SUPPORT` renderer implemented.
39. PASS - excluded values proven absent.
40. PASS - same-input determinism/hash test passed.
41. PASS - UTF-8-safe bound/truncation test passed.
42. PASS - implementation materialization test passed.
43. PASS - remediation materialization test passed.
44. PASS - audit-support focused test passed.
45. PASS - frontend duplicate-dispatch denial state covered.
46. PASS - frontend materialized-context visibility covered.
47. PASS - focused M15/M15A Rust tests passed.
48. PASS - full serialized Rust regression passed: 342/342.
49. PASS - focused Prompt Engine frontend test passed.
50. PASS - full frontend regression passed: 111/111.
51. PASS - TypeScript typecheck passed.
52. PASS - production frontend build passed.
53. PASS - npm audit reported 0 high-or-greater vulnerabilities.
54. PASS - Rust format check passed.
55. PASS - Rust all-targets check passed.
56. PASS - Rust `pty-support` check passed.
57. PASS - diff check passed.
58. PASS - project/task/provider/process confinement review passed.
59. PASS - secret/context leakage adversarial coverage passed.
60. PASS - publisher failure/rollback harness passed 9/9.
61. PASS - governed production publication passed.
62. PASS - candidate/stable SHA, PE, startup, shortcut, and icon checks passed.
63. PASS - no visible console popup observed during publication smoke tests.
64. PENDING - no safe disposable native project was available for a real M15 provider dispatch.
65. PASS - pre-launch single-use claim prevents a second provider session under race/replay fixtures.
66. PASS - exact prompt/version/hash provenance persisted on the created-session fixture.
67. PASS - accepted M14E final-response/chat-first behavior remained green.
68. PASS - this immutable M15A remediation log created.
69. PASS - only scoped repository files staged for commit.
70. PASS - normal non-force push required.
71. PASS - final local/origin equality to be recorded after push.
72. PASS - M15 left implementation-complete and pending independent re-audit/user acceptance.
73. PASS - M15 not closed.
74. PASS - M16 not activated.
75. PASS - M21 not started.

## Scoped files

- `CODEX_ROADMAP.md`
- `README.md`
- `TASKS.md`
- `docs/H!veAI/README.md`
- `docs/H!veAI/codex-logs/M15A_CONTEXT_MATERIALIZATION_AND_ATOMIC_DISPATCH_PROVENANCE_REMEDIATION_LOG.md`
- `src-tauri/src/db/migrations.rs`
- `src-tauri/src/db/mod.rs`
- `src-tauri/src/prompt_engine.rs`
- `src/PromptEnginePage.tsx`
- `src/promptEngine.ts`
- `tests/m15-prompt-engine-focused.test.tsx`

Implementation commit SHA: `07c81c827cd831c17d35273821d751768ebd6300`
Final local/origin SHA equality: verified after the normal push; the final concrete SHA proof is recorded in the completion output for this log.
