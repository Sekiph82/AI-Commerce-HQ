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
- [ ] Versioned migration framework
- [ ] projects/repositories/sources/snapshots
- [ ] tasks/dependencies/events
- [ ] prompts/versions
- [ ] agent sessions/events/tool calls/permissions
- [ ] audits/findings/test_runs/alerts/decisions
- [ ] GitHub cache/settings
- [ ] migration tests and failure recovery

## M05 Project Registry
- [ ] Add existing folder without mutation
- [ ] Detect git/remotes/default branch/GitHub identity
- [ ] Priority/builder/auditor/task-source settings
- [ ] Path repair/archive/remove-from-registry
- [ ] Search/sort/filter

## M06 Local Git Engine
- [ ] branch/HEAD/status/staged/unstaged/untracked
- [ ] remote/ahead-behind/commits/diff/conflicts/worktrees
- [ ] safe branch/commit/push interfaces
- [ ] temp-repo tests

## M07 Filesystem Watcher + snapshots
- [ ] watch project roots/task files/.hiveai
- [ ] debounce
- [ ] detect moved/missing repos
- [ ] project snapshots/evidence timestamps
- [ ] trigger task/git refresh
- [ ] large-repo protections

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
