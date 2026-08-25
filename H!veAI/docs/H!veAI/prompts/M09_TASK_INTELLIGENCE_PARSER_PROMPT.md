# M09 Task Intelligence Parser - Single Milestone Implementation Prompt

## Mission

Implement the entire M09 Task Intelligence Parser milestone in one bounded Codex run.

This is one milestone prompt. The numbered contracts below are implementation sections, not separate prompts or stop points.

M00-M08 are PASS/CLOSED.
M09 is the only authorized milestone.
Do not start M10.
Do not create an installer.
Do not redesign the H!veAI UI.

The goal is a deterministic, local-first, evidence-backed parser that consumes the already-approved M08 Task Source Discovery inventory and emits normalized task intelligence for later M10-M12 use.

Builder logs are claims only. Do not mark M09 PASS yourself. Stop after implementation, evidence, publication, log, commit, and push.

---

## Mandatory synchronization and reading

Work from:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Before reading this prompt locally:

```powershell
git fetch origin H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
```

Fast-forward only when safe. Never reset, rebase, force-checkout, force-push, or overwrite user-owned/untracked work.

Read completely before editing:

1. `H!veAI/AGENTS.md`
2. `H!veAI/CONSTITUTION.md`
3. `H!veAI/ARCHITECTURE.md`
4. `H!veAI/TASKS.md`
5. `H!veAI/docs/H!veAI/UI_LAYOUT_GOVERNANCE.md`
6. `H!veAI/docs/H!veAI/audits/M08_TASK_SOURCE_DISCOVERY_FINAL_CLOSURE_AUDIT.md`
7. `H!veAI/docs/H!veAI/audits/M08C_CUSTOM_ORDER_BACKCOMPAT_STRICT_REAUDIT.md`
8. `H!veAI/src-tauri/src/task_sources.rs`
9. `H!veAI/src-tauri/src/db/migrations.rs`
10. `H!veAI/src-tauri/src/lib.rs`
11. `H!veAI/src/taskSources.ts`
12. this prompt

At session start record branch, HEAD, remotes, worktrees, tracked changes, and untracked files. Preserve user-owned `start-demo.bat`, `task.md`, and any unrelated local files.

---

# Canonical UI Assets

Canonical user-owned asset root:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Canonical application background:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI\scene 3 starting point.png`

Canonical opening video:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI\videos and gifs\opening video.mp4`

Canonical sidebar logo source:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI\H!veAI logo.png`

Canonical shortcut icon source:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI\H!veAI small logo.png`

M09 must not modify, crop, resize, recolor, regenerate, recompress, replace, or rewrite any canonical PNG/MP4/logo bytes.

M09 is a parser/domain milestone. Preserve the accepted sidebar width/logo scale, startup video lifecycle, post-sidebar background, restrained liquid-glass treatment, Command Center geometry, Registry selection behavior, Task Sources route, footer, launcher, and shortcut.

Do not edit `pages.tsx`, route layout CSS, Command Center CSS, or visual assets unless a direct M09 regression proves a change is unavoidable. If that happens, record why and leave visual status pending manual acceptance.

---

# M09 architecture boundary

M08 is the only discovery authority. M09 must not invent a second repository crawler.

M09 may:

- consume M08-owned AVAILABLE source inventory;
- read approved source document bodies through a bounded, containment-checked production path;
- parse task structure deterministically without AI/network calls;
- persist M09-owned normalized task intelligence in existing H!veAI SQLite tables;
- expose bounded native parse/list commands;
- create sanitized parser fixtures and tests.

M09 must not:

- scan arbitrary repository files outside the M08 inventory;
- mutate registered project files;
- write `.hiveai` files into managed projects;
- use an LLM, network request, GitHub API, shell, child process, or external parser service;
- implement the M10 workflow state machine;
- write task transition events as if workflow execution occurred;
- start agents, prompts, audits, or CI;
- populate Command Center business truth yet;
- expose full source document bodies to the frontend;
- create a new DB migration unless existing M04 tables are objectively insufficient and the log proves why. Prefer existing `tasks`, `task_sources`, `task_dependencies`, and `settings`.

Preferred new native module:

`H!veAI/src-tauri/src/task_intelligence.rs`

Preferred frontend contract wrapper:

`H!veAI/src/taskIntelligence.ts`

---

# P01 - Secure M08-to-M09 source boundary

## Required production behavior

The parse operation must begin from a registered project and the current M08 source inventory.

