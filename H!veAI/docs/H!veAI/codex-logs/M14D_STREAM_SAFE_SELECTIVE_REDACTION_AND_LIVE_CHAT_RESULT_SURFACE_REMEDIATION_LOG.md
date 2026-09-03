# M14D Stream-Safe Selective Redaction and Live Chat Result Surface Remediation Log

Date: 2026-09-03
Repository: Sekiph82/AI-Commerce-HQ
Branch: H!veAI
Scope: M14-R46 through M14-R49 only

## Final state

M14D REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE

M14 remains open. M15-M20 remain planned/not activated. M21 remains planned/not started.

## Commit and sync evidence

- Pre-M14D synchronized commit: `3719e84970b24cf38568e31a2c2642e1f80c557e`.
- Post-implementation source commit: `7e06ac952b2c7b84f336145dce967703e48c2844`.
- Post-focused-evidence commit: `402643bb7837c249cef95a415589ac15bbbeb651`.
- The pre-log source commits were created only after the working tree was tested.
- The unrelated parent files `C:\Users\sekip\Desktop\start-demo.bat` and `C:\Users\sekip\Desktop\task.md` were preserved and were not staged.

## Root cause reproduced

The M14C Claude redactor lowercased each complete NDJSON record and replaced the entire record with `[REDACTED SENSITIVE OUTPUT]` when any generic substring such as `token`, `secret`, or `authorization` appeared. A synthetic non-secret record containing `input_tokens`, `output_tokens`, `thinking_tokens`, `rate_limit_info`, model metadata, a filename `secret.txt`, and assistant text was therefore destroyed. The same behavior was present in the Codex adapter.

The old redactor also discarded an unterminated record after its 4096-byte carry threshold and dropped the remainder through the next newline. A synthetic assistant record larger than 4096 bytes reproduced the destructive fallback.

The installed Claude Code `2.1.248` was sampled with the required harmless read-only ScrubBots prompt. The bounded stream included `rate_limit_event`, `system`, `assistant`, `user`, and `result` records. The operation exited `0` and produced 3769 extracted answer characters without modifying the project.

## Remediation

`src-tauri/src/stream_sanitizer.rs` now provides the shared pre-persistence contract used by Claude and Codex. Complete records are bounded at 256 KiB, parsed as JSON when valid, recursively sanitized by exact normalized key names, reserialized, and only then sent to capture and durable event persistence. Non-JSON text uses selective assignment and credential-span masking. `sk-...` and `Bearer ...` values remain protected across split reads. A genuinely over-bound record preserves its bounded prefix and emits `[PROVIDER RECORD TRUNCATED]`; it is never silently discarded.

The exact sensitive key policy is `api_key`, `apikey`, `access_token`, `refresh_token`, credential-bearing `token`, `password`, `authorization`, `secret`, and `client_secret`, with hyphen-to-underscore normalization. Ordinary token counters, rate-limit metadata, model metadata, natural-language `token`, and ordinary `secret` prose remain visible. Sensitive values are replaced with `[REDACTED SENSITIVE VALUE]` while neighboring fields remain intact.

Successful provider completion now checks sanitized stdout for meaningful assistant/result evidence. A zero exit without durable answer text receives a bounded `CLAUDE_ASSISTANT_EVIDENCE_UNAVAILABLE` or `CODEX_ASSISTANT_EVIDENCE_UNAVAILABLE` diagnostic instead of silently presenting normal success.

The Agents page now orders Project, Task ID, Prompt, Provider, and Start session. A dedicated Current conversation surface is rendered before compact history, auto-focuses only newly started or explicitly viewed sessions, shows restrained RUNNING progress, and keeps assistant Markdown visible after completion. Tool activity, technical details, Timeline, Raw events, Git evidence, and permission-model prose remain advanced/closed evidence. Redaction-only transport lines are omitted from the primary reader.

## Sanitize, persist, reload, projection proof

