# M11A REV3 - Consolidated Strict Closure After Latest Builder Log

## Authority

This is the single authoritative Codex entry prompt for the next H!veAI run.

It supersedes the execution order in:

- `H!veAI/docs/H!veAI/prompts/M11A_GLOBAL_COMMAND_CENTER_STRICT_CLOSURE_PROMPT.md`
- `H!veAI/docs/H!veAI/prompts/M11A_GLOBAL_COMMAND_CENTER_STRICT_CLOSURE_REV2_PROMPT.md`

Do not split this into M11B/M11C or multiple milestone runs. This is one bounded continuation of M11A.

Do not start M12.

Current strict completed roadmap count remains **11 / 20 = 55%** until independent M11 closure.

The latest builder run already implemented substantial M11A remediation. Preserve those source-level fixes. The purpose of REV3 is to merge:

1. the latest M11A builder result;
2. the independent post-log source audit;
3. the user's newer topbar/footer decision;
4. the user's single Project Dashboard tracking decision;
5. the cross-repository Project Dashboard/Akilta standard.

---

# Mandatory preflight

Work from:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Run first:

```powershell
git fetch origin H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
```

Fast-forward only when safe:

```powershell
git merge --ff-only origin/H!veAI
```

Never reset, rebase, force-push, overwrite user work, create `H!veAI\.git`, or modify unrelated parent-root files.

Preserve user-owned untracked:

- `start-demo.bat`
- `task.md`

Read in this order before editing:

1. `H!veAI/AGENTS.md`
2. `H!veAI/CONSTITUTION.md`
3. `H!veAI/TASKS.md`
4. `H!veAI/CODEX_ROADMAP.md`
5. `H!veAI/docs/H!veAI/codex-logs/M11A_GLOBAL_COMMAND_CENTER_STRICT_CLOSURE_LOG.md`
6. `H!veAI/docs/H!veAI/audits/M11_GLOBAL_COMMAND_CENTER_PROJECT_DASHBOARD_STRICT_AUDIT.md`
7. `H!veAI/docs/H!veAI/audits/M11A_POST_LOG_STRICT_REAUDIT_AND_PRODUCT_DELTA.md`
8. prior M11A prompt
9. prior M11A REV2 prompt
10. `H!veAI/docs/H!veAI/prompts/CROSS_REPO_SINGLE_DASHBOARD_AKILTA_ATTRIBUTION_PROMPT.md`
11. current `.hiveai/PROJECT_DASHBOARD.md`
12. current M07/M08/M09/M10/M11 source and tests touched below
13. this REV3 prompt in full

The post-log audit is the acceptance delta for this run.

---

# Task 0 - FIRST TASK: synchronize tracker truth

Before production code changes, update the prospective live tracker/status docs so they say:

- M00-M10 = PASS/CLOSED;
- strict completed = 11/20 = 55%;
- original M11 strict audit = historical FAIL;
- latest M11A builder run completed substantial remediation but M11 is still NOT CLOSED;
- `M11A_POST_LOG_STRICT_REAUDIT_AND_PRODUCT_DELTA.md` is the current independent decision;
- M11A REV3 = ACTIVE during this run;
- M12 remains BLOCKED;
- R11-R14/E04 plus P0/P1/P2 below are open;
- user native visual acceptance remains pending.

At minimum synchronize:

- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/README.md`
- `H!veAI/docs/H!veAI/README.md`

Do not rewrite historical prompts/logs/audits.

---

# Canonical UI assets and protected behavior

Canonical user-owned asset root:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Preserve:

- H!veAI sidebar logo and accepted geometry;
- accepted background position after sidebar;
- startup intro lifecycle and audible playback;
- X01 terminal-popup suppression;
- X02 startup audio / no same-process replay;
- stable EXE/shortcut/icon behavior;
- dark/glass application identity;
- Chrome-only Akilta external browser implementation unless explicitly changed below only in click-surface geometry;
- no Edge fallback;
- no terminal/console flash.

Required unchanged hashes:

- background: `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`
- opening video: `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`

Do not create an installer.

Do not modify any tracked external registered project repository in this H!veAI run.

In particular: **do not touch Bulk Edit** while its Etsy approval process is pending.

---

# P0 - ABSOLUTE FIRST PRODUCTION TASK: remove the bottom footer and move Akilta attribution into the topbar

The user explicitly wants the bottom footer removed because it consumes valuable vertical workspace.

Current source still renders `<footer className="global-footer">` in `src/components/Shell.tsx`. Remove that dedicated footer band completely and reclaim its height.

## Required placement

Move the existing Akilta attribution into the flexible topbar space between:

- left breadcrumb/title (`Workspace / Command Center`, etc.);
- right `Search workspace` control and topbar actions.

Conceptual desktop composition:

```text
[ Workspace / Command Center ]   [ Akilta wordmark + Built with ♥ for maximum productivity by Akilta ]   [ Search workspace ] [ actions ]
```

The attribution must look intentionally integrated into the topbar.

## Required attribution content

Reuse the existing tracked H!veAI Akilta asset. Do not redraw/regenerate it.

Visible content must retain:

`Built with ♥ for maximum productivity by Akilta`

Use the Akilta wordmark next to/within the attribution where it fits cleanly.

The **entire attribution target**, including wordmark + text, must be one clickable/focusable target.

Required tooltip/title:

`Developed by Akilta`

Required destination:

`https://www.akilta.com/`

For native H!veAI, keep the existing safe parameterless native external-browser command. Frontend must not supply an arbitrary URL.

Preserve:

- Google Chrome behavior;
- H!veAI remains open;
- no terminal flash;
- no Edge fallback.

## Responsive requirements

- no overlap with breadcrumb/title;
- no overlap with Search Workspace;
- do not materially increase topbar height;
- no horizontal overflow;
- at narrower widths, reduce/hide nonessential prefix presentation before compromising core navigation/search;
- keep at least an identifiable accessible Akilta target when space is constrained;
- old footer spacer/CSS must not remain as dead vertical space.

## P0 direct tests

Prove:

1. routed shell contains no bottom `global-footer` band;
2. exact credit appears in topbar at desktop width;
3. existing Akilta wordmark is used;
4. whole attribution is one accessible click/focus target;
5. tooltip/title is `Developed by Akilta`;
6. native click invokes only `hiveai_open_akilta`;
7. breadcrumb and Search Workspace remain visible/non-overlapping at representative desktop widths;
8. no outer vertical/horizontal overflow is introduced;
9. removed footer height becomes actual page workspace.

P0 must be completed before P1/P2.

---

# P1 - Implement real SINGLE-DASHBOARD watch architecture

## Product decision

For migrated projects, H!veAI actively watches only:

`.hiveai/PROJECT_DASHBOARD.md`

for routine **project-status intelligence** changes.

M08/M09 source discovery remains available for deep evidence/inventory, but TASKS/AGENTS/audits/logs/prompts/handoffs/roadmaps/source files must not each act as independent live project-status triggers once a project declares single-dashboard mode.

## Extended v1 contract

Keep backward-compatible identity:

```text
hiveaiDashboardSchema: hiveai-project-dashboard/v1
dashboardMode: source-map
```

Recognize:

```text
trackingMode: single-dashboard-watch
refreshPolicy: project-agent-maintained; H!veAI watches only .hiveai/PROJECT_DASHBOARD.md
```

Add `trackingMode` to the typed Project Dashboard resolver output.

Do not require a schema v2 for this extension.

## Actual watcher attachment must become narrow

Current source recursively watches the entire registered project root with `RecursiveMode::Recursive`. For a valid migrated `single-dashboard-watch` project, that is no longer acceptable for routine project-status tracking.

Implement a narrow production watcher configuration.

Preferred design:

- watch the physically contained `.hiveai` directory **non-recursively**;
- filter exactly `PROJECT_DASHBOARD.md` for project-status intelligence refresh;
- use only the minimum additional root-lifecycle watch genuinely required to retain missing/moved-root safety;
- do not recursively subscribe to the entire project source tree in single-dashboard mode;
- atomic replace/rename of PROJECT_DASHBOARD must still be detected;
- symlink/junction containment escape must remain rejected.

If Windows notify semantics require watching the parent `.hiveai` directory rather than the file itself, do that. The requirement is about the actual scope and event filtering, not a fragile file-handle trick.

### Single-dashboard event behavior

For `trackingMode: single-dashboard-watch`:

- change to `.hiveai/PROJECT_DASHBOARD.md` -> valid live project-status refresh trigger;
- change to `TASKS.md` only -> no automatic project-status/M09/M11 refresh;
- change to AGENTS/audit/log/prompt/handoff/roadmap/changelog/architecture/decision/src only -> no automatic project-status/M09/M11 refresh;
- explicit user `Rescan` may still perform bounded M08/M09 deep refresh;
- when PROJECT_DASHBOARD changes, one bounded existing M08/M09 refresh is allowed so internal evidence can be re-read at that signal, then M11 snapshot is refreshed;
- do not create another parser;
- do not write project files;
- no polling;
- no generated commits;
- no refresh loop caused by H!veAI itself.

### Git evidence in single-dashboard mode

Do not keep full-root recursion merely to preserve Git snapshots.

Choose the narrowest truthful approach:

- capture bounded Git evidence when PROJECT_DASHBOARD refresh occurs and on explicit rescan; or
- use a separately justified narrow Git metadata watch that does not re-expand to the whole project tree.

Document the final choice and test it.

### Legacy compatibility

If manifest is ABSENT, MALFORMED, STALE, or valid but does not declare `single-dashboard-watch`, preserve the existing safe legacy watcher behavior until that project is migrated.

Do not brick old projects.

## P1 production-path tests

Use real temp project roots/watcher paths and prove:

1. migrated single-dashboard project is not recursively watched across the full source tree for status changes;
2. `TASKS.md` write alone does not trigger routine M09/M11 refresh;
3. arbitrary `src/*` write alone does not trigger routine M09/M11 refresh;
4. `.hiveai/PROJECT_DASHBOARD.md` write triggers refresh;
5. atomic dashboard rename/replace triggers refresh;
6. dashboard signal causes bounded M09 evidence re-read exactly through the accepted production path;
7. Advanced source inventory remains available after explicit discovery/rescan;
8. legacy project retains old fallback behavior;
9. missing/moved project safety still works;
10. no duplicate watcher is attached after registry refresh/repair;
11. no cross-project event leakage.

Tests must fail against the current pre-REV3 recursive watcher implementation.

---

# P2 - Parse and consume the materialized Project Dashboard status contract

The Project Dashboard is no longer pointer-only in migrated projects.

It remains the single H!veAI-facing contract and also materializes bounded project status so H!veAI does not need to live-watch many internal files.

Read the shared authoring standard:

`H!veAI/docs/H!veAI/prompts/CROSS_REPO_SINGLE_DASHBOARD_AKILTA_ATTRIBUTION_PROMPT.md`

## Required sections

Support these optional materialized sections while preserving current Source authorities parsing:

- `## H!veAI live status`
- `## Current work`
- `## Blockers and waiting`
- `## Milestone summary`
- `## Quality and verification`
- `## Recent meaningful activity`
- `## Provenance`

### Required `H!veAI live status` fields

Parse exact labels from the standard:

- Project status
- Health
- Current milestone
- Current task
- Current task ID
- Current workflow state
- Progress
- Required actor
- Next action
- Waiting on
- Last meaningful update

Treat `UNKNOWN`, `NOT_VERIFIED`, `NONE`, empty optional data and real values distinctly.

Do not turn unknown into zero.

## Typed resolver additions

