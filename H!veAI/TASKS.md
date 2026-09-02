# H!veAI MASTER TASKS

Legend: `[x]` validated complete, `[~]` active/in progress, `[ ]` planned/pending, `[!]` blocked.

Package numbering such as `M08.01`, `M08.02`, etc. is a task/audit decomposition only. It does **not** mean a milestone must be split into separate Codex prompts. A milestone may still be implemented from one whole-milestone prompt.

## Canonical tracking rules

- This file is the canonical detailed milestone/task ledger.
- `CODEX_ROADMAP.md` mirrors milestone scope/dependencies at roadmap level.
- `docs/H!veAI/prompts/` contains authoritative builder prompts.
- `docs/H!veAI/codex-logs/` contains immutable builder execution claims/evidence logs.
- `docs/H!veAI/audits/` contains independent strict audits and final acceptance decisions.
- Builder logs are claims, not acceptance evidence.
- Historical failed prompts/logs/audits remain immutable and do not become active tasks again unless an independent later audit explicitly reopens a production defect.
- User-facing roadmap denominator remains **20** even though the historical baseline is labeled `M00` and release is `M20`.

## Current truth

- M00 through M12 are PASS/CLOSED.
- M08 presentation bootstrap, Task Source Discovery, remediation chain, and native `/tasks` manual acceptance are PASS/CLOSED.
- M09 original strict audit = historical FAIL.
- M09A strict re-audit = historical FAIL after closing the original seven findings but exposing two residual production defects.
- M09B strict re-audit = historical FAIL due residual R02C/E01C/E03C/E05 findings.
- M09C independent strict re-audit = historical CONDITIONAL, remediated by M09D.
- M09D independent final strict audit = PASS.
- M09 Task Intelligence Parser final closure = PASS/CLOSED.
- Pre-M10 Native UX Hotfix X01/X02 = PASS/CLOSED after independent source audit plus user native acceptance.
  - X01: terminal/console popup suppression accepted fixed after approximately 45 minutes of native runtime with no unwanted terminal windows.
  - X02: startup intro audio/replay behavior accepted fixed; audio works and same-process route navigation does not replay the intro.
- Strict completed milestone count is **14 / 20 = 70%** because M13 is now closed and the pre-M10 hotfix is not a numbered roadmap milestone.
- M10 original strict audit = historical FAIL with 5 MAJOR findings.
- M10A remediation, independent re-audit, and Akilta native click acceptance are complete; Akilta footer link = PASS/ACCEPTED.
- Original M11 implementation is a historical strict-audit FAIL with 8 MAJOR findings.
- M11A REV4, REV5, REV6, and REV7 remediation history remains immutable; all findings are closed by the accepted strict audits. M11A REV7 = PASS/CLOSED and the final Projects visual cleanup = PASS/CLOSED.
- M11 = PASS/CLOSED. M12, M12A R26, and M12B native Open Cockpit remediation = PASS/CLOSED on accepted strict evidence and user native/visual acceptance. M13/M13A/M13B/M13C/M13D/M13E = PASS/CLOSED on accepted strict re-audits and user native/visual evidence. M14 is ACTIVE/IMPLEMENTING; M15-M20 remain planned/blocked and M21 remains planned/not started.
- M21 remains planned and was not started.

---

# M00 - Fresh start with dedicated application root

### M00.01 - Repository/root proof
- [x] Prove the actual Git root.
- [x] Prove the dedicated `H!veAI` application child root.
- [x] Verify that `H!veAI` is not a nested Git repository.
- [x] Prevent accidental creation/use of `H!veAI/.git`.

### M00.02 - Canonical repository/branch
- [x] Verify GitHub repository `Sekiph82/AI-Commerce-HQ`.
- [x] Verify canonical branch `H!veAI`.
- [x] Correct/confirm remote configuration.
- [x] Establish safe synchronization rules without reset/rebase/force-push.

### M00.03 - Legacy source-material audit
- [x] Inspect the old parent AI-Commerce-HQ application.
- [x] Identify reusable desktop/runtime patterns.
- [x] Identify obsolete commerce/game-domain code that must not define H!veAI.
- [x] Treat the previous app as source material rather than the target architecture.

### M00.04 - Canonical documentation layout
- [x] Establish `H!veAI/docs/H!veAI/prompts/`.
- [x] Establish `H!veAI/docs/H!veAI/audits/`.
- [x] Establish `H!veAI/docs/H!veAI/codex-logs/`.
- [x] Establish version-controlled development protocol documentation.

### M00.05 - Governance baseline
- [x] Establish `AGENTS.md` audit/build governance.
- [x] Establish `CONSTITUTION.md` product/development constraints.
- [x] Establish builder-log-is-claim policy.
- [x] Establish independent audit before milestone closure.
- [x] Preserve user-owned root files outside milestone scope.

### M00.06 - Architecture/rebuild baseline
- [x] Establish H!veAI as a local-first AI Development Command Center.
- [x] Establish Tauri + React + TypeScript + Rust-native direction.
- [x] Establish future SQLite/domain/adapters roadmap.
- [x] Preserve parent repository as the Git root while application code stays under `H!veAI/`.

M00 PASS/CLOSED.

---

# M01 - Tauri 2 foundation

### M01.01 - Tauri 2 modernization
- [x] Upgrade Tauri dependencies/APIs to Tauri 2.
- [x] Establish Windows-native desktop entry point.
- [x] Preserve React/Vite integration.

### M01.02 - Capability/permission foundation
- [x] Add Tauri 2 capabilities.
- [x] Establish command allowlisting.
- [x] Avoid unrestricted frontend shell/filesystem privileges.

### M01.03 - H!veAI native identity
- [x] Rename active product identity to H!veAI.
- [x] Align native/application metadata.
- [x] Establish stable local application identity/path expectations.

### M01.04 - Native logging/notifications
- [x] Add native logging plugin foundation.
- [x] Add native notification foundation.
- [x] Establish native application log output location.

### M01.05 - Native status/readiness
- [x] Add bounded native status command.
- [x] Add frontend-ready signal.
- [x] Keep readiness independent from later startup visual effects.

### M01.06 - App-data migration policy
- [x] Define safe app-data migration policy.
- [x] Avoid destructive implicit migration behavior.
- [x] Preserve future database/runtime upgrade path.

### M01.07 - Native restart flow
- [x] Add native restart request command.
- [x] Route Settings restart through Tauri native lifecycle.
- [x] Avoid browser-only reload semantics.

### M01.08 - Real restart acceptance
- [x] Verify Windows launch/close behavior.
- [x] Verify Settings -> Restart H!veAI.
- [x] Obtain real user acceptance of native restart behavior.

M01 PASS/CLOSED.

---

# M02 - UI shell and design system

### M02.01 - Remove obsolete game-root experience
- [x] Remove GameWorld from the primary root flow.
- [x] Remove Three.js from the primary bundle/path where no longer needed.
- [x] Remove game/achievement-first presentation from the new command-center shell.

### M02.02 - Dark-first shell
- [x] Establish dark-first H!veAI design system.
- [x] Establish persistent application shell.
- [x] Establish sidebar and top command bar.

### M02.03 - Routing baseline
- [x] Add BrowserRouter routing.
- [x] Add Command Center route.
- [x] Add Projects route.
- [x] Add Project Cockpit route.
- [x] Add Tasks route foundation.
- [x] Add Agents route.
- [x] Add Audits route.
- [x] Add Activity route.
- [x] Add Settings route.

### M02.04 - Reusable UI primitives
- [x] Establish reusable accessible UI primitives.
- [x] Integrate controlled Framer Motion usage.
- [x] Establish keyboard/focus-friendly interaction baseline.

### M02.05 - Truthful application states
- [x] Add loading states.
- [x] Add bounded error states.
- [x] Add empty states.
- [x] Avoid fake live operational claims in placeholder content.

### M02.06 - Desktop shell geometry
- [x] Establish professional AI command-center composition.
- [x] Establish ~220px sidebar geometry.
- [x] Establish primary desktop viewport behavior.
- [x] Prevent inappropriate outer body overflow in accepted core layouts.

M02 PASS/CLOSED.

---

# M03 - Runtime refactor

