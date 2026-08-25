# M10A — Workflow State Machine Strict Closure

## Mission

First, implement the user-requested bounded footer link change described below. Then close the five MAJOR findings and five evidence gaps from:

`H!veAI/docs/H!veAI/audits/M10_WORKFLOW_STATE_MACHINE_STRICT_AUDIT.md`

This remains a bounded M10 remediation run. The only visible UI change allowed is the Akilta footer link described in Task 0.

Do not redesign the workflow architecture. Do not start M11/M12. Do not implement Project Dashboard runtime ingestion. Do not redesign any other visible UI. Do not create an installer.

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
5. `H!veAI/CODEX_ROADMAP.md`
6. `H!veAI/docs/H!veAI/prompts/M10_WORKFLOW_STATE_MACHINE_PROMPT.md`
7. `H!veAI/docs/H!veAI/audits/M10_WORKFLOW_STATE_MACHINE_STRICT_AUDIT.md`
8. current `workflow.rs`, `task_intelligence.rs`, TypeScript workflow contract, permissions/capability
9. `H!veAI/src/components/Shell.tsx` and current footer styling/tests
10. this prompt

Preserve user-owned untracked `start-demo.bat` and `task.md` if still present.

---

# Canonical UI Assets

Canonical user-owned asset root:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Preserve accepted logo/background/startup-video/launcher behavior.

Required unchanged hashes:

- background: `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`
- opening video: `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`

Preserve X01 terminal-popup suppression and X02 audible startup intro/replay behavior.

The footer must remain visually consistent with the accepted layout. No geometry, spacing, logo, wordmark, text, background, route, or shell redesign is allowed beyond making the visible final `Akilta` text actionable.

---

# Task 0 — FIRST TASK: make footer Akilta open www.akilta.com

Current footer production source is `H!veAI/src/components/Shell.tsx` and currently renders:

`Built with ♥ for maximum productivity by Akilta`

Required behavior:

- preserve the exact visible sentence and existing Akilta wordmark/layout;
- make the final visible word `Akilta` keyboard-accessible and clickable;
- clicking it must open exactly `https://www.akilta.com/` externally;
- H!veAI must remain open and must not navigate its Tauri WebView away from the application;
- obey the existing H!veAI external-browser policy: when browser choice is controlled by H!veAI, use Google Chrome, not Microsoft Edge;
- do not change the user's Windows default browser;
- do not silently fall back to Edge;
- do not construct shell command strings or use `cmd.exe` / PowerShell wrappers;
- do not expose a generic arbitrary-URL native command merely for this footer link;
- if a safe existing external-URL/Chrome helper already exists, reuse it;
- otherwise add the narrowest practical production path for this single allowlisted URL and keep it argument-safe;
- on Windows, any helper child-process launch must preserve the X01 no-console behavior and must not flash a terminal window;
- if Chrome is unavailable, fail truthfully rather than opening another browser silently.

Preferred frontend semantics:

- use an accessible link-like control for the final `Akilta` text;
- if an anchor is used, prevent in-WebView navigation and route the action through the safe external-open path;
- preserve current footer CSS appearance except a restrained hover/focus affordance if needed for accessibility.

Required automated evidence:

1. component/contract test proves the visible footer sentence is still exactly `Built with ♥ for maximum productivity by Akilta`;
2. test proves only the final `Akilta` text is the website action and targets the exact Akilta URL/path;
3. if a native command is added, test proves there is no arbitrary frontend URL parameter and the native constant is exactly `https://www.akilta.com/`;
4. test/inspection proves there is no Edge fallback and no shell-string invocation;
5. existing X01 terminal suppression and X02 startup audio/replay tests remain green.

Native manual acceptance for this user-requested UI change remains **PENDING USER ACCEPTANCE** after publication. Record in the log that the user must click `Akilta` in the published H!veAI footer and confirm that `www.akilta.com` opens in Google Chrome while H!veAI stays open and no terminal window appears.

Do this Task 0 before R01-R05 remediation.

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
- reappearance: `sourceActive=true`, `sourceRetired=false`, same task ID, refreshed parser metadata, same `created_at`, same workflow state/history.

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
- narrow workflow IPC/permission/capability;
- no arbitrary SQL/network/process launch from workflow APIs;
- no Project Dashboard runtime ingestion;
- X01/X02 native UX fixes;
- canonical asset hashes;
- stable QA launcher/icon;
- no installer.

The user-requested footer link is the only permitted visible UI change in this run. Do not alter any other application-shell visuals.

Do not mutate the historical M10 builder log or original M10 strict audit.

---

# Tracker truth

At the start of M10A, prospectively synchronize live tracker docs so they say:

- M00-M09 PASS/CLOSED;
- pre-M10 X01/X02 PASS/CLOSED;
- M10 original strict audit = FAIL with 5 MAJOR findings;
- M10A remediation ACTIVE/then IMPLEMENTATION COMPLETE / PENDING INDEPENDENT RE-AUDIT after automated gates pass;
- Akilta footer link = implementation task in this run, manual native acceptance PENDING until the user clicks it in the published app;
- strict completed count remains `10 / 20 = 50%`;
- M11/M12 remain blocked.

Do not mark M10 PASS/CLOSED yourself.

---

# Verification gates

Run and record exact commands/results:

1. focused Akilta footer-link tests;
2. focused M10 Rust tests;
3. full Rust tests;
4. relevant M09 integration tests;
5. TypeScript workflow contract tests;
6. full frontend tests;
7. `npm run typecheck`;
8. `npm run build`;
9. `npm audit --audit-level=high`;
10. `cargo fmt -- --check` using the repository manifest path;
11. `cargo check`;
12. `cargo test`;
13. `cargo build`;
14. publisher failure/rollback harness;
15. governed Tauri production `--no-bundle` QA publication;
16. stable EXE/shortcut/icon validation;
17. canonical background/video hashes;
18. no installer scan;
19. source/test proof that X01 terminal-popup suppression and X02 startup audio/replay remain intact.

Because Task 0 intentionally changes a clickable visible footer behavior, manual native acceptance is required after publication. Leave it PENDING for the user; do not fabricate PASS.

---

# M10A log

Create:

`H!veAI/docs/H!veAI/codex-logs/M10A_WORKFLOW_STATE_MACHINE_STRICT_CLOSURE_LOG.md`

Record:

- start branch/HEAD/origin equality;
- Task 0 implementation, exact URL, browser-opening design, changed files and tests;
- exact findings R01-R05 and E01-E05;
- changed files/symbols;
- pre-fix failure evidence for new regression tests;
- failed attempts retained chronologically;
- focused/full regression results;
- publication evidence;
- canonical hashes;
- manual Akilta-link acceptance = PENDING USER ACCEPTANCE;
- final implementation commit SHA;
- final log/tracker commit SHA;
- exact final local HEAD;
- exact final origin/H!veAI HEAD;
- exact `0 0` equality proof.

Stop after pushed M10A evidence. Do not start M11 or M12.
