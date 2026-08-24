# M02 — H!veAI UI Shell and Design System

You are continuing H!veAI development after independent M01 audit approval.

Do NOT start M03.

## Canonical locations

Git root:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

H!veAI application root:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

GitHub repository:
`https://github.com/Sekiph82/AI-Commerce-HQ`

Development branch:
`H!veAI`

Canonical product name:
`H!veAI`

The second character is an exclamation mark.

## Read first

Read completely before coding:

- `H!veAI/AGENTS.md`
- `H!veAI/CONSTITUTION.md`
- `H!veAI/ARCHITECTURE.md`
- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/docs/H!veAI/README.md`
- `H!veAI/docs/H!veAI/audits/M01_TAURI2_FOUNDATION_AUDIT.md`
- `H!veAI/docs/H!veAI/codex-logs/README.md`
- `H!veAI/docs/H!veAI/codex-logs/M00_FRESH_START_CODEX_LOG.md`
- `H!veAI/docs/H!veAI/codex-logs/M01_TAURI2_FOUNDATION_CODEX_LOG.md`
- this prompt

## Mandatory repository preflight

Run and record:

- `git rev-parse --show-toplevel`
- `git branch --show-current`
- `git rev-parse HEAD`
- `git remote -v`
- `git status --short`
- `git stash list`

Stop without modifying files if:

- Git root is not the canonical parent root,
- current branch is not `H!veAI`,
- origin is not `https://github.com/Sekiph82/AI-Commerce-HQ.git` or equivalent HTTPS form.

Preserve unchanged:

- existing `stash@{0}` from pre-M00 user package changes,
- untracked parent `start-demo.bat`,
- untracked parent `task.md`,
- legacy parent application code unless a repository-level ignore/meta change is strictly necessary.

## Durable Codex logs — mandatory and separate

Historical logs MUST remain separate and unchanged:

- `H!veAI/docs/H!veAI/codex-logs/M00_FRESH_START_CODEX_LOG.md`
- `H!veAI/docs/H!veAI/codex-logs/M01_TAURI2_FOUNDATION_CODEX_LOG.md`

Create a NEW M02 log before implementation:

`H!veAI/docs/H!veAI/codex-logs/M02_UI_SHELL_AND_DESIGN_SYSTEM_CODEX_LOG.md`

Record chronologically:

- timestamps,
- commands,
- relevant outputs,
- files inspected/changed,
- decisions and reasons,
- dependency changes,
- failures,
- fixes,
- tests,
- git state,
- commits,
- push status,
- final GitHub verification.

Never erase prior failures after fixing them.
Never record secrets or token values.

The M02 log MUST be committed and pushed to branch `H!veAI`.
Before reporting M02 complete, verify M00, M01, and M02 logs all exist separately on GitHub under:

`H!veAI/docs/H!veAI/codex-logs/`

## M02 objective

Build the production-quality visual shell and design system for the H!veAI desktop application.

M02 is UI architecture only.

Use static/mock data where necessary.

Do NOT implement real:

- project registry scanning,
- git repository engine,
- filesystem watcher,
- task intelligence parsing,
- Codex execution,
- Claude execution,
- GPT audits,
- GitHub integration,
- AI recommendations,
- persistent project/task database,
- arbitrary shell/process execution.

Those belong to later milestones.

## Product UX direction

H!veAI is an AI Development Command Center, not a game dashboard.

Visual character:

- dark-first desktop interface,
- graphite / near-black surfaces,
- restrained blue-violet AI accents,
- green success,
- amber waiting,
- red failed/blocked,
- distinct audit and human-required states,
- high-density but calm enterprise layout,
- crisp typography,
- deliberate motion,
- excellent keyboard/focus behavior.

Avoid:

- 3D office/game aesthetics,
- XP/revenue/achievement metaphors,
- excessive glow,
- decorative neural graphs,
- continuous animation,
- fake AI execution claims.

## Step 1 — inspect M01 frontend

Inspect actual child files before changing architecture:

