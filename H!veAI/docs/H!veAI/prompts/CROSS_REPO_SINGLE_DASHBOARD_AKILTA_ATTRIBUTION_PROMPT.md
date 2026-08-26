# Cross-Repository H!veAI Single-Dashboard + Akilta Attribution Prompt

## Mission

Apply the same H!veAI project-tracking and Akilta attribution standard to the CURRENT repository only.

This prompt is designed to be reused independently in every project repository. Do not assume another repository has the same framework, file casing, task conventions, or UI stack.

The target architecture is:

```text
project repository
  -> .hiveai/PROJECT_DASHBOARD.md   (the only file H!veAI actively watches for project-status changes)
  -> project-internal source files  (TASKS, roadmap, handoff, audits, logs, architecture, decisions, etc.)
     remain internal evidence/provenance and may be listed, but H!veAI must not require live watching of each one
```

The Project Dashboard is the single H!veAI-facing project contract. It is a materialized, truthful status export plus provenance map. Do not create a second H!veAI status/manifest file.

Do not fabricate project status. If a fact cannot be verified from repository truth, write `UNKNOWN`, `NOT_VERIFIED`, or `NONE` as appropriate.

---

# Safety / repository preflight

Before editing:

1. Prove the Git root and current branch.
2. Fetch the current remote branch safely.
3. Do not reset, rebase, force-push, delete user work, rewrite history, or change unrelated files.
4. Inventory existing project truth sources before changing the dashboard, including where present:
   - `.hiveai/PROJECT_DASHBOARD.md`
   - `TASKS.md` / `tasks.md`
   - `ROADMAP.md` / `PLANS.md`
   - `HANDOFF.md` / `handoff.md`
   - `PROGRESS.md`
   - `ARCHITECTURE.md` / design docs
   - `DECISIONS.md`
   - `AGENTS.md`
   - `CLAUDE.md`
   - `SECURITY.md`
   - audit/test/build/release evidence
5. Preserve the repository's existing filename casing and task conventions.
6. Do not convert history/changelog/progress prose into tasks unless the repository already treats it as task authority.

## Bulk Edit safety guard

If this repository is `Sekiph82/Bulk-Edit` and Etsy/commercial-access/policy approval is still pending:

- do not merge runtime UI/branding changes into `main`;
- do not alter Etsy API/OAuth/privacy/policy behavior;
- do not rewrite Etsy compliance documents;
- prepare the dashboard/attribution changes only on a safe branch or report them as `DEFERRED` if branch work would interfere with the review process;
- preserve the current Etsy approval surface exactly.

---

# Task 1 - Establish `.hiveai/PROJECT_DASHBOARD.md` as the single H!veAI watch contract

Use exactly:

`.hiveai/PROJECT_DASHBOARD.md`

If it already exists, update it in place. Do not create `.hiveai/PROJECT_STATUS.md`, another JSON manifest, a second dashboard, or a generated mirror file.

Keep backward-compatible v1 identity for now:

```text
hiveaiDashboardSchema: hiveai-project-dashboard/v1
dashboardMode: source-map
```

Add or update:

```text
trackingMode: single-dashboard-watch
refreshPolicy: project-agent-maintained; H!veAI watches only .hiveai/PROJECT_DASHBOARD.md
```

Unknown front-matter fields may be retained if they are already project truth.

## Required dashboard responsibilities

The dashboard must contain enough materialized project state for H!veAI to understand the project without live-watching every source file.

Keep the existing `## Source authorities` section as provenance/pointers, but explicitly state that those files are NOT independent H!veAI watch targets.

Add the following exact top-level sections if missing:

### `## H!veAI live status`

Use a compact table with these exact labels:

```text
| Field | Value |
| --- | --- |
| Project status | ACTIVE / PAUSED / WAITING / BLOCKED / COMPLETE / UNKNOWN |
| Health | HEALTHY / ATTENTION / BLOCKED / UNKNOWN |
| Current milestone | verified milestone/phase or NONE |
| Current task | verified current task or NONE |
| Current task ID | verified ID or NONE |
| Current workflow state | verified state or UNKNOWN |
| Progress | verified percentage/fraction or UNKNOWN |
| Required actor | HUMAN / CODEX / CLAUDE / GPT_AUDIT / CI / EXTERNAL / NONE / UNKNOWN |
| Next action | verified next action or NONE |
| Waiting on | verified dependency/person/external party or NONE |
| Last meaningful update | ISO-8601 timestamp if verified, otherwise UNKNOWN |
```

Do not invent percentages. A real unknown is better than `0%`.

### `## Current work`

Provide a bounded table of at most 10 current/relevant items:

```text
| ID | Item | Status | Owner/actor | Evidence/source |
```

Use the repository's actual task IDs/status names where available. Do not duplicate hundreds of historical tasks.

### `## Blockers and waiting`

List only current blockers, human decisions, external waits, failed audit/test gates, permissions, or release gates that are actually verified. If none, say `None verified`.

### `## Milestone summary`

Provide bounded current/recent milestone truth. Do not reproduce the entire historical roadmap.

### `## Quality and verification`

Summarize only current factual test/build/audit state. Include exact commands/results only when verified. Do not claim PASS from planned commands.

### `## Recent meaningful activity`

At most 10 items. Keep this to milestone/task/build/audit/release changes that matter to project state. It is a status summary, not a full log.

### `## Provenance`

List the internal files that support the materialized status, with their role. These are provenance only.

Example:

```text
- Task authority: `TASKS.md`
- Roadmap context: `ROADMAP.md`
- Handoff context: `HANDOFF.md`
- Architecture: `ARCHITECTURE.md`
- Historical evidence: `CHANGELOG.md`
```

End this section with this exact policy sentence:

`H!veAI actively watches only .hiveai/PROJECT_DASHBOARD.md for project-status changes; the sources above are internal project evidence/provenance and are not independent live-watch targets.`

---

# Task 2 - Make dashboard freshness part of the project's normal engineering workflow

The project agent/builder must update `.hiveai/PROJECT_DASHBOARD.md` whenever a meaningful project-state change occurs.

A meaningful project-state change includes at least:

- current task changes;
- task status changes;
- milestone/phase changes;
- new blocker or cleared blocker;
- waiting-human or waiting-external state;
- failed/passed audit or verification gate;
- release/deploy state changes;
- important decision that changes next work;
- canonical task authority changes;
- project completion/archival state.

Do NOT update the dashboard for trivial formatting-only changes unless they affect project state.

If `AGENTS.md`, `CLAUDE.md`, or an equivalent agent-governance file exists, add a short non-duplicative rule:

`Before ending a run that materially changes project state, refresh .hiveai/PROJECT_DASHBOARD.md so it remains the single H!veAI-facing status contract.`

Do not create a new agent-governance file solely for this rule if the repository has no such convention.

---

# Task 3 - Preserve internal source inventory, but separate it from H!veAI live tracking

Do not delete existing task/roadmap/handoff/audit/log/architecture files.

They remain useful project truth and may remain discoverable for diagnostics, audits, human navigation, or future deep inspection.

But the project must no longer require H!veAI to actively watch every one of them for routine dashboard refresh.

The repository-side contract is:

```text
internal files change
  -> project agent updates PROJECT_DASHBOARD.md when project state materially changes
  -> H!veAI sees PROJECT_DASHBOARD.md change
  -> H!veAI refreshes the project dashboard
```

Do not add background scripts that rewrite the dashboard on every filesystem change. Avoid commit noise and loops.

---

# Task 4 - Add Akilta attribution using the canonical local logo

Canonical local source asset:

`C:\Users\sekip\Desktop\akilta-wordmark-a1.svg`

