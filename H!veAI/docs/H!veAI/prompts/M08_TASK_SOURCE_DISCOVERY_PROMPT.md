# M08 — Task Source Discovery

## Purpose

Implement the real M08 Task Source Discovery milestone for H!veAI.

M08.00/M08.00B presentation work is already PASS/CLOSED. This prompt begins the actual domain milestone.

This milestone discovers and classifies task-relevant source documents for registered local projects. It does **not** parse tasks, infer task states, create workflow states, start agents, or implement M09+.

M07 remains PASS/CLOSED.
M08.00 and M08.00B remain PASS/CLOSED.

---

## Mandatory synchronization

Work from:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Run:

```powershell
git fetch origin H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
```

Fast-forward only if safe. Never reset, rebase, force-checkout, overwrite user changes, or force-push.

Read completely before editing:

1. `H!veAI/AGENTS.md`
2. `H!veAI/CONSTITUTION.md`
3. `H!veAI/ARCHITECTURE.md`
4. `H!veAI/TASKS.md`
5. `H!veAI/docs/H!veAI/UI_LAYOUT_GOVERNANCE.md`
6. `H!veAI/docs/H!veAI/audits/M08.00B_BACKGROUND_ALIGNMENT_AND_NATIVE_INTRO_FIX_STRICT_AUDIT.md`
7. `H!veAI/docs/H!veAI/audits/M08.00B_MANUAL_ACCEPTANCE.md`
8. this prompt

Before feature work, update `TASKS.md` truthfully so M08.00/M08.00B are closed and M08 Task Source Discovery is the active milestone. Do not rewrite historical remediation sections.

---

# Canonical UI Assets

User-owned canonical asset directory:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Canonical sidebar logo/image rules remain unchanged.

Canonical application background source:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI\scene 3 starting point.png`

Canonical opening video source:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI\videos and gifs\opening video.mp4`

Repository assets remain:

- `H!veAI/src/assets/hiveai-app-background.png`
- `H!veAI/src/assets/opening-video.mp4`

Do not modify, crop, regenerate, recolor, recompress, resize, or replace these asset bytes.

Preserve the accepted presentation contract:

- background begins after the sidebar and centers in `.main-area`;
- opening video plays only at cold native launch and native restart;
- startup overlay remains fixed and outside normal layout flow;
- current glass/glow treatment remains restrained and readable;
- approximately 220 px sidebar and enlarged one-piece H!veAI logo remain unchanged;
- Command Center single-viewport geometry remains unchanged.

---

# M08 domain boundary

M08 discovers **source documents** only.

M08 must NOT:

- parse headings/checklists into normalized tasks;
- create/update rows in `tasks`, `task_dependencies`, `task_events`, or `task_sources` for parsed task entities;
- infer task state, blocker, owner, acceptance criteria, milestone, workflow state, or next-best-task;
- create prompts or agent sessions;
- call Codex, Claude, GPT, GitHub, or any network service;
- mutate registered project files;
- write `.hiveai` config into project repositories;
- create an installer;
- start M09 or later milestones.

M09 Task Intelligence Parser will consume the discovered source inventory later.

Use the existing M04 `project_sources` table as the M08 persistence boundary unless direct source inspection proves a schema incompatibility. Do not add a migration merely for convenience. If a migration is genuinely necessary, stop and document why before broadening schema.

---

# M08.01 — Source kinds and standard discovery set

For every ACTIVE/MISSING registered project requested by explicit project ID, resolve the canonical registered project root through the existing Project Registry. Never accept an arbitrary unregistered root as authoritative input.

Discover these standard source families when they exist inside the project root.

## Root task/planning files

Case-insensitive filename matching on Windows, while preserving actual on-disk spelling:

- `TASKS.md`
- `tasks.md`
- `TASK.md`
- `PLANS.md`
- `PLAN.md`
- `PROGRESS.md`
- `ROADMAP.md`
- `AGENTS.md`
- `CLAUDE.md`
- `HANDOFF.md`
- `SESSION_HANDOFF.md`
- reasonable `*handoff*.md` root variants

Do not create duplicate records when multiple spelling rules resolve to the same physical file.

## Standard bounded directories

Inspect only bounded task-relevant directories when present:

- `tasks/`
- `plans/`
- `handoffs/`
- `.hiveai/`

Within these directories, discover text/Markdown/JSON/YAML source documents that are plausibly task/planning/handoff/config evidence.

Do NOT recursively crawl the whole repository looking for every `.md` file.

Default bounded recursive depth inside approved source directories: maximum 4 levels unless an existing H!veAI large-repo policy defines a stricter reusable limit.

## Excluded trees

Never descend into common generated/vendor/system trees, including at minimum:

- `.git/`
- `node_modules/`
- `dist/`
- `build/`
- `target/`
- `.next/`
- `coverage/`
- `.cache/`
- `.venv/`
- `venv/`
- vendor/dependency directories already excluded by the watcher/large-repo policy.

No source discovery should require reading those trees.

---

# M08.02 — Safe custom source paths

Support explicit project-specific custom source paths from H!veAI UI/API.

Custom path contract:

- user may add a file or directory path intended to contain task evidence;
- store custom path configuration in H!veAI-owned persistence, not in the registered project repository;
- prefer the existing `settings` table with a project-scoped deterministic key if no existing project-settings abstraction already fits;
- do not silently encode custom configuration into unrelated Registry fields;
- no project file mutation;
- relative paths are resolved against the registered root;
- absolute paths are accepted only if their physical/canonical target remains inside the registered root;
- `..` traversal escaping the root is rejected;
- an existing symlink/junction resolving outside the project root is rejected;
- a missing but syntactically safe custom path inside the root may remain configured and appear as `MISSING` rather than being silently deleted;
- normalize/dedupe equivalent paths case-insensitively on Windows;
- preserve display spelling separately from normalized comparison form.

Custom directory scanning must obey the same file/depth/size/ignore limits as standard source directories.

Provide explicit add/remove/update custom-path operations. Do not overload Project Registry registration or repair operations.

---

# M08.03 — Source classification and metadata

Define a structured discovery model returned through IPC and persisted through `project_sources`.

At minimum expose:

```ts
interface DiscoveredProjectSource {
  id: string
  projectId: string
  relativePath: string
  absolutePath: string
  sourceKind: string
  origin: "STANDARD" | "CUSTOM"
  status: "AVAILABLE" | "MISSING" | "TOO_LARGE" | "UNREADABLE"
  authorityClass: string
  priority: number
  sizeBytes: number | null
  modifiedAt: string | null
  discoveredAt: string
  contentHash: string | null
  depth: number
  warnings: string[]
}
```

Exact Rust/TypeScript naming may follow repository conventions, but the semantics must remain.

Recommended source kinds:

- `TASKS`
- `HANDOFF`
- `PROGRESS`
- `PLAN`
- `ROADMAP`
- `AGENTS`
- `CLAUDE`
- `HIVEAI_CONFIG`
- `CUSTOM`
- `OTHER_TASK_SOURCE` only for bounded approved-directory text files that do not match a stronger class.

Do not label arbitrary README/docs as task sources merely because they are Markdown.

## Authority and priority

Authority is discovery metadata, not semantic truth.

Use deterministic ordering such as:

1. explicit CUSTOM path supplied by the user;
2. `TASKS` / primary task list;
3. current HANDOFF/session handoff;
4. `PROGRESS`;
5. `PLAN`;
6. `ROADMAP`;
7. `CLAUDE` / `AGENTS` as instruction/context evidence;
8. bounded `OTHER_TASK_SOURCE`.

If two sources share a class, order deterministically by configured custom order, then freshness, then normalized relative path.

