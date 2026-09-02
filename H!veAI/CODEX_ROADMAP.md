# H!veAI Detailed Codex Roadmap

This roadmap mirrors the canonical task ledger in `TASKS.md` at milestone/package level.

Important: package numbering such as `M10.01`, `M10.02`, etc. is for traceability and auditability. It does **not** require separate Codex prompts. The default execution model remains one whole-milestone prompt, followed by independent audit and, only if necessary, bounded remediation.

User-facing roadmap denominator remains **20**. `M00` is the historical rebuild/rebaseline foundation; `M01` through `M20` are the product roadmap milestones.

## Current status

- M00-M12: PASS/CLOSED.
- M09 Task Intelligence Parser: PASS/CLOSED after the independent M09D final strict audit.
- Pre-M10 Native UX Hotfix X01/X02: PASS/CLOSED after independent source audit plus user native acceptance.
- M10 original strict audit: historical FAIL with 5 MAJOR findings.
- M10A remediation, independent re-audit, and Akilta native click acceptance: PASS/CLOSED.
- M11 original and remediation failures remain historical evidence; accepted strict audits close M11A REV7 and the final Projects visual cleanup.
- M11 = PASS/CLOSED.
- M12 = PASS/CLOSED, including M12A R26 and M12B route remediation, on accepted strict evidence and user native/visual acceptance. M13/M13A/M13B/M13C/M13D/M13E are PASS/CLOSED on accepted strict re-audits and user native/visual evidence. M14A closes M14-R35, M14-R36, and M14-R37; M14B closes M14-R38, M14-R39, and M14-R40; M14 remains implementation complete pending independent strict audit and user native/visual acceptance. M15-M20 remain planned/blocked, and M21 is not part of this roadmap run.
- M21 remains planned and was not started.
- Strict completed progress is 14/20 = 70% because the pre-M10 hotfix is not a numbered roadmap milestone.

---

## M00 - Fresh start / rebaseline

Purpose: establish a clean, governed H!veAI rebuild inside the existing parent repository.

Packages:
- M00.01 Repository/root proof.
- M00.02 Canonical repository/branch/remote baseline.
- M00.03 Legacy AI-Commerce-HQ source-material audit.
- M00.04 Canonical prompts/audits/logs document layout.
- M00.05 Governance/audit protocol.
- M00.06 Target architecture/rebuild baseline.

Exit: dedicated H!veAI root and authoritative governance established without creating a nested Git repo.

Status: PASS/CLOSED.

---

## M01 - Tauri 2 Foundation

Purpose: modern native Windows desktop foundation.

Packages:
- M01.01 Tauri 2 modernization.
- M01.02 Narrow capabilities/permissions.
- M01.03 H!veAI native identity.
- M01.04 Native logging/notifications.
- M01.05 Native status/frontend-ready foundation.
- M01.06 App-data migration policy.
- M01.07 Native restart flow.
- M01.08 Real Windows restart acceptance.

Exit: real Tauri 2 H!veAI app launches/restarts natively with narrow permissions.

Status: PASS/CLOSED.

---

## M02 - UI Shell / Design System

Purpose: replace the game/commerce root UI with a professional AI development command center.

Packages:
- M02.01 Remove obsolete GameWorld/primary Three.js flow.
- M02.02 Dark-first shell/sidebar/topbar.
- M02.03 Application routes.
- M02.04 Accessible reusable UI primitives and motion.
- M02.05 Loading/error/empty truthfulness.
- M02.06 Desktop shell geometry/overflow baseline.

Exit: stable professional H!veAI application shell.

Status: PASS/CLOSED.

---

## M03 - Runtime Refactor

Purpose: remove historical commerce runtime coupling and choose the permanent native boundary.

Packages:
- M03.01 Remove commerce-specific startup orchestration.
- M03.02 Inventory historical runtime/Python responsibilities.
- M03.03 Select Rust-native runtime architecture.
- M03.04 Remove always-on Python sidecar dependency.
- M03.05 Document frontend/native responsibility split.

Exit: H!veAI runs as a local-first Rust-native desktop foundation rather than a commerce sidecar host.

Status: PASS/CLOSED.

---

## M04 - SQLite / Migrations

Purpose: durable versioned local state foundation for projects, tasks, agents, prompts, audits, tests and GitHub sync.