Use M08 `task_sources::discover()` or an equivalently authoritative refreshed M08 path so parse input is current. Do not independently walk the repository.

Only parse source rows that are:

- M08-owned/versioned inventory;
- `AVAILABLE`;
- text/Markdown source kinds intended to carry task intelligence, including TASKS/TASK/PLAN/PROGRESS/ROADMAP/HANDOFF and approved CUSTOM Markdown/text sources.

`CLAUDE.md`, `AGENTS.md`, and other instruction sources may be read only as adapter context if absolutely required, but must not become tasks merely because they contain bullets. Generic task production from INSTRUCTION sources is prohibited.

Before reading every body:

1. resolve the registered physical project root;
2. reconstruct/validate the source path under that root rather than blindly trusting a stored absolute path;
3. canonicalize an existing target and re-check physical containment;
4. enforce M08 size bounds;
5. read bounded bytes;
6. require valid UTF-8 text;
7. recompute SHA-256 from the bytes actually parsed;
8. compare to the refreshed M08 content hash.

If the file changes between discovery and read, do at most one bounded rediscovery/retry. If it still changes, skip that source and emit a structured `SOURCE_CHANGED_DURING_PARSE` warning. Never loop indefinitely.

One unreadable/invalid source must not prevent other valid sources from parsing.

Parser-level hard bounds:

- maximum parsed tasks per project: 4096;
- maximum stored text per extracted scalar field: 4096 UTF-8 bytes after safe truncation;
- maximum acceptance criteria/blocker/dependency entries per task: 128 each;
- maximum parser warnings retained per project: 512.

When a parser bound is reached, return a structured warning. Do not silently truncate without evidence.

## Required direct tests

Add production-path tests proving:

1. parser input comes from M08 inventory, not arbitrary Markdown elsewhere in the repo;
2. INSTRUCTION source bullets do not become generic tasks;
3. outside-root/path traversal cannot be parsed;
4. a source changed after discovery is retried once and then either safely parsed from matching evidence or skipped with `SOURCE_CHANGED_DURING_PARSE`;
5. invalid UTF-8/unreadable source is isolated while another valid source still parses;
6. task/warning bounds produce structured warnings.

## PASS only if

The test would fail if M09 bypassed M08, trusted an arbitrary absolute path, parsed instruction bullets as tasks, or silently accepted a stale hash.

---

# P02 - Normalized task intelligence model and deterministic identity

## Required production model

Create explicit serializable Rust models for at least:

- `TaskIntelligenceSnapshot`
- `ParsedTask`
- `TaskEvidenceLocator`
- `TaskConfidence`
- `HandoffSummary`
- `ParserWarning`
- parser/adapter identity

A normalized task must expose, at minimum:

- deterministic task id;
- project id;
- source id/path/kind;
- title;
- parsed source status;
- canonical storage state;
- explicit task id when present;
- milestone/context heading;
- required actor hint when explicit;
- blockers;
- dependency references;
- next step when explicit;
- owner gate when explicit;
- external wait when explicit;
- acceptance criteria when explicit;
- confidence score and reasons;
- evidence locator with source path, one-based start/end line, heading path, and content hash;
- adapter id;
- warnings.

Use deterministic IDs with an M09-owned prefix so shared tables can be reconciled safely.

Recommended ownership forms:

- task source id: `m09src:<sha256>`
- task id: `m09task:<sha256>`

Identity rules:

1. If an explicit task ID is present, stable identity must primarily follow project + source + explicit task ID.
2. Without explicit ID, derive identity from project + normalized source path + normalized heading path + normalized task title + deterministic duplicate ordinal among identical siblings.
3. Inserting unrelated lines above a task must not change its id.
4. Two identical task titles under the same heading must remain distinct and deterministic.
5. Same text in two projects must never collide.

## Required direct tests

Prove:

- explicit-ID task remains same id after unrelated line movement;
- fallback task remains same id after unrelated line insertion above it;
- duplicate identical tasks get distinct deterministic ids and stay stable on repeated parse;
- same source text in two project ids creates different task ids.

## PASS only if

No production identity depends on current line number alone and repeated parsing is deterministic.

---

# P03 - Generic deterministic Markdown task parser

## Required syntax behavior

Implement a rule-based parser. Do not use AI/NLP inference.

Maintain Markdown heading context (`#` through `######`) and use it as evidence/milestone context.