### M03.01 - Remove commerce startup coupling
- [x] Stop commerce-specific orchestrators from H!veAI startup.
- [x] Remove assumptions that H!veAI launches a commerce automation runtime.

### M03.02 - Runtime responsibility inventory
- [x] Inventory historical Python/backend responsibilities.
- [x] Separate genuinely reusable responsibilities from obsolete sidecar behavior.

### M03.03 - Rust-native runtime decision
- [x] Select Rust-native runtime architecture.
- [x] Define privileged host work as native Rust responsibility.
- [x] Keep frontend focused on UI/contracts.

### M03.04 - Remove always-on sidecar requirement
- [x] Remove always-on Python sidecar dependency.
- [x] Avoid Python as a mandatory desktop runtime dependency.
- [x] Preserve local-first app operation.

### M03.05 - Runtime documentation
- [x] Document final runtime boundary.
- [x] Document frontend/native responsibility split.
- [x] Preserve architecture basis for Git/watcher/process milestones.

M03 PASS/CLOSED.

---

# M04 - SQLite and migrations

### M04.01 - Versioned migration framework
- [x] Add versioned migration framework.
- [x] Add schema version tracking.
- [x] Add migration reporting.

### M04.02 - Project/repository/source tables
- [x] Add `projects`.
- [x] Add `repositories`.
- [x] Add `project_sources`.
- [x] Add `git_snapshots`.

### M04.03 - Task/workflow foundation tables
- [x] Add `tasks`.
- [x] Add `task_dependencies`.
- [x] Add `task_sources`.
- [x] Add `task_events`.
- [x] Establish required foreign-key behavior.

### M04.04 - Prompt tables
- [x] Add `prompts`.
- [x] Add immutable `prompt_versions` foundation.
- [x] Add task/project relationships.

### M04.05 - Agent/session tables
- [x] Add `agent_sessions`.
- [x] Add `agent_events`.
- [x] Add `agent_tool_calls`.
- [x] Add `permission_requests`.

### M04.06 - Audit/test/decision tables
- [x] Add `audits`.
- [x] Add `audit_findings`.
- [x] Add `test_runs`.
- [x] Add `alerts`.
- [x] Add `decisions`.

### M04.07 - GitHub/settings tables and indexes
- [x] Add `github_sync_state`.
- [x] Add scoped `settings`.
- [x] Add required indexes for project/task/session/audit/settings access.

### M04.08 - Corruption/migration safety
- [x] Add corruption-safe integrity preflight.
- [x] Add migration backup behavior.
- [x] Add rollback evidence.
- [x] Add contention behavior/evidence.
- [x] Add failure-path evidence.

M04 PASS/CLOSED.

---

# M05 - Project Registry

### M05.01 - Register existing folders safely
- [x] Register an existing local project folder.
- [x] Avoid mutating registered project files.
- [x] Normalize/canonicalize registered path.
- [x] Create stable project identity.

### M05.02 - Git/GitHub identity discovery
- [x] Detect Git repository status.
- [x] Detect remotes.
- [x] Detect default branch information.
- [x] Derive GitHub owner/repository identity where available.

### M05.03 - Project settings
- [x] Add project priority.
- [x] Add preferred builder setting foundation.
- [x] Add preferred auditor setting foundation.
- [x] Add task-source policy/settings foundation.

### M05.04 - Registry lifecycle
- [x] Add path repair.
- [x] Add archive operation.
- [x] Add remove-from-registry without deleting project files.
- [x] Handle moved/missing roots truthfully.

### M05.05 - Registry search/sort/filter
- [x] Add project search.
- [x] Add deterministic sort/filter behavior.
- [x] Preserve Registry-backed project selection.

### M05.06 - Registry identity evidence
- [x] Add direct production-path Registry tests.
- [x] Prove path/identity repair behavior.
- [x] Prove unrelated project data remains isolated.

M05 PASS/CLOSED.

---

# M06 - Local Git Engine

### M06.01 - Repository status snapshot
- [x] Read current branch.
- [x] Read HEAD SHA.
- [x] Read staged changes.
- [x] Read unstaged changes.
- [x] Read untracked files.

### M06.02 - Remote divergence
- [x] Detect remotes.
- [x] Detect upstream relationship.
- [x] Compute ahead/behind.
- [x] Handle absent upstream safely.

### M06.03 - Commit/history inspection
- [x] Read bounded recent commits.
- [x] Read worktree information.
- [x] Detect conflict state.

### M06.04 - Safe diff engine
- [x] Generate deterministic bounded diffs.
- [x] Use `--no-ext-diff`.
- [x] Use `--no-textconv`.
- [x] Preserve binary diff metadata without treating binary content as text.

### M06.05 - Read-only/default-denied mutation boundary
- [x] Expose read-only Git inspection by default.
- [x] Keep write/mutation operations default-denied.
- [x] Reserve future Git mutations for permission-gated workflows.

### M06.06 - Native IPC/ACL
- [x] Add bounded Git snapshot command.
- [x] Add bounded Git diff command.
- [x] Add mutation-status boundary.
- [x] Add narrow Tauri permission/capability entries.

### M06.07 - Direct Git evidence
- [x] Use temporary real Git repositories in tests.
- [x] Exercise production command paths.
- [x] Validate status/diff/remote/worktree edge cases.
- [x] Validate binary diff handling.

M06 PASS/CLOSED.

---

# M07 - Filesystem Watcher and snapshots

### M07.01 - Watch registered projects
- [x] Watch Registry-backed project roots.
- [x] Watch task-relevant files/paths.
- [x] Watch `.hiveai`-relevant changes within bounded scope.

### M07.02 - Debounce and lifecycle bounds
- [x] Debounce filesystem event bursts.
- [x] Bound watcher count/lifecycle.
- [x] Avoid leaking watcher resources.
- [x] Clean up watchers on drop/shutdown.

### M07.03 - Missing/moved project detection
- [x] Detect moved/missing project roots.
- [x] Surface truthful watcher state.
- [x] Support repaired-root reattachment.

### M07.04 - Snapshot categories
- [x] Persist watcher/project snapshot evidence.
- [x] Distinguish Git-relevant refresh category.
- [x] Distinguish task/source-relevant refresh category.
- [x] Keep unrelated filesystem noise bounded.

### M07.05 - Git refresh integration
- [x] Trigger bounded Git snapshot refresh from Git-category changes.
- [x] Preserve Git Engine as the Git authority.
- [x] Record actual event evidence.

### M07.06 - Failure and persistence evidence
- [x] Exercise watcher failure behavior.
- [x] Exercise persistence failure behavior.
- [x] Exercise repaired-root behavior.
- [x] Exercise drop/cleanup behavior.
- [x] Exercise containment behavior.

### M07.07 - Production QA publisher
- [x] Establish production `--no-bundle` QA build flow.
- [x] Add publisher failure/rollback harness.
- [x] Publish stable `H!veAI/dev-bin/H!veAI.exe`.
- [x] Preserve stable desktop launcher/shortcut.
- [x] Validate frontend-ready marker.
- [x] Avoid dev-server dependency/forbidden dev ports.

### M07.08 - Launcher/icon behavior
- [x] Desktop shortcut targets stable EXE directly.
- [x] Shortcut does not launch BAT/CMD/PowerShell/npm/cargo/browser.
- [x] Shortcut uses stable H!veAI icon derived from canonical small logo.

### M07.09 - Global brand shell acceptance
- [x] Use one-piece H!veAI sidebar logo.
- [x] Remove separate emblem/text duplication.
- [x] Enlarge sidebar logo to accepted scale.
- [x] Obtain final manual logo acceptance.

### M07.10 - Restart/publisher closure
- [x] Verify native restart in published QA application.
- [x] Verify publisher rollback behavior.
- [x] Close remediation/evidence loop through final independent audit.

### Historical M07 remediation record
- [x] M07.02 historical strict-remediation continuation preserved.
- [x] M07.03 historical consolidated strict closure attempt preserved.
- [x] M07.04 historical automated remediation attempt preserved.
- [x] M07.05 historical bounded correctness/evidence remediation preserved.
- [x] M07.06 historical focused evidence closure preserved.
- [x] M07.07 historical Claude surgical remediation preserved.
- [x] M07.07A historical evidence-integrity stage preserved.
- [x] M07.07B native restart/final visual stage preserved.
- [x] M07.07C final PASS closure preserved.

