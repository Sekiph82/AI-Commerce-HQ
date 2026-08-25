# M10A — Workflow State Machine Strict Closure

## Mission

Close the five MAJOR findings and five evidence gaps from:

`H!veAI/docs/H!veAI/audits/M10_WORKFLOW_STATE_MACHINE_STRICT_AUDIT.md`

This is a bounded remediation of M10 only.

Do not redesign the workflow architecture. Do not start M11/M12. Do not implement Project Dashboard runtime ingestion. Do not change visible UI. Do not create an installer.

---

## Preflight

Work from:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Run:

```powershell
git fetch origin H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
```

Fast-forward only if safe:

```powershell
git merge --ff-only origin/H!veAI
```

Read:

1. `H!veAI/AGENTS.md`
2. `H!veAI/CONSTITUTION.md`
3. `H!veAI/ARCHITECTURE.md`
4. `H!veAI/TASKS.md`
5. `H!veAI/docs/H!veAI/prompts/M10_WORKFLOW_STATE_MACHINE_PROMPT.md`
6. `H!veAI/docs/H!veAI/audits/M10_WORKFLOW_STATE_MACHINE_STRICT_AUDIT.md`
7. current `workflow.rs`, `task_intelligence.rs`, TypeScript contract, permissions/capability

Preserve user-owned untracked `start-demo.bat` and `task.md` if still present.

---

# Canonical UI Assets

M10A must not visually change H!veAI.

Canonical user-owned asset root:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Preserve accepted logo/background/startup-video/launcher behavior.

Required unchanged hashes:

- background: `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`
- opening video: `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`

Preserve X01 terminal-popup suppression and X02 audible startup intro/replay behavior.

No visible M10A UI work.

---

# Fix R01 — latest workflow event must actually be latest

Current defect:

`task_read()` calls chronological `history_tx(..., 1)`, whose SQL orders ascending, so `latestEvent` is the oldest event.

Required production behavior:

- keep public history chronological/deterministic;
- add/use a dedicated latest-event query ordered:

```sql
ORDER BY occurred_at DESC, id DESC LIMIT 1
```

- `WorkflowTask.latestEvent` must equal the newest workflow event.

Required direct test:

`m10_latest_event_is_truly_latest`

Create at least three committed workflow events with deterministic timestamps/order and prove the returned `latestEvent.id`, `toState`, summary and occurredAt correspond to the final event.

PASS only if the pre-fix code fails this test.

---

# Fix R02 — one actor policy for mutation and read model

Current defect:

- mutation actor checks and `WorkflowTask.allowedActors` are separate hardcoded policies;
- read model reports actors that real mutation rejects;
- SYSTEM can be supplied for ordinary transitions beyond a clearly bounded internal policy.

Required production behavior:

Create one central actor-policy helper used by both:

1. transition validation;
2. read-model `allowedActors` derivation.

At minimum preserve these exact enforced policies:

- target `BUILDER_RUNNING` -> CODEX or CLAUDE only;
- target `AUDIT_RUNNING` -> GPT_AUDIT or CI only;
- target `VERIFY_RUNNING` -> CI only;
- resume from `WAITING_HUMAN` -> HUMAN only;
- resume from `DESIGN_GATE` -> HUMAN only;
- resume from `WAITING_EXTERNAL` -> EXTERNAL or HUMAN;
- resume from `BLOCKED` -> HUMAN, SYSTEM, plus any explicitly and safely evidenced blocker actor if the existing source model can prove one without trusting arbitrary frontend text.

SYSTEM rules:

- SYSTEM must not be a generic caller actor;
- allow SYSTEM only for explicit internal bookkeeping transitions that you enumerate in code and document in the M10A log;
- recovery continues to use SYSTEM internally;
- a frontend request claiming SYSTEM for a non-allowlisted semantic transition must fail with `WORKFLOW_ACTOR_NOT_ALLOWED`.

Do not create an authentication subsystem in M10A.

Required direct tests:

- `m10_read_model_actor_policy_matches_builder_transition`
- `m10_read_model_actor_policy_matches_audit_transition`
- `m10_read_model_actor_policy_matches_verify_transition`
- `m10_system_actor_is_rejected_outside_internal_allowlist`
- retain existing suspension actor tests and add a direct read-model assertion for their allowed actors.