Generic actionable task rows include:

- `- [ ] ...` -> parsed status `OPEN`
- `- [x] ...` / `- [X] ...` -> `DONE`
- `- [~] ...` -> `IN_PROGRESS`
- `- [!] ...` -> `BLOCKED`
- explicit task rows beginning with a supported structured prefix such as `TASK:` or a clearly recognized explicit task-id prefix.

Do not convert ordinary prose bullets into tasks simply because they contain verbs.

Support explicit status tokens when they are syntactically attached to a task, for example `[DONE]`, `[BLOCKED]`, `[WAITING]`, `[READY]`, `[IN PROGRESS]`. Do not classify arbitrary prose containing those words.

Milestone extraction must be explicit and deterministic from heading/task syntax, not guessed from free prose.

M09 does not implement M10 workflow transitions. For the existing `tasks.state` column use only this neutral canonical storage policy:

- explicit DONE -> `TASK_COMPLETE`;
- explicit BLOCKED -> `BLOCKED`;
- all other parsed tasks -> `BACKLOG`.

Preserve richer parser truth (`IN_PROGRESS`, `WAITING`, `READY`, etc.) in M09 metadata for M10 to interpret later.

Do not emit CODEX_RUNNING, AUDIT_REQUIRED, WAITING_OWNER, or other operational workflow states from inference in M09.

## Required direct tests

Use fixtures proving:

- all four checkbox markers;
- explicit status tags;
- ordinary bullet is not a task;
- heading stack and milestone context;
- DONE/BLOCKED/BACKLOG storage mapping;
- parser does not invent operational M10 states.

## PASS only if

The parser can parse the same fixture twice into byte-equivalent normalized semantic output except parse timestamp fields.

---

# P04 - Structured blockers, next step, actors, waits, dependencies, and acceptance criteria

## Required extraction rules

Extract structured child/adjacent metadata only from explicit labels associated with a task. Support at least these case-insensitive labels:

- `Blocker:` / `Blockers:` / `Blocked by:`
- `Depends on:` / `Dependency:` / `Dependencies:`
- `Next:` / `Next step:`
- `Owner:` / `Actor:` / `Required actor:`
- `Waiting for:`
- `External:` / `External wait:`
- `Acceptance:` / `Acceptance criteria:` / `AC:` / `Definition of Done:`

Associate indented child bullets/lines with the nearest parent task until the next sibling task or heading boundary.

Required actor normalization must be explicit and limited to:

- `Human`
- `Codex`
- `Claude`
- `GPT Audit`
- `CI`
- `External`

Unknown actor text remains evidence/metadata but `tasks.required_actor` must stay NULL rather than guessed.

Dependency rules:

- preserve every explicit dependency reference string in metadata;
- resolve to `task_dependencies` only when the reference matches one unambiguous parsed task explicit ID in the same project snapshot;
- unresolved/ambiguous references remain metadata plus structured warning;
- use M09-owned `dependency_kind = 'SOURCE_EXPLICIT'` for edges M09 writes.

Acceptance criteria must remain attached to the task that explicitly owns them.

## Required direct tests

Prove:

1. explicit blocker/next/owner/wait/AC fields attach to the correct task;
2. a casual sentence containing `blocked` does not become a blocker without structured syntax;
3. exact dependency ID resolves to one `SOURCE_EXPLICIT` edge;
4. unresolved and ambiguous refs do not create false DB edges and emit warnings;
5. unknown actor does not populate `required_actor`.

## PASS only if

Structured fields are source-evidenced and no free-prose actor/dependency guesses are persisted.

---

# P05 - Handoff current/next-session intelligence

## Required behavior

HANDOFF sources must produce structured handoff summary data without turning all handoff prose into fake tasks.

Recognize case-insensitive heading families for at least:

- current/current task/now/current state;
- next/next step/next steps/next session;
- blockers/blocked;
- waiting/waiting for/external wait.

Store one-based line evidence for each extracted handoff element.

Checklist tasks inside a handoff may still be parsed as tasks through the generic/adapted parser.

Plain narrative under a handoff heading is handoff summary evidence, not automatically a task.

## Required direct tests

Fixture must prove:

- Current and Next session sections are extracted separately;
- blocker/waiting summary is retained;
- narrative does not become a task;
- checklist under Next can become a task with correct locator.

## PASS only if

The handoff summary can be returned from persisted M09 state without rereading arbitrary files in the list operation.

---