Packages:
- M04.01 Versioned migrations/schema tracking.
- M04.02 Project/repository/source tables.
- M04.03 Task/dependency/source/event tables.
- M04.04 Prompt/version tables.
- M04.05 Agent/session/tool/permission tables.
- M04.06 Audit/test/alert/decision tables.
- M04.07 GitHub/settings tables and indexes.
- M04.08 Integrity/backup/rollback/contention/failure evidence.

Exit: corruption-safe, migration-tested SQLite foundation.

Status: PASS/CLOSED.

---

## M05 - Project Registry

Purpose: safely register and manage local projects without mutating project content.

Packages:
- M05.01 Register existing folders read-only.
- M05.02 Detect Git/remotes/default branch/GitHub identity.
- M05.03 Priority/builder/auditor/task-source settings.
- M05.04 Repair/archive/remove-from-registry lifecycle.
- M05.05 Search/sort/filter/selection.
- M05.06 Registry identity/isolation evidence.

Exit: Registry is the canonical project/root identity authority.

Status: PASS/CLOSED.

---

## M06 - Local Git Engine

Purpose: safe read-only-by-default local Git intelligence.

Packages:
- M06.01 Branch/HEAD/staged/unstaged/untracked status.
- M06.02 Remote/upstream/ahead-behind.
- M06.03 Commits/conflicts/worktrees.
- M06.04 Deterministic safe diff and binary metadata.
- M06.05 Default-denied mutation boundary.
- M06.06 Narrow native IPC/ACL.
- M06.07 Temp-repo/direct production-path evidence.

Exit: trustworthy local Git state/diff engine without unrestricted writes.

Status: PASS/CLOSED.

---

## M07 - Filesystem Watcher / Snapshots

Purpose: reactive bounded project change observation and durable refresh evidence.

Packages:
- M07.01 Watch Registry project roots/task-relevant paths.
- M07.02 Debounce/bounded watcher lifecycle.
- M07.03 Missing/moved/repaired-root behavior.
- M07.04 Git/task refresh categories and snapshots.
- M07.05 Git Engine refresh integration.
- M07.06 Watcher/persistence/cleanup/containment failure evidence.
- M07.07 Production no-bundle QA publisher and rollback harness.
- M07.08 Stable EXE/shortcut/icon behavior.
- M07.09 Global sidebar/logo acceptance.
- M07.10 Restart/publisher final closure.

Exit: bounded watcher + stable production QA launcher with final visual/restart acceptance.

Status: PASS/CLOSED.

---

## M08 - Task Source Discovery

Purpose: bounded discovery of authoritative project planning/progress/task sources. M08 remains the only filesystem source-discovery authority for M09+.

Packages:
- M08.00 Canonical background/opening-video presentation bootstrap.
- M08.01 Background alignment + process-scoped startup intro lifecycle.
- M08.02 Discovery source/status/metadata contract.
- M08.03 Root standard sources and handoff wildcard family.
- M08.04 Approved recursive `tasks/`, `plans/`, `handoffs/`, `.hiveai/` discovery.
- M08.05 Visited-entry/candidate/depth/size/custom-path work bounds.
- M08.06 Canonical physical containment/traversal/link safety.
- M08.07 SHA-256 and evidence metadata.
- M08.08 Custom source CRUD/status handling.
- M08.09 True positional custom ordering.
- M08.10 Legacy no-order backward compatibility.
- M08.11 Non-destructive SQLite `project_sources` reconciliation.
- M08.12 ACTIVE/MISSING/ARCHIVED project boundary.
- M08.13 Narrow native IPC/ACL.
- M08.14 Native Task Sources workspace.
- M08.15 Project-switch stale async race safety.
- M08.16 Visible frontend state evidence.
- M08.17 Direct Rust evidence matrix.
- M08.18 Full regression/no-bundle publication.
- M08.19 Native manual acceptance/final closure.

Exit: source inventory is deterministic, bounded, containment-safe, persisted, visible and user-accepted.

Status: PASS/CLOSED.

---

## M09 - Task Intelligence Parser

Purpose: convert M08-approved source bodies into deterministic normalized task/handoff intelligence without implementing workflow state transitions.

