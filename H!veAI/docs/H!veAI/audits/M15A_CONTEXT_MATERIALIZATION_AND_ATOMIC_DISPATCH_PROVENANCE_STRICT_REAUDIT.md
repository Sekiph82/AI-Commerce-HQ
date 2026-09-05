# M15A Context Materialization and Atomic Dispatch Provenance Strict Re-Audit

Date: 2026-09-05
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Audited branch HEAD: `63b363b345274b931bafccd3d331910160149019`
Parent authority commit: `10324b31c8f0a06776ededf88fcefa2ae915e81a`

## Verdict

**TECHNICAL PASS / USER NATIVE ACCEPTANCE STILL REQUIRED**

- BLOCKER: 0
- MAJOR: 0
- MINOR: 0
- M15-R54: CLOSED
- M15-R55: CLOSED

M15 remains OPEN only because the required native/user Prompt Engine acceptance has not yet been performed.
M16 MUST NOT activate until that acceptance is supplied.
M21 remains planned/not started.
Strict completed roadmap progress remains `15 / 20 = 75%`.

## Evidence independently reviewed

- M15 authoritative implementation prompt.
- M15 implementation log and implementation source.
- M15 strict audit that opened R54/R55.
- M15A authoritative remediation prompt.
- M15A remediation log.
- Actual branch metadata and current `H!veAI` branch HEAD.
- Commit `63b363b345274b931bafccd3d331910160149019`.
- `src-tauri/src/prompt_engine.rs`.
- `src-tauri/src/db/migrations.rs`.
- `src/PromptEnginePage.tsx`.
- `tests/m15-prompt-engine-focused.test.tsx`.

## R54 closure: durable single-use dispatch

R54 is CLOSED.

The remediation adds migration v12 with durable dispatch fields and a unique reservation index. The production dispatch path now:

1. validates provider, ACTIVE project, task ownership, prompt ownership, approval state and exact approved-body hash;
2. opens a SQLite `BEGIN IMMEDIATE` transaction;
3. atomically changes the exact approved version from `APPROVED/AVAILABLE` to `DISPATCHING/RESERVED` while persisting one unique reservation ID and exact prompt/version/hash/provider/project/task provenance;
4. requires exactly one changed row;
5. commits the reservation before calling Agent Session Center;
6. rejects replay/concurrent claims before provider start;
7. persists provider-start failure as `DISPATCH_FAILED/FAILED`;
8. finalizes session provenance and prompt dispatch provenance together inside a second `BEGIN IMMEDIATE` transaction with checked affected-row counts;
9. stops the owned session and records failure if finalization fails.

This removes the original start-before-provenance race and closes the duplicate-dispatch window identified by M15-R54.

The focused native-domain tests include concurrent claim/replay/failure/provenance fixtures and the builder reports `342/342` serialized Rust regression after remediation.

## R55 closure: bounded context materialization

R55 is CLOSED.

The generated prompt body now renders the already-bounded context manifest rather than sending only a manifest hash/count.

Production rendering includes INCLUDED values for bounded context items and only disposition/reference/reason for non-included items. Excluded values are not rendered. The implementation and remediation prompt renderers use this materialized projection, while `AUDIT_SUPPORT` now has a distinct renderer and mutation boundary.

Prompt-body bounding remains UTF-8 safe with an explicit truncation marker at the 65,536-byte boundary.

The focused tests independently show:

- included task/acceptance/dependency content appears;
- included approved source evidence appears;
- excluded sentinel content does not appear;
- omitted/excluded state is represented truthfully;
- audit-support output is distinct;
- oversized output is bounded/truncated.

This closes M15-R55.

## UI review

The existing Prompt Engine human flow remains explicit:

`Context -> Generate draft -> Review/Edit -> Approve -> Provider -> Dispatch`

The dispatch button is disabled unless the exact version is `APPROVED` and `dispatchState === AVAILABLE`. Version history surfaces approval and dispatch state without redesigning unrelated pages.

No M16 or M21 UI/runtime work was introduced.

## Verification record

Builder evidence reports:

- focused Prompt Engine Rust: 10 passed;
- migration Rust: 13 passed;
- full serialized Rust: 342 passed, 0 failed;
- frontend: 111 passed, 0 failed;
- TypeScript typecheck: PASS;
- production frontend build: PASS;
- npm high-severity audit: 0 vulnerabilities;
- Rust format/all-targets/pty-support: PASS;
- publisher rollback harness: 9/9 PASS;
- governed Tauri publication: PASS;
- stable executable SHA-256: `B88F513620CEA2670E993D71CF0BF1CE399E48AE766B0DB2D82E4C1831125ACA`;
- no visible console popup during governed publication smoke.

These are builder claims corroborated by the reviewed implementation and test sources, not a substitute for user native acceptance.

## Independent provenance correction

The M15A builder log contains an incorrect/non-resolvable line:

`Implementation commit SHA: 07c81c827cd831c17d35273821d751768ebd6300`

GitHub does not resolve that SHA.

The actual pushed `H!veAI` remediation commit is independently verified as:

`63b363b345274b931bafccd3d331910160149019`

Its parent is the authoritative M15A prompt commit:

`10324b31c8f0a06776ededf88fcefa2ae915e81a`

The branch metadata identifies `63b363b...` with commit message `fix: close M15 prompt engine remediation findings`.

Because historical builder logs are immutable evidence records, this re-audit does not rewrite that log. This audit records and supersedes the incorrect SHA claim for acceptance/provenance purposes. No production defect is opened for the logging typo.

## Native acceptance still required

The builder explicitly did not run a real M15-created native provider dispatch because it had no disposable registered project.

Therefore M15 is not yet PASS/CLOSED.

User native acceptance must prove, in the published stable H!veAI executable:

1. Prompt Engine opens normally with no unwanted console popup.
2. A safe registered project/task can generate a prompt whose body visibly contains meaningful bounded task/context evidence.
3. The generated prompt can be reviewed and explicitly approved.
4. Dispatch to Codex or Claude succeeds through Prompt Engine.
5. The resulting Agent Session Center session shows the expected final assistant response using the accepted M14E chat-first surface.
6. The session carries the exact prompt/version/hash/provider provenance.
7. The same approved version cannot be dispatched a second time.

After those native gates are accepted by the user, M15 may close and strict roadmap progress may advance to `16 / 20 = 80%`, activating M16 only then.

## Final boundary

M14: PASS/CLOSED.

M15A: TECHNICAL PASS.

M15: OPEN pending user native acceptance only.

M16-M20: blocked/planned.

M21: planned/not started.
