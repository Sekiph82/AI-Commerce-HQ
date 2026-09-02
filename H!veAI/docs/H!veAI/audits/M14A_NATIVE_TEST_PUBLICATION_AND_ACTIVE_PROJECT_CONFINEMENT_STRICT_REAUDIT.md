# M14A Native Test, Publication, and ACTIVE Project Confinement Strict Re-Audit

Date: 2026-09-02
Product: H!veAI
Branch: `H!veAI`
Implementation commit audited: `295a930f12d27dbc8d72fc7de96ef0a2c6647cc2`
Builder log: `docs/H!veAI/codex-logs/M14A_NATIVE_TEST_PUBLICATION_AND_ACTIVE_PROJECT_CONFINEMENT_REMEDIATION_LOG.md`
Prior audit: `docs/H!veAI/audits/M14_AGENT_SESSION_CENTER_CODEX_CLAUDE_IMPLEMENTATION_STRICT_AUDIT.md`

## Verdict

**PASS / M14A technical remediation accepted**

- BLOCKER: 0
- MAJOR: 0
- MINOR: 0
- Confidence: HIGH

M14 is **not yet closed**. Independent technical re-audit passes, but user native/visual acceptance remains required, including a real harmless Claude operation in the published stable native executable.

## Scope

This re-audit independently reviewed the M14A builder log together with the actual remediation commit and relevant production source. It evaluates only the prior M14 findings:

- M14-R35 native Rust test-loader failure
- M14-R36 governed publication/readiness failure
- M14-R37 exact ACTIVE registered-project confinement regression

It does not treat builder claims alone as closure evidence.

## Finding closure

### M14-R35 — CLOSED

The prior M14 audit rejected compile-only evidence because the Windows Rust test executable failed before harness entry with `0xc0000139 STATUS_ENTRYPOINT_NOT_FOUND`.

M14A records the exact loader failure, identifies the desktop-shell `comctl32.dll!TaskDialogIndirect` import path, isolates desktop-only Tauri/event dependencies from the test target under test configuration, and then executes the repaired harness rather than relying on `--no-run`.

Recorded executed evidence includes:

- focused accepted M13 Codex backend: `24 passed; 0 failed`
- focused M14 Agent Session Center: `9 passed; 0 failed`
- PTY lifecycle fixture: `1 passed; 0 failed`
- full serial Rust library regression with PTY support: `321 passed; 0 failed; 0 ignored`

The remediation does not alter DLL search paths or introduce a runtime loader bypass. Full all-target `cargo check`, formatting, frontend regressions, and publication gates were also rerun.

**Closure:** R35 is closed.

### M14-R36 — CLOSED

The prior M14 candidate built successfully but never produced the governed `HIVEAI_FRONTEND_READY` marker, so stable publication correctly did not occur.

M14A reproduces the failure with fresh WebView data, records that the process/WebView stayed alive on `about:blank`, and traces startup to the pre-migration SQLite backup. The existing ~110 MB database combined with a 5-page / 25 ms backup step exceeded the governed readiness window.

The remediation preserves bounded backup semantics while increasing backup progress to a bounded 1024-page step with a 1 ms delay. The governed publication then completed end-to-end:

- candidate readiness marker: PASS
- candidate smoke: PASS
- stable swap: PASS
- stable readiness marker: PASS
- candidate/stable SHA-256 equality: `4871C222F64E4FC97E9BFDAE95F441F7CED6A49D0DCC5DF346F785E3B1EB6EFF`
- publisher rollback/failure harness: 9/9 PASS
- PE / shortcut / icon checks: PASS
- no forbidden dev listener: PASS
- no visible console host in publisher smoke: PASS

The new M14 Agent Session Center is therefore present in the governed stable `dev-bin/H!veAI.exe`, rather than only in an unpublished candidate.

**Closure:** R36 is closed.

### M14-R37 — CLOSED

The prior M14 implementation weakened accepted M13 project confinement by rejecting only `ARCHIVED` rather than requiring exact `ACTIVE` status.

Actual production source at the remediation commit now checks:

```rust
if project.status != "ACTIVE" {
    return Err(match project.status.as_str() {
        "MISSING" => "AGENT_PROJECT_MISSING",
        "ARCHIVED" => "AGENT_PROJECT_ARCHIVED",
        _ => "AGENT_PROJECT_NOT_ACTIVE",
    }
    .into());
}
```

It then requires the registered normalized root to be an existing directory and canonicalizes it before operation use. The builder evidence also records direct adversarial coverage for ACTIVE, MISSING, ARCHIVED, unknown state, unavailable root, cross-project tasks, and cross-project session operations.

No frontend-controlled arbitrary cwd, executable, raw argument vector, shell, or PID surface was introduced.

**Closure:** R37 is closed.

## Accepted M14 boundaries preserved

The re-audit found no evidence that M14A weakened the accepted M13/M14 security and lifecycle boundaries. In particular:

- Codex remains behind the accepted native adapter path.
- Claude remains a bounded named provider, not a generic executable runner.
- provider selection is allowlisted to CODEX/CLAUDE.
- prompt size remains bounded.
- project/task/session confinement remains native.
- output remains bounded and statefully redacted before persistence.
- native background process policy remains in force.
- frontend receives no executable path/raw args/shell/PID primitive.
- M15-M20 were not activated.
- M21 was not started.

## Claude readiness status

Real Claude native readiness/version is present and was verified as `Claude Code 2.1.248`. The builder intentionally did not execute a real external Claude operation because doing so could consume authenticated provider resources. This is acceptable as a builder boundary, but it means user native acceptance must exercise one harmless Claude operation before M14 can close.

## Native acceptance required before M14 closure

The user must verify the newly published stable H!veAI executable, not an old candidate or browser preview. Minimum acceptance:

1. Open native `H!veAI/dev-bin/H!veAI.exe` and confirm no unwanted console flash.
2. Open Agents and confirm both CODEX and CLAUDE readiness cards/provider selection are present.
3. Select ScrubBots and confirm CLAUDE is the preferred/default provider while CODEX remains manually selectable.
4. Launch a harmless real Claude operation against ScrubBots, for example read-only repository inspection with an explicit no-modification instruction.
5. Confirm the Claude session reaches a truthful terminal state, output is readable in the vertical reader, and no unrelated project access or visible console window occurs.
6. Confirm provider badge, project, timing/state, timeline/output and any Git evidence display coherently.
7. Optionally run one Codex operation to verify no M13 regression after the shared Agent Session Center replacement.

If these native checks pass, M14 may be closed and M15 may then be activated through a separate closure/activation step.

## Roadmap state

- M00-M13: PASS/CLOSED
- M14: IMPLEMENTATION COMPLETE, M14A TECHNICAL STRICT RE-AUDIT PASS, pending user native/visual acceptance
- M15-M20: planned/blocked
- M21: planned/not started
- Strict completed progress remains `14 / 20 = 70%` until M14 user acceptance and closure are recorded.

## Final verdict

**M14A STRICT RE-AUDIT: PASS**

M14-R35, M14-R36, and M14-R37 are closed. Do not activate M15 yet. Await user native/visual acceptance of the published Codex + Claude Agent Session Center, including one real harmless Claude operation.