Packages:
- M09.01 M08-only source boundary and bounded read/hash verification.
- M09.02 One-refresh/one-retry source-change handling.
- M09.03 Normalized task/handoff/confidence/evidence model.
- M09.04 Generic headings/checklists/status/task-ID parser.
- M09.05 Structured inline/nested metadata and actor/owner gate.
- M09.06 Current/Next/Blocker/Waiting handoff intelligence and merge.
- M09.07 Deterministic task identity across content movement.
- M09.08 Path-specific task identity normalization.
- M09.09 Project/scalar/list/warning bounds.
- M09.10 Generic/FormuLab/ScrubBots/FMCG adapter boundary.
- M09.11 Deterministic confidence/evidence locators.
- M09.12 M09-owned stable UPSERT persistence and stale reconciliation.
- M09.13 SOURCE_EXPLICIT dependency resolution/persistence.
- M09.14 Narrow parse/list IPC and TS contract.
- M09.15 Direct production evidence matrix.
- M09.16 Original M09 strict-audit history.
- M09.17 M09A seven-finding remediation history.
- M09.18 M09B bounded-path/scalar micro-fix.
- M09.19 Final regression/publication/independent closure.

Current status:
- Original M10 strict audit: historical FAIL with 5 MAJOR findings.
- M10A remediation, independent re-audit, and Akilta footer native click acceptance: PASS/CLOSED.
- Original M09: historical FAIL.
- M09A: historical FAIL after exposing R01/R02.
- M09B/M09C: historical remediation records preserved.
- M09D independent final strict audit: PASS.
- M09 Task Intelligence Parser: PASS/CLOSED.

Status: PASS/CLOSED.

---

## Pre-M10 Native UX Hotfix

Purpose: close two user-reported native defects without contaminating parser/workflow scope.

Packages:
- X01 Suppress visible Git child-process console windows on Windows while preserving watcher/Git Engine behavior.
- X02 Restore audible startup intro autoplay while preserving process-scoped intro lifecycle and canonical video bytes.

Acceptance:
- X01: user observed approximately 45 minutes of native runtime with no unwanted terminal/console windows.
- X02: startup audio is accepted fixed and same-process navigation does not replay the intro.
- Independent source/config audit found 0 BLOCKER / 0 MAJOR / 0 MINOR and required only user native acceptance, which is now supplied.

Exit: native H!veAI can remain open without terminal-popup storms, and the startup intro behaves audibly without same-process replay.

Status: PASS/CLOSED.

---

## M10 - Workflow State Machine

Purpose: turn parser truth into explicit durable operational workflow truth.

Packages:
- M10.01 Canonical task/workflow states.
- M10.02 Actor model.
- M10.03 Allowed transition matrix/happy path/failure loops.
- M10.04 Evidence requirements per transition.
- M10.05 Human override with rationale.
- M10.06 Durable `task_events` transition history.
- M10.07 Restart/interruption recovery.
- M10.08 Narrow state-machine IPC/permissions.
- M10.09 Direct transition/evidence/recovery tests.
- M10.10 Regression/publication/strict audit.

Exit: every task operational state is explainable, evidence-backed and recoverable.

Status: PASS/CLOSED.

---

## M11 - Global Command Center

Purpose: make the existing command-center UI operational with live Registry/task/workflow data and resolve each project's declared Project Dashboard authority map.

Packages:
- M11.01 Live portfolio aggregation model + native `.hiveai/PROJECT_DASHBOARD.md` authority resolver with bounded/contained fallback to M08/M09.
- M11.02 KPI strip.
- M11.03 Project operation cards/current task/state/health using resolved authority roles where available.
- M11.04 Needs Your Attention.
- M11.05 Active Work Queue.
- M11.06 AI Engineering Brief surface/data contract with factual source provenance.
- M11.07 Recent Activity/search/filter.
- M11.08 Selected-project interaction/session memory.
- M11.09 Layout/performance/accessibility preservation.
- M11.10 Mounted tests, manifest present/absent/malformed/stale/containment evidence, no-double-count proof, publication, visual acceptance, strict audit.

Exit: one-screen truthful portfolio operations dashboard backed by Registry/M08/M09/M10 truth and the Project Dashboard authority manifest system.

Status: PASS/CLOSED.

---

## M12 - Project Cockpit

