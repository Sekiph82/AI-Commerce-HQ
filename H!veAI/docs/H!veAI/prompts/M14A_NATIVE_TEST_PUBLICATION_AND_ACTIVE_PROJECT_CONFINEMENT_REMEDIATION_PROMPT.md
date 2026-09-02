# M14A Native Test, Publication, and ACTIVE Project Confinement Remediation Prompt

Date: 2026-09-02
Product: H!veAI
Branch: `H!veAI`
Milestone: M14
Scope: close M14-R35, M14-R36, M14-R37 only

## Authority

This prompt is the authoritative bounded remediation for the independent audit:

`docs/H!veAI/audits/M14_AGENT_SESSION_CENTER_CODEX_CLAUDE_IMPLEMENTATION_STRICT_AUDIT.md`

Do not broaden scope. Preserve all accepted M13 boundaries and all M14 implementation areas not implicated by R35-R37. Do not start M15 or M21.

## Required preflight

1. `git fetch origin H!veAI`.
2. Synchronize with `origin/H!veAI` using fast-forward only.
3. Confirm exact branch `H!veAI`.
4. Preserve unrelated user files, especially untracked `start-demo.bat` and `task.md`.
5. Read the M14 prompt, M14 builder log, and M14 strict audit in full before editing.
6. Keep M13 PASS/CLOSED.
7. Keep M14 open at `14 / 20 = 70%` pending remediation, re-audit, and user native acceptance.

## R35: restore executable native Rust test gates

The M14 builder reported `0xc0000139 STATUS_ENTRYPOINT_NOT_FOUND` before Rust test harness entry, including on the pre-M14 baseline. Do not waive this.

You must:

- reproduce the loader failure with the exact failing test executable
- identify the missing/incorrect runtime dependency, DLL/toolchain/loader cause, or other exact host cause
- prove whether the failure is repository/toolchain/environment specific
- fix or isolate it in a governed way that allows the required native test executables to launch
- do not weaken tests, skip runtime execution, or replace execution with `--no-run`
- do not add unsafe DLL search/path hacks that weaken normal app security
- document exact loader evidence and remediation

After the fix, actually run and PASS:

- focused M13 Codex backend tests
- focused M14 Agent Session Center Rust tests
- Claude resolver/start/status/stop/recovery fixtures
- PTY lifecycle/resize fixtures where enabled by M14
- provider/project/task/session confinement adversarial tests
- redaction/durable-event tests
- retry provenance tests
- restart/orphan recovery tests
- full Rust library regression serially

Record exact pass/fail counts.

## R36: fix native candidate readiness and complete governed publication

The M14 candidate built but failed the publisher readiness smoke because no fresh `HIVEAI_FRONTEND_READY` marker was observed within 15 seconds. Stable bytes therefore remained M13E.

You must:

- reproduce the fresh candidate launch outside and inside the publisher smoke path
- determine whether the failure is startup crash, frontend load failure, permission/capability failure, migration/startup deadlock, logging/readiness race, window lifecycle issue, PTY/native dependency issue, or another exact cause
- inspect native logs and process lifetime rather than increasing the timeout blindly
- fix the real root cause
- preserve startup video/audio/icon behavior
- preserve no-visible-console behavior
- do not bypass the readiness marker or publisher smoke

Then rerun and PASS:

- publisher failure/rollback harness
- production Tauri `--no-bundle` build
- candidate PE validation
- fresh candidate frontend readiness marker
- candidate no-forbidden-dev-port check
- candidate no-visible-console check
- stable swap
- stable frontend readiness smoke
- candidate/stable SHA-256 equality
- desktop shortcut target validation
- desktop shortcut icon validation
- cleanup/no-candidate-left-behind checks

The normal user executable `H!veAI/dev-bin/H!veAI.exe` must contain the M14 implementation before this remediation can be considered complete.

## R37: restore exact ACTIVE registered-project confinement

The new provider-neutral M14 center currently rejects only `ARCHIVED` projects before path validation. This is weaker than the accepted M13 boundary.

You must:

