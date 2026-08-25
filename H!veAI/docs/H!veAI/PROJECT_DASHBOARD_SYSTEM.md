# H!veAI Cross-Repo Project Dashboard System

Status: DESIGN ACCEPTED FOR ROLLOUT
Schema: `hiveai-project-dashboard/v1`
Tracked repositories: 8
Excluded repositories: `Sekiph82/MasalGame`, `Sekiph82/Trial`

## 1. Goal

Give H!veAI a stable, non-destructive, future-proof way to understand every tracked repository without forcing every project to use identical human documentation.

The system uses a two-layer model:

1. Existing project-native files remain the source of truth for tasks, handoff, roadmap, architecture, decisions, security, progress, and agent instructions.
2. Every tracked repository receives one H!veAI-owned pointer manifest at `.hiveai/PROJECT_DASHBOARD.md`.

The manifest does **not** duplicate task content. It tells H!veAI which existing files are authoritative and which are secondary/history-only. This prevents the same task from appearing in several Markdown files and reduces false duplicate task extraction.

## 2. Why this architecture

Do not rewrite every repository into one giant identical documentation layout merely for H!veAI. Large projects already contain valuable project-specific documentation and history. Replacing it creates unnecessary regression risk.

Instead:

- preserve existing project documentation;
- normalize authority, not content duplication;
- keep `TASKS.md` / `tasks.md` as the detailed canonical task ledger when one already exists;
- use the manifest to identify legacy, secondary, history-only, and instruction-only sources;
- progressively normalize task ledgers to H!veAI checkbox conventions when a project is actively worked on;
- never fabricate tasks merely to satisfy a template.

## 3. Manifest contract

Every tracked repository should contain:

`/.hiveai/PROJECT_DASHBOARD.md`

The manifest is pointer-only metadata. It must not contain task checkboxes such as `[ ]`, `[x]`, `[~]`, or `[!]`.

Required fields:

- schema
- project key
- repository identity
- branch policy
- canonical task source
- handoff source
- roadmap source
- progress/history source
- architecture/design source
- decision source
- instruction sources
- security source
- build/test metadata sources
- secondary/legacy source notes
- refresh policy

## 4. Current H!veAI behavior vs future dashboard behavior

Current H!veAI already provides important pieces:

- M07 watches registered local project roots for filesystem changes.
- M08 discovers supported task/plan/progress/handoff/instruction sources.
- M09 parses supported explicit task intelligence without treating normal prose as tasks.

The v1 dashboard manifest is therefore safe to add now, but H!veAI does not yet have a dedicated manifest-ingestion engine. Until that is implemented, the manifest acts as a deterministic contract and migration guide while M07-M09 continue using their existing source discovery/parsing paths.

Future dashboard integration should be implemented in the Global Command Center / Project Cockpit layer, not by adding a second filesystem crawler. It should consume the manifest plus the existing M08/M09 inventories.

Target future behavior:

`watcher event -> M08 source refresh -> M09 task intelligence refresh -> manifest authority resolution -> Project Dashboard refresh`

No GitHub Action should rewrite the dashboard file automatically. H!veAI should derive live state from source evidence rather than committing generated status snapshots back into project repositories.

## 5. Standard task syntax

Where a canonical task ledger exists, progressively normalize it to:

- `[x]` validated complete
- `[~]` active/in progress
- `[ ]` planned/pending
- `[!]` blocked

Recommended structure:

```text
# PROJECT MASTER TASKS

## Current truth
...

# M01 / Phase 01
### M01.01 - Work package
- [x] completed item
- [~] active item
- [ ] planned item
```

Subpackage numbering is for traceability and auditability. It does not require separate builder prompts.

## 6. Non-task documentation rule

Avoid task checkbox syntax in architecture, progress history, security, decisions, and general design documents unless the item is intentionally a task source.

This prevents H!veAI from interpreting descriptive documentation as operational backlog.

## 7. Migration matrix

Legend:

- `CANONICAL` = already suitable as the primary source for that category.
- `SECONDARY` = useful context/history but must not override the canonical source.
- `MISSING` = no verified root source currently exists.
- `CONFLICT` = multiple files can describe overlapping project state and need authority clarification.
- `NORMALIZE LATER` = preserve now, progressively convert when the project is next actively worked on.

