# M14B Claude Real Operation and Agent Session Center Readability Remediation Log

Date: 2026-09-02
Branch: H!veAI
Repository: C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ
Builder: Codex

## Scope

M14B closes only M14-R38, M14-R39, and M14-R40. M14 remains open pending independent strict re-audit and user native/visual acceptance. M15 and M21 were not activated or started.

Pre-implementation commit: `b4590680509591a235dbd3857b4a2500a751a843`
Post-implementation commit: `1880a20`
Unrelated user-owned files preserved: `C:\Users\sekip\Desktop\start-demo.bat`, `C:\Users\sekip\Desktop\task.md`.

## R38 Claude evidence and fix

Installed direct executable:

`C:\Users\sekip\.local\bin\claude.exe`

Installed version:

`2.1.248 (Claude Code)`

Relevant installed help evidence:

- `--print`: print response and exit, useful for pipes.
- `--output-format <format>`: available formats include `text`, `json`, and `stream-json`; usable with `--print`.
- `--permission-mode <mode>`: includes `plan`.
- `--restricted`: restricted mode.
- `--verbose`: override verbose mode; required with `--print --output-format=stream-json`.
- `--input-format <format>`: text or stream-json for print mode.
- `--no-session-persistence`: disables session persistence.

Pre-fix reproduction, using the installed executable and the governed fixed arguments without `--verbose`:

`claude.exe --print --output-format stream-json --no-session-persistence --permission-mode plan --restricted`

Result: exit code `1`; exact diagnostic: `Error: When using --print, --output-format=stream-json requires --verbose`.

Corrected native policy:

`claude.exe --print --output-format stream-json --verbose --no-session-persistence --permission-mode plan --restricted`

The executable is resolved from a direct native candidate, the argument vector is fixed and allowlisted, the shell is disabled, the working directory is the validated ACTIVE registered project root, and the prompt is written through bounded stdin only. The prompt is absent from argv.

Harmless real native operation:

- Working directory: `C:\Users\sekip\Desktop\ScrubBots`.
- Prompt: read-only repository inspection; no writes, commits, or file creation requested.
- Exit code: `0`.
- Stream evidence reached native execution and included `rate_limit_event`, `system/init`, and `system/thinking_tokens` records.
- No argument-validation failure occurred.
- Git status was captured before and after; there was no status delta. Pre-existing ScrubBots untracked artifacts were preserved untouched.

## R39 and R40 implementation evidence

- Agents no longer selects `sessions[0]` on initial load.
- Persisted sessions render as compact provider/project/operation/state/time rows.
- A user-visible `View` action explicitly selects one session.
- Only one selected detail region is rendered, with an explicit close button.
- Completed and failed sessions do not expose invalid Stop actions; unsupported Resume is not actionable.
- The primary output is a compact vertical human-readable reader for both Codex and Claude.
- Assistant text, result text, tool/command summaries, and plain output are derived from persisted output/events without fabricating content.
- Technical details, Timeline, Raw events, Live terminal, and Git evidence are disclosure sections; raw JSON is not the default experience.
- Failed diagnostics remain visible in a concise diagnostic card, while raw stderr remains advanced evidence.
- Long paths and identifiers wrap within the reader and page-level horizontal overflow is prevented.
- Git changes remain sourced from the Git Engine authority.

## Test and verification evidence

- Focused M13 Codex and M14 Agent Session Center frontend tests: `10 passed, 0 failed`.
- Full frontend suite: `105 passed, 0 failed` across `12` files.
- `npm run typecheck`: PASS.
- `npm run build`: PASS; Vite transformed `1998` modules.
- `npm audit --audit-level=high`: PASS; `0 vulnerabilities`.
- `cargo fmt -- --check`: PASS.
- `cargo check --all-targets --features pty-support`: PASS.
- Full serial Rust regression: `321 passed, 0 failed, 0 ignored` with `--test-threads=1` and `pty-support`.
- `git diff --check`: PASS.
- Publisher failure/rollback harness: all `9` scenarios PASS.

## Governed publication evidence

