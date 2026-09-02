# M14C Chat-Style Agent Conversation and Readiness UI Removal Remediation Log

Date: 2026-09-02
Branch: H!veAI
Repository: C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ
Builder: Codex

## Scope and boundary

M14C implements only the Agent Conversation presentation remediation and the removal of the primary Provider readiness card. M14 remains open pending independent strict re-audit and user native/visual acceptance. M15, M20, and M21 were not activated or started. Project Dashboard manifest runtime ingestion was not implemented. Canonical opening-video bytes, terminal-popup behavior, startup-audio behavior, and the visible application shell outside this remediation were preserved.

Pre-implementation commit: `d29a026` (origin/H!veAI fast-forward baseline).
Post-implementation commit: `810857676dfea13e33aa681f93c00cc920f35ba2`.
Unrelated user-owned files preserved: `C:\Users\sekip\Desktop\start-demo.bat`, `C:\Users\sekip\Desktop\task.md`.

## Implementation evidence

Changed files:

- `src-tauri/src/agent_session_center.rs`
- `src-tauri/src/codex_adapter.rs`
- `src-tauri/src/db/migrations.rs`
- `src-tauri/src/db/mod.rs`
- `src-tauri/src/external_browser.rs`
- `src-tauri/src/lib.rs`
- `src/agentSessionCenter.ts`
- `src/pages.tsx`
- `src/styles.css`
- `tests/m13-codex-adapter-focused.test.tsx`
- `tests/m14-agent-session-center-focused.test.tsx`

Claude and Codex output now pass through one bounded provider-neutral conversation projection. Assistant segments are extracted in provider order, exact duplicates are removed, result text is accepted only as provider-emitted final text, system/rate-limit/process-policy/session envelopes and redaction-only records are excluded, and tool/command records become a maximum 12-item compact activity disclosure. Missing final text displays the truthful `No final assistant response was captured` message. The default transcript is a distinct `You` message followed by the provider assistant message.

Markdown is rendered as React nodes without untrusted HTML execution. The bounded renderer supports headings, paragraphs, bullets, numbered lists, inline/fenced code, tables, bold text, and HTTPS links. HTTPS links dispatch through the native `hiveai_open_external_url` command, whose Chrome launch policy rejects non-HTTPS, whitespace, and quote-bearing input and uses the existing hidden native browser process policy.

The entire large Provider readiness panel was removed from the primary Agents page. Native readiness remains fetched and start remains gated by the selected provider's truthful availability. Persisted sessions remain compact and unselected on initial load; one explicit View action selects one detail region, and Close clears it. Technical details, Live terminal, Timeline, Raw events, and Git evidence remain closed disclosures.

Migration 9 adds nullable `agent_sessions.prompt_body`. Claude and Codex store only the already validated explicit request prompt, bounded by the existing 64 KiB prompt limit; no argv or system text is persisted. Historical rows remain valid and render `Original prompt text was not persisted for this session.` The migration is transactional, idempotent, nullable, and backward compatible.

## Native operation evidence

Installed Claude executable: `C:\Users\sekip\.local\bin\claude.exe`.
Installed version: `2.1.248 (Claude Code)`.
Invocation: `--print --output-format stream-json --verbose --no-session-persistence --permission-mode plan --restricted`.
Prompt transport: bounded stdin only; prompt absent from argv.

Harmless real operation used the read-only acceptance prompt:

`Inspect this project read-only and give me a concise architecture summary. Do not modify, create, delete, rename, or commit any files.`

The operation ran against the ScrubBots project, exited `0`, and emitted native system/rate-limit and meaningful assistant stream records. The assistant response included a concise architecture summary. Git status before and after was unchanged. The frontend projection fixture exercises the same assistant stream shape and proves the default view removes system/rate-limit/tool envelope noise while preserving readable assistant text and compact activity.

## Test and publication evidence

- Focused M13 Codex plus M14 Agent Session Center frontend tests: `12 passed, 0 failed`.
- Full frontend suite: `107 passed, 0 failed` across `12` files.
- `npm run typecheck`: PASS.
- `npm run build`: PASS; Vite transformed `1998` modules.
- `npm audit --audit-level=high`: PASS; `0 vulnerabilities`.
- `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`: PASS.
- `cargo check --manifest-path src-tauri/Cargo.toml --all-targets`: PASS.
- `cargo check --manifest-path src-tauri/Cargo.toml --all-targets --features pty-support`: PASS.
- Full serial Rust regression: `322 passed, 0 failed, 0 ignored` with `cargo test --manifest-path src-tauri/Cargo.toml --lib -- --test-threads=1`.
- `git diff --check`: PASS.
- Publisher failure/rollback harness: `9/9` PASS.
- Governed production publisher: PASS; `scripts/publish-dev-qa.ps1` built Tauri production `--no-bundle`, smoke-tested candidate, swapped stable, smoke-tested stable, and preserved shortcut target/icon.
- Candidate SHA-256 (`src-tauri/target/release/hiveai-desktop.exe`): `0982D47069171B4C58F9758EAD25D99B0D50B45B3CCCEC3962B60D334EB37681`.
- Stable SHA-256 (`dev-bin/H!veAI.exe`): `0982D47069171B4C58F9758EAD25D99B0D50B45B3CCCEC3962B60D334EB37681`.
- Candidate and stable bytes are equal; stable is a valid Windows PE.
- Candidate and stable emitted fresh `HIVEAI_FRONTEND_READY`; publisher found no forbidden development listener and no visible console host.
- No installer was created.

