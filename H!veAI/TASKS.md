# H!veAI MASTER TASKS

Legend: [x] validated complete, [ ] incomplete, [~] active, [!] blocked,
[?] decision required.

## M00 Fresh start with dedicated application root
- [x] Prove Git root and H!veAI child application root
- [x] Verify official repository and branch
- [x] Correct remote to `Sekiph82/AI-Commerce-HQ`
- [x] Inspect old parent AI-Commerce-HQ application as source material
- [x] Inspect existing H!veAI child files
- [x] Add H!veAI foundation docs under the child root
- [x] Copy canonical prompts/audits/Codex-log docs under the child root
- [x] Create fresh M00 migration docs under the child root
- [x] Classify reusable vs commerce-specific code
- [x] Record baseline validation and security findings

M00 COMPLETE. Do not begin M01 until M00 is independently audited.

## M01 Tauri 2 foundation
- [x] Upgrade Tauri packages and Rust APIs
- [x] Add Tauri 2 capabilities
- [x] Rename active app identity to H!veAI
- [x] Define app-data migration policy
- [!] Verify Windows launch/close/restart
- [x] Add native logging/notifications

M01 COMPLETE with restart verification left as a manual blocked check. Do not
begin M02 until M01 is independently audited.

## M02 UI shell/design system
- [x] Remove GameWorld from root flow
- [x] Remove Three.js from primary bundle
- [x] Dark-first design system
- [x] Sidebar/top command bar/router
- [x] Routes: Command Center, Projects, Cockpit, Agents, Audits, Settings
- [x] Accessible reusable UI primitives + Framer Motion
- [x] Loading/error/empty states

M02 COMPLETE. Frontend, Rust/Tauri regression, bounded desktop smoke, and child-scope review passed. Do not begin M03 in this session.

## M03 Runtime refactor
- [x] Stop GMO/commerce orchestrators from H!veAI startup
- [x] Inventory Python responsibilities
- [x] Decide Rust-native vs retained sidecar
- [x] Define dormant child-process health/recovery boundary without spawning a sidecar
- [x] Document final runtime boundary

M03 COMPLETE. H!veAI uses a Rust-native runtime with no always-on Python sidecar. Do not begin M04 in this session.

## M04 SQLite + migrations
- [x] Versioned migration framework
- [x] projects/repositories/sources/snapshots
- [x] tasks/dependencies/events
- [x] prompts/versions
- [x] agent sessions/events/tool calls/permissions
- [x] audits/findings/test_runs/alerts/decisions
- [x] GitHub cache/settings
- [x] migration tests and failure recovery

M04 COMPLETE. H!veAI owns a Rust-native SQLite schema with versioned migrations, transactional failure recovery, and read-only database status IPC. Do not begin M05 in this session.

## M05 Project Registry
- [x] Add existing folder without mutation
- [x] Detect git/remotes/default branch/GitHub identity
- [x] Priority/builder/auditor/task-source settings
- [x] Path repair/archive/remove-from-registry
- [x] Search/sort/filter

M05 COMPLETE. H!veAI supports explicit, read-only project registration backed by the M04 SQLite layer. Do not begin M06 in this session.

## M06 Local Git Engine
- [x] branch/HEAD/status/staged/unstaged/untracked
- [x] remote/ahead-behind/commits/diff/conflicts/worktrees
- [x] safe branch/commit/push interfaces
- [x] temp-repo tests

M06 COMPLETE. H!veAI has a registry-resolved, read-only local Git engine with a default-denied mutation boundary. Do not begin M07 in this session.

## M07 Filesystem Watcher + snapshots
- [x] watch project roots/task files/.hiveai
- [x] debounce
- [x] detect moved/missing repos
- [x] project snapshots/evidence timestamps
- [x] trigger task/git refresh
- [x] large-repo protections

M07 implementation is remediated by the M07.01 strict quality gate. Historical M07 completion claims are not evidence of acceptance; M07.01 owns the current gate result.

