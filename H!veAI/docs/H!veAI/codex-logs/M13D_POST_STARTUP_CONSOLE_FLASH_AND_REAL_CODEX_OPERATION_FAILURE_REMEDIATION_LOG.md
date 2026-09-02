# M13D Post-Startup Console Flash and Real Codex Operation Failure Remediation Log

Date: 2026-09-02
Branch: `H!veAI`
Milestone: M13
Scope: R33 post-startup console flashes and R34 real Codex operation failure

## Final builder state

`M13D REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

R33 and R34 are closed by this remediation. M13 remains open. M14 and M21 were not started.

## Synchronized preflight

- Repository: `C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`
- Branch confirmed: `H!veAI`
- Command run: `git fetch origin H!veAI`
- Synchronization: fast-forward only from `3ebacaf` to `d3ae832`; no reset, rebase, force-push, or user-file discard.
- Unrelated untracked files preserved: `start-demo.bat`, `task.md`.
- Governance preserved: M13 open, strict progress `13 / 20 = 65%`, M14/M21 not started.
- Historical M13/M13A/M13B/M13C prompts, logs, audits, media, and accepted boundaries were not modified.

## R33 reproduction and proof

The baseline governed source launched direct native children with `std::process::Command` but did not set a Windows creation flag on the three Codex-owned background paths. A native Windows trace of the baseline direct invocation observed a console host process with command line `\\??\\C:\\WINDOWS\\system32\\conhost.exe 0x4` during the post-startup operation window. The short-lived parent chain was retained during the trace rather than relying only on a source search.

The responsible paths were identified as:

| Child executable | Parent | Trigger/timing | Baseline defect |
| --- | --- | --- | --- |
| `C:\\Users\\sekip\\AppData\\Local\\OpenAI\\Codex\\bin\\codex.exe` | governed `hiveai-desktop.exe` | Agents readiness/version refresh after the UI was already visible | `--version` probe lacked `CREATE_NO_WINDOW` |
| `C:\\Users\\sekip\\AppData\\Local\\OpenAI\\Codex\\bin\\codex.exe` | governed `hiveai-desktop.exe` | Start Codex operation from Agents | operation launch lacked `CREATE_NO_WINDOW` |
| `taskkill.exe` | governed `hiveai-desktop.exe` | owned-process stop/escalation after operation lifecycle | escalation helper lacked `CREATE_NO_WINDOW` |

Production Git ancestry checks were also audited: the existing Git engine no-window path was moved under the same shared policy, and the registry ancestry helper now uses it as well. Session-list polling only reads persisted rows and does not spawn a child.

## R34 reproduction and exact root cause

The harmless reproduction used disposable fixture `C:\\Users\\sekip\\AppData\\Local\\Temp\\hiveai-m13d-r34-fixture`, initialized as a Git repository, with no user project mutation.

Baseline selected executable:

`C:\\Users\\sekip\\AppData\\Local\\OpenAI\\Codex\\bin\\codex.exe`

Baseline fixed argument vector from the adapter:

`exec --json --sandbox workspace-write --cd <fixture> --ephemeral --skip-git-repo-check`

Prompt transport was bounded stdin. The installed executable reported `codex-cli 0.130.0-alpha.5`. The baseline real invocation exited `1`; redacted stderr identified the exact failure: the inherited Codex configuration selected `gpt-5.6-luna`, and this installed CLI reported that the model requires a newer Codex version. This was not an auth, stdin, cwd, sandbox, JSON, or argument-order failure.

The controlled follow-up against the same disposable fixture added the fixed model selector `--model gpt-5.5`. The CLI accepted the shape and emitted `thread.started` and `turn.started` before the bounded native probe window expired. A second probe with `--ignore-user-config` retained the same accepted shape and removed user-configured MCP startup from the operation contract, while authentication remained sourced from `CODEX_HOME`. This host did not complete the external authenticated model turn within the bounded probe window, so no false real-session `COMPLETED` claim is made here. The adapter’s harmless fake CLI fixture remains green and proves `COMPLETED` reconstruction, redaction, and durable event truth.

## Source changes

- Added `src-tauri/src/process_policy.rs` with one auditable Windows `CREATE_NO_WINDOW` background-child policy.
- Applied that policy to Codex readiness, Codex operation launch, owned `taskkill.exe` escalation, production Git engine commands, and registry Git ancestry checks.
- Updated the fixed Codex invocation with `--model gpt-5.5` and `--ignore-user-config`; direct native executable resolution, allowlisted arguments, registered cwd, bounded stdin, capture, redaction, persistence, and stop/recovery semantics remain intact.
- Added persisted `CODEX_PROCESS_FAILED` and `CODEX_PROCESS_CRASHED` diagnostic code/message evidence for natural terminal outcomes.
- Extended the existing Agents session details with bounded exit-code and diagnostic code/message facts; stderr remains the adapter-redacted bounded capture.
- Updated only current M13/M13D tracking surfaces. M13 remains open at 65%.

## Tests added or tightened

- Windows process-policy test asserts `CREATE_NO_WINDOW`.
- Exact fixed Codex argument-vector assertion covers the compatible model and ignored user-config contract, with no shell/arbitrary flag surface.
- Frontend failed-session fixture test verifies diagnostic code/message, exit evidence, redacted stderr, and absence of protected markers.
- Frontend completed-session fixture test verifies visible successful output evidence.
- Existing M13A owned-stop, M13B split-marker redaction/durable persistence, and M13C resolver/native-PE tests remain unchanged and green.

## Verification gates

- Focused Rust Codex adapter tests: PASS, 24 tests.
- Focused M13 frontend tests: PASS, 5 tests.
- Full Rust library regression: PASS, `312 passed; 0 failed`, serial execution.
- Full frontend regression: PASS, `100 passed; 0 failed`.
- TypeScript typecheck: PASS.
- Frontend production build: PASS.
- `npm audit --audit-level=high`: PASS, 0 vulnerabilities.
- `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`: PASS.
- `cargo check --manifest-path src-tauri/Cargo.toml --all-targets`: PASS.
- `git diff --check`: PASS.
- Publisher failure/rollback path: PASS through `scripts/publish-dev-qa.ps1` candidate/stable swap and bounded cleanup.
- Governed Tauri production `--no-bundle` publication: PASS.
- Candidate/stable PE, shortcut target, shortcut icon, and startup smoke: PASS.
- Published executable SHA-256: `3277A3104CF34C7FCA04C3B6F250DB3082DA501F3D2941940A70F2E6FCA5B2DE` for both candidate and stable.
- Native Codex readiness executable check: PASS; `codex-cli 0.130.0-alpha.5`, exit `0`.
- No-visible-console policy/native smoke: PASS for the governed publisher smoke and policy-backed readiness/start/stop paths; user post-startup visual acceptance remains pending.

## Commit and publication proof

- Implementation commit: `53c8b55e4ea395858b6a2e990b014747cbf59b8c`.
- Implementation push: `git push origin H!veAI` succeeded.
- At implementation publication: `git rev-parse HEAD` and `git rev-parse origin/H!veAI` both returned `53c8b55e4ea395858b6a2e990b014747cbf59b8c`.
- This log is the final immutable M13D evidence artifact and is committed/pushed after the implementation and governed publication gates.

## Acceptance boundary

User native/visual acceptance is still pending. The user must personally verify post-startup zero-console behavior, open Agents, readiness, and one harmless real Codex operation in the published executable. Independent strict re-audit is also pending.
