# M12 Project Cockpit - Authoritative Implementation Prompt

## Authority

This is the single authoritative implementation prompt for H!veAI milestone M12.

M11 is PASS/CLOSED. Strict completed roadmap progress is `12 / 20 = 60%`.

M12 is READY / ACTIVE FOR THIS IMPLEMENTATION RUN.

Do not start M13 or M21 in this run.

Do not reopen M11 unless a concrete regression caused by M12 is proven.

## Mandatory preflight

Work from the existing repository root:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Work only on branch:

`H!veAI`

Before editing:

1. `git fetch origin H!veAI`
2. record exact local `HEAD`;
3. record exact `origin/H!veAI`;
4. record `git rev-list --left-right --count HEAD...origin/H!veAI`;
5. synchronize only with safe fast-forward behavior;
6. never reset, rebase, force-push, rewrite history, or delete unrelated user work.

Read in full before implementation:

- `H!veAI/AGENTS.md`
- `H!veAI/CONSTITUTION.md`
- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/.hiveai/PROJECT_DASHBOARD.md`
- `H!veAI/docs/H!veAI/audits/M11_CLOSURE_AND_M12_ACTIVATION_STRICT_AUDIT.md`
- `H!veAI/docs/H!veAI/audits/M11A_REV7_UNICODE_STRUCTURED_IDENTITY_FINAL_STRICT_REAUDIT.md`
- `H!veAI/docs/H!veAI/audits/M11_PROJECTS_FINAL_VISUAL_CLEANUP_STRICT_AUDIT.md`
- relevant M08/M09/M10/M11 source and focused tests that define registry, task authority, workflow precedence, Project Dashboard parsing, provenance, watcher behavior, Command Center truth, and project routing.

Inspect the current production implementation before choosing architecture. Reuse existing models/services/components where safe. Do not create a parallel truth system.

## Immutable accepted product state

Preserve all accepted M11/native behavior unless M12 requires an additive project-scoped surface.

Current accepted startup asset is:

`H!veAI/src/assets/H!veAI.mp4`

Do not restore, reintroduce, or make production depend on the historical `opening-video.mp4` asset.

Preserve:

- current H!veAI native icon behavior;
- accepted audible startup playback and same-process no-replay behavior;
- no terminal/console flash;
- current Command Center visual composition;
- current Projects visual composition, including three-column desktop registry layout and Registry Boundary information living inside Add Project rather than a permanent side column;
- current Tasks visual composition;
- Akilta topbar attribution and safe external-open behavior;
- Registry read-only guarantees;
- `.hiveai/PROJECT_DASHBOARD.md` as the single H!veAI-facing live project status contract;
- M10 workflow-state precedence and historical event integrity;
- M11 structured/Unicode-safe operational identity and conservative duplicate suppression;
- external-project containment and no mutation of registered project repositories unless an explicit user action is part of an already accepted operation.

Do not touch Bulk Edit or another registered project repository.

## M12 mission

Implement the complete per-project **Project Cockpit** using the same resolved Project Dashboard authority/provenance model accepted in M11.

Exit criterion:

`Complete end-to-end project operations cockpit with truthful source authority and provenance.`

M12 must not invent state. Unknown, absent, degraded, malformed, stale, archived, or unavailable truth must remain explicit.

The Project Cockpit must be project-scoped. No selected-project route, async response, manifest, task source, workflow item, agent session, audit, Git state, test record, activity item, or file inventory may leak from another registered project.

## M12.01 - Cockpit shell and project-scoped loading

Implement a real project route/shell for opening a registered project cockpit.

Requirements:

- project-specific route loading using the existing registered project identity;
- async loading skeleton/state;
- truthful not-found/missing/archived/degraded states;
- never fall back to another project while selected data is missing or late;
- race-safe project switching;
- load the selected project's resolved Project Dashboard authority map;
- ensure manifest/source state is scoped to the selected project;
- keep route/reload behavior deterministic;
- preserve navigation back to portfolio/Projects.

Use the existing Project Dashboard resolver and accepted fallback semantics. Do not build a second parser or authority resolver unless a missing project-scoped API genuinely requires a shared refactor.

## M12.02 - Overview

Implement the project Overview surface with truthful operational summary.

Show, where evidence exists:

- project identity/name/path/repository state;
- project status and health;
- current milestone;
- current task hero;
- current workflow state;
- last completed meaningful action;
- next action;
- required actor;
- waiting/blocker state;
- useful source authority/provenance.

Avoid decorative duplication of facts already obvious from the same panel. Provenance should be understandable without overwhelming the page.

Unknown or unverified facts must not be replaced with fabricated defaults.

## M12.03 - Tasks

Implement the project-scoped Tasks surface using the manifest-declared canonical task authority and existing task intelligence.

Requirements:

- parsed tasks;
- status/state distinction;
- dependencies;
- blockers;
- acceptance criteria when present;
- evidence drawer/detail surface;
- source locator/navigation foundation;
- source provenance;
- duplicate suppression when the same operational task appears through Project Dashboard materialization and stronger canonical task/workflow evidence.

Do not convert roadmap/history/handoff prose into duplicate tasks unless the existing authority model explicitly classifies it as canonical task truth.

## M12.04 - Workflow

Implement project-scoped workflow operations using the M10 model.

Show:

- current state pipeline;
- transition history;
- actors;
- evidence requirements;
- failed/blocked/waiting states;
- human override visibility/control only where the existing M10 contract permits it.

Any manual transition/override must preserve event history. Never silently rewrite state.

## M12.05 - Agents

Implement a project-scoped Agents surface using existing persisted session truth only.

Show where available:

- sessions;
- provider;
- current state;
- duration/timing;
- task association;
- permission/wait state.

M12 does not implement the M13 Codex process adapter or M14 PTY center. Do not start providers/processes as part of M12 unless already supported by accepted existing behavior.

Clearly distinguish unavailable future capabilities from existing session evidence.

## M12.06 - Audit

Implement the project-scoped Audit surface.

Show:

- latest audit verdict;
- findings grouped by severity;
- requirement/coverage evidence where available;
- historical remediation/re-audit sequence;
- timestamps/provenance where verified.

Historical failed audits must remain visible as history rather than being overwritten by later PASS results.

## M12.07 - Git

Implement project-scoped Git visibility using the existing read-only Git engine.

Show truthfully:

- repository/non-repository state;
- branch;
- HEAD;
- dirty/clean state;
- ahead/behind where available;
- changed files;
- diff/evidence foundation;
- conflicts;
- worktree state where available.

Do not mutate branch, files, remotes, commits, worktrees, or repository configuration from ordinary cockpit loading.

## M12.08 - Tests, Activity, Files

Implement project-scoped surfaces for:

### Tests
- test-run history;
- result;
- command/source where persisted;
- timing where persisted;
- related task/evidence where available.

### Activity
- bounded meaningful timeline from existing persisted/workflow/project-dashboard evidence;
- truthful timestamps;
- Project Dashboard materialized activity remains `UNDATED` unless a real timestamp exists.

### Files / Project context
- bounded relevant-file/source inventory;
- evidence/source links;
- classify manifest-declared roadmap, handoff, history, architecture, instructions, audit/log sources as project context/provenance;
- do not render those classified sources as duplicate canonical tasks.

Avoid dumping an unbounded repository tree.

## M12.09 - Project Settings

Implement project-scoped settings and registry controls consistent with the accepted read-only boundary.

Include where already supported or safely implementable:

- registry metadata/settings;
- preferred builder/auditor values if existing data model supports them;
- task-source policy/custom source entry points if existing model supports them;
- Project Dashboard manifest status;
- resolved authority roles;
- warnings/degraded state;
- provenance/source map;
- path repair;
- archive;
- remove from registry.

Do not auto-rewrite `.hiveai/PROJECT_DASHBOARD.md`, TASKS, roadmap, handoff, tracker, or other registered-project files merely because H!veAI reads them.

Any operation that changes H!veAI's own registry must require explicit user action and preserve existing safety semantics.

## M12.10 - Manual correction controls

Where the existing model permits controlled corrections:

- require explicit human action;
- require rationale/evidence where appropriate;
- record a correction/event;
- preserve before/after state;
- never silently rewrite canonical project truth;
- never mutate external project tracker files merely to make H!veAI's view look cleaner.

If a requested correction cannot be made safely with current milestone infrastructure, expose the limitation truthfully rather than inventing a write path that belongs to M13-M15 or later.

## Interaction and navigation requirements

Project cards' existing `Open cockpit` action must open the correct registered project's cockpit.

Project shortcuts may open the same project-scoped cockpit if consistent with existing navigation behavior.

All cockpit tabs/panels must stay tied to one selected project identity.

At common desktop sizes:

- no accidental horizontal page overflow;
- avoid nested scrollbars unless a bounded evidence/code/diff surface genuinely requires one;
- preserve current H!veAI spacing, typography, backgrounds, borders, and visual hierarchy;
- do not redesign Command Center/Projects/Tasks as collateral work.

## Race, containment and truth tests

Add direct tests for at least these adversarial classes:

1. Open project A, switch quickly to B, then A response arrives late: B must remain selected and no A data may appear.
2. Missing/deleted registry path must show the correct selected-project degraded state, not another project.
3. Archived project must remain truthful and scoped.
4. Project A/B manifests and authority maps cannot cross-contaminate.
5. Duplicate task truth from Project Dashboard + canonical task authority is not double-rendered.
6. Manifest absent/malformed/stale/degraded fallback follows accepted M11 semantics.
7. Unicode and colon-bearing source identities preserved by M11 remain safe through cockpit rendering/dedup.
8. Manual correction records an event and does not silently rewrite prior history.
9. Git cockpit loading performs no mutation.
10. Registry path repair/archive/remove actions require explicit action and affect only H!veAI registry state according to existing guarantees.
11. Classified roadmap/handoff/history/architecture sources appear as context/provenance rather than duplicate tasks.
12. Unknown/unverified state remains explicit.

## Verification gates

Run the repository's full applicable verification suite.

At minimum:

- focused M12 frontend tests;
- relevant native/Rust M12 tests;
- preserved M08/M09/M10/M11 authority/workflow/watcher/identity regression tests;
- complete frontend test suite;
- complete Rust library test suite;
- TypeScript typecheck;
- frontend production build;
- `npm audit --audit-level=high`;
- `cargo fmt --all -- --check`;
- `cargo check`;
- `git diff --check`;
- governed production Tauri `--no-bundle` build/publication;
- established dev QA publisher failure harness;
- stable executable smoke test;
- terminal/console suppression verification.

If a gate fails, repair the scoped defect before declaring implementation complete.

Do not weaken or delete existing tests merely to make M12 pass.

## User visual acceptance

Publish the normal stable dev QA executable used by the existing workflow.

Do not claim user visual acceptance yourself.

Final M12 builder state must remain:

`IMPLEMENTATION COMPLETE / PENDING INDEPENDENT STRICT AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

