# M13C Windows Codex Executable Resolution Native Remediation Prompt

Date: 2026-09-02
Product: H!veAI
Branch: `H!veAI`
Milestone: M13 Codex Adapter
Finding: R32

## Authority

This prompt is the sole implementation authority for the M13C remediation of R32.

Before coding, safely synchronize `H!veAI` with `origin/H!veAI` using fetch plus fast-forward-only merge. Do not reset, rebase, force-push, delete user-owned work, or touch unrelated repositories.

Read in full before implementation:

- `H!veAI/AGENTS.md`
- `H!veAI/CONSTITUTION.md`
- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/.hiveai/PROJECT_DASHBOARD.md`
- `H!veAI/docs/H!veAI/prompts/M13_CODEX_ADAPTER_IMPLEMENTATION_PROMPT.md`
- `H!veAI/docs/H!veAI/prompts/M13A_COMMON_ADAPTER_STREAMING_AND_STOP_STRICT_REMEDIATION_PROMPT.md`
- `H!veAI/docs/H!veAI/prompts/M13B_STREAM_SAFE_REDACTION_AND_DURABLE_EVENT_TRUTH_REMEDIATION_PROMPT.md`
- `H!veAI/docs/H!veAI/audits/M13B_STREAM_SAFE_REDACTION_AND_DURABLE_EVENT_TRUTH_STRICT_REAUDIT.md`
- `H!veAI/docs/H!veAI/audits/M13_NATIVE_CODEX_READINESS_WIN32_EXECUTABLE_RESOLUTION_AUDIT.md`
- `H!veAI/src-tauri/src/codex_adapter.rs`
- relevant M13 frontend/native tests and Tauri command/capability wiring.

Do not start M14 or M21.

## Native failure to reproduce

In the governed `H!veAI/dev-bin/H!veAI.exe`, the Agents page currently reports:

- Provider: CODEX
- Readiness: PROBE_FAILED
- Version: Unknown
- Authentication: Unknown
- Diagnostic: `961 is not a valid Win32 application. (os error 193)`

Earlier safe local evidence proved a usable native Codex executable exists at:

`C:\Users\sekip\AppData\Local\OpenAI\Codex\bin\codex.exe`

and `codex.exe --version` returns a valid Codex version.

Do not guess. Reproduce and prove which exact filesystem candidate the current discovery function selects in the governed/native environment before changing code.

## R32 root constraint

Current Windows discovery accepts both `codex.exe` and extensionless `codex` if `is_file()` is true while walking PATH directories. This can select a non-PE script/shim in an earlier PATH directory and pass it directly to `Command::new`, producing Windows OS error 193 even though a valid `codex.exe` exists later.

## Required remediation

### 1. Windows-safe executable discovery

Refactor Codex executable discovery so direct Windows process launch only returns a candidate that is appropriate for `Command::new` as a native executable.

Requirements:

- On Windows, do not directly execute extensionless `codex` shell/npm shims.
- Prefer/accept `codex.exe` candidates.
- Search PATH deterministically across all entries.
- An invalid candidate must not terminate discovery if a later valid native executable exists.
- If useful, validate the candidate with bounded executable metadata or a bounded direct version probe, but do not add a shell wrapper.
- Do not hard-code only one user-specific absolute path as the production solution.
- A known-install-location fallback may be added only if it is deterministic, bounded, non-secret, and subordinate to the general safe resolver.
- Preserve truthful UNAVAILABLE/PROBE_FAILED states when no valid executable exists.

### 2. One resolver for readiness and launch

Readiness and actual session start must use the same safe resolved native executable policy so H!veAI cannot report one binary and launch another.

Do not add:

- `cmd.exe /c`
- PowerShell execution
- shell interpolation
- generic executable-path input
- arbitrary flags
- arbitrary command execution

### 3. Diagnostics without leaking environment

Improve bounded diagnostics enough to distinguish:

- no executable found,
- invalid/non-native candidate skipped,
- native candidate probe failed,
- valid native candidate selected.

Do not expose full PATH contents, credentials, environment secrets, or unbounded local filesystem data in the UI/logs.

### 4. Direct adversarial tests

Add deterministic native tests covering at minimum:

1. Earlier PATH directory contains an extensionless `codex` shim/file; later PATH directory contains valid `codex.exe`; resolver selects the `.exe`.
2. Invalid/non-native candidate is skipped rather than returned.
3. Multiple `.exe` candidates preserve deterministic first-valid ordering.
4. No valid native executable returns truthful unavailable state.
5. Readiness and start share the same resolver policy.
6. Existing prompt injection, process containment, streaming/redaction, durable event truth, stop/escalation, and restart reconciliation tests still pass.

Avoid tests that only assert string suffixes. Exercise the actual resolver behavior with disposable directories/fixtures.

### 5. Real native verification

After deterministic tests, verify against the user's real installed environment:

- prove which exact candidate the resolver selects, in a bounded builder log only;
- run the harmless direct `--version` probe;
- require successful version parsing;
- publish the governed `H!veAI/dev-bin/H!veAI.exe`;
- do not claim user native acceptance yourself.

The expected user-facing post-fix state is a truthful Codex readiness/version card without OS error 193.

## Regression and publication gates

Run all applicable gates from M13/M13A/M13B, including at minimum:

- focused resolver/readiness native tests;
- all `codex_adapter::tests`;
- full Rust library tests;
- focused M13 frontend tests;
- full frontend tests;
- TypeScript typecheck;
- production frontend build;
- dependency audit at high severity;
- `cargo fmt --check`;
- `cargo check`;
- `git diff --check`;
- publisher failure/rollback harness;
- governed production publication to `H!veAI/dev-bin/H!veAI.exe`;
- candidate/stable smoke, shortcut target, icon, no forbidden terminal/console regressions.

If any gate fails, do not report PASS.

## Governance/status rules

- Preserve all accepted M13/M13A/M13B findings and historical evidence.
- M13 remains OPEN pending independent strict re-audit and user native/visual acceptance.
- Keep strict roadmap progress at `13 / 20 = 65%`.
- Do not activate or implement M14.
- Do not start M21.

Update canonical trackers only to record truthful M13C remediation-pending-audit state. Do not mark M13 closed.

## Immutable builder log

Create:

`H!veAI/docs/H!veAI/codex-logs/M13C_WINDOWS_CODEX_EXECUTABLE_RESOLUTION_NATIVE_REMEDIATION_LOG.md`

The log must include:

- exact root cause and pre-fix selected candidate evidence;
- exact resolver policy after remediation;
- adversarial test evidence;
- real safe `codex.exe --version` evidence;
- all regression/publication results;
- exact files changed;
- implementation commit SHA;
- pushed local/origin equality proof;
- statement that M14/M21 were not started;
- statement that user native acceptance remains pending.

Final builder state:

**M13C REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE**

Commit and push all scoped changes to `origin/H!veAI` without force.