Do not claim that the newest file is automatically correct or authoritative.

## Freshness

Record filesystem modification time as evidence.

You may expose a derived non-semantic freshness bucket (`RECENT`, `AGING`, `OLD`, `UNKNOWN`) only if thresholds are deterministic and documented. Do not infer project status from mtime.

## Hashing and file-size limits

Compute SHA-256 for readable bounded-size source files so changes can be detected idempotently.

Do not read huge files into memory unbounded.

Choose and document a conservative source-file hash/read limit, preferably 2 MiB or lower unless an existing repository limit should be reused. Files above the limit must be reported `TOO_LARGE` with size metadata and no content parse.

M08 does not need source text content in the frontend. Do not expose entire file bodies through the discovery list command.

---

# M08.04 — Persistence and idempotency

Persist discovered source inventory in `project_sources` transactionally.

Required behavior:

- deterministic identity per project + normalized relative path + origin;
- repeated discovery with unchanged files does not create duplicates;
- existing records update content hash, mtime metadata, discovered timestamp, kind/status metadata as appropriate;
- newly discovered standard sources are inserted;
- auto-discovered standard sources that disappear are removed or marked stale according to one consistent documented policy;
- configured CUSTOM paths are not silently removed when their target disappears; they remain visible as `MISSING` until user removes configuration;
- discovery failure for one file should not corrupt the rest of the project's persisted inventory;
- use one bounded transaction for reconciliation where practical;
- do not mutate `tasks` or `task_sources` in M08.

Store structured metadata in `metadata_json` where the current schema lacks columns. Keep JSON schema/version explicit enough for M09 to consume safely.

Existing `project_sources` rows created by tests/legacy fixtures must not be duplicated blindly.

---

# M08.05 — Native API

Create a narrow Rust task-source discovery module/service and dedicated commands following existing H!veAI patterns.

Suggested command surface:

- `hiveai_task_sources_discover(project_id)`
- `hiveai_task_sources_list(project_id)`
- `hiveai_task_source_custom_paths_list(project_id)`
- `hiveai_task_source_custom_path_add(project_id, path)`
- `hiveai_task_source_custom_path_remove(project_id, path_or_id)`

Names may differ if repository conventions demand it, but keep the boundary narrow.

Every operation must resolve project identity through the existing Registry/DB and enforce physical containment.

Add one dedicated Tauri permission/capability entry such as `allow-task-source-discovery`; do not broaden foundation permissions or expose arbitrary filesystem read APIs to the frontend.

No shell command execution is required for source discovery.

No network access.

No Git mutation.

---

# M08.06 — Watcher integration boundary

Reuse M07 watcher evidence without turning M08 into a second watcher.

Requirements:

- discovery can be triggered explicitly by the frontend `Rescan sources` action;
- discovery should use current filesystem state directly and must not depend on a watcher event having fired first;
- if there is a clean bounded way to invalidate/refresh source inventory after relevant M07 watcher events without introducing polling or a new background subsystem, integrate it;
- otherwise keep M08 deterministic/manual-on-open/manual-rescan and document automatic event-driven refresh as a later integration point.

Do not duplicate watcher threads or register a second recursive watcher.

---

# M08.07 — Source UI

Replace the native Tauri placeholder on the existing `/tasks` page with a real **Task Sources** workspace for the currently selected registered project.

This is a source inventory UI, not the M09 parsed-task UI.

Required desktop UI:

- selected project identity from live Registry;
- clear heading such as `Task Sources`;
- summary: discovered available source count + missing/custom warnings;
- `Rescan sources` button;
- compact glass table/list showing at least:
  - relative path;
  - kind;
  - origin;
  - authority/priority label;
  - modified/freshness evidence;
  - status;
- custom source path manager with add/remove actions;
- loading state;
- empty state: `No task source files discovered` with guidance;
- missing project / unavailable filesystem error state;
- no fake task count, progress, workflow, owner, or completion claim.