M07 PASS/CLOSED.

---

# M08 - Task Source Discovery

### M08.00 - Presentation bootstrap
- [x] Copy/verify canonical global hive background asset.
- [x] Copy/verify canonical opening video asset.
- [x] Mount startup intro over the immediately mounted application.
- [x] Keep frontend-ready independent from intro duration.
- [x] Add restrained dark-glass/cyan-blue-violet styling.
- [x] Preserve accepted Command Center geometry.
- [x] Publish production QA build.
- [x] Obtain independent audit and user visual/lifecycle acceptance.

### M08.01 - Background alignment/native intro correction
- [x] Move canonical background to post-sidebar workspace.
- [x] Make startup intro a fixed fullscreen overlay rather than normal flow.
- [x] Remove intro-caused outer scrollbars.
- [x] Add process-scoped native startup-intro claim.
- [x] Show intro once per native process.
- [x] Replay intro on real native restart/new process.
- [x] Skip native-only intro behavior in browser preview.
- [x] Obtain user acceptance for video/background lifecycle.

### M08.02 - Discovery source contract
- [x] Make M08 the sole source-discovery authority for later parser milestones.
- [x] Define source metadata contract.
- [x] Define source status model: AVAILABLE/MISSING/UNREADABLE/TOO_LARGE/LIMIT_REACHED.
- [x] Define source origin/authority/priority/order semantics.

### M08.03 - Root standard sources
- [x] Discover `TASKS.md` / `TASK.md`.
- [x] Discover `PLANS.md` / `PLAN.md`.
- [x] Discover `PROGRESS.md`.
- [x] Discover `ROADMAP.md`.
- [x] Discover `HANDOFF.md` / `SESSION_HANDOFF.md`.
- [x] Discover general root `*handoff*.md` family.
- [x] Discover `AGENTS.md` and `CLAUDE.md` as instruction-class sources.

### M08.04 - Approved recursive directories
- [x] Recursively inspect `tasks/`.
- [x] Recursively inspect `plans/`.
- [x] Recursively inspect `handoffs/`.
- [x] Recursively inspect `.hiveai/` within discovery bounds.
- [x] Ignore `.git`, node_modules, dist/build/target/.next/coverage/cache/venv/vendor families.
- [x] Restrict discovery to plausible task-source file types.

### M08.05 - Filesystem work bounds
- [x] Maximum discovery depth = 4.
- [x] Maximum visited entries = 4096.
- [x] Maximum candidate files = 512.
- [x] Maximum source size/hash read = 2 MiB.
- [x] Maximum configured custom paths = 64.
- [x] Fix depth off-by-one behavior.
- [x] Emit structured discovery warning when a bound is reached.
- [x] Count actual filesystem work, not only accepted files.

### M08.06 - Physical containment and safety
- [x] Canonicalize registered project root.
- [x] Canonicalize physical source target.
- [x] Reject traversal outside root.
- [x] Reject absolute custom path outside root.
- [x] Reject physical symlink/junction escape outside root.
- [x] Isolate unreadable sources.
- [x] Preserve Windows OS-1314 link-test limitation as UNVERIFIED where applicable.

### M08.07 - Hash/evidence metadata
- [x] Compute SHA-256 for AVAILABLE source.
- [x] Record relative/absolute path.
- [x] Record source kind/origin/status/authority/priority.
- [x] Record size/modified/discovered time.
- [x] Record depth/warnings/schemaVersion/owner/sourceOrder.
- [x] Preserve deterministic source identity.
- [x] Prove persisted hash changes when source content changes.

### M08.08 - Custom source path CRUD
- [x] List custom source paths.
- [x] Add custom source path.
- [x] Remove custom source path.
- [x] Update/rename custom source path.
- [x] Reject duplicate/equivalent paths.
- [x] Handle case-equivalent path comparisons safely.
- [x] Surface CONFIGURED/MISSING/OUTSIDE_ROOT/UNREADABLE status.

### M08.09 - Positional custom ordering
- [x] Persist explicit custom order.
- [x] Implement true positional remove -> insert -> renumber semantics.
- [x] Prevent lexical tie-break from undoing requested reorder.
- [x] Keep custom sources before standard sources.
- [x] Prove visible UI order changes after native reorder.

### M08.10 - Legacy custom-order compatibility
- [x] Read pre-order M08 custom settings that lack `order`.
- [x] Normalize missing orders by persisted vector position.
- [x] Preserve normalized position during path-only rename.
- [x] Persist explicit contiguous `0..n` order on next mutation.
- [x] Directly verify normalized JSON/order persistence.

### M08.11 - SQLite `project_sources` reconciliation
- [x] Persist M08 inventory to `project_sources`.
- [x] Add explicit M08 owner/schema metadata.
- [x] Avoid blanket `DELETE WHERE project_id=?`.
- [x] Reconcile only M08-owned/current rows.
- [x] Narrow pre-version M08 adoption predicate.
- [x] Preserve unrelated/legacy rows.
- [x] Remove stale STANDARD source row when the physical source disappears.
- [x] Prove idempotent unchanged discovery.

### M08.12 - Project lifecycle boundary
- [x] ACTIVE project may discover.
- [x] MISSING project returns bounded unavailable error.
- [x] ARCHIVED project is rejected.
- [x] Missing registered root is handled safely.

### M08.13 - Native IPC and ACL
- [x] `hiveai_task_sources_discover`.
- [x] `hiveai_task_sources_list`.
- [x] `hiveai_task_source_custom_paths_list`.
- [x] `hiveai_task_source_custom_path_add`.
- [x] `hiveai_task_source_custom_path_remove`.
- [x] `hiveai_task_source_custom_path_update`.
- [x] Register narrow `allow-task-source-discovery` permission/capability.

### M08.14 - Task Sources workspace
- [x] Implement native `/tasks` Task Sources workspace.
- [x] Use Registry selected project.
- [x] Render source inventory table.
- [x] Render path/kind/origin/authority/priority/modified/status metadata.
- [x] Render Custom Source Paths panel.
- [x] Provide add/remove/update/reorder actions.
- [x] Provide Rescan Sources action.
- [x] Provide loading/error/empty states.
- [x] Avoid fake live task/workflow claims.

### M08.15 - Project-switch race safety
- [x] Add selected-project reference guard.
- [x] Add request-generation guard.
- [x] Prevent stale A-list response from overwriting B.
- [x] Prevent stale A-add refresh from overwriting B.
- [x] Prevent stale A-remove refresh from overwriting B.
- [x] Keep reorder refresh scoped to the current project.

### M08.16 - Frontend visible-state evidence
- [x] Prove rescan changes visible DOM inventory.
- [x] Prove custom add becomes visible after refresh.
- [x] Prove custom remove disappears after refresh.
- [x] Prove custom reorder changes DOM order.
- [x] Prove metadata columns render correctly.
- [x] Prove error state actually renders.

### M08.17 - Rust direct evidence matrix
- [x] Root standard source discovery.
- [x] Recursive approved-directory discovery.
- [x] Handoff wildcard family.
- [x] Visited-entry/candidate/depth limits.
- [x] Too-large/unreadable isolation.
- [x] Hash-change persistence.
- [x] AVAILABLE -> MISSING transition.
- [x] Removed STANDARD reconciliation.
- [x] Legacy row preservation.
- [x] Owner/schema persistence.
- [x] Custom add/remove/update/reorder.
- [x] Multi-CUSTOM + multi-STANDARD exact order.
- [x] Legacy no-order backward compatibility.
- [x] ACTIVE/MISSING/ARCHIVED boundaries.
- [x] Idempotency.

### M08.18 - Regression/publication
- [x] Focused Rust tests.
- [x] Focused frontend tests.
- [x] Full frontend suite.
- [x] Full Rust suite.
- [x] Typecheck/build/npm audit.
- [x] cargo fmt/check/build.
- [x] Publisher failure harness.
- [x] Production Tauri `--no-bundle` QA publication.
- [x] Stable EXE/desktop shortcut validation.
- [x] Canonical asset hash validation.
- [x] No installer.