- `H!veAI/package.json`
- `H!veAI/package-lock.json`
- `H!veAI/src/`
- `H!veAI/index.html`
- `H!veAI/vite.config.*`
- `H!veAI/tsconfig*`
- `H!veAI/postcss.config.*`
- current foundation styles
- current Tauri invocation surface

Keep React + TypeScript + Vite established in M01 unless repository evidence proves a blocker.

Do not migrate framework in M02.

## Step 2 — UI dependencies and design-system foundation

Use the child dependency set only.

Introduce only dependencies justified for the shell, preferably:

- Tailwind CSS
- shadcn/ui primitives or equivalent Radix-based accessible primitives
- Framer Motion
- Lucide icons
- a lightweight router appropriate for the desktop SPA if not already present

Do not introduce a heavyweight state/data framework merely for mock UI.

Document dependency decisions in the M02 log.

## Step 3 — design tokens

Create a reusable token system for at least:

- app background
- surface
- elevated surface
- border/subtle border
- primary/secondary/muted text
- focus ring
- accent
- success
- warning
- danger
- running
- audit
- human-required
- external-wait

Also define:

- typography scale,
- spacing rhythm,
- radius scale,
- shadow/elevation rules,
- status badge rules,
- motion timings,
- reduced-motion behavior.

Avoid hardcoding raw colors throughout feature components.

## Step 4 — application shell

Build the reusable H!veAI desktop shell.

### Left sidebar

Include:

- H!veAI identity
- Global Command Center
- Projects
- Tasks
- Agents
- Audit Center
- Activity
- Settings
- project shortcuts mock section
- bottom system/version area

### Top bar

Include:

- current page title
- search/command placeholder
- command palette trigger
- AI assistant placeholder
- sync/status placeholder
- notifications placeholder

### Main content

Router-controlled page content with consistent spacing and scroll behavior.

## Step 5 — routes

Implement UI routes/shells for:

- `/` Global Command Center
- `/projects` Projects
- `/projects/:id` Project Cockpit
- `/tasks` Tasks
- `/agents` Agent Sessions
- `/audits` Audit Center
- `/activity` Activity
- `/settings` Settings

Use mock/static fixtures only.

Do not imply mock state came from real repositories.

## Step 6 — reusable UI components

Create reusable components rather than one giant dashboard file.

At minimum:

- `PageHeader`
- `SectionHeader`
- `StatusBadge`
- `ActorBadge`
- `MetricCard`
- `ProgressIndicator`
- `ProjectOperationCard`
- `AttentionCard`
- `ActivityRow`
- `PrimaryActionButton`
- `EmptyState`
- `LoadingState`
- `ErrorState`
- tabs/navigation primitives

Suggested feature organization:

`H!veAI/src/features/`

with feature folders for command-center, projects, tasks, agents, audits, activity and settings.

Adapt exact file structure to the existing child app but keep domain boundaries clear.

## Step 7 — canonical UI state vocabulary

Use the canonical workflow states from H!veAI architecture/docs where present.

At minimum demonstrate visually:

- BACKLOG
- READY_FOR_IMPLEMENTATION
- CODEX_RUNNING
- CLAUDE_RUNNING
- AUDIT_REQUIRED
- AUDIT_PASSED
- AUDIT_FAILED
- FIX_REQUIRED
- VERIFY_REQUIRED
- WAITING_OWNER
- WAITING_EXTERNAL
- BLOCKED
- FAILED
- TASK_COMPLETE

Do not rely on color alone. Use labels/icons/text semantics.

## Step 8 — Global Command Center mock UI

Implement a polished static Global Command Center.

Include a KPI row such as:

- Total Projects
- Active
- Need Attention
- Agent Running
- Audit Required
- Waiting External

Include a project operations area with sample cards showing:

- project name
- milestone/phase
- current task
- progress
- health
- current state
- last action
- next action
- required actor
- primary action button

Use mock fixtures separated from components.

