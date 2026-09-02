# M14B Claude Real Operation and Session Center Readability Strict Re-Audit

Date: 2026-09-02
Branch: `H!veAI`
Scope: M14-R38, M14-R39, M14-R40
Implementation commit: `1880a209c1c003fc5db52864d8f9edef8bc4a2f4`
Builder log: `docs/H!veAI/codex-logs/M14B_CLAUDE_REAL_OPERATION_AND_SESSION_CENTER_READABILITY_REMEDIATION_LOG.md`

## Verdict

**TECHNICAL PASS / USER NATIVE-VISUAL ACCEPTANCE STILL REQUIRED**

Blocker: 0
Major: 0
Minor: 0

M14 remains open. M15 must not activate yet. M21 remains not started.

## Evidence reviewed

- Immutable M14B remediation log on `H!veAI`.
- Actual implementation commit `1880a209c1c003fc5db52864d8f9edef8bc4a2f4`.
- Current provider-neutral frontend contract in `src/agentSessionCenter.ts`.
- Current Agents UI implementation in `src/pages.tsx`.
- Prior accepted M13/M14A process, confinement, redaction, durability, no-visible-console, and publication boundaries.

## R38 - Correct real Claude invocation

**CLOSED.**

The pre-fix native failure was reproduced against installed Claude Code `2.1.248`: `--print --output-format stream-json` requires `--verbose`.

The governed fixed invocation now adds `--verbose` while preserving direct native executable resolution, fixed/allowlisted arguments, validated ACTIVE registered-project cwd, bounded stdin prompt transport, no shell, no arbitrary executable/args/PID surface, no-session-persistence, plan permission mode, restricted mode, no-visible-console policy, and pre-persistence redaction.

A harmless real Claude operation against ScrubBots reached the actual provider, exited `0`, produced native stream evidence, and showed no Git status delta.

## R39 - Persisted sessions no longer auto-expand

**CLOSED.**

The initial Agents view no longer auto-selects the first persisted session. Persisted sessions are presented as compact rows and require explicit user selection to open details. Only one selected detail region is rendered, with an explicit close/collapse path.

Terminal-state action truth is also improved: completed/failed sessions no longer expose a meaningless Stop action, and unsupported Resume is not presented as actionable.

## R40 - Readable session detail instead of raw event wall

**CLOSED.**

The default selected-session surface is now a human-readable vertical reader rather than raw JSON/event output. Assistant/result/plain output is elevated to the primary reading surface. Timeline, raw events, live terminal, Git evidence, and other technical evidence are retained behind disclosure sections rather than deleted.

Failed diagnostics remain visible in a concise primary diagnostic card. Long identifiers and paths are constrained/wrapped so the page does not require horizontal reading.

## Regression and publication evidence

Builder evidence reports:

- Focused frontend: 10 passed, 0 failed.
- Full frontend: 105 passed, 0 failed.
- Full serial Rust regression with PTY support: 321 passed, 0 failed.
- Typecheck/build/npm audit/cargo fmt/cargo check/git diff check: PASS.
- Publisher rollback harness: 9/9 PASS.
- Governed `--no-bundle` publication: PASS.
- Candidate and stable executable SHA-256 equal: `883E841228F09DBEEEAD0EBF89B842F42E1DD47CD9BB19C90F250C65A06633DE`.
- Fresh frontend-ready marker on candidate and stable: PASS.
- No visible console and no forbidden development listener: PASS.

## Remaining acceptance gates

Only user-native acceptance remains:

1. Open the stable Agents page and confirm no persisted session is expanded by default and the initial screen is visually clean.
2. Start a harmless ScrubBots Claude session and confirm it reaches a truthful successful terminal state, the primary output is readable vertically, raw technical evidence stays collapsed unless explicitly opened, and there is no console flash or page-level horizontal scrolling.

## Final state

`M14B TECHNICAL STRICT RE-AUDIT = PASS`

`M14 = IMPLEMENTATION COMPLETE / PENDING USER NATIVE-VISUAL ACCEPTANCE`

`M15 = BLOCKED`

`M21 = NOT STARTED`