## M07.02 Strict remediation continuation
- [ ] independent strict-audit regression closure
- [ ] manual-QA production launcher closure
- [ ] user-confirmed visual and in-app restart acceptance

M07.02 remains IN PROGRESS. The M07.02A launcher hotfix is implemented and locally smoke-tested; final milestone acceptance remains subject to the active strict gate.

## M07.03 Consolidated strict closure and single-viewport UI
- [ ] Re-audit M07 findings A01-A09 and B01-B04 against repository evidence
- [ ] Maintain production `--no-bundle` candidate publication with smoke test and rollback
- [ ] Maintain canonical single-viewport UI composition and UI governance compliance
- [ ] User visual acceptance of refreshed Desktop `H!veAI.lnk`

M07.03 was reopened as FAILED by the independent strict audit and is superseded by the active M07.04 automated remediation. Final UI status remains PENDING USER VISUAL ACCEPTANCE until the user inspects the refreshed Desktop shortcut build.

## M07.04 Automated closure remediation
- [ ] Rescan, watcher, registry, migration, SQLite, diff, and launcher strict findings
- [ ] Focused production-path evidence matrix
- [ ] Independent M07.04 audit closure
- [ ] User visual acceptance of refreshed Desktop `H!veAI.lnk`
- [ ] M01 actual in-app restart acceptance

M07.04 remains ACTIVE until independent audit. Aggregate tests alone do not establish automated closure. Do not start or create later milestone work.

## M07.05 Bounded correctness and evidence remediation
- [x] Correct watcher ordinary-refresh and attachment-gated rescan semantics
- [x] Track watcher roots and reattach when a registered path changes
- [x] Close legacy rowless Git repair path and sanitize binary diff evidence
- [x] Restore historical migration v5 body and add stable migration v7
- [x] Make Project Registry the live Tauri UI identity source with explicit route states
- [x] Apply bounded single-viewport UI corrections without redesigning the dashboard
- [x] Maintain production no-bundle publication, readiness, rollback, and shortcut checks
- [ ] Complete the independent focused R01-R15 evidence matrix
- [ ] User visual acceptance of refreshed Desktop H!veAI.lnk
- [ ] M01 actual in-app restart acceptance

M07.05 is the active strict remediation gate. Automated frontend, Rust, formatting,
production build, and publisher smoke gates pass; the milestone remains FAIL until
the required focused evidence matrix is complete. Final UI status is PENDING USER
VISUAL ACCEPTANCE. M08 is unstarted.

## M07.06 Focused evidence and runtime truth closure
- [x] Deterministic Git diff boundary with no-textconv and real binary paths
- [x] Dedicated frontend-ready IPC command, ACL, and post-mount call
- [x] SHA-256-proven publisher rollback and isolated temp-only failure harness
- [x] Direct Git diff, migration, SQLite, watcher, and Registry focused fixtures
- [x] Live selected-project session state and name-only in-place project rail
- [x] Live Tauri task/workflow/brief placeholders are neutral and truthful
- [x] Frontend focused Command Center and topbar-surface matrix
- [x] Full frontend/Rust regression, audit, build, and no-bundle publication gates
- [ ] User visual acceptance of refreshed Desktop H!veAI.lnk
- [ ] M01 actual in-app restart acceptance

M07.06 strict audit verdict: FAIL. Evidence matrices incomplete; SQLite corruption-safe
preflight ordering defect. Superseded by M07.07 surgical remediation.

## M07.07 Claude surgical remediation
- [x] SQLite corruption-safe read-only integrity preflight before WAL configuration
- [x] Complete watcher focused evidence matrix (13 cases)
- [x] Complete Registry identity focused evidence matrix (15 cases)
- [x] Complete frontend live-Registry / route-race focused tests (9 cases)
- [x] Finish launcher failure evidence harness (9 mapped cases)
- [x] Truthful TASKS.md and M07.07 Codex log
- [ ] User visual acceptance of refreshed Desktop H!veAI.lnk
- [ ] M01 actual in-app restart acceptance

