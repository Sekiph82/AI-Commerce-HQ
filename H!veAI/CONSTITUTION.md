# H!veAI Repository Constitution

## Purpose
H!veAI is a local-first AI Development Command Center. It must always show:
- every managed project and its health,
- current task,
- last completed task/action,
- next required action,
- required actor (Human, Codex, Claude, GPT Audit, CI, External),
- supporting evidence,
- an actionable control whenever the next step can be executed.

## Non-negotiable principles
1. **Local-first:** local repository state is authoritative for uncommitted changes and local sessions; GitHub is authoritative for remote PRs, issues, Actions, releases and remote refs.
2. **Evidence-first:** AI self-report never proves task completion. Completion needs repository/test/audit evidence.
3. **Human override wins:** user corrections override inferred state and are recorded as events.
4. **Separation of duties:** builder and auditor are separate roles. Auditor inspects actual diff, task requirements, tests and architecture rules.
5. **Explicit state machine:** UI and backend use shared enums, never arbitrary status strings.
6. **Actionable status:** PROMPT_REQUIRED → Generate Prompt; READY_FOR_IMPLEMENTATION → Run Builder; AUDIT_REQUIRED → Run Audit; FIX_REQUIRED → Fix with Builder; WAITING_OWNER → Review Decision.
7. **Safe by default:** scanning is read-only. Dependency install, deletion, push, PR creation and destructive shell actions require policy approval. Merge always requires explicit human approval.
8. **No hidden repo mutation:** registering/scanning a project must never edit it.

## Target stack
- Tauri 2
- Rust native core
- React + TypeScript + Vite
- Tailwind + shadcn/ui
- Framer Motion
- SQLite
- xterm.js
- Git
- Codex CLI/SDK adapter
- Claude Code adapter
- OpenAI/Codex audit/planning adapter
- GitHub integration

## Canonical task states
BACKLOG, PLANNING_REQUIRED, PROMPT_REQUIRED, PROMPT_READY,
READY_FOR_IMPLEMENTATION, CODEX_RUNNING, CLAUDE_RUNNING,
IMPLEMENTATION_COMPLETE, AUDIT_REQUIRED, GPT_AUDIT_RUNNING,
AUDIT_PASSED, AUDIT_FAILED, FIX_REQUIRED, RE_AUDIT_REQUIRED,
VERIFY_REQUIRED, VERIFY_RUNNING, WAITING_OWNER, DESIGN_GATE,
WAITING_EXTERNAL, BLOCKED, PAUSED, FAILED, TASK_COMPLETE.

## Canonical agent states
IDLE, QUEUED, STARTING, RUNNING, WAITING_PERMISSION, WAITING_USER,
STOPPING, COMPLETED, FAILED, CRASHED, ORPHANED, RATE_LIMITED.

## Event ledger
Every material transition is immutable and records:
id, project_id, task_id, timestamp, actor_type, actor_id, event_type,
from_state, to_state, summary, evidence_json, session_id, commit_sha, source.

## Repository-native sources
H!veAI may discover TASKS.md, tasks.md, PLANS.md, PROGRESS.md, ROADMAP.md,
CLAUDE.md, AGENTS.md, docs/handoffs and GitHub Issues/Milestones.
Projects may optionally add `.hiveai/project.yaml`, `.hiveai/tasks.yaml`,
`.hiveai/prompts/`, `.hiveai/audits/`, `.hiveai/handoffs/`.

## Definition of done
A milestone is complete only when implementation exists, relevant tests pass,
regressions pass, security is reviewed, recovery/error states exist,
documentation and TASKS.md reflect reality, diff is reviewed, and no secrets,
user data, caches or build junk are committed.

## Codex session rules
Every Codex session must read AGENTS.md, CONSTITUTION.md, ARCHITECTURE.md and
TASKS.md, inspect branch/HEAD/status, verify the current milestone from repo
evidence, run baseline tests before risky work, stay inside current scope,
add tests, verify, update docs/tasks, review diff, never force-push and never
silently modify another managed repository.

## AI-Commerce-HQ migration rule
The old parent AI-Commerce-HQ application is source material only. H!veAI is a
product rebuild under the child `H!veAI` application root, not a cosmetic rename
of parent runtime code. Upgrade to Tauri 2 before core orchestration work.
Retain reusable infrastructure only after audit. Do not destroy old local data
without an explicit backup and migration policy.

## V1 non-goals
No graph database, knowledge graph, 3D project graph, automatic PR merge,
cloud multi-user tenancy, mobile companion or remote execution farm.