Sample project names may include FormuLab, FMCG ERP, Scrubbots, PackLab 3D and Bulk Edit, but clearly mark fixture provenance in code and do not claim live repository connectivity.

## Step 9 — Needs Your Attention

Build a static attention surface with realistic examples such as:

- WAITING EXTERNAL
- AUDIT REQUIRED
- CLAUDE READY
- WAITING OWNER

Buttons must be safe placeholders.

When clicked, show a non-destructive message such as:

`Available in a later milestone.`

Do not fake execution.

## Step 10 — Active Work Queue

Create a polished table/list with:

- Project
- Task
- Stage
- Actor
- State
- Updated

Support responsive desktop resizing and clear status/actor presentation.

## Step 11 — AI Engineering Brief mock panel

Create UI-only Engineering Brief with:

- summary
- projects updated
- tasks completed
- needs-attention count
- recommended next action

No AI service call in M02.

## Step 12 — Activity feed

Create reusable event-feed visuals with:

- timestamp
- project
- actor
- event
- status

Fixture examples may include:

- implementation finished
- audit requested
- tests passed
- task moved to WAITING_EXTERNAL
- branch changed

No live event connection yet.

## Step 13 — Project Cockpit shell

Implement `/projects/:id` with tabs:

- Overview
- Tasks
- Workflow
- Agents
- Audit
- Git
- Tests
- Activity
- Files
- Settings

Overview should be richly implemented with:

- project header
- progress
- health
- current-task hero
- workflow pipeline
- last completed action
- next action
- recent activity
- project metrics

Other tabs may be polished placeholders.

## Step 14 — motion

Use Framer Motion intentionally for:

- page transition
- card entrance
- status change
- progress interpolation
- attention-panel entrance
- sidebar active indicator
- tab transition
- button feedback

Respect `prefers-reduced-motion`.

No continuous decorative motion.

## Step 15 — accessibility and desktop ergonomics

Implement:

- keyboard navigation
- visible focus states
- semantic buttons/links
- ARIA labels where needed
- sufficient contrast
- reduced motion
- sensible tab order
- no state communicated by color alone

The app should feel usable with mouse and keyboard on Windows.

## Step 16 — command palette shell

Create the UI shell for a command palette with mock navigation commands only.

Examples:

- Go to Command Center
- Open Projects
- Open Tasks
- Open Agents
- Open Audit Center
- Open Settings

Do not execute agents, git, shell or filesystem commands.

## Step 17 — preserve M01 native foundation

The M01 native status IPC should continue working.

Do not broaden Tauri capabilities merely for M02 visual work.

Do not remove native logging/notification foundations.

Review the M01 audit note about CSP localhost dev origins. If production build does not need them, safely separate/tighten production behavior. If changing CSP is not straightforward or could break dev flow, document the decision and defer explicitly rather than guessing.

## Step 18 — tests

Add tests appropriate to the child frontend stack.

At minimum cover:

- application shell renders
- sidebar navigation
- primary routes
- Global Command Center renders
- StatusBadge variants
- ProjectOperationCard
- Project Cockpit route
- command palette opens/closes
- safe placeholder action does not execute native/project operations
- reduced-motion behavior where practical

Do not rewrite unrelated legacy parent tests.

## Step 19 — verification

Run from the H!veAI workspace:

Frontend:

- dependency install/update as needed
- typecheck
- tests
- production build

Rust/Tauri regression:

- `cargo check --manifest-path H!veAI/src-tauri/Cargo.toml`
- `cargo test --manifest-path H!veAI/src-tauri/Cargo.toml`
- `cargo build --manifest-path H!veAI/src-tauri/Cargo.toml`

Desktop smoke:

- launch H!veAI in a bounded test
- verify the new shell renders
- verify sidebar navigation works
- verify Command Center and Project Cockpit routes render
- verify native status foundation still works
- verify close is clean
- verify no legacy commerce runtime starts

If automated visual verification is limited, state exactly what was and was not verified.

## Step 20 — M02 documentation

Create:

`H!veAI/docs/migration/M02_UI_SHELL_AND_DESIGN_SYSTEM.md`

Document:

- frontend/UI architecture
- design tokens
- component hierarchy
- routing
- fixture/mock-data boundary
- accessibility choices
- motion policy
- dependencies added
- CSP decision
- known limitations

Update only M02-related items in:

`H!veAI/TASKS.md`

Use `[x]` only when verified.

## Step 21 — commit and GitHub log verification

Before commit:

- verify no parent app source/package changes
- verify no secrets or generated artifacts are staged
- run `git diff --check`
- review staged diff

If M02 is genuinely complete, create a focused commit:

`feat(H!veAI): build command center UI shell`

The commit MUST include:

`H!veAI/docs/H!veAI/codex-logs/M02_UI_SHELL_AND_DESIGN_SYSTEM_CODEX_LOG.md`

Push normally to:

`origin/H!veAI`

Do not force push.

After push verify on GitHub that these three separate files exist:

- `M00_FRESH_START_CODEX_LOG.md`
- `M01_TAURI2_FOUNDATION_CODEX_LOG.md`
- `M02_UI_SHELL_AND_DESIGN_SYSTEM_CODEX_LOG.md`

If final verification requires a small log-only follow-up commit, make it normally and push it.

## M02 acceptance criteria

M02 is complete only if:

1. Production-quality H!veAI desktop shell exists.
2. H!veAI design tokens are centralized.
3. Sidebar/topbar/main layout exists.
4. Required routes render.
5. Global Command Center mock UI exists.
6. Project Cockpit shell exists.
7. Attention, queue, engineering brief and activity surfaces exist.
8. Reusable component architecture exists.
9. Mock data is isolated from components.
10. No real agent/project/git functionality is falsely implemented.
11. Accessibility fundamentals are present.
12. Reduced-motion behavior exists.
13. Frontend typecheck passes.
14. Frontend tests pass.
15. Production frontend build passes.
16. Rust/Tauri regression checks pass.
17. Bounded desktop smoke succeeds.
18. M01 native status IPC still works.
19. Parent application remains untouched.
20. M00/M01 logs remain separate and unchanged.
21. M02 log is committed, pushed and verified on GitHub.
22. M02 migration document exists.
23. TASKS reflects verified state only.

## Final response format

Return exactly:

1. M02 RESULT
2. VERIFIED GIT ROOT
3. VERIFIED H!veAI APPLICATION ROOT
4. CURRENT BRANCH / HEAD
5. UI STACK / DEPENDENCIES
6. DESIGN SYSTEM SUMMARY
7. ROUTES IMPLEMENTED
8. COMMAND CENTER SUMMARY
9. PROJECT COCKPIT SUMMARY
10. REUSABLE COMPONENTS
11. MOCK DATA BOUNDARY
12. ACCESSIBILITY / MOTION
13. CSP STATUS
14. FILES ADDED
15. FILES MODIFIED
16. PARENT FILES MODIFIED
17. FRONTEND TEST / BUILD RESULTS
18. RUST / TAURI REGRESSION RESULTS
19. WINDOWS SMOKE RESULT
20. M01 NATIVE IPC REGRESSION STATUS
21. CODEX LOG LOCAL PATH
22. CODEX LOG GITHUB PATH / VERIFICATION
23. PRESERVED M00/M01 LOG STATUS
24. PRESERVED STASH / USER FILE STATUS
25. COMMIT / PUSH STATUS
26. BLOCKERS / OPEN DECISIONS
27. EXACT NEXT MILESTONE

The exact next milestone is:

`M03 — Runtime Architecture Refactor`

IMPORTANT GOVERNANCE RULE:

Do NOT create, invent, recommend, or claim the existence of an M03 Codex prompt file.
Do NOT include a `RECOMMENDED NEXT CODEX PROMPT` section.
The next prompt is authored only by ChatGPT after independent M02 audit approval and will be committed separately under `H!veAI/docs/H!veAI/prompts/`.

Do NOT start M03.
Stop after M02.