Purpose: complete per-project operational workspace using the same resolved Project Dashboard authority/provenance model as M11.

Packages:
- M12.01 Project route/loading/missing-state shell + project-scoped resolved authority map.
- M12.02 Overview/current task hero + useful source provenance.
- M12.03 Tasks/dependencies/evidence drawer with manifest-declared canonical task authority and duplicate suppression.
- M12.04 Workflow pipeline/history/override controls.
- M12.05 Agents.
- M12.06 Audit.
- M12.07 Git.
- M12.08 Tests/Activity/Files with classified roadmap/handoff/history/architecture context.
- M12.09 Project Settings/Registry controls + manifest status/roles/warnings/provenance, without auto-rewriting project files.
- M12.10 Manual correction/event controls.
- M12.11 Mounted tests, authority/provenance rendering tests, publication, visual acceptance, audit.

Exit: complete end-to-end project operations cockpit with truthful source authority and provenance.

Status: PASS/CLOSED.

### M12A - Project-wide workflow history strict remediation

Bounded remediation for the M12 strict-audit R26 finding: selected-project workflow history is globally ordered with deterministic ties before the 200-event cap, with direct adversarial regression coverage.

Status: PASS/CLOSED; R26 independently re-audited.

### M12B - Native Open Cockpit route-loading remediation

Bounded remediation for the native registered-project cockpit route and Tauri
capability boundary. The exact project ID is preserved and native failures are
classified truthfully.

Status: PASS/CLOSED; accepted strict re-audit and user native/visual acceptance.

---

## M13 - Codex Adapter

Purpose: real provider adapter for project-scoped Codex sessions.

Packages:
- M13.01 Installation/version/auth readiness.
- M13.02 Common agent adapter contract.
- M13.03 Project/worktree cwd launch with containment.
- M13.04 stdout/stderr/exit/stream capture.
- M13.05 Project/task/session mapping.
- M13.06 Resume/stop/crash/orphan/restart recovery.
- M13.07 Process/argument permission boundary.
- M13.08 Direct process/security tests.
- M13.09 Regression/publication/audit.

Exit: Codex can be safely started/stopped/observed from H!veAI as a real agent session.

Status: PASS/CLOSED. M13A-M13E accepted strict remediation chain and user native/visual evidence are preserved as immutable provenance.

---

## M14 - Agent Session Center

Purpose: native PTY-backed live agent operations center.

Packages:
- M14.01 Rust PTY/process manager + xterm.js.
- M14.02 Session list/status/timing.
- M14.03 Bounded live terminal.
- M14.04 Session event/tool/prompt timeline.
- M14.05 Diff/changed-file evidence.
- M14.06 Stop/retry/restart recovery.
- M14.07 Permission UI/notifications.
- M14.08 Process/UI/security tests and audit.

Exit: H!veAI can supervise active builder sessions transparently and recoverably.

Status: IMPLEMENTATION COMPLETE / PENDING INDEPENDENT STRICT AUDIT + USER NATIVE/VISUAL ACCEPTANCE.

M14A remediation: R35 native test loader, R36 governed publication readiness, and R37 ACTIVE project confinement are CLOSED on remediation evidence. M14B remediation: R38 verified Claude invocation, R39 explicit persisted-session selection, and R40 compact vertical session reader are CLOSED on remediation evidence. Independent strict re-audit and user native/visual acceptance remain pending.

---

## M15 - Prompt Engine

Purpose: versioned, reviewable, provenance-preserving implementation/remediation prompts.

Packages:
- M15.01 Prompt kinds/schemas/types.
- M15.02 Immutable used-version storage.
- M15.03 Bounded context collector.
- M15.04 Implementation prompt generation.
- M15.05 Audit-driven remediation prompt generation.
- M15.06 Human review/edit/approve.
- M15.07 Provider dispatch + prompt/session provenance.
- M15.08 Version/context/provenance tests and audit.

Exit: every dispatched builder prompt is reproducible and traceable.

Status: PLANNED/BLOCKED.

---

## M16 - GPT Audit Engine

Purpose: bring the current independent strict-audit workflow into H!veAI itself.

