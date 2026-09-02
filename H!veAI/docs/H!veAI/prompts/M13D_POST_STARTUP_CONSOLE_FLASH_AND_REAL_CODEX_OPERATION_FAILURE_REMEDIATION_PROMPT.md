# M13D Post-Startup Console Flash and Real Codex Operation Failure Remediation Prompt

Date: 2026-09-02
Product: H!veAI
Branch: `H!veAI`
Milestone: M13
Scope: Native Windows post-startup console flash + real Codex session failure

## Authority and synchronization

Work only on branch `H!veAI` in repository `Sekiph82/AI-Commerce-HQ`.

Before changing anything:

1. `git fetch origin H!veAI`
2. Confirm current branch is `H!veAI`.
3. Fast-forward only with `git merge --ff-only origin/H!veAI` if needed.
4. Do not reset, rebase, force-push, discard, or overwrite user-owned work.
5. Preserve unrelated untracked files.

Read these before implementation:

- `H!veAI/AGENTS.md`
- `H!veAI/CONSTITUTION.md`
- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/.hiveai/PROJECT_DASHBOARD.md`
- `H!veAI/docs/H!veAI/audits/M13B_STREAM_SAFE_REDACTION_AND_DURABLE_EVENT_TRUTH_STRICT_REAUDIT.md`
- `H!veAI/docs/H!veAI/audits/M13C_WINDOWS_CODEX_EXECUTABLE_RESOLUTION_NATIVE_STRICT_REAUDIT.md`
- `H!veAI/docs/H!veAI/codex-logs/M13C_WINDOWS_CODEX_EXECUTABLE_RESOLUTION_NATIVE_REMEDIATION_LOG.md`
- current M13/M13A/M13B/M13C source and tests

Do not start M14 or M21.

## User-observed native failures

The latest governed native build now resolves a real native Codex executable correctly and shows:

- provider `CODEX`
- readiness `VERSION_VERIFIED_AUTH_UNKNOWN`
- version `codex-cli 0.130.0-alpha.5`

However, two native failures remain.

### R33 — post-startup console-window flashes

The user reports that H!veAI opens normally and the main UI is already visible. After the app is open, one or more console/terminal windows flash open and close automatically.

Important timing detail:

- this is **not** before H!veAI launches,
- this is **not** only after a Codex operation finishes,
- it happens after the H!veAI UI is already open.

Do not assume the source. Prove which subprocess creates each visible console window.

Potential sources to inspect include, but are not limited to:

- Codex readiness/version probes,
- background/native refresh work,
- helper executables,
- Git or watcher probes,
- task/process recovery,
- Codex lifecycle helpers,
- any command launched after frontend/native initialization.

The app must not create visible terminal/console windows during ordinary startup, post-startup background activity, readiness checks, session polling, or helper-process execution.

### R34 — real Codex operation immediately fails

After R32 was fixed, the user selected registered project `ScrubBots`, entered a harmless read-only prompt, and pressed `Start Codex operation`.

Observed behavior:

- a terminal window appeared,
- it closed almost immediately,
- the persisted H!veAI session became `FAILED`,
- no useful failure explanation was visible in the session card.

This is a real native failure, not a synthetic fixture failure.

## Required work

### A. Reproduce before fixing

Reproduce both R33 and R34 on Windows using the governed native H!veAI build.

For R33, instrument or trace process creation sufficiently to identify the executable, parent process, trigger, and timing of every H!veAI-owned visible console flash after the UI is already open.

Do not settle for inference from source alone.

For R34, run a bounded harmless operation against a disposable test repository or an explicitly safe local fixture first. Capture:

- selected executable,
- exact fixed argument vector,
- cwd,
- environment assumptions that matter to Codex,
- exit code,
- redacted stdout,
- redacted stderr,
- persisted agent events,
- authentication/sandbox diagnostics,
- final session state.

Do not run destructive commands or mutate a user project for diagnosis.

### B. Zero-visible-console policy on Windows

Introduce one auditable Windows subprocess creation policy for every H!veAI-owned native child/helper process that should be background-only.

At minimum verify and fix, where applicable:

- Codex `--version` readiness probe,
- Codex operation process launch,
- `taskkill.exe` or any stop/escalation helper,
- any Git/helper command invoked by H!veAI during post-startup background work,
- any other process proven by R33 tracing.

Requirements:

- no `cmd.exe /c` workaround,
- no PowerShell wrapper workaround,
- no shell interpolation,
- no arbitrary user-supplied executable or flags,
- no detached unmanaged process,
- preserve stdout/stderr capture where required,
- preserve owned-process containment and stop semantics,
- preserve M13A/M13B redaction and durable-event guarantees.

On Windows, background children must use the appropriate native process creation flags so they do not create visible console windows.

The fix must not suppress or hide diagnostic output from persistence. UI invisibility and evidence persistence are separate requirements.

### C. Fix the real Codex operation failure

Prove the exact root cause of the `FAILED` real Codex session.

Do not guess that it is authentication, stdin, argument ordering, sandbox mode, cwd, CLI version drift, JSON mode, or process flags. Capture evidence and fix the actual defect.

Preserve these boundaries:

- direct native `codex.exe` execution only,
- fixed allowlisted argument construction,
- prompt transported as bounded stdin data,
- registered-project cwd only,
- no shell execution,
- no command injection surface,
- bounded stdout/stderr streaming,
- pre-persistence redaction,
- durable captured-vs-persisted truth,
- owned-process stop/escalation semantics,
- restart reconciliation.

If the installed Codex CLI version requires a different fixed invocation contract, update the adapter only after proving the contract with a harmless real probe and add regression tests for the exact accepted CLI shape.

### D. Make failure evidence visible in Agents UI

A session card that only says `FAILED` is insufficient.

For failed sessions, show bounded, user-safe diagnostic evidence already persisted by the adapter, such as:

- diagnostic code,
- diagnostic message,
- exit code,
- bounded stderr excerpt if safely redacted,
- whether output evidence was degraded,
- termination type where useful.

Do not expose secrets, full environment variables, raw PATH, tokens, authorization headers, or unbounded output.

The UI must distinguish at least:

- STARTING
- RUNNING
- COMPLETED
- FAILED
- STOPPED
- CRASHED

Do not redesign the whole Agents page. Keep the accepted H!veAI visual language.

## Required adversarial tests

Add direct tests that fail before the remediation and pass after it.

At minimum cover:

1. Windows readiness probe uses no-visible-console process policy.
2. Windows Codex operation launch uses no-visible-console process policy.
3. Windows stop/escalation helper uses no-visible-console process policy.
4. A post-startup readiness refresh cannot flash a console window.
5. Repeated Agents-page refresh/readiness execution does not spawn visible consoles.
6. Session-list polling does not launch a Codex process or helper process.
7. Real/fake fixture CLI failure persists truthful exit/diagnostic evidence.
8. Failed-session UI renders diagnostic code/message and exit evidence without leaking protected markers.
9. Successful session remains COMPLETED and reconstructs redacted durable output correctly.
10. M13B split-marker redaction tests remain green.
11. M13B durable persistence truth tests remain green.
12. M13A owned-process stop/escalation tests remain green.
13. M13C resolver ordering/native-PE tests remain green.
14. No shell, arbitrary command, or arbitrary flag surface is introduced.
15. Existing startup video `H!veAI/src/assets/H!veAI.mp4`, native icon, audio behavior, Command Center, Projects, Tasks, and Project Cockpit remain unchanged.

Where Windows visible-console behavior cannot be asserted purely by unit tests, add an auditable process-creation policy abstraction plus a native governed smoke check that proves no console host/window appears during the relevant paths.

## Native acceptance gate

Do not claim user acceptance.

Before handing back, publish a fresh governed `H!veAI/dev-bin/H!veAI.exe` and verify locally:

1. Launch H!veAI.
2. Wait after the main UI is fully visible.
3. Trigger normal post-startup background activity and Agents readiness refresh.
4. Confirm zero terminal/console flashes.
5. Open Agents.
6. Confirm Codex readiness/version still works.
7. Run one harmless bounded Codex operation against a safe fixture.
8. Confirm no terminal/console window appears.
9. Confirm the session transitions truthfully and, on success, reaches COMPLETED with visible persisted output/evidence.
10. If deliberately forcing a failure fixture, confirm FAILED shows a useful bounded diagnostic in the UI.

User native/visual acceptance remains pending until the user personally tests the published executable.

## Verification gates

Run all relevant gates, including:

- focused native M13D tests,
- full `codex_adapter` tests,
- full Rust library regression serially if needed for the existing stop-fixture timing race,
- focused M13 frontend tests,
- new failed-session UI tests,
- full frontend regression,
- TypeScript typecheck,
- frontend production build,
- `npm audit --audit-level=high`,
- `cargo fmt --all -- --check`,
- `cargo check`,
- `git diff --check`,
- publisher failure/rollback harness,
- governed production Tauri `--no-bundle` publication,
- candidate/stable smoke,
- shortcut target/icon validation,
- explicit no-visible-console native smoke for post-startup readiness and Codex operation paths.

Do not weaken or bypass tests to obtain green results.

## Governance

M13 remains open during this remediation.

Do not increase strict roadmap progress above `13 / 20 = 65%`.

Do not mark M13 PASS/CLOSED.

Do not start M14 or M21.

Update only current M13/M13D status/provenance surfaces needed to truthfully record the remediation. Preserve historical prompts, logs, and audits unchanged.

## Builder log

Create this immutable log:

`H!veAI/docs/H!veAI/codex-logs/M13D_POST_STARTUP_CONSOLE_FLASH_AND_REAL_CODEX_OPERATION_FAILURE_REMEDIATION_LOG.md`

The log must include:

- synchronized preflight proof,
- exact R33 reproduction evidence,
- executable/parent/trigger responsible for each visible console flash,
- exact R34 reproduction evidence,
- real root cause of the failed Codex operation,
- exact source changes,
- exact tests added,
- native no-console policy proof,
- real harmless Codex operation proof,
- failed-session UI evidence,
- all verification results,
- governed publication result,
- implementation commit SHA,
- push/equality proof,
- explicit statement that user native acceptance is still pending.

Final builder state must be exactly:

`M13D REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

Stop after M13D. Do not start M14 or M21.