| Repository | Tasks | Handoff | Roadmap / Plan | Progress / History | Architecture / Design | Decisions | Agent instructions | Security | Main migration issue |
|---|---|---|---|---|---|---|---|---|---|
| `AI-Commerce-HQ` (`H!veAI` branch) | `H!veAI/TASKS.md` CANONICAL for H!veAI; root `TASKS.md` is legacy parent-app history | MISSING | `H!veAI/CODEX_ROADMAP.md` CANONICAL | `H!veAI/docs/H!veAI/audits/` + `codex-logs/` SECONDARY evidence | `H!veAI/ARCHITECTURE.md` CANONICAL | `H!veAI/CONSTITUTION.md` governance | `H!veAI/AGENTS.md` | MISSING dedicated file | Root legacy task truth must not override active H!veAI child truth. |
| `Bulk-Edit` | `TASKS.md` CANONICAL | `HANDOFF.md` CANONICAL | `ROADMAP.md` CANONICAL | `CHANGELOG_AI.md` history; `PROJECT_STATUS.md` snapshot SECONDARY | `ARCHITECTURE.md` CANONICAL | `DECISIONS.md` CANONICAL | `CLAUDE.md` | `SECURITY.md` | CONFLICT risk from TASKS + HANDOFF + PROJECT_STATUS repeating current state. Manifest resolves authority. |
| `FormuLab` | MISSING dedicated canonical task ledger | MISSING | MISSING verified root roadmap | `PROGRESS.md` CANONICAL history | MISSING verified root architecture file | MISSING verified root decision ledger | `AGENTS.md`, `CLAUDE.md` | `SECURITY.md` | Main gap is no dedicated canonical task ledger. Do not convert `PROGRESS.md` into task truth automatically. |
| `Scrubbots` | `tasks.md` CANONICAL | MISSING | task plan currently embedded in `tasks.md` | `CHANGELOG.md` SECONDARY history | MISSING | MISSING | `CLAUDE.md` | MISSING | Good task source already exists; missing handoff/architecture separation. |
| `fmcg-erp-system` | `TASKS.md` CANONICAL | MISSING | `PLANS.md` CANONICAL planning context | task history substantially lives in `TASKS.md` | MISSING verified root architecture file | MISSING verified root decision ledger | `AGENTS.md`, `CLAUDE.md` | MISSING verified root security file | Very large TASKS/PLANS corpus; manifest must prevent plan/history duplication from becoming competing task truth. |
| `ARES-LLM` | `TASKS.md` CANONICAL | MISSING | MISSING | MISSING | `DESIGN.md` CANONICAL design context | MISSING | MISSING | MISSING | Clean base. Needs handoff/roadmap/agent governance later, but no rewrite is required now. |
| `PackLab-3D` | `tasks.md` CANONICAL | `handoff.md` CANONICAL | MISSING | MISSING | design/brand docs are SECONDARY, no verified architecture ledger | MISSING | `claude.md` | MISSING | Existing lowercase filenames are valid and should not be renamed merely for style. |
| `move-in-range` | MISSING | MISSING | MISSING | MISSING | MISSING verified root architecture ledger | MISSING | `AGENTS.md` | `SECURITY.md` | No task ledger exists. Keep manifest truthful; create tasks only from verified project work, not from guesses. |

## 8. Authority rules

Priority order for project status and task truth:

1. canonical task ledger identified by the manifest;
2. explicit handoff current/next/blocker/waiting sections;
3. roadmap/plan context;
4. progress/changelog/history;
5. architecture/design/decision/security context;
6. instruction files (`AGENTS.md`, `CLAUDE.md`) are never task authority.

If no canonical task ledger exists, H!veAI must show `TASK AUTHORITY NOT YET CANONICALIZED` instead of inventing one.

## 9. Tracked repository set

Track:

- `Sekiph82/AI-Commerce-HQ`
- `Sekiph82/Bulk-Edit`
- `Sekiph82/FormuLab`
- `Sekiph82/Scrubbots`
- `Sekiph82/fmcg-erp-system`
- `Sekiph82/ARES-LLM`
- `Sekiph82/PackLab-3D`
- `Sekiph82/move-in-range`

Do not include in H!veAI portfolio tracking:

- `Sekiph82/MasalGame`
- `Sekiph82/Trial`

## 10. Future-project bootstrap

For every new serious project, create `.hiveai/PROJECT_DASHBOARD.md` at registration time. The project may keep its own documentation style as long as the manifest truthfully maps its authorities.

If the new project has no task ledger yet, H!veAI should record that fact rather than generate speculative tasks.

## 11. Safety invariants

- Never delete or rename project-native documentation solely to satisfy H!veAI.
- Never duplicate the complete task ledger into the dashboard manifest.
- Never automatically commit generated H!veAI status back to project repositories.
- Never treat builder/agent claims as authoritative without source/test evidence.
- Never make instruction files task authority.
- Never infer missing roadmap, security, or architecture content.
- Preserve exact filename casing in source maps.
