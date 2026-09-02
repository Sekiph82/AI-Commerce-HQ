# M13D Post-Startup Console Flash and Real Codex Operation Failure Strict Re-Audit

Date: 2026-09-02
Branch: `H!veAI`
Milestone: M13
Scope: R33 and R34 only

## Verdict

**PASS / R33 CLOSED / R34 CLOSED**

Technical remediation is accepted. M13 remains open until user native/visual acceptance confirms zero post-startup console flashes and one harmless real Codex operation behaves truthfully in the published executable.

No M14 or M21 work is accepted or started by this audit.

## Evidence reviewed

- Authoritative M13D remediation log:
  `docs/H!veAI/codex-logs/M13D_POST_STARTUP_CONSOLE_FLASH_AND_REAL_CODEX_OPERATION_FAILURE_REMEDIATION_LOG.md`
- Implementation commit:
  `53c8b55e4ea395858b6a2e990b014747cbf59b8c`
- `src-tauri/src/process_policy.rs`
- `src-tauri/src/codex_adapter.rs`
- Current M13 tracking changes in canonical status surfaces

## R33 strict result: CLOSED

The prior defect was real: direct native background children were launched without an explicit Windows no-console creation policy. M13D introduces one shared `background_command()` policy using Windows `CREATE_NO_WINDOW = 0x08000000` and routes the relevant owned background children through it.

Accepted covered paths include:

1. Codex readiness/version probe.
2. Codex operation launch.
3. Owned `taskkill.exe` escalation.
4. Production Git helper process paths brought under the same background-child policy.

The implementation does not add shell wrappers, `cmd.exe /c`, PowerShell launch indirection, arbitrary executable override, or arbitrary user-supplied process flags.

The builder also recorded an actual native baseline `conhost.exe` trace before the fix. This is materially stronger than source-only inference and is consistent with the user's observed post-startup terminal flashes.

User visual confirmation remains required because source/process-policy correctness cannot by itself prove that no unrelated Windows or third-party child creates a visible console in the user's final runtime environment.

## R34 strict result: CLOSED

The failed real operation was reproduced against a disposable local fixture with the actual selected native Codex executable. Evidence isolates the failure to inherited model configuration selecting `gpt-5.6-luna`, which the installed `codex-cli 0.130.0-alpha.5` rejected as requiring a newer Codex version.

The adapter now uses a fixed governed invocation including:

- `exec`
- `--json`
- `--model gpt-5.5`
- `--sandbox workspace-write`
- registered project `--cd`
- `--ephemeral`
- `--ignore-user-config`
- `--skip-git-repo-check`
- bounded prompt over stdin

This preserves the accepted no-shell, fixed-argument, registered-cwd, bounded-prompt, output-redaction, durable-event, owned-process, stop/recovery, and executable-resolution boundaries from M13/M13A/M13B/M13C.

The controlled follow-up reached `thread.started` and `turn.started`, which proves the previously failing invocation shape is accepted by the installed CLI. The builder correctly did not fabricate a completed authenticated external model turn when the bounded native verification window expired.

## Failure truth and UI evidence

M13D adds explicit terminal diagnostics for natural process failure/crash states and surfaces bounded exit code, diagnostic code/message, and adapter-redacted stderr in Agents session details.

This is accepted because a future failure can now be diagnosed from durable session evidence instead of presenting only a bare `FAILED` label.

## Execution gates

The following execution gates are required evidence for this remediation and are recorded PASS in the immutable builder log:

1. Focused Rust Codex adapter suite: 24 PASS.
2. Focused M13 frontend suite: 5 PASS.
3. Full Rust library regression, serial: 312 PASS, 0 FAIL.
4. Full frontend regression: 100 PASS, 0 FAIL.
5. TypeScript typecheck: PASS.
6. Frontend production build: PASS.
7. `npm audit --audit-level=high`: PASS, 0 vulnerabilities.
8. Rust formatting check: PASS.
9. Rust `cargo check --all-targets`: PASS.
10. `git diff --check`: PASS.
11. Publisher failure/rollback path: PASS.
12. Governed production Tauri `--no-bundle` publication: PASS.
13. Candidate/stable PE and startup smoke: PASS.
14. Desktop shortcut target/icon validation: PASS.
15. Native Codex readiness executable check: PASS, installed CLI reports `codex-cli 0.130.0-alpha.5`.
16. No-visible-console policy/native smoke for governed readiness/start/stop paths: PASS.
17. Real disposable-fixture Codex invocation: accepted fixed shape reached `thread.started` and `turn.started`; no false `COMPLETED` claim.
18. Historical M13A stop, M13B stream redaction/durable persistence, and M13C native executable resolver tests remain green.

These gates are execution requirements, not documentation-only checks. Any future M13 closure run must preserve this evidence and must not reinterpret a partial or timed-out real external model turn as a successful completed session.

## Residual acceptance boundary

No technical strict finding remains open in R33/R34.

Before M13 can be canonically closed, the user must personally verify in the published native executable:

1. H!veAI opens normally.
2. After the UI is fully visible, no terminal/console windows flash during idle post-startup behavior.
3. Opening Agents and running readiness does not flash a terminal.
4. Starting one harmless real Codex operation does not flash a terminal.
5. The operation reaches a truthful running/completed or explicit failed state with visible bounded diagnostics/output evidence.

Until those native checks are accepted, strict roadmap progress remains `13 / 20 = 65%` and M14 remains blocked.

## Final state

- R33: CLOSED
- R34: CLOSED
- M13 technical strict re-audit: PASS
- M13 user native/visual acceptance: PENDING
- M13 canonical closure: NOT YET
- M14: BLOCKED / NOT STARTED
- M21: PLANNED / NOT STARTED
