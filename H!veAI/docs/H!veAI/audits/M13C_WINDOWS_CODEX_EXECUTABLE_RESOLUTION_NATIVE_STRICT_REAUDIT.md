# M13C Windows Codex Executable Resolution Native Strict Re-Audit

Date: 2026-09-02
Product: H!veAI
Branch: `H!veAI`
Scope: M13C / R32 only

## Verdict

**PASS / R32 CLOSED / M13 REMAINS OPEN PENDING USER NATIVE/VISUAL ACCEPTANCE**

Severity count:
- BLOCKER: 0
- MAJOR: 0
- MINOR: 0

Confidence: HIGH.

## Evidence reviewed

- `docs/H!veAI/codex-logs/M13C_WINDOWS_CODEX_EXECUTABLE_RESOLUTION_NATIVE_REMEDIATION_LOG.md`
- implementation commit `750d777fbf39a7831dbcc66ad201ea89e38a564f`
- actual `src-tauri/src/codex_adapter.rs` resolver, readiness, start, probe, and focused regression implementation
- canonical tracker changes in the implementation commit

Builder logs were treated as claims and were checked against production source.

## R32 re-audit

### Original defect

The prior Windows resolver accepted either `codex.exe` or extensionless `codex` from PATH. On the user's machine, the extensionless npm shim could be selected before the actual native Codex executable, leading direct `CreateProcess`/`Command::new()` launch to fail with Windows error 193 (`not a valid Win32 application`).

### Current implementation

R32 is closed.

The resolver now:

1. uses a single `resolve_codex_executable()` path for both readiness and real session start;
2. scans PATH deterministically and then uses the bounded `%LOCALAPPDATA%\OpenAI\Codex\bin` fallback;
3. on Windows accepts only the direct candidate name `codex.exe` and explicitly skips extensionless `codex` shims;
4. validates candidate PE structure before selection, including `MZ`, bounded PE-header offset, `PE\0\0`, supported machine values, and PE optional-header magic;
5. skips invalid candidates rather than aborting resolution, allowing a later valid native executable to win;
6. keeps direct `Command::new(executable)` execution and the existing fixed argument policy, without adding `cmd.exe`, PowerShell, shell interpolation, generic executable overrides, or arbitrary flags;
7. returns bounded diagnostics that distinguish no executable from invalid/non-native candidate cases.

Readiness and `start()` both consume the same resolution object, eliminating the earlier policy split risk.

## Adversarial coverage assessment

The M13C implementation adds focused resolver coverage for the important Windows ordering cases:

- earlier extensionless `codex` shim followed by valid `codex.exe`;
- invalid `.exe` followed by valid `.exe`;
- multiple valid candidates with deterministic first-valid PATH order;
- only invalid/non-native candidates;
- shared readiness/start resolver policy.

The builder also reports preservation of the pre-existing M13/M13A/M13B process-containment, prompt-injection, streaming/redaction, durable-event, stop/escalation, and restart-reconciliation regressions.

## Real-environment evidence

The builder reproduced the pre-fix native failure against the exact extensionless npm shim and then reported post-fix selection of:

`C:\Users\sekip\AppData\Local\OpenAI\Codex\bin\codex.exe`

with a harmless direct `--version` probe returning:

`codex-cli 0.130.0-alpha.5`

The governed published executable remained `H!veAI/dev-bin/H!veAI.exe`, and publication/smoke/shortcut/icon/no-console gates were reported PASS.

## Residual note

The log reports that two normal parallel full-Rust invocations exposed a pre-existing stop-fixture timing race while the required serial full regression passed. No evidence reviewed here shows that race to be an R32 production regression, and the M13C implementation did not broaden stop/process semantics. It therefore does not reopen R29 or block R32 closure.

## Governance decision

- R32: **PASS/CLOSED**.
- M13/M13A/M13B accepted technical boundaries remain preserved.
- M13 technical strict audit is now PASS through M13C.
- M13 is **not yet milestone-closed** because native/user acceptance must confirm that the newly published H!veAI executable now reports the real Codex readiness/version and can start a bounded Codex session without the Windows 193 failure.
- Strict roadmap count remains `13 / 20 = 65%` until M13 is formally closed.
- Do not start M14 or M21 yet.

## Required native acceptance

Using the newly published `H!veAI/dev-bin/H!veAI.exe`, verify:

1. Agents -> Codex adapter no longer shows `PROBE_FAILED` / Windows error 193.
2. Readiness shows the real Codex version (`codex-cli 0.130.0-alpha.5` or the currently installed version) and an honest authentication/readiness state.
3. Start a safe bounded Codex operation against a registered ACTIVE project.
4. Confirm a persisted session appears and reaches a truthful terminal state with bounded output/diagnostic evidence.
5. If practical, exercise Stop on a running owned session and confirm the UI remains stable.

If those native checks pass, M13 may proceed to canonical closure and M14 activation in a separate status-transition run.