Preserve the accepted H!veAI neon liquid glass visual system.

The canonical background remains centered in the post-sidebar `.main-area`.

Do not redesign the Command Center.

Do not replace the Project rail behavior.

Do not add a large new global navigation item; `/tasks` already exists.

Browser preview may retain deterministic fixtures if needed, but native Tauri mode must use live Registry + native discovery IPC and must never masquerade fixtures as live data.

---

# M08.08 — Security and containment

This milestone reads local project metadata and is security-sensitive.

Required rules:

- canonicalize/physically resolve existing paths before reading;
- verify every physical target remains under the registered physical root;
- reject ordinary `..` escape;
- reject absolute outside-root custom paths;
- reject symlink/junction escape when the platform can resolve it;
- never follow a discovered symlinked directory outside the root;
- do not expose unrestricted arbitrary file reads to frontend;
- do not log full source file content;
- do not log credentials/tokens from paths or contents;
- content hash calculation must be bounded;
- avoid TOCTOU where practical by rechecking containment before opening files if resolution/read are separated;
- errors must identify the source path safely without leaking file content.

If Windows test environment cannot create a symlink/junction due privilege policy, record the exact OS error and mark only that test `UNVERIFIED`; do not substitute an ordinary outside path and claim symlink PASS. Ordinary outside-root rejection must still be directly tested.

---

# M08.09 — Direct evidence matrix

Add focused Rust tests that call production discovery/persistence functions, not duplicated helper logic.

At minimum prove:

1. root `TASKS.md` discovery;
2. case-insensitive standard filename handling without duplicates;
3. bounded `tasks/` directory discovery;
4. bounded `handoffs/` directory discovery;
5. ignored `.git/node_modules/dist/build/target` trees are not traversed;
6. ordinary outside-root custom path rejected;
7. `..` traversal escape rejected;
8. safe custom file inside root accepted;
9. safe custom missing path remains configured and returns `MISSING`;
10. custom path dedupe on Windows-normalized equivalent input;
11. repeated discovery is idempotent in `project_sources`;
12. file modification changes SHA-256/content-hash evidence;
13. deleted STANDARD source reconciles according to documented stale policy;
14. deleted CUSTOM target remains configured/missing;
15. oversized source returns `TOO_LARGE` without content parsing;
16. unreadable source is isolated without corrupting other inventory where deterministically testable;
17. deterministic source-kind/authority/priority ordering;
18. non-Git registered project discovery works;
19. missing/unavailable registered project returns bounded error/state;
20. discovery does not write to the registered project tree;
21. no rows are created in `tasks` or `task_sources` by M08 discovery;
22. physical symlink/junction escape rejected when environment allows, otherwise exact `UNVERIFIED` reason recorded.

Persistence tests must directly inspect `project_sources` rows after production discovery.

Frontend focused tests must prove at minimum:

1. native `/tasks` uses selected live Registry project ID;
2. loading state before source response;
3. discovered source rows render real path/kind/origin/status data;
4. rescan calls production IPC and refreshes visible inventory;
5. custom add triggers native add then refresh;
6. custom remove triggers native remove then refresh;
7. empty state is truthful;
8. missing/error state is truthful;
9. changing selected project refreshes source inventory for that project, not stale prior identity;
10. delayed response from project A cannot overwrite newer selected project B source UI;
11. browser preview fixtures remain clearly non-live and do not invoke native filesystem commands;
12. Command Center selection/layout regressions remain green;
13. M08.00 startup video/background presentation regressions remain green.

Do not count source-code string assertions as substitutes for runtime state-transition tests when the transition can be exercised directly.

---

# M08.10 — Performance limits

Document and enforce bounded discovery.

At minimum define constants for:

- maximum approved-directory recursion depth;
- maximum number of discovered candidate files per project;
- maximum individual source file bytes eligible for hashing;
- maximum custom path count per project.

