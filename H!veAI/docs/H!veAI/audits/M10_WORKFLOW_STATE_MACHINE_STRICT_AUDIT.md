# M10 Workflow State Machine — Independent Strict Audit

Date: 2026-08-26
Branch: `H!veAI`
Prompt base commit: `baa8c000f7405a0aae6bd3353b1c7dcc5bed63fe`
Builder implementation commit: `b4ca040ac9df94e2f2d7f13eaaa77f644327f5ae`
Builder evidence/log commit / audited branch HEAD before this audit: `d0ce598588f00d9e919a4865fd0d2371272e6334`

## VERDICT

**FAIL**

- BLOCKER: 0
- MAJOR: 5
- MINOR: 5
- Confidence: HIGH
- Regression risk: MEDIUM-HIGH until the read-model/recovery/override defects are corrected.
- M10 remains **NOT CLOSED**.
- M11/M12 remain blocked.
- Strict completed roadmap count remains **10 / 20 = 50%**.

The implementation is substantial and the core transactional state machine, M09 ownership integration, idempotency, evidence ownership checks, narrow IPC/ACL, and no-UI scope are largely correct. The milestone cannot close because several production behaviors conflict with the explicit M10 contract and would feed incorrect workflow truth into M11/M12.

---

## 1. CONTRACT RECOVERY

M10 was required to provide:

1. canonical finite workflow states and actors;
2. exact happy-path and audit-remediation transition matrix;
3. bounded, validated evidence gates;
4. atomic stale-client-safe state/event transitions;
5. deterministic request idempotency;
6. explicit HUMAN override with durable decision/event history;
7. restart recovery for stale RUNNING states;
8. M09/M10 ownership integration preserving workflow state/history across reparse/stale/reappearance;
9. narrow native IPC/permissions and typed TypeScript wrappers;
10. a truthful read model for M11/M12, including latest event and allowed actor/state information;
11. direct tests for the required production behaviors;
12. no M11/M12 UI, no dashboard manifest runtime ingestion, no agent/prompt/audit-engine expansion.

The prompt explicitly states that RUNNING states must not survive restart as if work were active, that SYSTEM is bounded to bootstrap/recovery/internal bookkeeping, that AUDIT_FAILED requires real follow-up audit evidence, that suspension states have deterministic resume truth, and that the read model must expose the latest workflow event and transition actor truth.

---

## 2. BRANCH / HEAD / DIFF SCOPE

Compared:

`baa8c000f7405a0aae6bd3353b1c7dcc5bed63fe..d0ce598588f00d9e919a4865fd0d2371272e6334`

The branch is two commits ahead of the prompt base and the changed-file scope is bounded to M10 implementation/contracts, M09 integration, permissions/capability, tracking docs, and builder log.

Production scope inspected:

- `H!veAI/src-tauri/src/workflow.rs`
- `H!veAI/src-tauri/src/task_intelligence.rs`
- `H!veAI/src-tauri/src/lib.rs`
- `H!veAI/src-tauri/permissions/foundation.toml`
- `H!veAI/src-tauri/capabilities/default.json`
- `H!veAI/src/workflow.ts`
- `H!veAI/tests/workflow-contract.test.ts`
- `H!veAI/TASKS.md`
- builder log and milestone prompt

No visible M11/M12 UI implementation or Project Dashboard runtime ingestion was introduced.

---

## 3. ACCEPTANCE CRITERIA MATRIX