- Claude: `agent_session_center::tests::sanitized_claude_assistant_survives_persist_and_reload` proves sanitized assistant JSON is persisted as a `STREAM_STDOUT` event, reloaded from SQLite, and retains `The project is healthy.` while excluding the synthetic credential.
- Codex: existing bounded stream persistence tests passed with the shared sanitizer and retain structured provider output across durable event rows and reload.
- Frontend: `m14-agent-session-center-focused.test.tsx` projects representative Claude/Codex assistant records into the vertical reader, preserves Markdown, shows the live answer in Current conversation, and excludes raw envelopes and repeated redaction-only lines.
- No secret value was written to the repository, test output artifact, or this log.

## Native and publication evidence

- Real read-only ScrubBots Claude operation: exit `0`; terminal completed; stream types included `assistant` and `result`; extracted answer was non-empty; no project Git delta was observed.
- Representative H!veAI persistence/reload proof: PASS as listed above. User-native reload/navigation confirmation remains a user acceptance gate.
- Governed publisher: `scripts/publish-dev-qa.ps1` PASS; production Tauri `--no-bundle`, candidate smoke, stable smoke, readiness marker, no forbidden development listener, no-visible-console check, shortcut target/icon validation, and rollback safeguards passed.
- Published stable path: `dev-bin\H!veAI.exe`.
- Candidate/stable SHA-256: `CA98036D58151895CAFFAAE3EC8A2F6DE5DC84D7660850B86146BF4230DD21C7`.
- Release build SHA-256 matched stable SHA-256 exactly.
- No candidate or rollback artifact remained after publication.

## Test evidence

- Frontend full suite: 108 passed, 0 failed, 12 files.
- Agent Session Center focused frontend suite: 7 passed.
- Codex focused frontend suite: 6 passed.
- Serial Rust library regression: 327 passed, 0 failed, 0 ignored.
- Targeted shared sanitizer/provider redaction tests: passed.
- `npm run typecheck`: PASS.
- `npm run build`: PASS, 1998 Vite modules.
- `npm audit --audit-level=high`: PASS, 0 vulnerabilities.
- `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`: PASS.
- `cargo check --manifest-path src-tauri/Cargo.toml --all-targets`: PASS.
- `cargo check --manifest-path src-tauri/Cargo.toml --features pty-support`: PASS.
- `git diff --check`: PASS.
- Publisher failure/rollback harness: 9/9 PASS.

## Gate ledger

