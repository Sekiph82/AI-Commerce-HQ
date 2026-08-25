# H!veAI Architecture

## Decision
Build H!veAI as a Tauri 2 local-first desktop application on top of the
historical AI-Commerce-HQ repository, but treat it as a rebuild.

## Reusable AI-Commerce-HQ patterns
- Tauri desktop packaging/lifecycle
- Rust-managed child backend lifecycle
- backend health polling and restart
- FastAPI/API boundary during migration
- WebSocket heartbeat/broadcast/replay pattern
- BaseAgent abstraction pattern
- async SQLite pattern
- EventRecord concept
- React/TypeScript/Vite/Tailwind/Zustand tooling

## Replace/remove
- Three.js office/game world and robots
- revenue/XP/achievements
- Etsy/Fiverr/Trading/YouTube/TikTok orchestrators
- commerce products and publication approval flows
- platform rooms/desks
- Tauri 1.x APIs
- ad-hoc SQLite migrations

## Target architecture

Frontend:
Global Command Center, Project Cockpit, Agents, Audit Center, AI Brief/Chat.

Rust native core:
Project Registry, Git Engine, filesystem watcher, PTY/process manager,
worktree manager, permission engine, notifications, lifecycle.

Domain services:
Task Source Discovery, Task Parser/Normalizer, Workflow State Machine,
Prompt Engine, Audit Engine, Next Best Task Engine, Engineering Brief.

Adapters:
Codex, Claude Code, OpenAI/Codex Audit, GitHub.

SQLite:
projects, repositories, project_sources, git_snapshots, tasks,
task_dependencies, task_sources, task_events, prompts, prompt_versions,
agent_sessions, agent_events, agent_tool_calls, permission_requests,
audits, audit_findings, test_runs, alerts, decisions, github_sync_state,
settings, migrations.

M08 Task Source Discovery is the bounded pre-parser boundary: it resolves only
registered project roots, discovers standard and explicitly configured custom
source documents, records physical-path/hash/status metadata in
`project_sources`, and exposes no source bodies or parsed task entities. It
does not write `tasks`, `task_sources`, project files, `.hiveai` configuration,
or start background discovery workers.

## Project Registry
Stores project identity, local path, git remote, GitHub owner/repo, default
branch, priority, preferred builder/auditor and task-source policy.
Registration is read-only.

## Git Engine
Reads branch, HEAD, staged/unstaged/untracked files, ahead/behind, commits,
diff, conflicts and worktrees. Writes pass through Permission Engine.

## Task Intelligence
M08-owned, versioned, AVAILABLE source inventory is the sole parse boundary.
M09 reconstructs and physically validates each source path under the Registry
root, checks the bounded UTF-8 body hash, and emits structured warnings rather
than reading arbitrary repository files. The deterministic parser persists
M09-owned `m09src:`/`m09task:` rows, `SOURCE_EXPLICIT` dependency edges, and a
project snapshot in existing SQLite tables without writing task events or
managed project files. `list` reads the persisted snapshot only. Generic,
FormuLab, ScrubBots, and FMCG ERP adapters are selected by exact Registry
identity and only add evidenced structural conventions; generic is the safe
fallback. M09 storage maps DONE to TASK_COMPLETE, BLOCKED to BLOCKED, and all
other parsed statuses to BACKLOG while retaining richer parser truth in
metadata for the future M10 workflow state machine.

## Workflow
Happy path:
BACKLOG → PLANNING_REQUIRED → PROMPT_REQUIRED → PROMPT_READY →
READY_FOR_IMPLEMENTATION → BUILDER_RUNNING → IMPLEMENTATION_COMPLETE →
AUDIT_REQUIRED → AUDIT_RUNNING → AUDIT_PASSED → VERIFY_REQUIRED →
VERIFY_RUNNING → TASK_COMPLETE.

Failure loop:
AUDIT_FAILED → FIX_REQUIRED → READY_FOR_IMPLEMENTATION → BUILDER_RUNNING →
IMPLEMENTATION_COMPLETE → RE_AUDIT_REQUIRED → AUDIT_RUNNING.

## Agent adapter contract
Each provider supports availability check, start, resume, stop, status and
streamed events. A session always belongs to one project and one task/freeform
operation, normally inside a project cwd or isolated worktree.

## Audit Engine
Audit reads task requirements, acceptance criteria, actual diff, changed
files, tests, architecture rules and builder logs as secondary evidence.
Returns PASS/FAIL/CONDITIONAL with severity findings, requirements coverage,
test confidence, regression risk and remediation prompt.

## Global Command Center
Must show portfolio KPIs, project operation cards, current task, last action,
next action, required actor, Needs Your Attention, Active Work Queue,
AI Engineering Brief and live activity.

## Project Cockpit
Tabs: Overview, Tasks, Workflow, Agents, Audit, Git, Tests, Activity, Files,
Settings. Current-task card must always explain where we are, why, evidence,
next step and primary action.

## Security
Use Tauri 2 capabilities and command allowlisting. Frontend must not get an
unrestricted shell. Secrets must be redacted from logs and stored securely.
Merge always requires human approval.

## Migration strategy
Keep Git operations at the parent repository root and put new H!veAI application
work under the child `H!veAI` directory. Treat the old parent application as
source material only. Audit and reuse infrastructure deliberately, replace the
commerce product domain and game UI, migrate Tauri first, then database and
orchestration.
