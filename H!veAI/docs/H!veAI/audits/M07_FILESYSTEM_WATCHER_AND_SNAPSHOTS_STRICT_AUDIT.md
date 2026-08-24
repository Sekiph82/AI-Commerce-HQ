# M07 — Filesystem Watcher and Snapshots Strict Audit

Product: H!veAI
Audit mode: Strict evidence-first governance
Verdict: FAIL — remediation required before M08

## 1. Scope

This audit evaluates the published M07 implementation against the authoritative M07 prompt, actual source, tests, TASKS state, Codex claims, and branch publication state. The Codex log is treated as a claim set, not as proof.

Audited branch: `H!veAI`
Published branch head observed during audit: `2429a575b8fdf7cf82729519e1b310ad6e7c6af6`
M07 implementation commit: `f1293db`
M07 merge/publication path included `22c97fc` and final documentation verification.

## 2. Contract summary

M07 required a Rust-owned watcher scoped to registered project roots, bounded queues, debounce/coalescing, safe overflow => `RESCAN_REQUIRED`, normalized project-relative event paths, safe missing/moved project behavior, explicit reattachment/rescan, bounded Git refresh integration, no file-content persistence, controlled startup/shutdown, and evidence-backed tests.

## 3. Acceptance criteria matrix

- AC-01 Rust-owned watcher manager: PASS
- AC-02 Registered-root scope: PARTIAL
- AC-03 No arbitrary frontend watch path: PASS at IPC boundary
- AC-04 Event normalization: PASS
- AC-05 Debounce/coalescing: PASS
- AC-06 Bounded queue/buffer: PASS
- AC-07 Overflow/error preserves safe rescan-required semantics: FAIL
- AC-08 Generated-directory exclusions: PASS
- AC-09 Missing/moved handling without registry deletion: PASS/PARTIAL
- AC-10 Repaired path can be reattached/refreshed: UNVERIFIED by direct test evidence
- AC-11 Snapshot/evidence timestamps maintained: PASS, but timestamp convention is inconsistent with earlier UTC policy and must be remediated
- AC-12 Git-relevant event triggers bounded M06 Git refresh: PARTIAL/UNVERIFIED by direct focused integration test evidence
- AC-13 Raw file contents/full diffs not persisted automatically: PASS by code inspection
- AC-14 Watcher startup/shutdown lifecycle controlled: PASS/PARTIAL
- AC-15 Restrained frontend watcher health: PASS by implementation claim/regression build; no independent visual acceptance
- AC-16 Project Registry regression: PASS by preserved suite claim, not independently re-run by auditor
- AC-17 Git Engine preserved/default-denied: PASS by source continuity
- AC-18 Runtime/database regressions: PASS by published test claim
- AC-19 No legacy sidecar: PASS by smoke claim
- AC-20 Parent application untouched: PASS by publication/containment claim
- AC-21 Historical logs unchanged: PASS by repository state claim
- AC-22 M07 log committed/pushed/verified: PASS
- AC-23 M07 docs exist: PASS by published milestone claim
- AC-24 TASKS reflects verified M07 state only: FAIL/PARTIAL because M07 is marked fully complete despite unresolved strict-audit defects
- AC-25 Canonical UI Assets governance intact: PASS governance-wise; visual fidelity remains separately unverified

## 4. Blocking / major findings

### FINDING M07-B01 — Rescan-required state can be cleared without an actual explicit rescan
Severity: BLOCKER
Confidence: HIGH

Overflow or watcher error sets `status.rescan_required = true` and marks the project degraded/overflow. However, `refresh_project_snapshot()` reads that flag, persists it, then unconditionally sets `status.rescan_required = false` after any refresh. A subsequent ordinary accepted event can therefore clear the correctness alarm without an explicit rescan/reconciliation.

This violates the M07 requirement that overflow/error produce `RESCAN_REQUIRED` rather than silently losing correctness.

Required fix:
- Preserve `rescan_required=true` until an explicit reconciliation/rescan operation succeeds.
- Ordinary debounce refresh must never clear an overflow/error rescan requirement.
- Add focused tests for overflow -> ordinary event -> still requires rescan, then explicit rescan -> clears only on success.

### FINDING M07-M01 — Root containment is not fail-closed in `relative_path()`
Severity: MAJOR
Confidence: HIGH