M07.07 strict audit = FAIL because several named failure-path tests were state-only
or simulated without exercising the claimed failure. M07.07 historical implementation
and log remain immutable. M07.07A is the active evidence-integrity closure. M08 remains
unstarted and blocked.

## M07.07A Evidence integrity closure
- [~] Replace misleading SQLite contention and backup tests with real DB paths and private cfg(test) failpoints
- [~] Replace watcher name-only tests with real failpoint, lifecycle, root, event, Git, and persistence evidence
- [~] Replace frontend pre-baked states with mounted-app Registry transitions and same-instance route race
- [~] Replace publisher smoke placeholder with an actual spawned child-process cleanup assertion
- [ ] Independent M07.07A strict audit
- [ ] User visual acceptance of refreshed Desktop H!veAI.lnk
- [ ] M01 actual in-app restart acceptance

M07.07A remains ACTIVE until its independent audit. The Windows symlink containment
case is UNVERIFIED because link creation was denied by the environment with OS error
1314. Do not claim human-only pending status before independent audit. M08 remains
unstarted and blocked.

## M07.07B Final visual and restart closure
- [x] Replace visible sidebar branding with one-piece canonical H!veAI logo PNG
- [x] Expose native Settings -> Restart H!veAI action
- [x] Correct Projects Git metadata copy and remove milestone suffix from product chrome
- [x] Update durable sidebar branding governance while preserving shortcut small-logo rule
- [x] Run focused/full regression and production no-bundle publication
- [ ] User final M07 visual acceptance
- [x] User real in-app restart acceptance

M07.07B strict audit = CONDITIONAL. Command Center selection visual behavior was
user-approved from real Desktop H!veAI.lnk screenshots. M07.07C is the active final
visual/bookkeeping closure for the enlarged sidebar logo. M07 final closure still
requires user enlarged-logo visual acceptance and real in-app restart acceptance.
M08 remains blocked and unstarted.

## M07.07C Sidebar logo scale visual correction
- [x] Re-verify canonical source and repository logo identity without changing asset bytes
- [x] Make the visible one-piece sidebar logo width-driven and materially larger
- [x] Preserve the approximately 220 px sidebar and approved Command Center layout
- [x] Record M07.07B bookkeeping corrections prospectively in a new immutable log
- [x] Run automated regression gates and republish the stable no-bundle QA build
- [x] Independent M07.07C strict audit
- [x] User enlarged-logo visual acceptance
- [x] User real in-app restart acceptance (already PASS; no restart-path changes)

Native Settings restart manual gate = PASS, documented in
`docs/H!veAI/audits/M07.07B_NATIVE_RESTART_MANUAL_ACCEPTANCE.md`. M07.07C strict
audit = PASS, with visual acceptance documented in
`docs/H!veAI/audits/M07.07C_SIDEBAR_LOGO_MANUAL_ACCEPTANCE.md`.

## Current M07 closure truth

M07 strict audit and M07.07C audit are PASS. M07 is PASS/CLOSED. The real native
restart acceptance and enlarged-sidebar-logo visual acceptance are complete and
documented by their audits. Historical remediation notes above remain audit
history; they do not reopen M07.

## M08.00 App presentation bootstrap
- [x] Copy and verify the canonical global hive background asset
- [x] Copy and verify the canonical native opening video asset
- [x] Mount the native startup intro over an immediately mounted App
- [x] Keep intro play-once behavior scoped to the native lifecycle
- [x] Adapt restrained neon liquid glass styling to shared buttons, panels, and real tables
- [x] Preserve M07 Command Center geometry and behavior
- [x] Run focused and full frontend/native/publisher gates
- [x] Publish the validated no-bundle desktop QA build
- [ ] Independent M08.00 audit
- [ ] User visual and lifecycle acceptance

M08.00 presentation + neon-glass bootstrap is implemented and published, pending
independent audit and user visual/lifecycle acceptance. M08 Task Source Discovery
is not implemented by this pass and remains unstarted.

