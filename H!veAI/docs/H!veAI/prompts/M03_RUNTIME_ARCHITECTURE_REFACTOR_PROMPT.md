# M03 — H!veAI Runtime Architecture Refactor

You are continuing H!veAI development after independent M02 audit approval.

Do NOT start M04.

## Mandatory fetch-before-prompt preflight

Before reading milestone prompt files:

```powershell
git fetch origin H!veAI
```

Then compare:

```powershell
git rev-list --left-right --count HEAD...origin/H!veAI
```

If local HEAD is behind `origin/H!veAI` and there are no conflicting local tracked changes:

```powershell
git merge --ff-only origin/H!veAI
```

Then read the authoritative audit and milestone prompt from the updated local checkout.

Never assume missing local prompt/audit files are absent from GitHub before fetching.

Do not use reset, destructive checkout, force-push, or automatic rebase to satisfy this preflight. If fast-forward cannot be performed safely, STOP and report the exact divergence or conflicting tracked changes.

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

## Read after synchronization

Read completely before coding:

- `H!veAI/AGENTS.md`
- `H!veAI/CONSTITUTION.md`
- `H!veAI/ARCHITECTURE.md`
- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/docs/H!veAI/README.md`
- `H!veAI/docs/H!veAI/audits/M02_UI_SHELL_AND_DESIGN_SYSTEM_AUDIT.md`
- `H!veAI/docs/H!veAI/codex-logs/README.md`
- historical M00, M01 and M02 Codex logs
- this prompt

## Repository preflight

Run and log:

- `git rev-parse --show-toplevel`
- `git branch --show-current`
- `git rev-parse HEAD`
- `git remote -v`
- `git status --short`
- `git stash list`

Stop without modifying product files if:

- Git root is not the canonical parent root,
- current branch is not `H!veAI`,
- origin is not the official `Sekiph82/AI-Commerce-HQ` repository,
- synchronization cannot be completed safely.

Preserve unchanged:

- the pre-M00 user stash,
- untracked parent `start-demo.bat`,
- untracked parent `task.md`,
- legacy parent application source except read-only inspection,
- historical M00/M01/M02 Codex logs.

## Durable M03 Codex log

Create a NEW log before implementation:

`H!veAI/docs/H!veAI/codex-logs/M03_RUNTIME_ARCHITECTURE_REFACTOR_CODEX_LOG.md`

The log must include:

- fetch-before-prompt commands and results,
- synchronized starting HEAD,
- all architecture evidence inspected,
- dependency/security advisory investigation,
- runtime-boundary decision and rationale,
- commands and relevant outputs,
- failures and corrections,
- tests,
- files changed,
- commit/push status,
- final GitHub verification.

Never erase prior milestone logs or rewrite past failures.
Never record secrets.

## M03 objective

Define and implement the **active H!veAI runtime boundary** so the new desktop application is no longer conceptually coupled to the legacy AI-Commerce-HQ commerce runtime.

M03 is about runtime architecture, lifecycle and containment.

It is NOT the milestone for:

- SQLite schema implementation,
- Project Registry,
- full Git Engine,
- filesystem watcher,
- task parsing,
- Codex/Claude agent execution,
- PTY terminal execution,
- GPT audit engine,
- GitHub integration,
- arbitrary shell/process execution.

Those belong to later milestones.

## Core principle

The legacy parent application is source material only.

H!veAI must not automatically start or depend on legacy:

- GMO orchestrator,
- Etsy/Fiverr/trading/YouTube/TikTok workers,
- old FastAPI commerce endpoints,
- old marketplace credentials,
- port 8765 commerce backend,
- old parent Tauri lifecycle.

Any retained pattern must be reimplemented under the child H!veAI boundary with explicit ownership and narrow capabilities.

## Step 1 — inventory the legacy runtime read-only

Inspect the old parent architecture without launching commerce operations.

At minimum inventory, where present:

- `backend/main.py`
- backend startup/shutdown hooks
- websocket manager/event buffering
- database initialization
- orchestrator startup logic
- BaseAgent lifecycle
- Etsy/Fiverr/trading/YouTube/TikTok modules
- health endpoints
- old Tauri Rust process spawning/lifecycle
- child-process restart behavior
- ports and environment-variable dependencies

Create an evidence table containing:

- legacy responsibility,
- source file/path,
- startup trigger,
- external side effect risk,
- credentials/network dependency,
- reusable pattern yes/no,
- intended H!veAI destination: Rust native / retained sidecar / later adapter / archive only.

Do not run inherited commerce runtime to complete the inventory.

## Step 2 — decide the final active runtime boundary

Based on evidence, make an explicit architecture decision between:

A. Rust-native H!veAI core with no always-on Python sidecar,
B. Rust-native core plus a new H!veAI-owned narrowly scoped sidecar,
C. another architecture only if strongly justified by actual repository evidence.

Default preference is the smallest secure runtime surface, not maximum reuse.

The decision must answer:

- what starts when H!veAI starts,
- which component owns lifecycle,
- whether any child process exists,
- how health is represented,
- how crash/restart is handled,
- how logs are separated,
- how shutdown is handled,
- how future adapters will plug in,
- which legacy runtime components are permanently excluded.

Do not silently retain the old FastAPI backend just because it exists.

## Step 3 — implement a child-owned runtime domain boundary

Under `H!veAI/src-tauri/`, introduce a clean runtime module/domain appropriate to the chosen architecture.

At minimum model structured runtime state such as:

- component id,
- display name,
- kind,
- state,
- health,
- started_at if applicable,
- last_heartbeat if applicable,
- restart_count,
- last_error sanitized,
- ownership/source.

Use explicit states suitable for future runtime supervision, for example:

- STOPPED
- STARTING
- HEALTHY
- DEGRADED
- STOPPING
- FAILED
- DISABLED

Do not add arbitrary executable paths supplied by the frontend.
Do not add a generic shell command.
Do not expose unrestricted process spawning.

## Step 4 — safe runtime status IPC

Add or extend a narrow Tauri command such as:

`hiveai_runtime_status`

It should return structured H!veAI-owned runtime information only.

Expected M03 behavior must make it obvious that legacy commerce runtime is disabled/not part of the active H!veAI runtime.

If the chosen architecture has no active sidecar yet, status should truthfully report the native runtime and any future/disabled components rather than fabricate a running service.

Preserve `hiveai_native_status` from M01.

## Step 5 — lifecycle and recovery foundation

If M03 introduces any H!veAI child process, it must have:

- explicit executable ownership,
- fixed/allowlisted launch configuration,
- no frontend-supplied arbitrary command,
- bounded startup timeout,
- health check,
- graceful stop,
- kill fallback only after timeout,
- bounded restart/backoff policy,
- sanitized errors,
- restart counter,
- clean desktop shutdown handling,
- unit/integration tests that do not contact real external services.

If M03 chooses no sidecar, implement and test the runtime-supervisor boundary in a way that does not spawn external processes, and document process supervision as dormant infrastructure for later approved adapters.

Do not create a fake process merely to satisfy this step.

## Step 6 — legacy commerce containment proof

Add regression protection proving the active child app does not start the parent commerce backend.

At minimum verify/document:

- port 8765 is not required by H!veAI,
- no legacy commerce backend is spawned by child Tauri startup,
- no inherited marketplace/trading/social orchestrator is imported or launched by the H!veAI child app,
- H!veAI startup succeeds without parent Python dependencies.

Prefer automated source/config regression checks where practical.

## Step 7 — M02 dependency advisory follow-up

The M02 audit carried forward 3 npm advisories: 2 high and 1 critical.

From the H!veAI child workspace run a scoped security review using the package manager's supported audit tooling.

Record for every high/critical advisory:

- package,
- installed version,
- dependency path,
- severity,
- advisory identifier/title if available,
- production vs dev dependency,
- direct vs transitive,
- whether the vulnerable code path is applicable/reachable in packaged H!veAI,
- compatible fixed version if available,
- remediation decision.

Rules:

- do not use `npm audit fix --force`,
- do not perform unrelated major-version churn blindly,
- apply targeted compatible fixes when safe and verify all M02 tests/build afterward,
- if unresolved, document exact risk and target milestone.

A critical advisory may remain only if there is a clear evidence-based non-applicability/mitigation rationale. Otherwise M03 must not be marked fully complete.

## Step 8 — CSP carry-forward

Reassess the M02 localhost CSP carry-forward only as part of runtime architecture.

Do not break Tauri development flow.

If Tauri supports a clean dev-vs-production CSP distinction in the current setup, tighten production behavior safely and test it.

Otherwise retain the current verified behavior and document the precise pre-release hardening requirement.

Do not broaden CSP.

## Step 9 — frontend runtime surface

Integrate only enough UI to expose truthful M03 runtime status.

A small runtime/system status surface may show:

- native app healthy,
- active runtime architecture mode,
- sidecar state if one actually exists,
- legacy commerce runtime disabled,
- sanitized last error if applicable.

Do not redesign the M02 shell.
Do not implement Project Registry or agent/session controls.
Do not add fake start-agent buttons.

## Step 10 — tests

Add meaningful tests for the runtime boundary.

Rust tests should cover as applicable:

- runtime state serialization,
- valid state transitions,
- disabled legacy component representation,
- restart/backoff calculation if implemented,
- sanitized error behavior,
- no arbitrary command surface,
- supervisor behavior using mocks/fakes rather than real commerce processes.

Frontend tests should cover any new runtime status UI and ensure it does not claim legacy services are running.

M03 should improve on the M01 state where `cargo test` had zero meaningful tests.

## Step 11 — verification

Run from the child workspace:

Frontend:

- `npm run typecheck`
- `npm test`
- `npm run build`

Rust/Tauri:

- `cargo fmt --manifest-path H!veAI/src-tauri/Cargo.toml -- --check` if supported by the current toolchain invocation, or equivalent formatting verification
- `cargo check --manifest-path H!veAI/src-tauri/Cargo.toml`
- `cargo test --manifest-path H!veAI/src-tauri/Cargo.toml`
- `cargo build --manifest-path H!veAI/src-tauri/Cargo.toml`

Desktop smoke:

- launch H!veAI in a bounded smoke test,
- verify M02 shell still renders,
- verify `hiveai_native_status` still works,
- verify M03 runtime status works,
- verify port 8765 remains unused,
- verify no legacy commerce child process starts,
- verify clean shutdown.

Do not contact real marketplace/trading/social APIs.

## Step 12 — documentation

Create:

`H!veAI/docs/migration/M03_RUNTIME_ARCHITECTURE_REFACTOR.md`

Document:

- legacy runtime inventory,
- final active runtime decision,
- component ownership table,
- startup/shutdown sequence,
- health/recovery policy,
- legacy components excluded,
- runtime IPC surface,
- security/capability impact,
- npm advisory disposition,
- CSP disposition,
- tests and smoke evidence,
- remaining technical debt.

Update only M03-related items in:

`H!veAI/TASKS.md`

Use `[x]` only for verified items, `[!]` for genuinely blocked verification, and do not mark M04 work complete.

## Step 13 — containment and final diff review

Before commit verify:

- new active product/runtime changes are under `H!veAI/`,
- parent legacy source is unmodified,
- historical logs are unchanged,
- no `.env`, secrets, DBs, runtime logs, node_modules, dist, target, caches or binaries are staged,
- no broad Tauri shell/process/network permissions were added without explicit M03 justification,
- `git diff --check` passes.

## Commit and push

If M03 is genuinely complete, create a focused commit:

`refactor(H!veAI): establish runtime architecture boundary`

The commit MUST include:

`H!veAI/docs/H!veAI/codex-logs/M03_RUNTIME_ARCHITECTURE_REFACTOR_CODEX_LOG.md`

Push normally to:

`origin/H!veAI`

Do not force push.

Because ChatGPT may add audit/governance commits while Codex works, if the first push is rejected:

1. fetch `origin/H!veAI`,
2. inspect the remote commits,
3. preserve all user/local work,
4. integrate only by safe fast-forward/rebase of unpublished Codex work when conflict-free and appropriate,
5. never force push,
6. record the full event in the M03 log.

After push verify M00, M01, M02 and M03 logs exist separately on GitHub under:

`H!veAI/docs/H!veAI/codex-logs/`

If the log needs a final GitHub-verification entry, use a small normal log-only follow-up commit.

## M03 acceptance criteria

M03 is complete only if:

1. Legacy runtime responsibilities are inventoried with evidence.
2. Active H!veAI runtime boundary is explicitly decided and documented.
3. Legacy commerce orchestrators are not part of H!veAI startup.
4. Child Tauri runtime has a structured runtime-state domain.
5. A narrow truthful runtime-status IPC exists.
6. No arbitrary shell/process execution surface is added.
7. Any child process introduced has bounded health/recovery/shutdown behavior and tests.
8. H!veAI starts without parent commerce backend/Python runtime dependency unless a new sidecar is explicitly justified.
9. Port 8765 remains unnecessary/unopened by H!veAI.
10. M01 native status still works.
11. M02 shell and routes regressions pass.
12. High/critical npm advisories are individually investigated and safely remediated or explicitly justified.
13. Frontend typecheck/tests/build pass.
14. Rust check/tests/build pass with meaningful M03 tests.
15. Bounded Windows smoke succeeds.
16. Parent legacy source/package files remain unmodified.
17. Historical M00/M01/M02 logs remain separate and unchanged.
18. M03 log is committed, pushed and verified on GitHub.
19. M03 migration document exists.
20. TASKS reflects verified state only.

## Final response format

Return exactly:

1. M03 RESULT
2. FETCH-BEFORE-PROMPT SYNC RESULT
3. VERIFIED GIT ROOT
4. VERIFIED H!veAI APPLICATION ROOT
5. CURRENT BRANCH / HEAD
6. LEGACY RUNTIME INVENTORY SUMMARY
7. FINAL ACTIVE RUNTIME DECISION
8. COMPONENT OWNERSHIP / STARTUP MODEL
9. RUNTIME STATES / HEALTH MODEL
10. RUNTIME IPC COMMANDS
11. CHILD PROCESS / RECOVERY STATUS
12. LEGACY COMMERCE CONTAINMENT RESULT
13. PORT 8765 RESULT
14. DEPENDENCY ADVISORY REVIEW
15. CSP STATUS
16. FILES ADDED
17. FILES MODIFIED
18. PARENT FILES MODIFIED
19. FRONTEND TEST / BUILD RESULTS
20. RUST / TAURI TEST RESULTS
21. WINDOWS SMOKE RESULT
22. M01 NATIVE IPC REGRESSION STATUS
23. M02 UI REGRESSION STATUS
24. CODEX LOG LOCAL PATH
25. CODEX LOG GITHUB PATH / VERIFICATION
26. PRESERVED HISTORICAL LOG STATUS
27. PRESERVED STASH / USER FILE STATUS
28. COMMIT / PUSH STATUS
29. BLOCKERS / OPEN DECISIONS
30. EXACT NEXT MILESTONE

The exact next milestone is:

`M04 — SQLite and Versioned Migrations`

IMPORTANT GOVERNANCE RULE:

Do NOT create, invent, recommend, or claim the existence of an M04 Codex prompt file.
Do NOT include a `RECOMMENDED NEXT CODEX PROMPT` section.
The next prompt is authored only by ChatGPT after independent M03 audit approval and committed separately under `H!veAI/docs/H!veAI/prompts/`.

Do NOT start M04.
Stop after M03.