Expose a bounded typed materialized status object in `ProjectDashboardResolution`, reusable by M12 later.

At minimum include:

- tracking mode;
- live project status;
- health;
- milestone;
- current task title;
- current task ID;
- declared workflow state;
- progress raw/normalized when safely parseable;
- required actor;
- next action;
- waiting-on;
- last meaningful update;
- current work rows;
- blockers/waiting rows or bounded text facts;
- quality/verification facts;
- recent meaningful activity facts;
- provenance entries.

## Bounds

Use explicit production bounds. At minimum:

- materialized section line remains within existing manifest line bound;
- scalar <= existing safer scalar limit, never unbounded;
- Current work <= 10 rows;
- Blockers/waiting <= 10 items;
- Milestone summary <= 10 items;
- Quality/verification <= 10 items;
- Recent meaningful activity <= 10 items;
- Provenance <= 32 entries;
- materialized warning count uses existing bounded warning collector;
- no source body is recursively read because it appears under Provenance.

Reject malformed table shapes safely without crashing the portfolio.

## Truth precedence

For migrated single-dashboard mode:

1. M10 remains the strongest authority for H!veAI-owned operational workflow state when a matching task identity exists.
2. Materialized Project Dashboard status is the primary project-summary/status export.
3. M09 internal task intelligence is supporting/deep evidence refreshed only at dashboard signal or explicit rescan.
4. Source-authority pointers are provenance.

If Dashboard status conflicts with stronger M10 truth:

- do not overwrite M10;
- surface a bounded explicit conflict warning;
- show the stronger value in operational workflow fields;
- preserve declared dashboard value in provenance if useful.

Do not count the same task twice because it exists in dashboard Current work and M09.

## Command Center integration

For a valid single-dashboard project:

- Current Project/Current Task should consume materialized dashboard status when no stronger M10 match exists;
- Next action / Waiting on / milestone / health may consume materialized verified fields;
- task/progress numbers only appear when the dashboard explicitly supplies a verified numeric/fractional value or M09 provides authoritative known truth;
- no fake `0/0`;
- dashboard materialized status must not silently create M10 task events.

---

# P3 - Dogfood the extended contract on H!veAI's own Project Dashboard

Update only the H!veAI repository's own:

`.hiveai/PROJECT_DASHBOARD.md`

Do not touch external project repositories.

Add:

```text
trackingMode: single-dashboard-watch
refreshPolicy: project-agent-maintained; H!veAI watches only .hiveai/PROJECT_DASHBOARD.md
```

Add/update the materialized status sections required by the shared cross-repo standard using only verified H!veAI repository truth.

For the current run, it must truthfully reflect:

- M00-M10 PASS/CLOSED;
- M11/M11A current active/not-closed state;
- strict completed count 11/20 = 55%;
- REV3 current work;
- M12 blocked;
- verified latest audit status;
- exact next action at builder exit;
- relevant test/build/audit state only after it is actually run.

Do not fabricate timestamps/percentages beyond verified tracker/evidence.

Keep Source authorities as provenance pointers.

This dogfood fixture must be used in resolver/Command Center tests.

---

# P4 / R14 - Correct legacy fallback attention and health semantics

Current source treats any `dashboard.warnings` as project ATTENTION and emits Needs Your Attention rows for every dashboard warning. This makes a supported legacy `ABSENT` manifest look like an operational failure.

Fix the contract.

## Required severity semantics

### Informational / no operational attention by itself

- `ABSENT` manifest when legacy fallback is supported;
- benign non-actionable warning that does not invalidate authority;
- source-authority provenance note.

### Configuration attention

- MALFORMED;
- STALE repository identity;
- UNAVAILABLE manifest/root when it should be available;
- rejected canonical task authority;
- active single-dashboard refresh degraded/failing;
- explicit materialized-status conflict with stronger M10 truth.

### PARTIAL

Missing secondary context/provenance may be shown as partial/config detail, but must not automatically be called workflow failure.