| Area | Result | Audit note |
|---|---|---|
| Canonical state enum/string set | PASS | Exact finite state set exists. |
| Canonical actor enum/string set | PASS | Exact actor strings exist. |
| Happy-path transition matrix | PASS | Production matrix follows required sequence. |
| Audit failure/re-audit routing | PASS | Prior failed audit routes later implementation to RE_AUDIT_REQUIRED. |
| Direct jump rejection | PASS | Matrix violations return structured error. |
| Evidence ownership validation | PASS | Table-backed refs are checked for same task/project where applicable. |
| Prompt approval / builder / audit / test gates | PARTIAL | Core gates exist; audit-failure acceptance is too broad. See R05. |
| Atomic transition/state update | PASS | Event + state update occur in one SQLite transaction. |
| Expected-state stale-client guard | PASS | Current DB state checked inside transaction. |
| Request idempotency | PASS | Deterministic task/request identity + semantic fingerprint exists. |
| HUMAN override persistence | PARTIAL | Decision/event atomicity exists, but override into suspension can create an unreadable/unresumable task. See R03. |
| Suspension/resume semantics | PARTIAL | Normal suspension path is strong; override suspension path is broken and blocker actor extension is incomplete. |
| Restart recovery | PARTIAL | Active-project running tasks are recovered, but archived-project running tasks are excluded. See R04. |
| M09 reparse state preservation | PASS | Managed task state is preserved. |
| M09 stale task history preservation | PASS | Managed stale task survives and is source-retired. |
| M09 reappearance preservation | PASS | Stable ID/state/created_at/history preservation is implemented. |
| M09 no-history stale cleanup | PASS | Existing deletion behavior preserved. |
| Bounded history/list IPC | PASS | 1..500 validation exists. |
| Workflow read model | FAIL | latestEvent and allowedActors are not truthful. See R01/R02. |
| Narrow IPC / ACL | PASS | Dedicated commands and permission only. |
| TypeScript contract | PASS/PARTIAL | Commands/enums/limits are represented; read-model defects originate in native source. |
| Required direct evidence | PARTIAL | Several named tests are weaker than the prompt/log claim. See E01-E04. |
| No M11/M12/dashboard runtime scope creep | PASS | Scope boundary preserved. |
| Canonical UI/native hotfix preservation | PASS by diff scope | No visible UI/audio/terminal source change in M10 diff. |
| Final local/origin equality | UNVERIFIED locally | Remote HEAD is visible; local equality is asserted using `SELF` placeholders in builder log. See E05. |

---

# 4. BUILDER CLAIMS VS REPOSITORY TRUTH

The builder log correctly describes many implemented contracts, including the exact state set, M09 ownership preservation, SQLite transaction usage, request idempotency, ACL, TypeScript wrappers, no UI work, and publication claims.

However, these claims overstate closure in several areas:

- the read model does not actually return the latest workflow event;
- `allowedActors` is not derived from the same enforcement logic as transitions;
- override-to-suspension is not resumable;
- recovery does not cover archived transient tasks;
- audit-failure evidence accepts any non-PASS result;
- the restart test does not cover all three transient states despite the log saying it does;
- the deterministic history-order test does not verify ordering.

Builder regression/publication command results remain claims. No repository contradiction was found for the stated build counts, but passing suites do not override the direct production defects below.

---

# 5. FILE / SYMBOL EVIDENCE

## R01 — MAJOR — `latestEvent` returns the oldest event, not the latest

Production `history_tx()` orders workflow events:

```sql
ORDER BY occurred_at ASC, id ASC LIMIT ?2
```

`task_read()` then calls:

```rust
let events = history_tx(tx, task_id, 1)?;
...
latest_event: events.into_iter().next(),
```

With more than one workflow event, this returns the earliest event. The prompt explicitly requires the read model to expose the **latest workflow event summary/time**. M11/M12 would therefore receive stale historical truth for current-project cards, recent state context, recovery context, and attention surfaces.

Required fix:

- implement a dedicated latest-event query using `ORDER BY occurred_at DESC, id DESC LIMIT 1`, or make a helper whose ordering is explicit;
- keep chronological history output deterministic separately;
- add a direct test with at least three committed workflow events proving `latestEvent` equals the final event.

## R02 — MAJOR — Actor enforcement and read-model actor truth drift apart

`validate_actor()` restricts only selected transition targets/resume states. `task_read()` separately hardcodes a broad `allowed_actors` list for every ordinary state:

```text
HUMAN, CODEX, CLAUDE, GPT_AUDIT, CI, EXTERNAL
```

This is incorrect for states such as:

- `READY_FOR_IMPLEMENTATION -> BUILDER_RUNNING`, where only CODEX/CLAUDE are accepted;
- `AUDIT_REQUIRED/RE_AUDIT_REQUIRED -> AUDIT_RUNNING`, where only GPT_AUDIT/CI are accepted;
- `VERIFY_REQUIRED -> VERIFY_RUNNING`, where only CI is accepted.

At the same time the public transition API can accept `SYSTEM` for ordinary transitions not covered by the three special target checks, even though the prompt limits SYSTEM to bounded bootstrap/recovery/internal bookkeeping. The read model omits SYSTEM in ordinary states while the mutation layer may accept it, so the two contracts disagree in both directions.

Required fix:

- create one central actor-policy function used by both mutation validation and read-model generation;
- explicitly encode the bounded SYSTEM allowlist rather than treating SYSTEM as a generic caller-provided actor;
- return only actors that the mutation path would actually accept for the currently allowed normal transition(s);
- add direct tests that compare read-model actors to real mutation acceptance for builder, audit, verify, suspension resume, and at least one SYSTEM-negative case.

## R03 — MAJOR — HUMAN override into a suspension state can make the task unreadable/unresumable

Normal suspension transitions persist `suspendedState` and `resumeState`. `override_state()` does not.

An override such as:

```text
BACKLOG -> WAITING_HUMAN
```

writes a `WORKFLOW_OVERRIDE` event whose evidence JSON contains request/decision/rationale/evidenceRefs but no deterministic resume state. Immediately afterwards `task_get()` sees a suspension state and calls `resume_state()`. Because M10 history exists but the override event has no `resumeState`, the read can fail with `WORKFLOW_RESUME_MISSING`. A normal resume transition from that state fails for the same reason.

This violates the explicit human-override and suspension/read-model contracts.

Required fix:

- when an override target is a suspension state, persist deterministic `suspendedState` + `resumeState` using the same safe-running-state semantics as normal suspension;
- for hold-to-hold override, preserve the existing deterministic resume target rather than resuming into another hold state;
- enforce WAITING_EXTERNAL external-reference semantics on override if it is used as a suspension target;
- prove task_get works immediately after the override and the task can normally resume to the stored target;
- preserve atomic decision + event + state update.

## R04 — MAJOR — Restart recovery excludes archived projects, allowing stale RUNNING truth to survive restart

`recover_stale()` selects candidates with:

```sql
WHERE p.status='ACTIVE'
  AND t.state IN ('BUILDER_RUNNING','AUDIT_RUNNING','VERIFY_RUNNING')
```

The M10 contract says RUNNING states are transient and must not survive a native restart as if external work were still alive. Read/history is intentionally allowed for archived projects. If a workflow-managed task is archived while in a RUNNING state, or the project is archived before the next startup, this query leaves the task in that RUNNING state indefinitely. The app can then read and display a false active-execution claim after restart.

Recovery is internal truth repair and does not require the project to be mutable or its filesystem root to be present.

Required fix:

- recover workflow-managed transient states regardless of ACTIVE/ARCHIVED lifecycle status and regardless of missing project path;
- keep normal user mutation rules ACTIVE-only;
- add direct recovery tests for BUILDER_RUNNING, AUDIT_RUNNING, VERIFY_RUNNING and an archived project; also cover a missing-path ACTIVE project if practical;
- preserve idempotent second pass.

## R05 — MAJOR — `AUDIT_RUNNING -> AUDIT_FAILED` accepts any non-PASS audit result

Production gate logic computes:

```rust
let pass = result.eq_ignore_ascii_case("PASS");
if (to == WorkflowState::AuditPassed) != pass { ... }
```

Therefore every value other than PASS, including unknown, pending, malformed, or non-final audit result strings, can satisfy `AUDIT_FAILED`.

The contract requires a real result that requires follow-up and specifically operates under the H!veAI PASS / CONDITIONAL / FAIL audit model. An unknown/pending value must not be converted into an operational failed-audit event.

Required fix:

- accept `PASS` only for AUDIT_PASSED;
- accept final follow-up values `FAIL` or `CONDITIONAL` for AUDIT_FAILED using case-insensitive canonical comparison;
- reject all other/unknown/non-final results with `WORKFLOW_EVIDENCE_INCOMPATIBLE`;
- add direct FAIL, CONDITIONAL, PASS-negative, and unknown/PENDING-negative tests.

---

# 6. FOCUSED TEST EVIDENCE

The production suite contains many strong direct tests for matrix behavior, idempotency, stale expected state, ownership, normal suspension, override atomicity, M09 state/history preservation, and no-history cleanup.

The following evidence gaps remain.

## E01 — MINOR — deterministic history-order test does not test order

`m10_history_is_bounded_and_deterministically_ordered` asserts only `len() == 1`. It does not verify event order, tie-break behavior, or latest-event correctness.

## E02 — MINOR — restart-recovery test covers only BUILDER_RUNNING

The prompt requires builder/audit/verify transient recovery coverage. The named test seeds only BUILDER_RUNNING and checks one recovery plus idempotent second pass. Production source has branches for all three, but direct evidence is incomplete.

## E03 — MINOR — M09 stale/reappearance tests under-assert explicit contract steps

The stale test checks SQL survival/sourceActive/event count but does not directly assert that `task_intelligence::list()` excludes the retired task. The reappearance test does not directly assert sourceActive=true after reactivation. The reparse test does not directly assert the changed parser title/metadata alongside state preservation.

## E04 — MINOR — lifecycle test name says archived or missing, body covers archived only

Production `ensure_project_mutable()` checks path existence, but the direct test does not exercise a missing registered root.

## E05 — MINOR — final equality log uses `SELF` placeholders

The remote branch is visible at `d0ce598588f00d9e919a4865fd0d2371272e6334`, but the log records final local HEAD/origin HEAD as `SELF`. GitHub proves the pushed remote state, not the local checkout equality after the final push. Future logs should record the concrete final SHA or an exact command output when possible.

---

# 7. REGRESSION EVIDENCE

Builder log claims:

- frontend 79 tests PASS;
- Rust 216 tests PASS;
- focused M10 Rust 24 tests PASS;
- source discovery/task intelligence/watcher/Git/database subsets PASS;
- typecheck/build/npm audit/cargo fmt/check/build PASS;
- publisher failure harness and governed no-bundle publication PASS;
- canonical background/video hashes unchanged;
- X01/X02 source fixes preserved;
- no installer.

These claims are plausible and consistent with the diff. They do not close the five direct production defects.

---

# 8. SECURITY / SAFETY REVIEW

Positive findings:

- no unrestricted shell/network/process capability added;
- workflow IPC surface is narrow;
- frontend-provided expected state is checked against DB state inside the transaction;
- evidence rows are ownership-checked;
- request/summary/evidence bounds exist;
- no raw SQL/event insertion exposed to frontend;
- M09 workflow-history preservation avoids cascade deletion of M10 events;
- human override decision/event/state updates are transactional.

Open safety concern:

- actor policy must be centralized and SYSTEM must not be broadly forgeable as a normal actor beyond explicit internal bookkeeping transitions (R02).

---

# 9. ARCHITECTURE CONSISTENCY

PASS on major architectural boundaries:

- existing `tasks.state` and `task_events` are reused;
- no parallel workflow table;
- M09 remains parser/source authority;
- M10 is operational state authority;
- no M11/M12/UI/dashboard runtime expansion;
- stable M09 task IDs remain event anchors;
- M10 restart recovery is native Rust.

Read-model correctness is not sufficient yet for downstream M11/M12 consumption because of R01/R02/R03/R04.

---

# 10. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

Current tracker correctly leaves independent strict audit unchecked and does not mark M10 PASS/CLOSED. However, M10.02/M10.04/M10.05/M10.07/M10.09 checkboxes are too optimistic after this audit because actor truth, audit-failure evidence, override suspension, restart recovery, and direct evidence remain incomplete.