## Explicit gates

1. PASS - fetched `origin/H!veAI` and synchronized with fast-forward-only merge.
2. PASS - exact branch `H!veAI` confirmed.
3. PASS - unrelated user-owned files preserved.
4. PASS - authoritative M14, M14A, M14B prompts/logs/audits and current implementation/test evidence read.
5. PASS - initial Agents behavior reproduced from current implementation and focused tests.
6. PASS - selected-session raw/event-heavy behavior reproduced from current implementation and prior M14 evidence.
7. PASS - large Provider readiness panel reproduced before removal from current implementation and prior M14 evidence.
8. PASS - backend provider readiness checks preserved.
9. PASS - large Provider readiness panel removed from the primary Agents page.
10. PASS - provider availability still truthfully gates start actions.
11. PASS - initial Agents page auto-selects no persisted session.
12. PASS - persisted session list remains compact.
13. PASS - explicit View selects exactly one session.
14. PASS - explicit close clears selected session detail.
15. PASS - new-session prompt renders as a distinct user conversation message.
16. PASS - historical session without stored prompt text renders the truthful placeholder.
17. PASS - Claude assistant stream-json fixture produces readable assistant text.
18. PASS - Claude rate-limit/system records are excluded from the primary assistant message.
19. PASS - Claude raw tool-result JSON is excluded from the primary assistant message.
20. PASS - Claude repeated redaction-only records are excluded from the primary assistant message.
21. PASS - Claude multiple assistant segments preserve order without duplication.
22. PASS - Codex assistant text fixture produces readable assistant text.
23. PASS - Codex event envelopes are excluded from the primary assistant message.
24. PASS - Codex command/tool output is separated into activity.
25. PASS - tool activity is bounded to 12 evidence-derived summaries.
26. PASS - raw tool/event payloads are hidden by default.
27. PASS - Technical details is closed by default.
28. PASS - Timeline is closed by default.
29. PASS - Raw events is closed by default.
30. PASS - Git evidence is closed by default.
31. PASS - completed sessions expose no invalid Stop action.
32. PASS - Resume remains capability-gated.
33. PASS - failed diagnostics remain concise and visible without raw-event expansion.
34. PASS - long Markdown paragraphs wrap vertically.
35. PASS - long paths and code blocks cannot create page-level horizontal scrolling.
36. PASS - Markdown headings, lists, code, and tables render safely as React nodes.
37. PASS - provider Markdown has no untrusted HTML execution path.
38. PASS - external links use the bounded native HTTPS browser policy.
39. PASS - prompt remains absent from provider process argv.
40. PASS - stdin prompt transport remains bounded.
41. PASS - exact ACTIVE registered-project confinement tests pass.
42. PASS - cross-project task/session authorization tests pass.
43. PASS - redaction-before-persistence tests pass.
44. PASS - output and event bound tests pass.
45. PASS - restart/orphan recovery regression passes.
46. PASS - focused Claude backend regression passes.
47. PASS - focused Codex backend regression passes.
48. PASS - full serial Rust regression, `322/322`.
49. PASS - focused Agent Session Center frontend regression, `12/12`.
50. PASS - full frontend regression, `107/107`.
51. PASS - `npm run typecheck`.
52. PASS - `npm run build`.
53. PASS - `npm audit --audit-level=high`, zero vulnerabilities.
54. PASS - cargo format check.
55. PASS - cargo all-target check and required `pty-support` check.
56. PASS - `git diff --check`.
57. PASS - publisher rollback/failure harness, `9/9`.
58. PASS - governed production `--no-bundle` publication.
59. PASS - fresh candidate `HIVEAI_FRONTEND_READY`.
60. PASS - stable executable `HIVEAI_FRONTEND_READY` after swap.
61. PASS - candidate/stable SHA-256 equality.
62. PASS - no forbidden development listener.
63. PASS - no visible console during publisher startup/readiness/native smoke.
64. PASS - real Claude readiness/version remains valid.
65. PASS - harmless real ScrubBots Claude session reached provider execution and exited successfully.
66. PASS - real Claude stream produced a meaningful assistant response; the chat projection and native stream evidence contain no default raw event wall.
67. PASS - read-only Claude operation produced no ScrubBots Git status delta.
68. PASS - M15-M20 were not activated.
69. PASS - M21 was not started.
70. PENDING USER ACCEPTANCE - user confirms the readiness card is gone and the initial Agents page is visually clean in stable H!veAI.exe.
71. PENDING USER ACCEPTANCE - user confirms a selected Claude/Codex session reads like normal AI chat and raw activity remains hidden by default.

## Final state

M14C REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE

M14 was not closed. M15 and M21 remain untouched.