PASS only if read-model actors exactly match what the production transition validator would accept for the current normal transition/resume.

---

# Fix R03 — override into suspension must remain readable and resumable

Current defect:

Normal suspension events persist `suspendedState`/`resumeState`; `WORKFLOW_OVERRIDE` does not. An override into `WAITING_HUMAN`, `WAITING_EXTERNAL`, `BLOCKED`, or `DESIGN_GATE` can therefore leave `task_get()` and normal resume with `WORKFLOW_RESUME_MISSING`.

Required production behavior:

When HUMAN override targets a suspension state:

- persist `suspendedState`;
- persist deterministic `resumeState`;
- if prior state was `BUILDER_RUNNING`, resume target = `READY_FOR_IMPLEMENTATION`;
- if prior state was `AUDIT_RUNNING`, resume target = `AUDIT_REQUIRED` or `RE_AUDIT_REQUIRED` based only on real M10 workflow history;
- if prior state was `VERIFY_RUNNING`, resume target = `VERIFY_REQUIRED`;
- otherwise resume target = prior state;
- if overriding hold -> hold, carry forward the existing deterministic resume target rather than using another hold state as the target;
- `WAITING_EXTERNAL` override must require a bounded `EXTERNAL_REFERENCE` locator;
- decision + override event + state update remain atomic;
- actor remains HUMAN.

Required direct tests:

- `m10_override_to_waiting_human_is_readable_and_resumable`
- `m10_override_running_to_suspension_uses_safe_resume_prerequisite`
- `m10_override_hold_to_hold_preserves_original_resume_target`
- `m10_override_waiting_external_requires_external_reference`

Each test must call the real `task_get()`/transition path after override.

PASS only if the pre-fix code fails the first test.

---

# Fix R04 — restart recovery must repair stale RUNNING truth across project lifecycle

Current defect:

`recover_stale()` filters `p.status='ACTIVE'`, so workflow-managed RUNNING tasks under archived projects survive restart as false active execution truth.

Required production behavior:

- recovery is internal truth repair and must consider workflow-managed transient tasks regardless of project ACTIVE/ARCHIVED status;
- missing local project root must not prevent recovery;
- normal user workflow mutation remains ACTIVE-project-only;
- recover:
  - BUILDER_RUNNING -> READY_FOR_IMPLEMENTATION
  - AUDIT_RUNNING -> AUDIT_REQUIRED or RE_AUDIT_REQUIRED according to M10 workflow history
  - VERIFY_RUNNING -> VERIFY_REQUIRED
- append one SYSTEM `WORKFLOW_RECOVERY` event per repaired transient state;
- second recovery pass without new transient state adds zero events;
- do not start/recover actual external processes.

Required direct tests:

- strengthen `m10_restart_recovery_demotes_stale_running_states` to cover BUILDER_RUNNING, AUDIT_RUNNING and VERIFY_RUNNING;
- `m10_restart_recovery_repairs_archived_project_transient_state`
- `m10_restart_recovery_repairs_missing_root_transient_state`
- prove second pass is idempotent.

If retaining a candidate batch cap, do not silently present tasks beyond the cap as safely recovered. Either process deterministic bounded batches to completion or fail startup/recovery truthfully if the safety bound is exceeded.

---

# Fix R05 — AUDIT_FAILED requires canonical final follow-up result

Current defect:

Any audit result except PASS currently satisfies `AUDIT_RUNNING -> AUDIT_FAILED`.

Required production behavior:

H!veAI audit verdict semantics are:

```text
PASS
CONDITIONAL
FAIL
```

Use case-insensitive comparison of persisted audit result.

- PASS -> may satisfy AUDIT_PASSED only;
- FAIL -> may satisfy AUDIT_FAILED;
- CONDITIONAL -> may satisfy AUDIT_FAILED;
- unknown/non-final values such as PENDING/RUNNING/empty/garbage -> `WORKFLOW_EVIDENCE_INCOMPATIBLE`.

Required direct tests:

- `m10_audit_failed_accepts_fail_result`
- `m10_audit_failed_accepts_conditional_result`
- `m10_audit_failed_rejects_pass_result`
- `m10_audit_failed_rejects_unknown_nonfinal_result`