### M08.19 - Native visual/manual acceptance and closure
- [x] Open Task Sources in native app.
- [x] Switch among multiple registered projects.
- [x] Confirm correct per-project inventory.
- [x] Confirm readable table/panel layout.
- [x] Confirm no sidebar/topbar/background/glass regression.
- [x] Preserve historical M08/M08A/M08B/M08C evidence.
- [x] Final M08 PASS/CLOSED.
- [x] Unlock M09.

M08 PASS/CLOSED.

---

# M09 - Task Intelligence Parser

### M09.01 - M08-to-M09 source boundary
- [x] Consume only M08-owned AVAILABLE inventory.
- [x] Exclude instruction-only AGENTS/CLAUDE sources from task production.
- [x] Reconstruct source path under Registry root.
- [x] Canonicalize/contain physical target.
- [x] Bounded UTF-8 read with SHA-256 verification.
- [x] Avoid a second independent filesystem crawler.

### M09.02 - Source-change retry
- [x] Detect hash mismatch between M08 evidence and read body.
- [x] Perform exactly one M08 rediscovery.
- [x] Re-resolve/re-canonicalize refreshed path.
- [x] Perform one second bounded read.
- [x] Accept stable single edit.
- [x] Skip after a second mutation with structured warning.

### M09.03 - Normalized task model
- [x] Define deterministic task snapshot model.
- [x] Include project/source identity.
- [x] Include title/status/storage state.
- [x] Include explicit ID/milestone/actor.
- [x] Include blockers/dependencies/next step/owner gate/external wait/acceptance criteria.
- [x] Include confidence/evidence/warnings/adapter ID.

### M09.04 - Generic Markdown parser
- [x] Parse headings/milestone context.
- [x] Parse `[ ]`, `[x]`, `[~]`, `[!]` checklist markers.
- [x] Parse explicit task rows/IDs.
- [x] Parse WAITING/READY/IN PROGRESS prefix status tags.
- [x] Keep status words in casual prose from overriding structured status.
- [x] Use neutral M09 storage mapping rather than implementing M10.

### M09.05 - Structured metadata
- [x] Parse inline blocker/dependency/next/actor/wait/acceptance labels.
- [x] Parse nested metadata blocks.
- [x] Keep nested values attached to the nearest task.
- [x] Parse owner gate separately from required actor.
- [x] Normalize known actors.
- [x] Preserve unknown actor source evidence without inventing canonical actor.
- [x] Avoid free-prose blocker inference.

### M09.06 - Handoff intelligence
- [x] Parse Current summary.
- [x] Parse Next summary.
- [x] Parse Blocker summary.
- [x] Parse Waiting/External summary.
- [x] Keep narrative summary separate from checklist tasks.
- [x] Merge multiple HANDOFF sources deterministically in M08 order.
- [x] Preserve one-based evidence locators.

### M09.07 - Deterministic task identity
- [x] Explicit-ID task identity survives unrelated line insertion/movement.
- [x] Fallback identity uses normalized heading path/title.
- [x] Identical sibling tasks remain distinct/repeatable.
- [x] Same text across different projects never collides.
- [x] Normalize heading case/whitespace for semantic identity.
- [x] M09B path-specific identity collision fix implemented; independent M09B re-audit accepted R01 while M09B remained historical FAIL.

### M09.08 - Source/path identity correctness
- [x] Preserve meaningful filesystem whitespace in path identity.
- [x] Normalize separators/path syntax without prose whitespace collapse.
- [x] Apply platform-equivalent case policy safely.
- [x] Prove `plans/a b.md` and `plans/a  b.md` produce distinct task IDs and both survive SQLite persistence.
- [x] Independent M09B re-audit accepted R01 while M09B remained historical FAIL.

### M09.09 - Parser bounds and warning model
- [x] Project-wide max task budget = 4096.
- [x] Structured `TASK_LIMIT_REACHED`.
- [x] UTF-8-safe 4096-byte field bounds for task-body fields.
- [x] Metadata entry bound = 128 with specific warning.
- [x] Project warning cap = 512.
- [x] M09B bounded explicit ID, milestone/headings, evidence heading path, locator text, handoff values and other source-derived persisted scalars.
- [x] M09B deduplicated repeated equivalent truncation warnings.
- [x] M09C fixes residual R02C bounded working-identity defect.
- [x] M09C independent strict re-audit recorded as CONDITIONAL with one bounded E01C evidence item.
- [x] M09D strengthens retry-containment evidence with a canonicalizable outside-root fixture and exact containment-message assertion.
- [x] Independent M09D final strict audit = PASS and final M09 closure accepted.

### M09.10 - Adapter boundary
- [x] Generic adapter safe fallback.
- [x] Exact Registry identity selects eligible special adapter.
- [x] `conventionMatched` stays false until actual structure matches.
- [x] FormuLab FVL-specific convention evidenced and bonus-gated.
- [x] Generic TASK syntax in FormuLab receives no FormuLab bonus.
- [x] ScrubBots generic-safe adapter selection.
- [x] FMCG ERP generic-safe adapter selection.
- [x] ScrubBots distinct non-generic convention truthfully UNVERIFIED.
- [x] FMCG distinct non-generic convention truthfully UNVERIFIED.

### M09.11 - Confidence/evidence locators
- [x] Deterministic confidence score/reasons.
- [x] Explicit ID/context/metadata reasons.
- [x] Adapter bonus only for evidenced per-task match.
- [x] Source path/content hash/start/end lines/heading path locator.
- [x] M09B bounds all source-derived evidence scalars; M09C leaves persisted evidence bounds unchanged.

### M09.12 - SQLite persistence
- [x] Persist M09-owned `m09src:` task sources.
- [x] Persist M09-owned `m09task:` tasks.
- [x] Persist owner/schema metadata.
- [x] Preserve unrelated/legacy tasks/sources/settings.
- [x] Do not write `task_events` from M09.
- [x] UPSERT stable tasks rather than delete/reinsert.
- [x] Preserve `created_at`/existing task events for stable identity.
- [x] Selectively remove only stale M09-owned tasks/sources.

### M09.13 - Dependency persistence
- [x] Resolve unambiguous explicit task references.
- [x] Keep unresolved/ambiguous references in metadata with warning.
- [x] Persist `SOURCE_EXPLICIT` dependency edges transactionally.
- [x] Reconcile dependency edges idempotently without duplicates.

### M09.14 - Native IPC/ACL
- [x] `hiveai_task_intelligence_parse`.
- [x] `hiveai_task_intelligence_list`.
- [x] Narrow `allow-task-intelligence` permission/capability.
- [x] No route-driven automatic parser worker.
- [x] TypeScript native contract wrappers.

### M09.15 - Direct evidence matrix
- [x] Stable one-edit retry.
- [x] Second mutation after retry.
- [x] Outside-root/invalid UTF-8 isolation.
- [x] Project-wide task limit.
- [x] Multibyte scalar bound.
- [x] Metadata bound.
- [x] Nested metadata/owner gate/casual prose negative evidence.
- [x] ID movement/heading normalization/project isolation.
- [x] Prefix status tags.
- [x] Handoff section separation/merge.
- [x] Stable UPSERT/event-history preservation.
- [x] Stale task/source reconciliation.
- [x] Dependency idempotency.
- [x] M09B path-collision regression test implemented.
- [x] M09B oversized heading/handoff/explicit-ID/determinism tests implemented.
- [x] M09B strengthened retry containment/handoff exact order/stale-source evidence implemented.
- [x] M09C direct fixed identity key, task-ID stability, and stale dependency/settings tests implemented.
- [x] M09D direct retry-containment test uses a real canonicalizable outside-root file and exact warning-message assertion.

### M09.16 - M09 original strict audit history
- [x] Original M09 implementation completed by builder.
- [x] Independent strict audit found 7 MAJOR findings.
- [x] Historical M09 audit preserved as FAIL.