# P06 - Generic + FormuLab + ScrubBots + FMCG adapters

## Required architecture

Create a small parser-adapter abstraction, for example:

```text
TaskParserAdapter
  id()
  matches(project/source context)
  parse/augment generic normalized evidence
```

Required adapter ids:

- `generic`
- `formulab`
- `scrubbots`
- `fmcg-erp-system`

Generic is always the safe fallback.

Repo-specific adapters must not be empty name aliases that claim special parsing without evidence.

Before coding adapter-specific rules, inspect the registered projects through the current Project Registry + M08 source inventory, read-only:

- FormuLab
- ScrubBots
- fmcg-erp-system / FMCG ERP equivalent registered project

Do not hardcode local absolute user paths. Resolve through Registry.

For each available target project:

1. identify at least one real structural convention not already fully represented by the generic fixture, such as task-id notation, section naming, status notation, handoff naming, or nested acceptance/blocker structure;
2. implement only that evidenced convention;
3. create a tiny sanitized synthetic fixture that reproduces the structure, not the user's real task content;
4. record the inspected source path and structural convention in the M09 log without copying sensitive/full document content.

If a target project is not registered/available, do not invent its content. Keep the adapter selectable and generic-safe, mark the missing real-project convention `UNVERIFIED` in the log, and do not falsely claim adapter-specific evidence PASS.

## Required direct tests

At minimum:

- generic fixture selects generic;
- FormuLab fixture selects `formulab` and proves its evidenced convention;
- ScrubBots fixture selects `scrubbots` and proves its evidenced convention;
- FMCG fixture selects `fmcg-erp-system` and proves its evidenced convention;
- a similarly named unrelated project does not accidentally select the wrong adapter.

## PASS only if

Each claimed adapter-specific rule is traceable to actual local/repository structural evidence or is truthfully marked UNVERIFIED instead of invented.

---

# P07 - Safe persistence and idempotent reconciliation

## Use existing schema

Prefer the existing M04 tables:

- `tasks`
- `task_sources`
- `task_dependencies`
- `settings`

Do not add a migration unless unavoidable.

## Ownership contract

M09 must never blanket-delete shared tables.

Use explicit ownership:

- deterministic `m09src:` ids in `task_sources`;
- deterministic `m09task:` ids plus `metadata_json.owner = 'M09_TASK_INTELLIGENCE_PARSER'` and `schemaVersion = 1` in `tasks`;
- M09 dependency edges use `dependency_kind = 'SOURCE_EXPLICIT'` and only originate from M09-owned tasks;
- project-level parser snapshot/handoff/warnings may use one H!veAI-owned settings key such as `task_intelligence.snapshot.<project_id>` with owner/schema metadata.

Persist in one transaction per successful project parse.

A task metadata payload must contain enough normalized/evidence data for `list` to reconstruct task intelligence without rereading source files.

Required reconciliation behavior:

- repeated unchanged parse -> same M09 ids and row counts;
- changed metadata for same identity -> update, not duplicate;
- removed source/task -> stale M09-owned rows removed;
- unrelated/legacy `tasks`, `task_sources`, dependencies, settings remain byte-for-byte untouched;
- M09 never writes `task_events`;
- project source files are never mutated.

## Required direct SQL tests

Prove directly in SQLite:

1. M09 owner/schema metadata;
2. unchanged parse idempotency;
3. task/source removal reconciliation;
4. unrelated legacy task/source/settings preservation;
5. exact dependency edge ownership;
6. `task_events` count unchanged at zero for M09 fixture;
7. no registered project file bytes changed.

## PASS only if

The tests would fail under any `DELETE ... WHERE project_id = ?` blanket reconciliation strategy.

---

# P08 - Native IPC and TypeScript contract, no visible UI change

## Required commands

Expose only two new bounded native commands unless a third is strictly necessary:

- `hiveai_task_intelligence_parse(project_id)`
- `hiveai_task_intelligence_list(project_id)`

`parse` may read approved M08 sources and persist M09-owned app-data state.
`list` must read persisted M09 state only and must not crawl/read managed project files.

Add one narrow Tauri permission such as:

`allow-task-intelligence`

Add only that permission to the main-window capability.

Do not grant shell, unrestricted filesystem, network, or broad allow-all permission.

Add a TypeScript contract wrapper in `src/taskIntelligence.ts` with typed models and invoke wrappers for future M11/M12 use.