Suggested starting bounds if no stricter existing policy exists:

- depth: 4;
- candidate files: 512;
- hash/read size: 2 MiB;
- custom paths: 64.

On limit hit, return a structured warning rather than silently hanging or scanning forever.

Discovery should be deterministic and cancellable by command completion/normal process shutdown. Do not spawn permanent source-discovery worker threads in M08.

---

# M08.11 — Documentation and tracker truth

Update `H!veAI/ARCHITECTURE.md` only if necessary to document the concrete M08 discovery boundary.

Update `H!veAI/docs/H!veAI/UI_LAYOUT_GOVERNANCE.md` only for durable Task Sources UI rules, without changing accepted presentation rules.

Update `H!veAI/TASKS.md` truthfully:

- M08.00 PASS/CLOSED;
- M08.00B PASS/CLOSED with manual acceptance;
- M08 Task Source Discovery implementation state according to real evidence;
- M09 remains unstarted.

Do not modify historical M08.00/M08.00B logs or audits.

---

# Allowed implementation scope

Expected files may include:

- new `H!veAI/src-tauri/src/task_sources.rs` or equivalent bounded module;
- `H!veAI/src-tauri/src/lib.rs` command registration;
- `H!veAI/src-tauri/permissions/foundation.toml`;
- `H!veAI/src-tauri/capabilities/default.json`;
- existing DB helpers only as needed for `project_sources`/settings access;
- `H!veAI/src/taskSources.ts` or equivalent typed frontend IPC adapter;
- `/tasks` page implementation in existing page/component structure;
- focused frontend tests;
- focused Rust tests;
- `H!veAI/TASKS.md`;
- bounded architecture/governance updates;
- new immutable M08 builder log.

Do not edit canonical PNG/MP4/logo bytes.
Do not create an installer.
Do not implement M09.

---

# Verification

Run focused source-discovery Rust tests and focused frontend tests first.

Then from `H!veAI`:

```powershell
npm run typecheck
npm test -- --reporter=dot
npm run build
npm audit --audit-level=high
```

Then:

```powershell
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo check --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
```

Then:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\tests\publish-dev-qa-failure-harness.ps1
```

Only if every gate passes:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\publish-dev-qa.ps1
```

Preserve:

- `H!veAI\dev-bin\H!veAI.exe`
- `C:\Users\sekip\Desktop\H!veAI.lnk`
- existing shortcut icon rule;
- M08.00 opening-video behavior.

No installer.

---

# Required immutable builder log

Create:

`H!veAI/docs/H!veAI/codex-logs/M08_TASK_SOURCE_DISCOVERY_LOG.md`

Record at minimum:

- synchronized base HEAD;
- exact changed files from Git;
- exact discovery source kinds and standard paths;
- exact ignore rules and performance bounds;
- exact physical-containment strategy;
- custom-path persistence strategy;
- whether symlink/junction test was PASS or exact UNVERIFIED OS reason;
- `project_sources` persistence/reconciliation strategy;
- explicit confirmation `tasks` and `task_sources` were not populated by M08;
- exact native IPC commands;
- exact ACL/capability changes;
- exact focused Rust test names/results;
- exact focused frontend test names/results;
- full frontend/Rust/audit/harness results;
- publication result + stable EXE SHA-256;
- final local HEAD;
- final `origin/H!veAI` HEAD;
- explicit local == remote equality proof;
- no canonical visual asset byte changes;
- no installer;
- no M09 work;
- manual visual status for `/tasks` Task Sources UI as `PENDING USER VISUAL ACCEPTANCE` unless the user has inspected it after publication.

Builder logs are claims, not independent evidence.

---

# Final stop condition

Stop after M08 Task Source Discovery implementation, tests, publication, log, commit, and push.

Do not start M09 Task Intelligence Parser.
Do not create an installer.
Do not begin M10 or later work.