Historical builder log must remain immutable. Corrective truth belongs in this audit, M10A remediation prompt/log, and prospective tracker updates.

---

# 11. FINAL REPOSITORY STATE

Audited remote branch HEAD before audit publication:

`d0ce598588f00d9e919a4865fd0d2371272e6334`

Implementation commit:

`b4ca040ac9df94e2f2d7f13eaaa77f644327f5ae`

No force-push/history rewrite evidence was observed. Parent untracked local user files are not visible from GitHub and remain outside this audit's remote proof.

---

# 12. OPEN CROSS-MILESTONE FINDINGS

- Pre-M10 terminal-popup defect: CLOSED.
- Pre-M10 startup-audio/replay defect: CLOSED.
- M09 final parser closure: remains accepted; M10's narrow M09 ownership integration is substantially correct.
- Project Dashboard manifest runtime ingestion: still correctly reserved for M11/M12.

No earlier milestone is reopened by this audit.

---

# 13. DEFECTS BY SEVERITY

### BLOCKER

None.

### MAJOR

- R01: latestEvent returns oldest workflow event.
- R02: actor enforcement/read model drift; SYSTEM policy not bounded centrally.
- R03: override into suspension can become unreadable/unresumable.
- R04: archived transient tasks are skipped by restart recovery.
- R05: AUDIT_FAILED accepts arbitrary non-PASS audit results.

### MINOR

- E01: history order test does not assert order.
- E02: recovery test does not cover audit/verify.
- E03: M09 integration tests under-assert active-list/reactivation/parser-update steps.
- E04: missing-path lifecycle mutation test absent.
- E05: final local/origin equality uses non-concrete `SELF` placeholders.

---

# 14. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

Non-blocking after the above fixes:

- consider a dedicated helper for `latest_workflow_event()` rather than overloading chronological-history logic;
- centralize transition policy (next states + actors + evidence gate descriptor) to reduce M10/M11 drift;
- consider explicit read-model indication when the latest event is `WORKFLOW_RECOVERY`, so M11 can surface restart-recovery attention without inference;
- consider recovery behavior when more than the current bounded candidate batch exists; do not silently leave false RUNNING truth if scale exceeds the selected bound.

---

# 15. UNVERIFIED ITEMS

- actual local checkout equality after the builder's final push;
- builder-reported command execution counts beyond repository-source consistency;
- Windows runtime publication smoke behavior was not re-run by the independent auditor in this GitHub-only audit.

No new manual UI acceptance is required for M10 because visible UI was not changed.

---

# 16. REGRESSION RISK

**MEDIUM-HIGH** until remediation.

The core write path is mostly sound, but the defects affect the exact operational truth that M11/M12 would consume: latest action, actors, suspension resume, restart liveness, and final audit result classification.

---

# 17. AUDIT CONFIDENCE

**HIGH**.

The findings are based on direct source/test inspection and explicit prompt requirements. R01, R03, R04, and R05 are deterministic from the current production code. R02 is directly observable by comparing `validate_actor()` with `task_read()` and the SYSTEM contract.

---

# 18. FINAL VERDICT

**FAIL**

M10 must not be marked PASS/CLOSED and M11/M12 must remain blocked. A bounded M10A remediation should correct R01-R05 and tighten E01-E05 without redesigning the state machine or starting later milestones.

---

# 19. REQUIRED REMEDIATION

Create one bounded M10A run that:

1. fixes truthful latest-event selection;
2. centralizes actor policy and read-model actor derivation, including bounded SYSTEM policy;
3. makes override-to-suspension deterministic/resumable and enforces external-wait evidence;
4. recovers transient workflow state across archived/missing project lifecycle without enabling archived user mutations;
5. restricts audit-failed evidence to canonical final follow-up results;
6. strengthens the named direct tests and M09 integration assertions;
7. preserves M09 state/history ownership integration, idempotency, atomicity, ACL, no-UI scope, X01/X02, assets, launcher, and no-installer policy;
8. runs full regression/publication and records concrete final remote/local equality evidence;
9. stops for independent re-audit before M11/M12.
