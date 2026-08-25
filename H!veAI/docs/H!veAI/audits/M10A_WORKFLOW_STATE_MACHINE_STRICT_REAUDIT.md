# M10A Workflow State Machine — Independent Strict Re-Audit

Date: 2026-08-26
Branch: `H!veAI`
M10A prompt commit: `3ac888939d29a4131cafd8ced0667a880367d8e2`
M10A implementation commit: `493d993054bd9f121a8b15ecd47976f65de4e676`
Builder evidence/log commit / audited branch HEAD before this audit: `e0df1855f36871a51af908e6bb4489408baf6256`

## VERDICT

**CONDITIONAL**

- BLOCKER: 0
- MAJOR: 0
- MINOR: 1
- NOTE: 1
- Confidence: HIGH
- Regression risk: LOW-MEDIUM

The five production MAJOR findings from the original M10 strict audit are closed in current source, and E01-E04 direct evidence gaps are materially closed. The added Akilta footer-link implementation is narrow, source-level correct, permission-bounded, fixed-URL only, Chrome-only on Windows, and preserves the no-console launch requirement.

M10 workflow/domain closure is source-level **PASS**. Overall M10A remains **CONDITIONAL** only because the user-requested visible Akilta link still requires native click acceptance, and the builder log's final local/origin equality proof remains a MINOR evidence gap.

Do not start M11 until the user performs the one native Akilta click acceptance and the live tracker is closed prospectively.

Strict roadmap count remains `10 / 20 = 50%` until that final closure update.

---

## 1. Scope and branch truth

Compared:

`3ac888939d29a4131cafd8ced0667a880367d8e2..e0df1855f36871a51af908e6bb4489408baf6256`

Two commits were added:

- `493d993054bd9f121a8b15ecd47976f65de4e676` — M10A production remediation + Task 0 footer link + tracker synchronization;
- `e0df1855f36871a51af908e6bb4489408baf6256` — M10A builder evidence/log publication.

Changed production scope is bounded to:

- workflow remediation in `src-tauri/src/workflow.rs`;
- the fixed Akilta external-browser path;
- Tauri command registration/permission/capability;
- footer link and its minimal style/test;
- tracker/docs/log updates.

No M11/M12 runtime work or Project Dashboard manifest ingestion was introduced.

---

## 2. Original MAJOR findings

### R01 — PASS — latest event is now truly latest

`history_tx()` remains chronological using:

```sql
ORDER BY occurred_at ASC, id ASC
```

A separate `latest_event_tx()` now uses:

```sql
ORDER BY occurred_at DESC, id DESC LIMIT 1
```

`task_read()` consumes `latest_event_tx()` rather than `history_tx(..., 1)`.

Direct test `m10_latest_event_is_truly_latest` seeds three deterministic events and asserts final event ID, `to_state`, summary, timestamp, plus chronological public history.

### R02 — PASS — actor policy is centralized

A single `actor_policy(from, to)` is now used by:

- mutation validation through `validate_actor()`;
- read-model actor derivation through `allowed_actors_for_state()`.

Enforced policies include:

- builder start/completion: CODEX or CLAUDE;
- audit start/result: GPT_AUDIT or CI;
- verify start/completion: CI;
- WAITING_HUMAN/DESIGN_GATE resume: HUMAN;
- WAITING_EXTERNAL resume: EXTERNAL or HUMAN;
- BLOCKED resume: HUMAN or SYSTEM;
- SYSTEM ordinary transitions are rejected except explicit internal bookkeeping transitions.

The current M09 model does not expose a typed, blocker-specific actor reference that can safely extend BLOCKED without interpreting free text. The conservative HUMAN/SYSTEM behavior is accepted rather than inventing an actor.

Direct tests cover builder/audit/verify actor read truth, SYSTEM rejection, and suspension actor behavior.

### R03 — PASS — override suspension is readable and resumable

`override_state()` now:

- validates evidence ownership;
- requires external reference when overriding to WAITING_EXTERNAL;
- persists `suspendedState` and deterministic `resumeState`;
- maps RUNNING states to safe prerequisites;
- carries the original resume state across explicit hold-to-hold override;
- keeps decision + override event + task state inside one SQLite transaction.

Direct tests exercise `task_get()` and real resume behavior after override.

### R04 — PASS — restart recovery covers archived/missing projects

`recover_stale()` no longer filters candidates to ACTIVE projects. It considers workflow-managed transient states across project lifecycle and does not depend on filesystem-root presence.

Recovery targets remain:

- BUILDER_RUNNING -> READY_FOR_IMPLEMENTATION;
- AUDIT_RUNNING -> AUDIT_REQUIRED / RE_AUDIT_REQUIRED according to real workflow history;
- VERIFY_RUNNING -> VERIFY_REQUIRED.

The candidate bound is explicit: if more than 4096 stale candidates exist, recovery returns `WORKFLOW_RECOVERY_BOUNDS`, which propagates through Tauri setup as a startup/recovery failure instead of silently presenting unrecovered RUNNING truth.