### M09.17 - M09A remediation history
- [x] Fix real source-change retry.
- [x] Fix project/scalar bounds for initial covered fields.
- [x] Fix name-only adapter bonus behavior.
- [x] Fix nested metadata/owner gate.
- [x] Fix movement identity tests/heading normalization.
- [x] Fix checklist status/handoff merge.
- [x] Fix delete/reinsert persistence with stable UPSERT.
- [x] Independent M09A re-audit completed.
- [x] M09A historical verdict = FAIL because R01/R02 remained.

### M09.18 - M09B bounded-identity micro-fix
- [x] Add path-specific task identity normalization.
- [x] Preserve meaningful repeated filename whitespace.
- [x] Bound all source-derived persisted scalars.
- [x] Add structured/deduplicated truncation evidence.
- [x] Strengthen retry-containment evidence.
- [x] Assert exact handoff merge order.
- [x] Add real stale-source reconciliation evidence.
- [x] Record ScrubBots/FMCG convention status truthfully as UNVERIFIED.
- [x] M09B implementation commit/log present.
- [x] Complete governed final publication/equality evidence.
- [x] Independent strict M09B re-audit found residual R02C/E01C/E03C/E05 defects.

### M09.19 - M09C final bounded identity micro-fix
- [x] Replace raw duplicate-ordinal working keys with fixed-size digest keys.
- [x] Stream task-ID identity hashing without giant formatted identity strings.
- [x] Add direct R02C fixed-key/stability/determinism tests.
- [x] Add retry-containment evidence test; direct containment proof remained conditional in the M09C audit.
- [x] Complete stale-source/settings/dependency SQL evidence.
- [x] Run full regression, security, publisher, and final remote equality gates.
- [x] Independent strict M09C re-audit completed as CONDITIONAL with 0 BLOCKER / 0 MAJOR / 1 MINOR.

### M09.20 - M09D retry-containment evidence closure
- [x] M09C production identity and publication evidence remain accepted; E01C direct retry evidence was the sole residual item.
- [x] M09D evidence-only retry-containment implementation/test work completed.
- [x] M09D focused/full regression and governed publication gates completed.
- [x] Independently verify final M09D branch/source truth.
- [x] Independently verify final M09D publication/equality evidence.

### M09.21 - M09 final regression/publication closure
- [x] Final M09 audit verdict = PASS.
- [x] Mark M09 PASS/CLOSED with no BLOCKER/MAJOR remaining.
- [x] Unlock M10 after the pre-M10 native UX hotfix independent audit and user native acceptance both PASS.

M09 PASS/CLOSED.

---

# PRE-M10 NATIVE UX HOTFIX QUEUE

### X01 - Suppress spawned Git console windows
- [x] Apply Windows `CREATE_NO_WINDOW`-style creation flags to Git child processes.
- [x] Preserve stdout/stderr capture and exit handling.
- [x] Prove watcher-triggered Git refresh still works.
- [x] Prove no visible console/terminal windows appear while H!veAI stays open: user observed approximately 45 minutes with no unwanted terminal windows.
- [x] Run Git Engine/watcher/full regression and republish QA EXE.
- [x] Native manual acceptance = PASS.

### X02 - Restore startup intro audio
- [x] Remove unconditional muted startup-video playback.
- [x] Configure reliable audible WebView2 autoplay without weakening unrelated security settings.
- [x] Preserve canonical opening video bytes.
- [x] Startup intro audio behavior accepted fixed by the user.
- [x] Same-process route navigation does not replay the intro.
- [x] Obtain user manual acceptance.
- [x] Native manual acceptance = PASS.

Closure evidence: `docs/H!veAI/audits/PRE_M10_NATIVE_UX_HOTFIX_X01_X02_MANUAL_ACCEPTANCE_CLOSURE.md`.

Pre-M10 Native UX Hotfix X01/X02 PASS/CLOSED.

---

# M10 - Workflow State Machine

### M10.01 - Canonical workflow states
- [x] Define canonical task states from backlog through completion.
- [x] Define builder/auditor/verification states.
- [x] Define blocked/waiting/design-gate states.
- [x] Separate parser truth from operational workflow truth.

### M10.02 - Actor model
- [x] Define Human/Codex/Claude/GPT Audit/CI/External actors.
- [x] Define required actor rules per transition.
- [x] Preserve unknown/external actor evidence safely.

### M10.03 - Transition matrix
- [x] Define allowed state transitions.
- [x] Reject invalid direct jumps.
- [x] Define happy path.
- [x] Define audit-failure/remediation/re-audit loop.
- [x] Define blocked/waiting resume transitions.

### M10.04 - Evidence requirements
- [x] Define evidence required for each transition.
- [x] Link source/task/test/audit/session evidence.
- [x] Prevent transition on missing/insufficient evidence.

### M10.05 - Human override
- [x] Add explicit human override event.
- [x] Require rationale.
- [x] Preserve immutable transition history.
- [x] Avoid silent state mutation.

### M10.06 - Persistent task events
- [x] Persist workflow transition events in `task_events`.
- [x] Preserve chronological state history.
- [x] Keep stable M09 task IDs as event anchors.

### M10.07 - Restart recovery
- [x] Reconstruct current operational state after app restart.
- [x] Detect interrupted/running states safely.
- [x] Avoid falsely claiming agents/audits are still running.

### M10.08 - Native service/IPC
- [x] Add narrow state-machine operations.
- [x] Add list/current-state operations.
- [x] Add permission boundaries for mutating transitions.

### M10.09 - Direct state-machine tests
- [x] Happy-path matrix tests.
- [x] Invalid-transition tests.
- [x] Evidence-gate tests.
- [x] Human-override tests.
- [x] blocked/waiting tests.
- [x] restart-recovery tests.

### M10.10 - Regression/audit/closure
- [x] Full Rust/frontend/security regression.
- [x] Production QA publication.
- [x] Independent strict audit.
- [x] Close M10 before M11/M12 live operational UI is unlocked.

M10 PASS/CLOSED.

M10A remediation, independent re-audit, and Akilta footer native click acceptance PASS/CLOSED.

---

# M11 - Global Command Center

### M11.00 - User-observed UX closure scope
- [x] P0 remove the bottom footer and move the complete Akilta attribution into the topbar between Workspace and Search Workspace.
- [x] P1 implement actual SINGLE_DASHBOARD watcher attachment/filtering for migrated projects, with legacy fallback.
- [x] P2 parse and consume bounded materialized `.hiveai/PROJECT_DASHBOARD.md` status sections without weakening M10 precedence.
- [x] P3 dogfood the extended dashboard contract on H!veAI's own `.hiveai/PROJECT_DASHBOARD.md`.
- [x] P4 correct informational ABSENT fallback and explicit configuration-attention semantics.
- [x] P5 omit unproved GPT Audit and CI actors from audit/test activity.
- [x] UX01 one-screen Command Center composition with no outer/nested operational-panel scrollbars at accepted desktop sizes.
- [x] UX02 remove the full-width home Recent Activity panel; retain compact selected-project activity and dedicated Activity navigation.
- [x] UX03 Project Dashboard Contract is the normal Task Sources surface; advanced M08 inventory remains available behind explicit disclosure.
- [x] UX04 preserve `.hiveai/PROJECT_DASHBOARD.md` as the single user-facing project entry contract without auto-rewriting project files.

### M11.01 - Live portfolio data model
- [x] Replace placeholder KPI data with Registry/task/workflow-backed data with unknown/error truth preserved.
- [x] Define and verify the complete portfolio aggregation contract.
- [x] Add and directly verify the native Project Dashboard authority resolver for `.hiveai/PROJECT_DASHBOARD.md` on registered projects.
- [x] Resolve declared task/handoff/roadmap/history/architecture/instruction roles without duplicating the same task truth across sources.
- [x] Fall back safely to existing M08/M09 authority when a manifest is absent, malformed, stale, or references unavailable files, with explicit degraded state.
- [x] Keep manifest reads bounded, containment-safe, deterministic, and project-scoped.
- [x] Avoid fake live metrics.

### M11.02 - KPI strip
- [x] Project count/health.
- [x] Active tasks with unknown semantics when M09 truth is unavailable.
- [x] Blocked/waiting attention counts from real workflow evidence.
- [x] Running agents/audits when real.
- [x] CI/audit health indicators where available.