## Health truth

Do not count a project as HEALTHY merely because there are no workflow rows if project health is genuinely unknown.

Use explicit UNKNOWN where task/status health cannot be established.

Do not let informational ABSENT fallback inflate Needs Your Attention.

Add direct tests for:

- ABSENT legacy -> fallback, no attention solely from absence;
- MALFORMED -> config attention;
- STALE -> config attention;
- PARTIAL secondary source missing -> canonical authority preserved and no fake workflow failure;
- NOT_CANONICALIZED / unavailable health -> does not fabricate HEALTHY unless materialized dashboard truth actually proves health.

---

# P5 / E04 - Remove fabricated audit/test actors from Recent Activity

Current `read_activity` hard-codes:

- every audit row actor = `GPT Audit`;
- every test row actor = `CI`.

The existing SQLite schema does not prove those actor identities on `audits` or `test_runs`.

Required behavior:

- actor/provider is null when the row does not prove one;
- do not infer provider from table type;
- keep real provider on agent/session evidence;
- keep real actor_type on workflow events;
- if a future schema/evidence relation genuinely proves audit/test actor, use that relation explicitly, not a string constant.

Frontend already supports optional actor. Omit unproved labels.

Direct test must seed audit/test rows and prove actor is null rather than GPT Audit/CI.

---

# P6 - Preserve every latest M11A source fix already closed

Do not regress the current implementation while changing watcher/dashboard semantics.

Keep and re-test:

- R01 real bounded M10 workflow integration;
- R02 null/unknown metric semantics;
- R03 TASK_COMPLETE exclusion from current task;
- R04 contained directory provenance;
- R05 refresh health failure/recovery and last-good preservation;
- R06 mixed real evidence aggregation;
- R08 warning cardinality bounds;
- E01 neutral browser preview;
- E02 structured brief provenance;
- UX01 bounded one-screen Command Center intent;
- UX02 no giant full-width Recent Activity on Command Center home;
- UX03 raw source inventory behind Advanced source inventory;
- stable selected-project/generation/listener cleanup behavior.

The latest builder log claims these were implemented and source review confirms the production paths are present. REV3 must treat them as regression-protected, not redesign them unnecessarily.

---

# P7 - Task Sources UI wording for single-dashboard mode

The default Task Sources screen should explain the distinction cleanly.

For a migrated project display, at minimum:

```text
Project Intelligence / Dashboard Contract
Entry contract: .hiveai/PROJECT_DASHBOARD.md
Live tracking: SINGLE_DASHBOARD
Internal evidence sources: <discovered count>
Manifest: <status>
Refresh: <status/time>
```

The raw table remains under:

`Advanced source inventory`

When expanded, add a clear label such as:

`Internal evidence / provenance. These files are not independent live-watch targets in SINGLE_DASHBOARD mode.`

Do not delete M08 inventory.

Do not show the raw 15-source table by default.

Explicit Rescan may refresh deep inventory.

---

# P8 - Command Center layout/native acceptance safeguards

The latest builder source removed the giant home Recent Activity panel and bounded visible rows. Preserve that.

After P0 removes the footer, use the recovered height to make the home dashboard more comfortable, not to add another large panel.

At normal desktop widths:

- no page-level Command Center vertical scrollbar;
- no project-rail scrollbar;
- no Needs Your Attention scrollbar;
- no Active Work Queue scrollbar;
- Active Work Queue fully visible;
- System Status compact;
- panels do not overlap;
- footer no longer exists;
- topbar attribution does not collide with Search Workspace;
- current task area does not show fake `0 / 0`.

Use bounded visible rows + `View all` affordances instead of nested scrolling.

Test representative desktop widths including approximately 1280 and 1536/1600.

User native visual acceptance remains pending after publication.

---

# Native Rust test execution is mandatory

The latest builder log diagnosed Windows `STATUS_ENTRYPOINT_NOT_FOUND` and used a shell-local embedded common-controls manifest through RUSTFLAGS.