Use the exact existing SVG asset. Do not redraw, regenerate, trace, substitute, or hotlink the `file:///` path in production.

Copy the SVG into a project-owned tracked asset location appropriate to the stack, for example:

- web: `public/brand/akilta-wordmark.svg` or the project's equivalent static asset directory;
- desktop: the existing frontend/static asset directory;
- game: the existing UI/branding asset directory;
- documentation-only/backend-only project: a tracked docs/assets location for README attribution.

Preserve the source SVG's visual proportions and transparency.

## Attribution behavior

For projects with a persistent user-facing UI, add a tasteful global Akilta attribution similar in intent to a platform-credit mark, without copying another company's styling.

Required behavior:

- visible Akilta wordmark;
- adjacent English text: `Developed by Akilta`;
- the whole attribution, logo + text, is one clickable target;
- destination: exactly `https://www.akilta.com/`;
- hover/focus tooltip/title: exactly `Developed by Akilta`;
- keyboard accessible;
- external navigation must not destroy the current app session;
- for web apps use safe external-link behavior such as `target="_blank" rel="noopener noreferrer"` where appropriate;
- for native desktop apps use the project's existing safe external-browser mechanism instead of arbitrary shell execution;
- no terminal/console flash;
- do not add tracking parameters;
- do not claim partnership/endorsement beyond the factual `Developed by Akilta` attribution.

## Placement

Choose the least intrusive global placement that fits the existing product:

- preferred: compact persistent footer / bottom-of-shell credit;
- if the project already has a dense footer, integrate into that footer without adding excessive height;
- for a desktop command-center style app, a compact topbar or bottom-shell credit is acceptable if it preserves more workspace;
- for games, use Credits/About/Settings if a persistent in-game footer would harm gameplay;
- for backend/CLI-only projects with no UI, do NOT create a UI solely for attribution. Instead add a linked Akilta wordmark + `Developed by Akilta` to the primary README/docs landing surface and record `No runtime UI attribution surface exists` in the dashboard.

At common desktop widths the attribution must not create a new vertical scrollbar or overlap application controls.

## Visual quality

- keep the wordmark legible but subordinate to the product brand;
- preserve aspect ratio;
- no stretched logo;
- no giant empty footer band;
- use existing typography/color system;
- hover/focus state should be subtle and accessible;
- avoid visual imitation of T-Soft or any other third-party brand treatment.

---

# Task 5 - Tests and acceptance

Use the repository's real framework/tests.

At minimum verify where applicable:

1. `.hiveai/PROJECT_DASHBOARD.md` exists exactly once;
2. it retains `hiveai-project-dashboard/v1` and `dashboardMode: source-map` compatibility;
3. it declares `trackingMode: single-dashboard-watch`;
4. it contains the required materialized status sections;
5. current status values come from verified repository evidence;
6. no duplicate H!veAI status/manifest file was created;
7. the Akilta SVG is project-owned and tracked;
8. the whole Akilta attribution is clickable;
9. exact link is `https://www.akilta.com/`;
10. exact hover/focus text is `Developed by Akilta`;
11. user-facing layout does not gain unwanted overflow;
12. build/typecheck/test gates pass for affected surfaces;
13. no secrets, credentials, generated caches, build outputs, or local absolute paths are committed;
14. historical task/audit/progress files remain intact.

Do not mark manual visual acceptance PASS yourself. Leave visual acceptance pending for the user.

---

# Required final report

Report:

- repository and branch;
- exact dashboard path;
- whether the dashboard was created or updated;
- verified task authority/provenance sources used;
- dashboard materialized status summary;
- governance file changed, if any;
- exact tracked Akilta asset path;
- exact UI/docs attribution component/file changed;
- tests/build results;
- any `UNKNOWN` / `NOT_VERIFIED` fields and why;
- exact commit SHA(s);
- whether the change is merged or only on a branch;
- user visual acceptance still pending.

Stop after this repository only. Do not edit any other repository from the same run.