- Publisher: `scripts/publish-dev-qa.ps1`.
- Production mode: `npm run tauri:build -- --no-bundle`.
- Candidate smoke test: PASS, fresh `HIVEAI_FRONTEND_READY`, stable H!veAI window title, no forbidden development port, no visible console host.
- Stable smoke test after swap: PASS, fresh `HIVEAI_FRONTEND_READY`, no forbidden development port, no visible console host.
- Stable executable: `H!veAI/dev-bin/H!veAI.exe`.
- Candidate/release SHA-256: `883E841228F09DBEEEAD0EBF89B842F42E1DD47CD9BB19C90F250C65A06633DE`.
- Stable SHA-256: `883E841228F09DBEEEAD0EBF89B842F42E1DD47CD9BB19C90F250C65A06633DE`.
- Candidate and stable bytes are equal.
- Desktop shortcut target and icon remained governed and correct.
- No installer was created.

## Explicit gates

1. PASS - fetched `origin/H!veAI` and fast-forward synchronized.
2. PASS - exact branch `H!veAI`.
3. PASS - unrelated user-owned files preserved.
4. PASS - M14B prompt and relevant M14 audit evidence read in full.
5. PASS - pre-fix `stream-json requires --verbose` failure reproduced.
6. PASS - installed version `2.1.248 (Claude Code)` recorded.
7. PASS - installed CLI help inspected for governed flags.
8. PASS - corrected fixed Claude invocation implemented.
9. PASS - bounded stdin-only prompt transport preserved.
10. PASS - no-shell, no-arbitrary-executable, no-arbitrary-args review.
11. PASS - direct native Claude resolver tests.
12. PASS - exact ACTIVE project confinement tests.
13. PASS - cross-project task/session authorization tests.
14. PASS - redaction-before-persistence tests.
15. PASS - output/event bound tests.
16. PASS - retry/recovery lifecycle tests.
17. PASS - focused Claude backend tests executed.
18. PASS - focused Codex backend regressions executed.
19. PASS - full serial Rust regression, `321/321`.
20. PASS - focused Agent Session Center frontend tests.
21. PASS - full frontend suite, `105/105`.
22. PASS - no persisted session auto-selected on initial load.
23. PASS - compact persisted session list.
24. PASS - one explicit selected detail.
25. PASS - explicit close/collapse selected detail.
26. PASS - human-readable Claude output presentation contract.
27. PASS - human-readable Codex output presentation contract.
28. PASS - Raw events and Timeline collapsed by default.
29. PASS - failed diagnostic shown once in default detail view.
30. PASS - invalid Stop/Resume actions hidden or capability-gated.
31. PASS - long IDs/paths cannot create page-level horizontal overflow.
32. PASS - `npm run typecheck`.
33. PASS - `npm run build`.
34. PASS - `npm audit --audit-level=high`, no high vulnerabilities.
35. PASS - cargo format check.
36. PASS - cargo check all targets with `pty-support`.
37. PASS - `git diff --check`.
38. PASS - real Claude readiness/version in native environment.
39. PASS - harmless real Claude operation passed argument validation and reached provider execution.
40. PASS - harmless real Claude operation exited `0` with native stream evidence.
41. PASS - read-only operation produced no Git status delta or unrelated change.
42. PASS - no visible console during readiness, operation, or governed smoke tests.
43. PASS - governed publisher rollback/failure harness, `9/9`.
44. PASS - governed production `--no-bundle` publication.
45. PASS - candidate emitted fresh `HIVEAI_FRONTEND_READY`.
46. PASS - stable executable emitted fresh `HIVEAI_FRONTEND_READY` after swap.
47. PASS - stable bytes equal accepted candidate bytes by SHA-256.
48. PASS - no forbidden development listener.
49. PASS - M15-M20 not activated.
50. PASS - M21 not started.
51. PENDING USER ACCEPTANCE - user opens stable Agents page and confirms clean compact initial visual.
52. PENDING USER ACCEPTANCE - user starts ScrubBots Claude session and confirms readable vertical output and truthful final state.

## Final state

M14B REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE

M14 was not closed. M15 and M21 remain untouched.