until both gates actually occur.

## Canonical status updates

During the run, truthfully update only the existing canonical/current H!veAI status files used by this repository.

At implementation start they may state M12 ACTIVE.

At implementation end they must state:

- M00-M11 remain PASS/CLOSED;
- strict completed roadmap count remains `12/20 = 60%` until M12 independently closes;
- M12 implementation complete pending independent strict audit and user native/visual acceptance;
- M13 remains blocked/not started;
- M21 remains planned/not started.

Do not mark M12 PASS/CLOSED yourself.

## Immutable builder log

Create:

`H!veAI/docs/H!veAI/codex-logs/M12_PROJECT_COCKPIT_IMPLEMENTATION_LOG.md`

The log must include:

- exact synchronized preflight HEAD/origin/divergence;
- architecture chosen and reused existing services/models;
- exact files changed;
- package-by-package M12.01-M12.11 implementation summary;
- explicit deferred items, if any, with reason;
- project-scoping/race/containment evidence;
- Project Dashboard authority/provenance behavior;
- manual correction behavior;
- Git read-only evidence;
- exact test commands/results;
- full frontend/Rust counts;
- publication/failure-harness results;
- stable executable technical smoke evidence;
- implementation commit SHA(s);
- exact final HEAD;
- exact fetched `origin/H!veAI`;
- exact `HEAD...origin/H!veAI` count;
- confirmation that no external registered repository or Bulk Edit was modified;
- confirmation M13/M21 were not started;
- final builder state pending independent strict audit + user acceptance.

Commit and push all scoped changes to `origin/H!veAI`.

Stop after M12 implementation and evidence publication. Do not start M13 or M21.