## M08 Task Source Discovery
- [ ] discover TASKS/tasks/PLANS/PROGRESS/ROADMAP/CLAUDE/AGENTS/handoffs
- [ ] custom paths
- [ ] priority/authority/freshness
- [ ] source UI

## M09 Task Intelligence Parser
- [ ] headings/checklists/milestones/status tags/task IDs
- [ ] blockers/next-step/owner-gate/external-wait parsing
- [ ] handoff current/next session parsing
- [ ] confidence/evidence locator
- [ ] generic adapter
- [ ] FormuLab/Scrubbots/FMCG adapters
- [ ] regression fixtures

## M10 Workflow State Machine
- [ ] canonical task/actor states
- [ ] transition matrix
- [ ] evidence requirements
- [ ] human override event
- [ ] blocked/waiting/design-gate states
- [ ] audit/fix/re-audit loop
- [ ] restart recovery

## M11 Global Command Center
- [ ] KPI strip
- [ ] project operation cards
- [ ] current task/progress/health/state
- [ ] last action/next action/required actor
- [ ] primary action
- [ ] Needs Your Attention
- [ ] Active Work Queue
- [ ] active agents/audits/waits
- [ ] live activity/search/filter/motion

## M12 Project Cockpit
- [ ] Overview/Tasks/Workflow/Agents/Audit/Git/Tests/Activity/Files/Settings
- [ ] Current Task hero
- [ ] pipeline
- [ ] evidence drawer
- [ ] last completed / last activity / next action
- [ ] manual correction controls

## M13 Codex adapter
- [ ] discover/version/auth readiness
- [ ] common agent interface
- [ ] start in repo/worktree cwd
- [ ] stdout/stderr/exit capture
- [ ] persist/resume/stop/recover
- [ ] stream events
- [ ] task/session mapping
- [ ] permissions

## M14 Agent Session Center
- [ ] PTY + xterm.js
- [ ] active sessions/status/timer/terminal
- [ ] event timeline/diff/changed files
- [ ] stop/retry
- [ ] restart recovery
- [ ] permission UI + notifications

## M15 Prompt Engine
- [ ] schemas/types/versioning
- [ ] immutable used versions
- [ ] context collector
- [ ] implementation/remediation prompts
- [ ] review/edit/approve/dispatch
- [ ] prompt-session provenance

## M16 GPT Audit Engine
- [ ] audit context + structured result
- [ ] diff/files/tests/architecture inspection
- [ ] findings/severity/coverage/confidence/risk
- [ ] PASS/FAIL transitions
- [ ] remediation prompt
- [ ] re-audit
- [ ] Audit Center

## M17 Claude Code adapter
- [ ] discover/version/auth
- [ ] start/resume/continue/stop
- [ ] structured stream/log mapping
- [ ] waiting permission/user detection
- [ ] completion/crash/orphan detection
- [ ] common adapter compliance

## M18 GitHub integration
- [ ] repositories/branches/commits/PRs/issues/Actions/releases
- [ ] local/remote reconciliation
- [ ] rate limits/offline cache
- [ ] failed-CI inspection/retry
- [ ] permission-gated PR creation

## M19 Next Best Task AI + Engineering Brief
- [ ] priority/dependency/blocker scoring
- [ ] owner/external gate awareness
- [ ] agent availability
- [ ] audit/CI priority
- [ ] critical-path/context-switch heuristic
- [ ] explainable project + portfolio recommendations
- [ ] morning brief / since-last-visit / attention synthesis

## M20 Project Chat + hardening + release
- [ ] portfolio/project Q&A
- [ ] action-capable chat with execution preview
- [ ] command palette
- [ ] keyboard navigation
- [ ] credential security/log redaction/process allowlisting
- [ ] DB backup/restore
- [ ] performance and large-task tests
- [ ] installer + clean-machine Windows test
- [ ] user/security/privacy docs
- [ ] release audit
- [ ] H!veAI v1.0.0