1. PASS - safe fetch and fast-forward-only sync.
2. PASS - exact `H!veAI` branch.
3. PASS - unrelated user-owned files preserved.
4. PASS - R46-R49 audit and prior M14 evidence read in full.
5. PASS - current whole-record generic `token` false-positive reproduced.
6. PASS - current 4096-byte destructive record behavior reproduced.
7. PASS - actual installed Claude stream-json shapes sampled safely.
8. PASS - bounded sanitizer architecture documented before provider wiring.
9. PASS - sanitization occurs before durable stream persistence.
10. PASS - valid JSON remains parseable after sanitization.
11. PASS - `input_tokens` does not redact the whole record.
12. PASS - `output_tokens` does not redact the whole record.
13. PASS - `thinking_tokens` does not redact the whole record.
14. PASS - rate-limit token/usage metadata does not redact the whole record.
15. PASS - natural-language `token` does not redact the whole message.
16. PASS - assistant text survives beside token-count metadata.
17. PASS - exact `api_key` value is redacted.
18. PASS - exact `access_token` value is redacted.
19. PASS - exact `refresh_token` value is redacted.
20. PASS - exact credential-bearing `token` value is redacted.
21. PASS - `password` value is redacted.
22. PASS - `authorization` and Bearer credential are redacted.
23. PASS - `secret` and `client_secret` values are redacted.
24. PASS - nested `sk-...` credential-like values are redacted.
25. PASS - neighboring non-sensitive JSON fields survive credential redaction.
26. PASS - plain-text token-usage line survives.
27. PASS - plain-text credential assignment masks only its value.
28. PASS - split-read credential marker/value remains protected.
29. PASS - a record larger than 4096 bytes containing assistant text is retained.
30. PASS - hard provider-record upper bound is explicit and tested.
31. PASS - over-bound records produce truthful bounded truncation evidence.
32. PASS - global output byte/event bounds are preserved.
33. PASS - persistence writer durability/retry behavior is preserved.
34. PASS - Claude representative stream survives sanitize, persist, reload.
35. PASS - Codex representative stream survives sanitize, persist, reload.
36. PASS - real read-only ScrubBots Claude operation exits `0`.
37. PASS - real operation terminal completes.
38. PASS - real operation contains meaningful assistant/result evidence.
39. PASS - SQLite reload proof retains meaningful assistant/result evidence.
40. PASS - frontend projection renders representative answer, not the missing-answer placeholder.
41. PASS - no secret values appear in sanitized evidence, transcript, logs, or screenshots.
42. PASS - read-only operation creates no Git status delta.
43. PASS - raw event sequence IDs remain durable audit truth.
44. PASS - primary chat shows no sequence IDs or raw stream envelopes.
45. PASS - repeated redaction-only transport chatter does not dominate the primary reader.
46. PASS - transport/event wording cannot imply separate AI attempts.
47. PASS - start form order is Project, Task ID, Prompt, Provider, Start.
48. PASS - Provider is below Prompt immediately before Start.
49. PASS - Current conversation exists directly below the start form.
50. PASS - initial Agents navigation auto-selects no historical session.
51. PASS - new session auto-focuses only the newly created session.
52. PASS - new session user prompt appears in Current conversation.
53. PASS - RUNNING uses compact progress, not raw JSON.
54. PASS - safely captured assistant response appears in Current conversation.
55. PASS - completed response remains visible without advanced evidence.
56. PASS - history is below Current conversation.
57. PASS - history rows remain compact and explicit-View only.
58. PASS - Activity, Technical details, Timeline, Raw events, and Git evidence are closed by default.
59. PASS - permission-model prose is outside primary chat.
60. PASS - long Markdown wraps vertically with no page-level horizontal scroll.
61. PASS - Markdown rendering remains safe with no untrusted HTML.
62. PASS - HTTPS external-link policy regression tests pass.
63. PASS - exact ACTIVE project confinement tests pass.
64. PASS - cross-project task/session authorization tests pass.
65. PASS - prompt remains absent from argv and stdin is bounded.
66. PASS - arbitrary executable, args, PID, and shell controls remain rejected.
67. PASS - no-visible-console regression passes.
68. PASS - restart, orphan, and stop lifecycle regression passes.
69. PASS - focused Claude backend tests pass.
70. PASS - focused Codex backend tests pass.
71. PASS - full serial Rust regression passes.
72. PASS - focused Agent Session Center frontend tests pass.
73. PASS - full frontend suite passes.
74. PASS - `npm run typecheck` passes.
75. PASS - `npm run build` passes.
76. PASS - `npm audit --audit-level=high` passes.
77. PASS - Rust format check passes.
78. PASS - all-target Rust check passes.
79. PASS - `pty-support` Rust check passes.
80. PASS - `git diff --check` passes.
81. PASS - publisher rollback/failure harness passes 9/9.
82. PASS - governed production no-bundle publication passes.
83. PASS - fresh candidate `HIVEAI_FRONTEND_READY` passes.
84. PASS - stable `HIVEAI_FRONTEND_READY` after swap passes.
85. PASS - candidate/stable SHA-256 equality passes.
86. PASS - no forbidden development listener passes.
87. PASS - no visible console in stable native smoke passes.
88. PASS - M15-M20 remain inactive.
89. PASS - M21 remains not started.
90. USER ACCEPTANCE PENDING - user must confirm the real Claude answer is visible in the native chat result area.
91. USER ACCEPTANCE PENDING - user must confirm no redaction/event wall in normal use and the requested layout is correct.

## Remaining state

Only gates 90 and 91 remain pending. M14D is complete for independent strict re-audit and user native/visual acceptance. M14 is not closed.