Do not connect M09 data to Command Center or redesign `/tasks` in this milestone.

## Required direct tests

- Rust tests exercise production parse/list functions;
- TypeScript focused test mocks Tauri invoke and proves exact command names/arguments for parse/list;
- browser-rendered routes do not begin invoking M09 automatically merely because the module exists.

## PASS only if

M09 remains explicit/on-demand and introduces no background parser worker or route-driven hidden project reads.

---

# P09 - Error isolation, status boundaries, and truthful warnings

## Required behavior

Reuse the registered-project boundary already established by M08:

- ACTIVE project may parse;
- MISSING/unavailable registered root -> bounded error;
- ARCHIVED -> rejected.

Structured warning codes should be stable strings. Include at least the warnings needed by implemented behavior, such as:

- `SOURCE_CHANGED_DURING_PARSE`
- `INVALID_UTF8`
- `SOURCE_READ_FAILED`
- `TASK_LIMIT_REACHED`
- `WARNING_LIMIT_REACHED`
- `UNRESOLVED_DEPENDENCY`
- `AMBIGUOUS_DEPENDENCY`
- adapter evidence warning when applicable.

One bad source must not fabricate a clean PASS snapshot. Snapshot warnings must expose isolation failures.

Never log full source bodies. Error/log messages may include bounded relative source path and error class, not secrets or entire content.

## Required direct tests

- archived project rejected;
- missing root rejected;
- bad source + good source returns good tasks plus warning;
- warning list is bounded;
- no source body appears in serialized warning/error output.

---

# P10 - Evidence locator and deterministic confidence

## Required evidence locator

Every task must carry:

- relative source path;
- source content hash;
- one-based start line;
- one-based end line;
- heading path;
- explicit locator text/id only when source-evidenced.

Line ranges must cover the task row and its explicitly attached structured child metadata, not unrelated sibling tasks.

## Required confidence

Confidence must be deterministic, bounded `0.0..1.0`, and accompanied by machine-readable reason strings.

Use a simple explicit scoring policy and document it in code. Recommended policy:

- checklist task base: `0.70`;
- explicit `TASK:` row base: `0.65`;
- explicit task ID: `+0.10`;
- explicit structured status: `+0.05`;
- heading/milestone context: `+0.05`;
- one or more structured blocker/next/actor/dependency/AC fields: `+0.05`;
- evidenced repo-specific adapter convention: `+0.05`;
- cap at `1.00`.

Do not create generic tasks below base actionable-task confidence. Confidence is evidence quality, not business priority.

## Required direct tests

- exact locator lines for a task with child AC/blocker lines;
- sibling task is outside the locator range;
- deterministic confidence and reasons on repeat parse;
- adapter-specific bonus occurs only when its evidenced convention matched.

---

# Required fixture set

Create small sanitized fixtures under a dedicated M09 fixture location, preferably:

`H!veAI/tests/fixtures/m09/`

Required fixtures:

1. generic checklist/headings/status fixture;
2. blockers/dependencies/next/actor/AC fixture;
3. handoff current/next fixture;
4. FormuLab structural fixture;
5. ScrubBots structural fixture;
6. FMCG ERP structural fixture;
7. duplicate-task identity fixture;
8. false-positive prose fixture.

Fixtures must be synthetic and minimal. Do not copy private project documents wholesale into the H!veAI repo.

---

# Expected changed-file scope

Expected production changes should center on:

- `H!veAI/src-tauri/src/task_intelligence.rs` new
- `H!veAI/src-tauri/src/lib.rs`
- `H!veAI/src-tauri/permissions/foundation.toml`
- `H!veAI/src-tauri/capabilities/default.json`
- `H!veAI/src/taskIntelligence.ts` new
- focused M09 TypeScript test
- M09 fixture files
- `H!veAI/ARCHITECTURE.md`
- `H!veAI/TASKS.md`
- `H!veAI/docs/H!veAI/codex-logs/M09_TASK_INTELLIGENCE_PARSER_LOG.md` new

`Cargo.toml` may change only if a small parser dependency is genuinely justified. Prefer deterministic local Rust logic and existing dependencies. If adding a dependency, explain why native/std logic was insufficient and run audit/security gates.

Do not change existing M08 audit/log/prompt history.

---

# Tracker truth

At the beginning of implementation, update current truth prospectively:

- M00-M08 PASS/CLOSED;
- M09 ACTIVE;
- M10+ blocked/unstarted.

