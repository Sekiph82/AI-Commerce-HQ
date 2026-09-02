# M14 Agent Session Center Codex + Claude Implementation Strict Audit

Date: 2026-09-02
Product: H!veAI
Branch: `H!veAI`
Implementation commit audited: `d8ed475d44e39102abc20523ff8e73e7a80727e9`
Builder log: `docs/H!veAI/codex-logs/M14_AGENT_SESSION_CENTER_CODEX_CLAUDE_IMPLEMENTATION_LOG.md`

## Verdict

**FAIL / M14 NOT CLOSED**

- BLOCKER: 0
- MAJOR: 3
- MINOR: 0
- Confidence: HIGH

M13 closure is accepted from the prior independent strict/native evidence and user acceptance. M14 implementation exists, but the required native validation/publication chain is incomplete and one M13 confinement boundary regressed in the new provider-neutral center. M14 must remain open. M15-M20 remain blocked. M21 remains not started.

## Accepted implementation areas

The implementation materially adds the intended M14 foundation:

- provider-neutral Codex + Claude session center
- Claude native discovery/readiness and fixed direct invocation
- bounded stdin/output, pre-persistence redaction, durable session/event evidence
- shared Agents surface and persisted-session reader
- provider preference persistence
- no arbitrary executable/argument/PID surface exposed to frontend
- Codex M13 adapter preserved as a provider
- Git snapshot/diff authority remains separate from agent prose
- retry/recovery/permission limitation surfaces are present
- M15/M21 were not started

These areas should be preserved during remediation.

## Findings

### M14-R35 MAJOR: Required native Rust execution gates did not run successfully

The builder log explicitly records that focused M13 backend tests, focused M14 Rust tests, PTY runtime tests, redaction/recovery runtime tests, and the full Rust library regression were blocked before the test harness by Windows exit `0xc0000139 STATUS_ENTRYPOINT_NOT_FOUND`.

A compile-only/no-run result is not equivalent to the prompt's required runtime test execution. The prompt required actual provider/session/process lifecycle and regression gates, especially because M14 introduces a new native session center, Claude process ownership, optional PTY support, stop/retry/recovery behavior, and migration changes.

The builder states the loader failure also reproduces on the pre-M14 baseline. That is useful root-cause evidence, but it does not convert the blocked required gates into PASS.

**Closure requirement:** diagnose and repair or isolate the host/toolchain loader problem sufficiently to execute the required focused and full native Rust test suites. Record actual runtime pass counts. Do not waive these gates merely because `cargo check` and `cargo test --no-run` succeed.

### M14-R36 MAJOR: Governed production publication did not complete; stable H!veAI.exe still contains M13E bytes

The builder log explicitly says governed publication built a candidate but the readiness smoke did not observe a fresh `HIVEAI_FRONTEND_READY` marker within 15 seconds. The publisher therefore did not swap stable bytes.

Candidate SHA-256:
`9C23825366AAB43810AE3EB92809EEBF808F53E7CBC745657A59DD7A5F424AF8`

Stable SHA-256 remained:
`00D9B76684E02492B6994A4901A275CD1DABD3CB47E6EB598B05C7B695904492`

Therefore the user's normal `H!veAI/dev-bin/H!veAI.exe` is not the audited M14 implementation. Native user acceptance cannot truthfully begin from the stable executable yet.

**Closure requirement:** determine why the fresh M14 candidate stays alive without emitting the readiness marker, fix the native startup/readiness regression, rerun publisher failure/rollback harness, complete governed `--no-bundle` publication, prove candidate/stable SHA equality, prove shortcut/icon target integrity, and prove zero forbidden console flash.

### M14-R37 MAJOR: New provider-neutral project validation weakens the accepted M13 ACTIVE-project confinement

The new `agent_session_center.rs` validates operation projects with:

`if project.status == "ARCHIVED" { ... }`

and otherwise permits the project if `normalized_path` resolves to a directory.

The accepted M13 adapter boundary required the registered project to be `ACTIVE`. M14's authoritative prompt required all accepted M13 security/process boundaries to be preserved. Rejecting only `ARCHIVED` is a weaker contract and can permit non-ACTIVE registry states if the path still exists.

**Closure requirement:** restore exact ACTIVE-project confinement for both Codex and Claude start/retry/session mutation paths, use canonical registered root validation, and add direct adversarial tests for `MISSING`, `ARCHIVED`, unknown, cross-project task, and cross-project session cases.

## Important non-finding / pending acceptance

The builder did not launch a real Claude operation because it may contact the external provider and require authentication/billing. That is acceptable as a builder limitation only if the native implementation is successfully published first and the user then performs the explicit harmless Claude acceptance test. This remains pending and is not itself a MAJOR defect.

## Required remediation state

Create a bounded M14A remediation that closes R35-R37 only. Preserve all accepted M13/M14 implementation boundaries that are not implicated by these findings. Do not start M15 or M21.

After remediation, M14 must remain `IMPLEMENTATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE` until an independent re-audit passes and the user verifies both provider surfaces in the published native executable.