### M11.03 - Project operation cards
- [x] Current task with M10 completion precedence.
- [x] Current workflow state.
- [x] Last action.
- [x] Next action.
- [x] Required actor.
- [x] Project health/progress.
- [x] Use resolved Project Dashboard authority roles for project summaries where available.

### M11.04 - Needs Your Attention
- [x] Human decisions/gates.
- [x] Blocked tasks.
- [x] External waits.
- [x] Failed audits/CI/test evidence.
- [x] Permission requests.

### M11.05 - Active Work Queue
- [x] Active builder sessions.
- [x] Active audits.
- [x] Pending verification.
- [x] Waiting/blocked work.

### M11.06 - AI Engineering Brief surface
- [x] Create deterministic data contract for brief inputs.
- [x] Surface current project/portfolio situation.
- [x] Include structured resolved dashboard source provenance so factual statements remain traceable.
- [x] Keep AI-generated recommendations clearly separate from factual state.

### M11.07 - Recent Activity
- [x] Show one deterministic mixed timeline of real task/workflow/agent/audit/test/Git/snapshot activity.
- [x] Add search/filter to the mixed timeline.
- [x] Bound long histories globally and deterministically.

### M11.08 - Selected-project interaction
- [x] Keep project rail names-only.
- [x] Click selects in place.
- [x] Current Project panel updates.
- [x] `Open cockpit` remains the explicit navigation action.
- [x] Session remembers selected project.

### M11.09 - Layout/performance/accessibility
- [x] Preserve accepted sidebar/background/glass system.
- [x] Avoid outer-body overflow at accepted desktop viewports.
- [x] Avoid unnecessary nested scrollbars.
- [x] Preserve keyboard/focus behavior.

### M11.10 - Tests/audit/closure
- [x] Mounted live-data tests including error/unknown/provenance states.
- [x] Stale project-switch/race tests.
- [x] Manifest present/absent/malformed/stale/cross-project/directory containment tests and actual native direct execution.
- [x] Prove authority resolution does not double-count duplicate task sources.
- [x] Full regression/publication after M11A REV5 remediation.
- [x] User visual acceptance if layout materially changes.
- [x] Independent strict audit.

M11A REV7 = PASS/CLOSED. M11 final Projects visual cleanup = PASS/CLOSED. M11 = PASS/CLOSED.

---

# M12 - Project Cockpit

### M12.01 - Cockpit shell/data loading
- [x] Project-specific route loading.
- [x] Async loading skeleton.
- [x] No fallback to another project on missing/late data.
- [x] Truthful missing/archived state.
- [x] Load the selected project's resolved Project Dashboard authority map without leaking another project's manifest/source state.

### M12.02 - Overview tab
- [x] Project identity/health.
- [x] Current task hero.
- [x] Current workflow state.
- [x] Last completed action.
- [x] Next action/required actor.
- [x] Show source provenance/authority where useful without overwhelming the user.

### M12.03 - Tasks tab
- [x] Parsed tasks with status/state distinction.
- [x] Dependencies/blockers/acceptance criteria.
- [x] Evidence drawer.
- [x] Source locator navigation foundation.
- [x] Respect manifest-declared canonical task authority and avoid duplicate task rendering.

### M12.04 - Workflow tab
- [x] State pipeline.
- [x] Transition history.
- [x] Evidence requirements.
- [x] Human override visibility/control.

### M12.05 - Agents tab
- [x] Project-scoped sessions.
- [x] Session status/duration/provider.
- [x] Permission/wait state.

### M12.06 - Audit tab
- [x] Latest audit verdict.
- [x] Findings/severity.
- [x] Requirement coverage.
- [x] Re-audit/remediation history.

### M12.07 - Git tab
- [x] Branch/HEAD/status.
- [x] Ahead/behind.
- [x] Changed files/diff.
- [x] Conflicts/worktrees.

### M12.08 - Tests/Activity/Files tabs
- [x] Test-run history.
- [x] Activity timeline.
- [x] Bounded relevant-file inventory.
- [x] Evidence links.
- [x] Surface manifest-declared roadmap/handoff/history/architecture sources as classified project context, not duplicate tasks.

### M12.09 - Project Settings tab
- [x] Registry settings.
- [x] Preferred builder/auditor.
- [x] Task-source policy/custom source entry points.
- [x] Show Project Dashboard manifest status, resolved authority roles, warnings, and source provenance.
- [x] Do not auto-rewrite project manifests or tracker files merely because H!veAI reads them.
- [x] Path repair/archive/remove-from-registry controls.

### M12.10 - Manual correction controls
- [x] Controlled human corrections.
- [x] Require rationale/evidence.
- [x] Record correction event.
- [x] Avoid silent state rewriting.

### M12.11 - Tests/audit/closure
- [x] Mounted project-switch/race tests.
- [x] Evidence rendering tests.
- [x] Project Dashboard authority/provenance rendering tests.
- [x] Full regression/publication.
- [x] User visual acceptance.
- [x] Independent strict audit and accepted M12B remediation audit.

M12 PASS/CLOSED.

### M12A - Project-wide workflow history strict remediation

- [x] R26: query the selected project's workflow history globally before applying the bounded 200-event cockpit limit.
- [x] Add adversarial cross-task starvation, deterministic tie-order, project-isolation, and derived-activity regression coverage.

M12A R26 REMEDIATION PASS/CLOSED.

### M12B - Native Open Cockpit route-loading remediation

- [x] Reproduce the native route failure against governed registered-project data and isolate the failing IPC permission boundary.
- [x] Allow the registered Project Cockpit snapshot command through the main-window capability with least privilege.
- [x] Preserve exact project IDs across Command Center and Projects navigation.
- [x] Distinguish unknown, registered unavailable, and native cockpit snapshot failures in the route error state.
- [x] Add direct frontend/native regression coverage and preserve M12A R26 coverage.
- [x] Run full regression and governed publication; repeat user native/visual acceptance remains pending.

M12B NATIVE OPEN COCKPIT REMEDIATION PASS/CLOSED.

---

# M13 - Codex Adapter

### M13.01 - Codex availability/readiness
- [x] Detect Codex installation/version.
- [x] Detect auth/readiness without exposing credentials.
- [x] Surface unavailable/misconfigured state truthfully.

### M13.02 - Common agent adapter contract
- [x] Implement provider-neutral availability/start/resume/stop/status contract.
- [x] Map Codex to common session/event model.

### M13.03 - Project-scoped process start
- [x] Start Codex in registered project/worktree cwd.
- [x] Validate cwd containment.
- [x] Avoid arbitrary shell execution.

### M13.04 - Session output capture
- [x] Capture stdout.
- [x] Capture stderr.
- [x] Capture exit code.
- [x] Stream bounded structured events.

### M13.05 - Task/session mapping
- [x] Attach session to one project.
- [x] Attach session to one task or explicit freeform operation.
- [x] Preserve prompt/version provenance when available.

### M13.06 - Resume/stop/recovery
- [x] Represent Codex resume as unsupported where no stable safe mechanism is available.
- [x] Stop process cleanly.
- [x] Detect crashed/orphaned process.
- [x] Recover truthful state after H!veAI restart.

### M13.07 - Permission boundary
- [x] Define allowed process launch arguments.
- [x] Block arbitrary command injection.
- [x] Record permission-sensitive operations.

### M13.08 - Direct process tests
- [x] Availability tests.
- [x] cwd/containment tests.
- [x] stdout/stderr/exit tests.
- [x] stop/crash/recovery tests.
- [x] malformed/injection input tests.

### M13.09 - Regression/audit/closure
- [x] Full security/process regression.
- [x] Production QA publication.
- [ ] Independent strict audit.

M13 PASS/CLOSED. M13A-M13E accepted strict remediation chain and user native/visual evidence are preserved as immutable provenance.

## M13A - Common adapter, streaming, and stop strict remediation

- [x] Close R27 with a provider-neutral native adapter contract implemented by Codex only.
- [x] Close R28 with bounded incremental structured stdout/stderr events and pre-persistence redaction.
- [x] Close R29 with truthful clean-stop limitation, bounded grace, and owned process-tree escalation.
- [x] Add adversarial common, streaming, process, security, and recovery regression tests.
- [x] Run full regression, security, and governed publication gates.
- [ ] Independent strict re-audit and user native/visual acceptance.