- require project status exactly `ACTIVE` before any Codex or Claude operation starts or retries
- canonicalize and use the registered project root
- reject `MISSING`, `ARCHIVED`, unknown/unregistered, and unavailable/non-directory project roots
- preserve exact task ownership checks
- reject cross-project task IDs
- reject cross-project session stop/retry/permission/status mutation attempts
- do not add an arbitrary cwd/worktree escape
- keep frontend free of raw executable paths, PIDs, shell names, and arbitrary argument vectors

Add direct adversarial tests for all cases above.

## Preserve accepted provider behavior

Preserve:

- Codex M13 adapter behavior and safe fixed invocation
- Claude direct native resolver/readiness
- Claude bounded stdin and fixed arguments
- stateful pre-persistence redaction
- durable event/session truth
- no arbitrary shell/executable/argument primitive
- owned process lifecycle and bounded stop/escalation
- provider-neutral session center API
- shared vertical session reader
- Git Engine authority for changed files/diffs
- provider preference persistence
- truthful Claude auth unknown state
- truthful unsupported resume where not safely supported
- no visible console windows

Do not hardcode behavior by project name at runtime.

## Explicit execution gates

Run and record every gate individually:

1. Fetch + ff-only sync.
2. Exact branch/worktree verification.
3. Audit/prompt/log read proof.
4. Reproduce `0xc0000139` loader failure before fix.
5. Capture exact loader/root-cause evidence.
6. Prove repaired Rust test executable launches.
7. Focused M13 Codex backend tests PASS.
8. Focused M14 Rust tests PASS.
9. Claude resolver/lifecycle fixture tests PASS.
10. PTY lifecycle/resize tests PASS where compiled/enabled.
11. ACTIVE/MISSING/ARCHIVED/unknown project adversarial tests PASS.
12. Cross-project task/session confinement tests PASS.
13. Redaction/durable persistence tests PASS.
14. Retry provenance tests PASS.
15. Restart/orphan recovery tests PASS.
16. Full Rust library regression serially PASS.
17. Focused M14 frontend tests PASS.
18. Full frontend tests PASS.
19. TypeScript typecheck PASS.
20. Frontend production build PASS.
21. `npm audit --audit-level=high` PASS.
22. `cargo fmt --all -- --check` PASS.
23. `cargo check --all-targets` PASS.
24. `git diff --check` PASS.
25. Reproduce candidate readiness failure before fix if still reproducible.
26. Prove root cause of missing readiness marker.
27. Publisher failure/rollback harness PASS.
28. Governed Tauri `--no-bundle` publication PASS.
29. Candidate readiness marker PASS.
30. Stable readiness marker PASS.
31. Candidate/stable SHA equality PASS.
32. Shortcut target/icon PASS.
33. No visible console/native post-startup smoke PASS.
34. No forbidden dev port PASS.
35. Real Codex readiness PASS.
36. Real Claude readiness/version PASS.
37. If safely feasible without side effects, harmless real Claude operation against a disposable or explicitly safe registered ACTIVE fixture; otherwise leave only this one gate pending user acceptance and explain exactly why.
38. Confirm M15-M20 not activated.
39. Confirm M21 not started.
40. Confirm roadmap remains `14 / 20 = 70%` until independent re-audit and user acceptance.

## Required tracking and log

Update only current M14/M14A tracking surfaces truthfully.

Create immutable remediation log:

`H!veAI/docs/H!veAI/codex-logs/M14A_NATIVE_TEST_PUBLICATION_AND_ACTIVE_PROJECT_CONFINEMENT_REMEDIATION_LOG.md`

The log must include:

- exact `0xc0000139` root cause and fix
- exact publisher readiness root cause and fix
- ACTIVE confinement source/test evidence
- all 40 gate results individually
- exact implementation commit SHA
- exact stable/candidate SHA-256 after successful publication
- confirmation stable `H!veAI/dev-bin/H!veAI.exe` contains M14A bytes
- confirmation M15/M21 untouched

Final builder state must be exactly:

`M14A REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

Stop after M14A. Do not activate M15.