Direct tests cover all three transient states, second-pass idempotency, archived project recovery, and missing-root recovery.

### R05 — PASS — AUDIT_FAILED accepts only final canonical follow-up results

Audit result is trimmed and compared case-insensitively through uppercase normalization:

- PASS -> AUDIT_PASSED;
- FAIL -> AUDIT_FAILED;
- CONDITIONAL -> AUDIT_FAILED;
- all other values -> `WORKFLOW_EVIDENCE_INCOMPATIBLE`.

Direct FAIL/CONDITIONAL/PASS-negative/PENDING-negative evidence exists.

---

## 3. Evidence gaps E01-E05

### E01 — PASS

History test now asserts chronological IDs and deterministic `id ASC` tie-break, plus bounded result semantics.

### E02 — PASS

Recovery evidence now directly covers builder, audit and verify transient states and an idempotent second pass.

### E03 — PASS

M09 integration tests now prove:

- parser title/metadata refresh on reparse while operational state/history stay intact;
- retired managed task is absent from `task_intelligence::list()` but remains in SQL with history;
- reappearance restores source-active truth while preserving task ID, `created_at`, workflow state and events.

### E04 — PASS

Direct project lifecycle evidence now includes an ACTIVE registered project whose physical root is missing: normal mutation rejects while history remains readable, and internal recovery remains path-independent.

### E05 — MINOR — final local/origin equality is still not concretely recorded inside the log

The M10A log says the exact final SHA pair and `0 0` divergence are included in the builder's final delivery response, but the persisted log itself does not contain the concrete final local HEAD and origin/H!veAI SHA pair after the evidence commit.

GitHub independently proves the pushed remote audited HEAD `e0df1855f36871a51af908e6bb4489408baf6256`; it cannot prove the builder's local checkout equality after the final push.

This is a publication/evidence bookkeeping gap, not a production workflow defect. It does not warrant another code remediation run by itself.

---

## 4. Task 0 — Akilta footer link

### Source verdict — PASS

The footer preserves the visible sentence:

`Built with ♥ for maximum productivity by Akilta`

Only the final visible `Akilta` word is an actionable anchor with exact href:

`https://www.akilta.com/`

Native behavior:

- Tauri click prevents in-WebView navigation;
- invokes parameterless `hiveai_open_akilta`;
- native code owns constant `AKILTA_URL = "https://www.akilta.com/"`;
- no frontend-provided URL is accepted;
- Windows resolver checks Google Chrome paths only;
- no Edge fallback is present;
- no `cmd.exe` or PowerShell wrapper is used;
- Chrome receives separate `--new-window` and URL arguments;
- Windows process uses `creation_flags(0x08000000)` to preserve X01 no-console behavior;
- command is separately allowlisted through a narrow Tauri permission.

Focused frontend evidence verifies the exact footer sentence, exact href, and command invocation. Native source test fixes the URL constant.

### Native user acceptance — UNVERIFIED / PENDING

User must verify on the published native build:

1. click footer `Akilta`;
2. `https://www.akilta.com/` opens in Google Chrome;
3. H!veAI remains open;
4. no terminal window flashes.

### NOTE — Chrome-unavailable error presentation

The native command returns a structured error when Chrome is unavailable and does not silently fall back to another browser. The frontend currently fires the invoke without presenting that rejection in visible UI. This still satisfies the no-fallback safety boundary, but a later UX polish milestone may choose to surface the error to the user.

---

## 5. Regression / security / architecture

Builder log claims, consistent with inspected source:

- focused frontend/Task0/M10 tests PASS;
- focused M10 Rust 39 tests PASS;
- full frontend 80 tests PASS;
- full Rust 232 library tests PASS;
- typecheck/build/npm-audit/cargo fmt/check/test/build PASS;
- publisher failure harness 9/9 PASS;
- governed Tauri production `--no-bundle` publication PASS;
- canonical background/video hashes unchanged;
- X01 terminal suppression and X02 startup audio/replay behavior preserved;
- no installer.

Source inspection found no arbitrary workflow SQL, generic external URL command, shell-string launch, network call, M11/M12 implementation, or Project Dashboard runtime ingestion in this diff.

---

## 6. Closure decision

### M10 workflow/domain

**PASS**

The five original production MAJOR findings are closed. No new BLOCKER or MAJOR production defect was found in the M10A workflow remediation.

### M10A combined run

**CONDITIONAL**

Only two non-production closure items remain:

1. native user acceptance of the visible Akilta footer link;
2. accepted MINOR E05 final-local-equality evidence gap.

No M10B code prompt should be created at this point.

After the user confirms the Akilta native click behavior, prospectively update live tracker docs to:

- M10 PASS/CLOSED;
- M11 READY;
- strict completed count `11 / 20 = 55%`;
- Akilta footer link accepted fixed;
- M12 remains blocked behind M11 as planned.
