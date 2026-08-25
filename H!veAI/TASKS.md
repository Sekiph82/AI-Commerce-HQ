# H!veAI MASTER TASKS

Legend: [x] validated complete, [~] active, [ ] planned/not started, [!] blocked.

## Current truth

- M00 through M07 are PASS/CLOSED.
- M01 real native Settings restart is PASS and user-accepted.
- M08.00/M08.00B presentation bootstrap is PASS/CLOSED, including opening video, post-sidebar hive background alignment, and restrained glass/glow styling.
- M08 Task Source Discovery is the only active milestone.
- M09 and later milestones are blocked until M08 implementation, independent strict audit, and any required manual acceptance are complete.
- Strict completed milestone count: 8 / 20.

Historical M07 remediation details remain preserved in `docs/H!veAI/audits/`, `docs/H!veAI/prompts/`, and `docs/H!veAI/codex-logs/`. They are audit history only and are not active work.

---

## M00 Fresh start with dedicated application root
- [x] Prove Git root and H!veAI child application root
- [x] Verify official repository and branch
- [x] Correct remote to `Sekiph82/AI-Commerce-HQ`
- [x] Inspect old parent AI-Commerce-HQ application as source material
- [x] Establish H!veAI foundation docs, child-root governance, and migration baseline

M00 PASS/CLOSED.

## M01 Tauri 2 foundation
- [x] Upgrade Tauri packages and Rust APIs
- [x] Add Tauri 2 capabilities
- [x] Rename active app identity to H!veAI
- [x] Define app-data migration policy
- [x] Add native logging/notifications
- [x] Verify Windows launch/close/restart through real Settings -> Restart H!veAI flow

M01 PASS/CLOSED. Native restart acceptance is documented in `docs/H!veAI/audits/M07.07B_NATIVE_RESTART_MANUAL_ACCEPTANCE.md`.

## M02 UI shell/design system
- [x] Remove GameWorld from root flow
- [x] Remove Three.js from primary bundle
- [x] Dark-first design system
- [x] Sidebar/top command bar/router
- [x] Routes: Command Center, Projects, Cockpit, Agents, Audits, Settings
- [x] Accessible reusable UI primitives + Framer Motion
- [x] Loading/error/empty states

M02 PASS/CLOSED.

## M03 Runtime refactor
- [x] Stop commerce-specific orchestrators from H!veAI startup
- [x] Inventory Python responsibilities
- [x] Select Rust-native runtime architecture
- [x] Remove always-on sidecar requirement
- [x] Document final runtime boundary

M03 PASS/CLOSED.

## M04 SQLite + migrations
- [x] Versioned migration framework
- [x] projects/repositories/sources/snapshots
- [x] tasks/dependencies/events
- [x] prompts/versions
- [x] agent sessions/events/tool calls/permissions
- [x] audits/findings/test_runs/alerts/decisions
- [x] GitHub cache/settings
- [x] corruption-safe integrity preflight, migration backup, rollback, contention, and failure evidence

M04 PASS/CLOSED.

## M05 Project Registry
- [x] Add existing folder without mutation
- [x] Detect git/remotes/default branch/GitHub identity
- [x] Priority/builder/auditor/task-source settings
- [x] Path repair/archive/remove-from-registry
- [x] Search/sort/filter
- [x] Registry identity repair evidence matrix

M05 PASS/CLOSED.

## M06 Local Git Engine
- [x] branch/HEAD/status/staged/unstaged/untracked
- [x] remote/ahead-behind/commits/diff/conflicts/worktrees
- [x] deterministic safe diff with `--no-ext-diff` / `--no-textconv`
- [x] binary diff metadata handling
- [x] default-denied mutation boundary
- [x] temp-repo/direct production-path tests

M06 PASS/CLOSED.

## M07 Filesystem Watcher + snapshots
- [x] watch registered project roots/task-relevant files/.hiveai
- [x] debounce and bounded watcher lifecycle
- [x] detect moved/missing repos
- [x] project snapshots/evidence timestamps
- [x] trigger bounded task/git refresh categories
- [x] large-repo protections
- [x] repaired-root reattachment and actual event evidence
- [x] Git-category / non-Git-category snapshot evidence
- [x] watcher failure, persistence failure, drop cleanup, containment evidence
- [x] production no-bundle publisher, rollback, launcher, frontend-ready, and UI closure
- [x] real Settings restart acceptance
- [x] final enlarged one-piece H!veAI sidebar logo acceptance

M07 PASS/CLOSED. Final strict closure is documented in `docs/H!veAI/audits/M07.07C_SIDEBAR_LOGO_SCALE_VISUAL_CORRECTION_STRICT_AUDIT.md` and the manual logo acceptance in `docs/H!veAI/audits/M07.07C_SIDEBAR_LOGO_MANUAL_ACCEPTANCE.md`.

### Historical M07 remediation chain

The following are superseded historical gates, not active tasks:

- M07.02: historical strict-remediation continuation, superseded.
- M07.03: historical consolidated strict closure attempt, failed and superseded.
- M07.04: historical automated remediation attempt, superseded.
- M07.05: historical bounded correctness/evidence remediation, superseded.
- M07.06: historical focused evidence closure, strict audit FAIL and superseded.
- M07.07: historical Claude surgical remediation, strict audit FAIL because several named tests did not exercise the claimed failures.
- M07.07A: historical evidence-integrity closure stage, superseded by later closure work.
- M07.07B: historical final visual/restart closure, CONDITIONAL before final logo scale acceptance.
- M07.07C: PASS and final M07 closure.

Do not treat any historical M07 subsection as active work. Detailed evidence remains immutable in its original audit/log/prompt files.

---

## M08.00 App presentation bootstrap
- [x] Copy and verify canonical global hive background asset
- [x] Copy and verify canonical native opening video asset
- [x] Mount startup intro over immediately mounted App
- [x] Preserve frontend-ready independence
- [x] Add restrained neon liquid-glass styling to buttons, panels, tables, and focus states
- [x] Preserve M07 Command Center geometry and behavior
- [x] Run focused and full frontend/native/publisher gates
- [x] Publish the validated no-bundle desktop QA build
- [x] Independent M08.00 audit
- [x] User visual and lifecycle acceptance

M08.00 presentation + neon-glass bootstrap is PASS/CLOSED with independent audit
and user visual/lifecycle acceptance.

## M08.00B Background alignment and native intro remediation
- [x] Move the canonical background to the post-sidebar main workspace
- [x] Add a fixed fullscreen startup overlay with contained video and no normal-flow overflow
- [x] Add a native process-scoped startup claim command and narrow ACL permission
- [x] Add focused frontend and Rust evidence for claim, failure, lifecycle, and layout behavior
- [x] Independent M08.00B strict audit
- [x] User manual acceptance of the refreshed native build

M08.00B is PASS/CLOSED with independent audit and user manual acceptance.

- [x] Run focused/full frontend, Rust, publisher and no-bundle publication gates
- [x] Fix startup video normal-flow/scrollbar defect
- [x] Move hive background to post-sidebar `.main-area`
- [x] Add native process-scoped startup intro claim
- [x] Cold-launch intro manual acceptance
- [x] Native restart intro replay manual acceptance
- [x] Post-sidebar background alignment manual acceptance

M08.00/M08.00B PASS/CLOSED. Strict audit: `docs/H!veAI/audits/M08.00B_BACKGROUND_ALIGNMENT_AND_NATIVE_INTRO_FIX_STRICT_AUDIT.md`. Manual acceptance: `docs/H!veAI/audits/M08.00B_MANUAL_ACCEPTANCE.md`.

## M08 Task Source Discovery
- [x] Original bounded source-discovery implementation
- [x] Native `/tasks` Task Sources workspace and narrow ACL
- [ ] Independent strict audit closure: original M08 audit is FAIL
- [ ] User visual acceptance of the native Task Sources workspace

M08 original implementation remains historical evidence. Its strict audit is FAIL;
M08A is the only active closure remediation. M09 remains blocked and unstarted.

## M08A Task Source Discovery Strict Closure
- [x] Close F01 filesystem work bounds and structured warning evidence
- [x] Close F02 custom update, persisted order, and equivalent-path removal
- [x] Close F03 M08-owned, versioned, non-destructive `project_sources` reconciliation
- [x] Close F04-F05 mounted stale list/mutation transitions and truthful UI evidence
- [x] Close F06 direct SQLite, unreadable-source, limits, status, and containment evidence
- [x] Close F07 immutable remediation log with individual test/equality evidence
- [x] Close N01 archived-project boundary and N02 containment-aware custom status
- [ ] Independent M08 strict re-audit
- [ ] User visual acceptance of the remediated native Task Sources workspace

M08A remains ACTIVE until every strict-audit major finding has direct production
and evidence closure. M09 remains blocked and unstarted.

## M09 Task Intelligence Parser
- [ ] headings/checklists/milestones/status tags/task IDs
- [ ] blockers/next-step/owner-gate/external-wait parsing
- [ ] handoff current/next session parsing
- [ ] confidence/evidence locator
- [ ] generic adapter
- [ ] FormuLab/Scrubbots/FMCG adapters
- [ ] regression fixtures

M09 BLOCKED until M08 PASS.

## M10 Workflow State Machine
- [ ] canonical task/actor states
- [ ] transition matrix
- [ ] evidence requirements
- [ ] human override event
- [ ] blocked/waiting/design-gate states
- [ ] audit/fix/re-audit loop
- [ ] restart recovery

## M11 Global Command Center
- [ ] KPI strip backed by live task intelligence
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

---

## Milestone policy

- The active milestone is M08 Task Source Discovery only.
- M08 internal numbered sections inside the authoritative prompt are work sections, not separate milestones or separate prompts.
- Do not start M09 until M08 implementation is independently audited PASS.
- Builder logs are claims, not acceptance evidence.
- Historical audits/logs/prompts are immutable evidence and remain the authoritative record for failed/superseded remediation stages.