Do not claim native evidence from `cargo test --no-run`.

Use the narrow repository/shell-local workaround only if still required. Do not change global machine state.

Run real assertions.

At minimum include current production-path tests for:

- Project Dashboard parsing and bounds;
- trackingMode parsing;
- materialized status parsing;
- single-dashboard watcher attachment and event filtering;
- TASKS change does not auto-refresh migrated project;
- Dashboard change does refresh migrated project;
- atomic replace behavior;
- legacy watcher fallback;
- M10 workflow integration;
- current-task completion precedence;
- ABSENT/MALFORMED/STALE attention semantics;
- no-double-count materialized dashboard vs M09;
- mixed activity with null unproved audit/test actors;
- warning bounds;
- cross-project containment;
- H!veAI dogfood manifest.

If native assertions cannot execute, STOP and leave M11 blocked. Do not publish a PASS claim.

---

# Full verification

After focused tests:

```text
cargo fmt -- --check
cargo check
cargo test
npm run typecheck
npm test
npm run build
npm audit --audit-level=high
git diff --check
```

Use Windows shell spelling as required.

Run the governed publisher/failure harness from `AGENTS.md` and produce the stable no-bundle QA EXE.

No installer.

Verify canonical background/opening-video hashes unchanged.

Verify source/config regression for:

- X01 terminal suppression;
- X02 startup audio/no replay;
- Akilta Chrome external open;
- no Edge fallback;
- stable shortcut/icon.

---

# Security / boundary review

Before final push explicitly inspect:

- every changed watcher attachment path and recursion mode;
- physical containment for `.hiveai` watch target;
- no arbitrary frontend filesystem path;
- no new generic filesystem IPC;
- no arbitrary shell/process/network access;
- no source body in event payloads;
- no project-file write by H!veAI runtime;
- no generated dashboard commit loop;
- no cross-project cache/event leakage;
- no external project repository edits;
- no Bulk Edit edits.

---

# Final repository equality evidence

The new immutable builder log must contain concrete final implementation/evidence equality proof.

After implementation/evidence commit is pushed:

```powershell
git fetch origin H!veAI
git rev-parse HEAD
git rev-parse origin/H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
```

Persist actual SHA values and `0 0` divergence in the log.

Do not leave `pending` placeholders.

If a final log-only commit follows, record both implementation/evidence equality and the final log commit SHA truthfully without creating an infinite self-reference loop.

---

# Builder log

Create a new immutable log rather than rewriting the prior M11A log:

`H!veAI/docs/H!veAI/codex-logs/M11A_REV3_CONSOLIDATED_STRICT_CLOSURE_LOG.md`

Record:

- starting local/origin SHA and divergence;
- Task 0 changes;
- P0-P8 implementation by file/symbol;
- why the actual watcher scope is narrower in single-dashboard mode;
- actual trackingMode/materialized-status parser contract;
- legacy fallback behavior;
- R14 health/attention correction;
- E04 actor-provenance correction;
- H!veAI dogfood dashboard update;
- pre-fix failing focused evidence where practical;
- actual Rust test command/count/result;
- frontend test count/result;
- full gates;
- publisher attempts/final result;
- asset hashes;
- security review;
- proof no external tracked project repositories changed;
- final implementation/evidence SHA equality;
- user visual/native acceptance still pending;
- independent re-audit still pending.

---

# Builder exit state

Only after all required production work, actual native/frontend tests, full gates and governed publication succeed:

- M00-M10 remain PASS/CLOSED;
- M11A REV3 = IMPLEMENTATION COMPLETE / PENDING INDEPENDENT RE-AUDIT + USER VISUAL/NATIVE ACCEPTANCE;
- M11 remains NOT PASS/CLOSED;
- strict completed remains 11/20 = 55%;
- M12 remains BLOCKED.

Do not mark M11 PASS/CLOSED yourself.

STOP. Do not start M12.