Packages:
- M16.01 Audit input contract.
- M16.02 PASS/CONDITIONAL/FAIL + severity structured result.
- M16.03 Source/diff/test/config evidence verification.
- M16.04 Audit/finding persistence.
- M16.05 Remediation/re-audit loop.
- M16.06 Audit Center UI.
- M16.07 Truthfulness/UNVERIFIED/security policy.
- M16.08 Known-good/bad fixture tests and independent audit.

Exit: H!veAI can independently audit implementation evidence and drive bounded remediation.

Status: PLANNED/BLOCKED.

---

## M17 - Claude Code Adapter

Purpose: add a second builder provider using the same safe agent contract.

Packages:
- M17.01 Installation/version/auth/quota readiness.
- M17.02 Common adapter compliance.
- M17.03 Start/resume/continue/stop.
- M17.04 Structured output/event mapping.
- M17.05 Permission/user-wait/quota detection.
- M17.06 Completion/crash/orphan/restart recovery.
- M17.07 Process/security tests.
- M17.08 Regression/publication/audit.

Exit: Codex and Claude operate through one consistent H!veAI agent model.

Status: PLANNED/BLOCKED.

---

## M18 - GitHub Integration

Purpose: reconcile local development truth with GitHub repositories/PRs/issues/Actions/releases.

Packages:
- M18.01 Repository/branch/commit reads.
- M18.02 PR inspection and permission-gated creation.
- M18.03 Issues.
- M18.04 Actions/CI inspection and gated retry.
- M18.05 Releases/tags.
- M18.06 Cache/rate-limit/offline behavior.
- M18.07 Local/remote reconciliation.
- M18.08 Least-privilege mutation/security.
- M18.09 Remote/offline/reconciliation tests and audit.

Exit: H!veAI understands remote GitHub state without silently overwriting local work.

Status: PLANNED/BLOCKED.

---

## M19 - Next Best Task AI + Engineering Brief

Purpose: explainable prioritization across the project portfolio.

Packages:
- M19.01 Candidate eligibility from task/workflow truth.
- M19.02 Priority/dependency/blocker/audit/CI/context-switch scoring.
- M19.03 Agent/actor availability awareness.
- M19.04 Explainable next-task recommendation.
- M19.05 Portfolio ranking.
- M19.06 Morning/since-last-visit Engineering Brief.
- M19.07 Factual-state vs AI-recommendation separation/evidence.
- M19.08 Deterministic scoring/explanation tests and audit.

Exit: H!veAI can recommend what to do next and explain why.

Status: PLANNED/BLOCKED.

---

## M20 - Project Chat + Hardening + Release

Purpose: final action-capable assistant, security/performance hardening, installer and v1.0 release.

Packages:
- M20.01 Portfolio/project grounded chat.
- M20.02 Action-capable chat with execution preview/permission.
- M20.03 Command palette.
- M20.04 Keyboard/accessibility/reduced-motion completion.
- M20.05 Credential/log/process/capability security hardening.
- M20.06 Database backup/restore.
- M20.07 Performance/large-project/task/history testing.
- M20.08 Final UX/error/offline/branding hardening.
- M20.09 Installer + clean-machine Windows testing.
- M20.10 User/security/privacy/troubleshooting docs.
- M20.11 Full release audit.
- M20.12 H!veAI v1.0.0 release/tag/artifacts/human acceptance.

Exit: no unresolved BLOCKER/MAJOR findings, clean-machine install works, H!veAI v1.0.0 is releasable.

Status: PLANNED/BLOCKED.

---

## Dependency path

`M00 -> M01 -> M02 -> M03 -> M04 -> M05 -> M06 -> M07 -> M08 -> M09 -> pre-M10 UX hotfix (PASS/CLOSED) -> M10 -> M11/M12 -> M13 -> M14 -> M15 -> M16 -> M17 -> M18 -> M19 -> M20`

## Builder execution rule

Before each milestone, Codex must read `AGENTS.md`, `CONSTITUTION.md`, `ARCHITECTURE.md`, `TASKS.md`, and the authoritative milestone prompt; inspect/synchronize Git safely; run baseline tests; implement only the current milestone; add direct tests; run full regression/publication gates; update prospective tracking/logs truthfully; review the diff; commit/push without rewriting history; then stop for independent audit.

## Exit rule

A builder may not mark a milestone PASS/CLOSED because code was generated or tests were claimed green. Final closure requires independent audit acceptance and any required manual/native acceptance.