M13A REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE.

## M13B - Stream-safe redaction and durable event truth remediation

- [x] Close R30 with bounded stateful stream redaction before persistence.
- [x] Close R31 with a bounded single stream persistence writer, retry recovery, and explicit degradation evidence.
- [x] Add adversarial split-marker, UTF-8, cap, concurrency, retry, terminal-failure, durable-row, and terminal-state tests.
- [x] Run full regression, security, and governed publication gates.
- [ ] Independent strict re-audit and user native/visual acceptance.

M13B REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE.

## M13D - Post-startup console flash and real Codex operation failure remediation

- [x] Close R33 with one Windows no-visible-console policy for Codex, owned-process escalation, and production Git helper children.
- [x] Close R34 with a fixed compatible Codex invocation and truthful persisted failed-session evidence.
- [x] Surface bounded diagnostic code, message, exit code, and redacted stderr in Agents session details.
- [x] Run focused/full regression, security, and governed publication gates.
- [ ] Independent strict re-audit and user native/visual acceptance.

M13D REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE.

## M13C - Windows Codex executable resolution native remediation

- [x] Reproduce the earlier PATH extensionless shim selection and native Windows error 193.
- [x] Close R32 with deterministic native `codex.exe` resolution shared by readiness and start.
- [x] Add disposable adversarial resolver coverage for shim ordering, invalid candidates, first-valid ordering, unavailable state, and shared policy.
- [x] Run full regression, security, and governed publication gates.
- [ ] Independent strict re-audit and user native/visual acceptance.

M13C REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE.

## M13E - Agent session output vertical reader native UX remediation

- [x] Replace the horizontal persisted-session output presentation with a full-width vertical reader.
- [x] Wrap long JSON, paths, and unrecognized lines without changing persisted output truth.
- [x] Preserve completed/failed metadata, diagnostics, stderr, truncation markers, and redaction visibility.
- [x] Add focused frontend evidence for long output, completed output, failed diagnostics, and redaction markers.
- [x] Run full frontend/native regression, security, formatting, build, and governed publication gates.
- [ ] Independent strict re-audit and user native/visual acceptance.

M13E REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE.

---

# M14 - Agent Session Center

### M14.01 - PTY foundation
- [x] Add Rust PTY/process manager foundation and owned provider process lifecycle.
- [x] Add xterm.js terminal surface.
- [x] Keep process ownership native.

### M14.02 - Session list/status
- [x] Active sessions.
- [x] Provider/project/task.
- [x] Status/timer/start/end.
- [x] Waiting/permission/crash state.

### M14.03 - Live terminal
- [x] Stream terminal output.
- [x] Bound retained buffer/history.
- [x] Handle terminal resize.
- [x] Prevent secret leakage where possible.

### M14.04 - Session timeline
- [x] Agent events.
- [x] Tool calls.
- [x] Prompt/version reference.
- [x] Git/diff/test events.

### M14.05 - Diff/changed files
- [x] Project/session changed-file view.
- [x] Reuse Git Engine diff authority.
- [x] Avoid trusting agent claims alone.

### M14.06 - Stop/retry/recovery
- [x] Stop running session.
- [x] Retry failed operation with provenance.
- [x] Recover orphaned sessions after restart.

### M14.07 - Permission UI
- [x] Show truthful provider-managed permission limitation.
- [x] Approve/deny explicitly where a provider exposes a controllable mechanism.
- [x] Record unsupported decision attempts truthfully.
- [x] Use notifications for waiting attention where supported.

### M14.08 - Tests/audit/closure
- [x] PTY/process lifecycle tests and compile coverage.
- [x] UI stream tests.
- [x] restart recovery tests and controlled evidence.
- [x] security/redaction tests.
- [x] Full regression/publication attempted and documented with exact host blockers.
- [ ] Independent strict audit.

M14 IMPLEMENTATION COMPLETE / PENDING INDEPENDENT STRICT AUDIT + USER NATIVE/VISUAL ACCEPTANCE.

---

# M15 - Prompt Engine

### M15.01 - Prompt schemas/types
- [ ] Define prompt kinds.
- [ ] Define implementation/remediation/audit-support prompt structures.
- [ ] Define project/task/session provenance.

### M15.02 - Versioning
- [ ] Persist prompt versions.
- [ ] Never mutate a prompt version already used by a session.
- [ ] Track current version separately.

### M15.03 - Context collector
- [ ] Collect task requirements.
- [ ] Collect project/source evidence.
- [ ] Collect architecture/governance constraints.
- [ ] Collect relevant Git/test/audit context.
- [ ] Keep context bounded and explainable.

### M15.04 - Implementation prompt generation
- [ ] Generate builder-ready prompt from current task/context.
- [ ] Include exact acceptance behavior/tests.
- [ ] Avoid irrelevant governance noise.

### M15.05 - Remediation prompt generation
- [ ] Consume audit findings.
- [ ] Generate defect-focused remediation prompt.
- [ ] Require tests that fail on pre-fix behavior.

### M15.06 - Review/edit/approve
- [ ] Human can review prompt.
- [ ] Human can edit before dispatch.
- [ ] Human approval/dispatch is explicit.

### M15.07 - Dispatch/provenance
- [ ] Dispatch approved prompt to selected adapter.
- [ ] Attach exact prompt version to session.
- [ ] Preserve immutable provenance.

### M15.08 - Tests/audit/closure
- [ ] Version immutability tests.
- [ ] Context-bound tests.
- [ ] Prompt/session provenance tests.
- [ ] Full regression/publication.
- [ ] Independent strict audit.

M15 PLANNED/BLOCKED until M14.

---

# M16 - GPT Audit Engine

### M16.01 - Audit input contract
- [ ] Task requirements/acceptance criteria.
- [ ] Actual Git diff/changed files.
- [ ] Test results.
- [ ] Architecture/governance rules.
- [ ] Builder logs as secondary claims only.

### M16.02 - Structured audit result
- [ ] PASS/CONDITIONAL/FAIL verdict.
- [ ] BLOCKER/MAJOR/MINOR/NOTE severity.
- [ ] Requirement coverage.
- [ ] Confidence.
- [ ] Regression risk.

### M16.03 - Source-level verification
- [ ] Inspect production symbols/configuration.
- [ ] Inspect direct test bodies.
- [ ] Detect misleading test names/claims.
- [ ] Verify final branch/diff scope.

### M16.04 - Audit persistence
- [ ] Persist audit.
- [ ] Persist findings.
- [ ] Link project/task/session/test evidence.
- [ ] Preserve re-audit history.

### M16.05 - Remediation loop
- [ ] Convert failed findings into bounded remediation input.
- [ ] Dispatch through Prompt Engine after review/approval.
- [ ] Re-audit resulting implementation.

### M16.06 - Audit Center UI
- [ ] Current verdict.
- [ ] Findings/severity.
- [ ] Coverage/confidence/risk.
- [ ] Historical audit/remediation chain.

### M16.07 - Security/truthfulness
- [ ] Never allow advertiser/network influence on audit outcome.
- [ ] Never treat builder self-assessment as independent evidence.
- [ ] Clearly mark UNVERIFIED evidence.

### M16.08 - Tests/audit/closure
- [ ] Known-good/known-bad fixture audits.
- [ ] Misleading-test detection cases.
- [ ] Remediation/re-audit loop tests.
- [ ] Full regression/publication.
- [ ] Independent release-gate audit of the audit engine itself.

M16 PLANNED/BLOCKED until M15.

---

# M17 - Claude Code Adapter

### M17.01 - Claude availability/readiness
- [ ] Detect Claude Code installation/version.
- [ ] Detect auth/readiness safely.
- [ ] Surface unavailable/quota/waiting states truthfully.

### M17.02 - Common adapter compliance
- [ ] Implement same provider-neutral interface as Codex.
- [ ] Keep project/task/session identity consistent.

### M17.03 - Start/resume/continue/stop
- [ ] Start in registered project/worktree cwd.
- [ ] Resume/continue supported session.
- [ ] Stop cleanly.

