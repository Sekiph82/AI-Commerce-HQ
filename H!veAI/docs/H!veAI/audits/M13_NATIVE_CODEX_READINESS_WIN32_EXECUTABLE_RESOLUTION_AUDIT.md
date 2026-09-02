# M13 Native Codex Readiness Win32 Executable Resolution Audit

Date: 2026-09-02
Product: H!veAI
Branch: `H!veAI`
Milestone: M13 Codex Adapter

## Verdict

**FAIL / NATIVE ACCEPTANCE BLOCKED**

- BLOCKER: 0
- MAJOR: 1
- MINOR: 0

M13B's technical strict re-audit remains accepted for R30/R31, but native acceptance exposed a new runtime defect in executable discovery/readiness.

## Native evidence

User-native screenshot of the governed `H!veAI/dev-bin/H!veAI.exe` shows the Agents page rendering:

- Provider: `CODEX`
- Readiness: `PROBE_FAILED`
- Version: `Unknown`
- Authentication: `Unknown`
- Diagnostic: `961 is not a valid Win32 application. (os error 193)`

The UI itself loads and the project selector/start-operation surface renders, but Codex readiness is not operational, so M13 cannot be accepted closed.

## R32 — Windows executable discovery can select a non-Win32 `codex` shim

Severity: **MAJOR**

### Source evidence

`discover_codex_executable()` walks each PATH directory and accepts either `codex.exe` or extensionless `codex` when `candidate.is_file()` is true. On Windows this means an extensionless script/shim can be selected before the actual native `codex.exe` in a later PATH entry.

The selected path is then passed directly to `Command::new(path)` for the `--version` readiness probe. If the selected `codex` file is not a PE/Win32 executable, Windows returns OS error 193 (`not a valid Win32 application`).

This is consistent with the user's native screenshot and with earlier builder evidence that a valid native executable exists at:

`C:\Users\sekip\AppData\Local\OpenAI\Codex\bin\codex.exe`

and responds successfully to `codex.exe --version`.

### Why this is a production defect

M13's readiness contract must truthfully detect the usable installed Codex executable. A PATH entry containing a non-native `codex` shim must not cause H!veAI to report the real provider as unusable when a valid `codex.exe` exists elsewhere.

The defect also risks the same wrong executable being selected for actual `start()` operations because launch uses the same discovery function.

### Required remediation

1. Make Windows executable discovery extension-aware and executable-type safe.
2. On Windows, prefer/accept only valid executable candidates appropriate for direct `Command::new` launch, at minimum `codex.exe`; do not execute extensionless shell/npm shims directly.
3. Search all PATH entries deterministically and do not stop on an invalid candidate.
4. Preserve the existing safe direct-process boundary. Do not introduce `cmd.exe`, PowerShell, shell wrappers, arbitrary executable overrides, or generic command execution.
5. Add adversarial discovery tests with:
   - an earlier PATH directory containing an extensionless `codex` shim/file,
   - a later PATH directory containing valid `codex.exe`,
   - proof the native `.exe` is selected;
   - invalid/non-executable candidate skipped;
   - no-valid-executable state remains truthful.
6. Re-run the real installed Codex readiness probe from the governed native app path and require a successful version result before claiming native acceptance.

## Accepted prior findings

- R27 provider-neutral adapter contract: CLOSED.
- R28 incremental bounded structured output: CLOSED subject to R30/R31 follow-up.
- R29 owned-tree stop lifecycle: CLOSED.
- R30 stream-safe stateful redaction: CLOSED.
- R31 durable event truth: CLOSED.

R32 is independent of those accepted remediations and does not reopen them.

## Milestone state

- M13 technical source audit for previous findings: PASS.
- M13 native/visual acceptance: **FAIL due R32**.
- Strict roadmap progress remains `13 / 20 = 65%`.
- M14 remains blocked/not started.
- M21 remains planned/not started.