PASS only if the pre-fix implementation fails the unknown/non-final case.

---

# Tighten E01-E05 evidence

## E01 history order

Strengthen `m10_history_is_bounded_and_deterministically_ordered` so it asserts actual chronological event IDs/order and a deterministic tie-break, not only length.

## E02 recovery evidence

Ensure the recovery test exercises all three transient states and idempotency.

## E03 M09 integration evidence

Strengthen direct assertions:

- reparse: parser title/metadata actually refresh while state/event count stay unchanged;
- stale: `task_intelligence::list()` does not contain the retired task;
- reappearance: `sourceActive=true`, sourceRetired=false, same task ID, refreshed parser metadata, same created_at, same workflow state/history.

Do not weaken existing M09 no-history stale deletion.

## E04 missing project root

Add direct mutation test proving ACTIVE project with missing registered root rejects normal mutation but history remains readable. Recovery of a transient task must still repair internal truth as specified above.

## E05 concrete final equality evidence

The M10A log must record concrete SHAs, not `SELF` placeholders.

After every final code/test/log/tracker commit:

```powershell
git fetch origin H!veAI
git rev-parse HEAD
git rev-parse origin/H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
```

Record the exact two SHAs and exact `0 0` divergence in the final log.

---

# Preserve already-correct M10 behavior

Do not regress:

- exact canonical state strings;
- happy path and re-audit matrix;
- expected-state conflict protection;
- request-id idempotency/fingerprint conflict;
- task/project evidence ownership checks;
- normal suspension semantics;
- HUMAN override decision/event/state atomicity;
- M09 reparse state preservation;
- M09 stale managed-history preservation;
- M09 same-ID reappearance history preservation;
- M09 no-history stale deletion;
- history/list bounds;
- archived/missing read/history availability;
- narrow IPC/permission/capability;
- no arbitrary SQL/shell/network/process launch;
- no visible UI work;
- no Project Dashboard runtime ingestion;
- X01/X02 native UX fixes;
- canonical asset hashes;
- stable QA launcher/icon;
- no installer.

Do not mutate historical M10 builder log or original audit.

---

# Tracker truth

At the start of M10A, prospectively synchronize live tracker docs so they say:

- M00-M09 PASS/CLOSED;
- pre-M10 X01/X02 PASS/CLOSED;
- M10 original strict audit = FAIL with 5 MAJOR findings;
- M10A remediation ACTIVE/then IMPLEMENTATION COMPLETE / PENDING INDEPENDENT RE-AUDIT after all gates pass;
- strict completed count remains `10 / 20 = 50%`;
- M11/M12 remain blocked.

Do not mark M10 PASS/CLOSED yourself.

---

# Verification gates

Run and record exact commands/results:

1. focused M10 Rust tests;
2. full Rust tests;
3. relevant M09 integration tests;
4. TypeScript workflow contract tests;
5. full frontend tests;
6. `npm run typecheck`;
7. `npm run build`;
8. `npm audit --audit-level=high`;
9. `cargo fmt -- --check` using the repository manifest path;
10. `cargo check`;
11. `cargo test`;
12. `cargo build`;
13. publisher failure/rollback harness;
14. governed Tauri production `--no-bundle` QA publication;
15. stable EXE/shortcut/icon validation;
16. canonical background/video hashes;
17. no installer scan.

No manual visual acceptance is required unless you unexpectedly change visible UI. You must not change visible UI.

---

# M10A log

Create:

`H!veAI/docs/H!veAI/codex-logs/M10A_WORKFLOW_STATE_MACHINE_STRICT_CLOSURE_LOG.md`

Record:

- start branch/HEAD/origin equality;
- exact findings R01-R05 and E01-E05;
- changed files/symbols;
- pre-fix failure evidence for the new regression tests;
- failed attempts retained chronologically;
- focused/full regression results;
- publication evidence;
- canonical hashes;
- final implementation commit SHA;
- final log/tracker commit SHA;
- exact final local HEAD;
- exact final origin/H!veAI HEAD;
- exact `0 0` equality proof.

Stop after pushed M10A evidence. Do not start M11 or M12.