### M17.04 - Structured stream mapping
- [ ] Map Claude output/events to common session events.
- [ ] Capture stdout/stderr/exit.
- [ ] Preserve bounded logs.

### M17.05 - Permission/wait detection
- [ ] Detect waiting-for-user state.
- [ ] Detect permission request.
- [ ] Detect rate/quota limitation where observable.
- [ ] Surface actionable attention state.

### M17.06 - Crash/orphan recovery
- [ ] Detect completion/crash/orphan.
- [ ] Recover truthful state after H!veAI restart.

### M17.07 - Security/process tests
- [ ] cwd containment.
- [ ] argument injection resistance.
- [ ] output/event mapping.
- [ ] stop/resume/recovery.

### M17.08 - Regression/audit/closure
- [ ] Full process/UI/security regression.
- [ ] Production QA publication.
- [ ] Independent strict audit.

M17 PLANNED/BLOCKED until common session/prompt/audit infrastructure exists.

---

# M18 - GitHub Integration

### M18.01 - Repository/branch/commit reads
- [ ] Read repository identity.
- [ ] Read branches/commits.
- [ ] Reconcile local branch/HEAD with remote.

### M18.02 - Pull requests
- [ ] Read PR metadata/diff/status/comments.
- [ ] Link PR to project/task/session where possible.
- [ ] Permission-gated PR creation only.

### M18.03 - Issues
- [ ] Read project-relevant issues.
- [ ] Map explicit issue/task relationships.
- [ ] Avoid guessing implicit ownership.

### M18.04 - GitHub Actions
- [ ] Read workflow runs/jobs/steps/log summaries.
- [ ] Surface failed CI.
- [ ] Permission-gated retry where supported.

### M18.05 - Releases
- [ ] Read releases/tags.
- [ ] Surface release state for project context.

### M18.06 - Cache/rate limits/offline behavior
- [ ] Persist bounded GitHub cache/sync cursor.
- [ ] Respect rate limits.
- [ ] Degrade truthfully when offline/stale.

### M18.07 - Local/remote reconciliation
- [ ] Compare local Git Engine state to GitHub remote state.
- [ ] Detect divergence/stale data.
- [ ] Never overwrite local work automatically.

### M18.08 - Security/permissions
- [ ] Least-privilege connector actions.
- [ ] Human approval for remote mutations.
- [ ] Secret/token redaction.

### M18.09 - Tests/audit/closure
- [ ] Mocked/fixture remote tests.
- [ ] rate-limit/offline tests.
- [ ] local/remote reconciliation tests.
- [ ] Full regression/publication.
- [ ] Independent strict audit.

M18 PLANNED/BLOCKED until M17/core agent flows.

---

# M19 - Next Best Task AI and Engineering Brief

### M19.01 - Candidate task eligibility
- [ ] Use M09 task intelligence + M10 workflow state.
- [ ] Exclude completed/ineligible tasks.
- [ ] Respect dependencies/blockers/gates.

### M19.02 - Priority scoring
- [ ] Project/task priority.
- [ ] Dependency critical path.
- [ ] Audit/CI failure urgency.
- [ ] Human/external wait penalties.
- [ ] Context-switch cost.

### M19.03 - Agent availability awareness
- [ ] Consider Codex/Claude availability.
- [ ] Avoid recommending work requiring unavailable actor.
- [ ] Surface Human-required work separately.

### M19.04 - Explainable recommendation
- [ ] Recommend next project/task.
- [ ] Explain why.
- [ ] Show blockers/dependencies/evidence.
- [ ] Show why alternatives ranked lower where useful.

### M19.05 - Portfolio recommendation
- [ ] Rank work across projects.
- [ ] Avoid starving lower-priority critical work.
- [ ] Respect explicit human project focus when set.

### M19.06 - Engineering Brief
- [ ] Morning brief.
- [ ] Since-last-visit summary.
- [ ] What changed.
- [ ] What needs attention.
- [ ] Recommended next actions.

### M19.07 - Truthfulness and AI boundary
- [ ] Separate factual state from AI recommendation.
- [ ] Cite project/task/audit/Git/CI evidence.
- [ ] Mark uncertain/unavailable data.

### M19.08 - Tests/audit/closure
- [ ] Deterministic scoring fixture tests.
- [ ] blocker/dependency/actor tests.
- [ ] explanation consistency tests.
- [ ] Full regression/publication.
- [ ] Independent strict audit.

M19 PLANNED/BLOCKED until M18.

---

# M20 - Project Chat, hardening and release

### M20.01 - Project/portfolio chat
- [ ] Portfolio Q&A.
- [ ] Project-specific Q&A.
- [ ] Task/workflow/audit/Git/test grounded answers.
- [ ] Evidence-aware responses.

### M20.02 - Action-capable chat
- [ ] Translate chat intent into bounded action proposal.
- [ ] Show execution preview before mutation.
- [ ] Require permission/human approval for sensitive actions.
- [ ] Record provenance/result.

### M20.03 - Command palette
- [ ] Global command palette.
- [ ] Search projects/tasks/actions.
- [ ] Keyboard-first operation.

### M20.04 - Accessibility/keyboard navigation
- [ ] Complete keyboard navigation audit.
- [ ] Focus management.
- [ ] Accessible labels/states.
- [ ] Motion/reduced-motion behavior.

### M20.05 - Credential/process security hardening
- [ ] Credential storage review.
- [ ] Secret/log redaction.
- [ ] Process executable/argument allowlisting.
- [ ] Permission review.
- [ ] Tauri capability review.

### M20.06 - Database backup/restore
- [ ] User-accessible backup.
- [ ] Restore validation.
- [ ] Corruption/recovery test.
- [ ] Version compatibility policy.

### M20.07 - Performance/scale hardening
- [ ] Large Registry test.
- [ ] Large task/source test.
- [ ] Large activity/session/audit history test.
- [ ] Memory/CPU/startup responsiveness review.

### M20.08 - UX hardening
- [ ] Final layout/scrollbar audit.
- [ ] Error/offline/missing-project states.
- [ ] Notification quality.
- [ ] Remove stale placeholders/fake data.
- [ ] Final canonical branding check.

### M20.09 - Installer/clean-machine Windows test
- [ ] Create installer only at this milestone.
- [ ] Install on clean Windows environment.
- [ ] Validate shortcut/icon/startup/restart.
- [ ] Validate no dev tooling/server requirement.
- [ ] Validate uninstall behavior.

### M20.10 - User/security/privacy documentation
- [ ] User guide.
- [ ] Project/agent/audit workflow guide.
- [ ] Security/privacy behavior.
- [ ] Backup/restore instructions.
- [ ] Troubleshooting.

### M20.11 - Release audit
- [ ] Full milestone regression M00-M20.
- [ ] Security audit.
- [ ] Dependency audit.
- [ ] Installer/clean-machine evidence.
- [ ] No unresolved BLOCKER/MAJOR findings.

### M20.12 - H!veAI v1.0.0
- [ ] Version/tag/release notes.
- [ ] Final production build/installers.
- [ ] Release artifacts verified.
- [ ] Final human acceptance.
- [ ] H!veAI v1.0.0 released.

M20 PLANNED/BLOCKED until M19 and final hardening gates.

---

# Milestone policy

- M00-M09 are PASS/CLOSED.
- Pre-M10 Native UX Hotfix X01/X02 is PASS/CLOSED after independent audit and user native acceptance.
- M10 original strict audit is historical FAIL; M10A remediation is IMPLEMENTATION COMPLETE / PENDING INDEPENDENT RE-AUDIT.
- M13C R32 remediation is complete; M13 remains open pending independent strict re-audit and user native/visual acceptance. M14-M20 remain planned/blocked.
- M11/M12 must implement the Project Dashboard manifest authority resolver before treating `.hiveai/PROJECT_DASHBOARD.md` as live runtime truth.
- Each future milestone should be executed as one bounded milestone unless an actual independent audit requires a remediation prompt.
- Subpackage numbering is for traceability, source/evidence mapping, and progress visibility, not an instruction to generate many tiny prompts.
- Every milestone closes only after production implementation, direct evidence, full regression, governed publication where applicable, and independent audit acceptance.