At the end, mark each M09 checklist line `[x]` only when directly implemented and evidenced:

- headings/checklists/milestones/status tags/task IDs;
- blockers/next-step/owner-gate/external-wait parsing;
- handoff current/next session parsing;
- confidence/evidence locator;
- generic adapter;
- FormuLab/ScrubBots/FMCG adapters;
- regression fixtures.

Do not mark M09 PASS/CLOSED. Final tracker status after builder completion must be:

`M09 implementation complete, PENDING INDEPENDENT STRICT AUDIT`.

If adapter-specific real evidence is unavailable, leave that exact sub-item truthful rather than checking it falsely.

---

# Focused evidence requirements

Run focused M09 Rust tests separately before the full suite.

The M09 log must list every focused Rust test name with individual PASS/FAIL, not only a count.

Run the focused TypeScript M09 adapter test separately and record its exact test name(s).

For each P01-P10 contract, the log must contain this compact self-audit block:

```text
P0X
Production symbol(s):
Exact focused test(s):
Pre-fix/missing behavior the test would catch:
Post-fix behavior proved:
PASS / FAIL / UNVERIFIED:
```

Do not claim PASS merely because a plausibly named test exists.

For tests that are meant to close a defect class, explain why the assertion exercises the production path rather than helper-only logic.

---

# Full verification

From `H!veAI` run:

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

Then run the existing publisher failure harness:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\tests\publish-dev-qa-failure-harness.ps1
```

Only if every automated gate passes, publish the stable QA build:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\publish-dev-qa.ps1
```

Required publication invariants:

- Tauri production `--no-bundle` development QA build;
- stable `H!veAI/dev-bin/H!veAI.exe` refreshed only after candidate validation;
- stable Desktop `H!veAI.lnk` still targets the EXE directly;
- shortcut icon remains canonical derivative;
- no installer;
- no browser-hosted H!veAI shell;
- no visible console regression;
- canonical PNG/MP4/logo hashes unchanged.

Because M09 should make no visible UI change, no new manual visual gate is required unless you changed visible production UI despite this prompt.

---

# Required immutable M09 log

Create only:

`H!veAI/docs/H!veAI/codex-logs/M09_TASK_INTELLIGENCE_PARSER_LOG.md`

Record:

- synchronized starting HEAD;
- final implementation/publication commit;
- exact changed files;
- P01-P10 self-audit blocks;
- exact parser ownership/schema/id policy;
- exact neutral `tasks.state` mapping used by M09;
- exact structured-label grammar;
- exact confidence formula;
- exact adapter selection rules;
- FormuLab/ScrubBots/FMCG inspected evidence paths and only structural convention summaries;
- any adapter `UNVERIFIED` reason;
- every focused Rust test name + individual result;
- every focused TypeScript M09 test name + individual result;
- full frontend and Rust test counts;
- typecheck/build/audit result;
- direct SQL persistence/idempotency/preservation evidence;
- source-tree non-mutation evidence;
- publisher harness result;
- stable EXE SHA-256 and size;
- canonical asset hashes unchanged;
- no installer;
- no M10 work;
- final local HEAD and `origin/H!veAI` equality after push.

Do not rewrite the log after it is treated as historical evidence except for one immediately-following documentation-only publication-equality correction if technically necessary. Prefer to record equality correctly in the initial publication sequence.

---

# Prohibited shortcuts

Do not:

- parse every Markdown bullet as a task;
- use regex/string test names as a substitute for production-path assertions;
- hardcode task intelligence for known projects;
- copy real private task documents into test fixtures;
- trust builder comments as evidence;
- blanket-delete rows from shared DB tables by project id;
- infer workflow states beyond the neutral M09 policy;
- write task events;
- invoke M10 state transitions;
- mutate registered project files;
- hide parse failures by returning an empty clean snapshot;
- add background parsing workers;
- start M10 after M09 tests pass.

---

# Final stop condition

Stop after all M09 implementation, focused evidence, full regression/security gates, stable QA publication, immutable log, commit, push, and local/origin equality verification.

If any P01-P10 production requirement or required adapter evidence remains materially incomplete, record `FAIL` or `UNVERIFIED` truthfully and stop. Do not patch around the audit contract by weakening tests.

If automated/source evidence is clean, final builder status must be:

`M09 IMPLEMENTATION COMPLETE - PENDING INDEPENDENT STRICT AUDIT`

Do not start M10.