`relative_path()` uses `path.strip_prefix(root).unwrap_or(path)`. If a raw event path is outside the registered root, it falls back to the original path instead of rejecting it. That can convert an absolute/out-of-scope path into a frontend/internal normalized string.

This is weaker than the contract requiring project-scoped relative paths and no unnecessary raw absolute path exposure.

Required fix:
- `strip_prefix(root)` failure must return `None`/reject.
- Add Windows path/case/canonicalization-aware containment tests.
- Add rename tests where one side is outside root and ensure no out-of-scope path is emitted.

### FINDING M07-M02 — Test claims exceed direct evidence
Severity: MAJOR
Confidence: HIGH

The M07 Codex log claims direct coverage for rename support, repaired-path behavior, Git-refresh behavior, manager registry preservation and other watcher requirements. The actual visible watcher test module contains 10 tests, but does not directly prove all of those claims. In particular, there is no direct focused test proving:
- repaired-path reattachment end-to-end,
- Git-category event -> persisted Git snapshot integration,
- rescan-required persistence until explicit rescan,
- fail-closed outside-root event rejection,
- rename normalization across edge cases.

Required fix:
- Add explicit tests for every claimed behavior.
- Update M07/M07.01 log language to distinguish directly tested, code-inspected and manually smoke-tested behavior.

## 5. Cross-milestone findings carried into M07.01

The following previously identified issues remain open and are mandatory remediation scope:

1. M06 binary diff metadata-only violation: binary patch payload can remain in returned text.
2. M05 timestamp convention uses epoch-string values instead of canonical UTC/RFC3339.
3. M05 project repair repository-identity validation is too weak/fails open in some remote-missing/change cases.
4. M04 SQLite needs production hardening: WAL, busy timeout, synchronous policy, integrity strategy, backup/recovery, lock/corruption handling, pre-migration backup policy.
5. M03 module-wide `#![allow(dead_code)]` suppression must be narrowed.
6. M01/M02 production CSP still includes development localhost/WebSocket origins.
7. M01 restart full-cycle remains blocked/manual and lacks strong automated/controlled verification.
8. Canonical dashboard visual fidelity has not received a true manual/reference QA pass.
9. Development manual-QA launcher is required: stable Desktop `H!veAI.lnk` opening current validated H!veAI executable without installer or `.bat` dependency.
10. M07 timestamp helper repeats the epoch-string inconsistency and must join the timestamp remediation.

## 6. Architecture / security assessment

Positive:
- Rust-owned watcher architecture is appropriate.
- `notify` is a suitable Windows-native watcher choice.
- Bounded sync channel, debounce and per-project throttling are directionally sound.
- IPC remains project-ID based and does not expose arbitrary watch paths.
- No user file contents are automatically persisted.
- M06 mutation remains default-denied.

Risks requiring M07.01:
- correctness state can be lost after overflow,
- path containment is not strictly fail-closed,
- timestamp representation is inconsistent across persistence layers,
- evidence/test claims need stricter truthfulness.

## 7. TASKS / documentation truthfulness

`TASKS.md` marks all M07 items `[x]` and states `M07 COMPLETE`. Under strict governance that is not currently truthful because AC-07 fails and AC-10/AC-12 are not fully evidenced. M07.01 must either correct status during remediation or document that M07 closure is superseded by M07.01 strict remediation.

Historical logs should not be rewritten. Corrections must be additive in audit/remediation records.

## 8. Regression risk

Risk: MEDIUM-HIGH

The watcher is infrastructure for M08/M09. Losing `rescan_required` after dropped events could allow later task-source/task-intelligence layers to reason over stale evidence while the UI reports a healthy state. Root-containment weakness could also pollute normalized event evidence.

## 9. Audit confidence

HIGH for code-level findings M07-B01 and M07-M01.
MEDIUM-HIGH for test-evidence gaps because repository-visible tests were inspected, while the auditor did not independently execute the Windows test suite.

## 10. Final verdict

FAIL — M07.01 remediation is required before M08.

M07 implementation is substantial and directionally sound, but strict closure is denied because a correctness-critical overflow/rescan invariant is violated and root containment is not fail-closed.

Next authorized work:
`M07.01 — Cross-Milestone Remediation, Hardening & Manual QA Readiness`

Do not start M08 until M07.01 receives a strict PASS.
