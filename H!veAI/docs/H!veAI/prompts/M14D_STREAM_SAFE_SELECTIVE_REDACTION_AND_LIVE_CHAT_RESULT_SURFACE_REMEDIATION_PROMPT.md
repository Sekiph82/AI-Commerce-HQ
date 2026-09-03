# M14D Stream-Safe Selective Redaction and Live Chat Result Surface Remediation Prompt

Date: 2026-09-03
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Milestone: M14
Scope: native acceptance remediation only

## Mission

Close only M14 native findings R46-R49 from:

`H!veAI/docs/H!veAI/audits/M14_NATIVE_ACCEPTANCE_CLAUDE_RESPONSE_REDACTION_AND_CHAT_SURFACE_AUDIT.md`

The current stable M14C build can run Claude to `COMPLETED / exit code 0`, but a real user operation produced dozens of `[REDACTED SENSITIVE OUTPUT]` records and the primary chat displayed `No final assistant response was captured` instead of Claude's answer.

This is not acceptable. Diagnose and fix the persistence/redaction pipeline so real Claude/Codex assistant answers survive safely, then finish the Agents page as a chat-first execution surface.

Do not close M14. Do not activate M15. Do not start M21. M14 remains pending independent strict re-audit and explicit user native/visual acceptance after M14D.

## Proven defect that must be treated as root evidence

Current production `src-tauri/src/agent_session_center.rs` has a `StreamRedactor` with `MAX_REDACTION_CARRY_BYTES = 4096` and whole-record logic equivalent to:

- lowercase the entire newline-delimited record,
- if the record contains `api_key`, `apikey`, `token`, `password`, `secret`, `authorization`, or `sk-`, replace the **entire record** with `[REDACTED SENSITIVE OUTPUT]`,
- if an unterminated record exceeds 4096 bytes, replace/discard it until newline.

Current Codex redaction uses materially similar whole-record substring classification. Preserve the M13B pre-persistence security guarantee, but do not preserve this destructive false-positive behavior.

Normal Claude `stream-json` records may contain words/keys such as `input_tokens`, `output_tokens`, `thinking_tokens`, rate-limit metadata, usage metadata, or other non-secret `token` strings. A valid assistant message may coexist in the same JSON record. Whole-record substring redaction therefore destroys non-secret assistant evidence.

The user's observed sequence count is event sequencing, not 73 separate Claude attempts. Do not redesign runtime around an assumption that Claude retried 73 times.

## R46 — Replace destructive whole-record redaction with selective pre-persistence sanitization

### Non-negotiable security rule

**Secrets must still be sanitized before any provider output is persisted.**

Do not persist raw provider JSON first and clean it later.

### Required design

Implement one bounded sanitizer contract shared where practical by Claude and Codex. Provider-specific parsing is allowed where their output shapes require it.

For newline-delimited JSON provider records:

1. Buffer a complete record within an explicit, documented maximum provider-record bound.
2. Parse JSON before persistence when valid JSON is available.
3. Recursively sanitize sensitive values while preserving the rest of the JSON structure.
4. Re-serialize only the sanitized structure for durable raw evidence.
5. Feed the sanitized structure/text into conversation projection.

Do **not** classify an entire record as sensitive merely because the serialized record contains the substring `token`, `secret`, etc.

### Key classification

At minimum, tests must prove that ordinary metadata fields are retained:

- `input_tokens`
- `output_tokens`
- `thinking_tokens`
- token counts / usage counters
- `rate_limit_info`
- model metadata
- natural-language assistant content containing the word `token`
- filenames or prose containing `secret` as a normal word when no credential value is present

Sensitive credential-bearing fields must have their **values** redacted, including at least proven variants such as:

- exact `api_key`
- exact `apikey`
- exact `access_token`
- exact `refresh_token`
- exact credential-bearing `token`
- exact `password`
- exact `authorization`
- exact `secret`
- exact `client_secret`

Use controlled normalization for key names, not unrestricted substring matching. Add only variants supported by real risk/evidence.

### Value classification

Regardless of field name, preserve existing protection for credential-like values. At minimum tests must cover:

- `sk-...` style keys,
- `Bearer <credential>` values,
- other existing credential formats already protected by accepted M13B evidence.

Redact the sensitive value only. Preserve neighboring non-sensitive assistant/message/tool metadata.

### Plain-text non-JSON records

For stderr or genuinely non-JSON text, use a bounded selective sanitizer that masks actual credential spans/assignments rather than replacing any line containing a generic word such as `token`.

A line such as `token usage: 1234` must remain visible.
A line such as `authorization: Bearer abc...` must redact the credential value.

### Overlong records

The current 4096-byte discard behavior is unacceptable for normal Claude stream-json.

Implement a bounded strategy that:

- can accommodate normal installed-Claude stream records observed in native testing,
- never allocates without a fixed upper bound,
- never silently discards a valid assistant answer solely because a record exceeded 4096 bytes,
- emits explicit truncation/oversize diagnostics if a hard bound is genuinely exceeded,
- preserves as much safe meaningful assistant evidence as the contract permits,
- keeps the existing global output/event bounds.

Do not simply increase a constant without proving the real stream shapes and testing boundary behavior.

## R47 — Assistant-response durability becomes an explicit acceptance invariant

A successful freeform Claude/Codex operation must preserve meaningful user-facing assistant/result evidence when the provider emitted it.

For the native acceptance prompt:

`Inspect this project read-only and summarize its repository structure. Do not modify any files.`

require all of the following:

1. provider process exits `0`,
2. session terminal state is `COMPLETED`,
3. persisted sanitized provider evidence contains a meaningful assistant/result segment,
4. reloading the session from SQLite still yields that assistant/result segment,
5. the frontend chat projection renders the same answer,
6. no secret material is exposed,
7. Git status has no delta for the read-only prompt.

If process exit is `0` but meaningful assistant evidence was locally lost due to parser/redaction/persistence degradation, do not silently present a normal successful chat. Surface a bounded truthful diagnostic distinct from a genuine provider response of zero text.

Do not fabricate an answer.

## R48 — Sequence/event UX must not look like repeated AI attempts

Preserve durable event sequence IDs for audit truth, but stop making transport event count look like multiple attempts.

Primary chat surface:

- never shows raw sequence numbers,
- never shows repeated `[REDACTED SENSITIVE OUTPUT]` entries,
- never shows raw `STREAM_STDOUT` / `STREAM_OUTPUT` envelopes.

Advanced Timeline/Raw events:

- remain closed by default,
- may show sequence IDs when explicitly opened,
- should compact consecutive redaction-only or equivalent transport records where doing so does not destroy audit truth, for example `18 sanitized transport records`, with the underlying raw sanitized events still available in Raw events if required,
- clearly label these as provider/transport events, not agent attempts.

## R49 — Finish the Agents page as a live chat/result surface

Apply the user's exact requested hierarchy.

### Start form order

The visible start form must be ordered:

1. Project
2. Task ID
3. Prompt
4. Provider
5. Start session button

Move the Provider selector **below the Prompt field**, immediately before the Start button.

Provider preference persistence may remain unchanged.

### Current conversation/result area

Immediately below the start form, before historical session history, create a dedicated large `Current conversation` / `Latest result` surface.

Behavior:

- Initial Agents navigation: no historical persisted session auto-opens.
- When the user explicitly clicks `View` on history, that session may populate the conversation surface.
- When the user starts a **new** session, automatically focus/select only that newly-created session and show it in the conversation surface.
- While RUNNING, show the user's prompt and a restrained progress/status state. Do not dump raw stream JSON.
- As assistant text becomes safely available, render it in the Claude/Codex response area.
- On completion, keep the full answer visible in the same surface.
- The answer must read like ChatGPT/Claude: Markdown headings, paragraphs, bullets, code blocks, tables and safe HTTPS links.
- The conversation area must have comfortable vertical reading and no page-level horizontal scroll.
- `You` prompt and `Claude` / `Codex` answer are the dominant content.

### History placement

Move/keep `Active and persisted sessions` as a compact history section **below** the current conversation/result surface.

The history rows should remain compact:

- provider,
- operation,
- project,
- state,
- time,
- View.

Do not expand old sessions automatically.

### Advanced evidence

Under the conversation area, keep closed by default:

- Activity / tool actions,
- Technical details,
- Timeline,
- Raw events,
- Git evidence.

The visible permission-model prose is not primary conversational content. Move it into Technical details or another advanced disclosure unless a live permission state requires user action.

## Preserve accepted architecture and security boundaries

Do not regress any accepted M13/M14 guarantees:

- exact ACTIVE registered-project confinement,
- canonical registered project cwd,
- cross-project task/session rejection,
- no arbitrary cwd,
- no arbitrary executable,
- no arbitrary args,
- no arbitrary PID control,
- no shell wrappers,
- bounded stdin prompt transport,
- prompt absent from argv,
- direct native executable validation,
- no-visible-console policy,
- pre-persistence redaction,
- durable event/session truth,
- bounded output/event persistence,
- event writer durability guarantees,
- process ownership and stop semantics,
- restart/orphan recovery,
- provider-neutral session model,
- governed Git evidence,
- safe Markdown / no untrusted HTML,
- bounded HTTPS external navigation,
- governed publication and rollback,
- M15/M21 untouched.

## Required investigation before implementation

Before changing code:

1. Fetch `origin/H!veAI` and fast-forward only.
2. Confirm exact branch `H!veAI`.
3. Read the M14, M14A, M14B, M14C prompts/logs/audits and the new R46-R49 native audit in full.
4. Inspect current:
   - `src-tauri/src/agent_session_center.rs`
   - `src-tauri/src/codex_adapter.rs`
   - event persistence/database loading code
   - `src/agentSessionCenter.ts`
   - Agents implementation in `src/pages.tsx`
   - relevant CSS
   - focused tests.
5. Reproduce the false-positive redaction with representative Claude stream-json containing token-count fields plus assistant text.
6. Reproduce the >4096 record behavior before fixing it.
7. If safe and available, read the existing failed native session evidence from the local H!veAI database read-only to characterize which provider record types were redacted. Never print or log secret values.
8. Inspect actual installed Claude `2.1.248` stream-json output shape with a disposable/read-only operation before finalizing parser assumptions.

## Explicit execution gates

All gates are mandatory unless marked USER ACCEPTANCE.

1. PASS — safe fetch and fast-forward-only sync.
2. PASS — exact `H!veAI` branch.
3. PASS — unrelated user-owned files preserved.
4. PASS — R46-R49 audit and prior M14 evidence read in full.
5. PASS — current whole-record generic `token` false-positive reproduced.
6. PASS — current 4096-byte destructive record behavior reproduced.
7. PASS — actual installed Claude stream-json shapes sampled safely.
8. PASS — sanitizer architecture documented before code change.
9. PASS — sanitization occurs before durable stream persistence.
10. PASS — valid JSON structure remains parseable after sanitization.
11. PASS — `input_tokens` does not redact the whole record.
12. PASS — `output_tokens` does not redact the whole record.
13. PASS — `thinking_tokens` does not redact the whole record.
14. PASS — rate-limit token/usage metadata does not redact the whole record.
15. PASS — natural-language word `token` does not redact the whole message.
16. PASS — assistant text survives alongside token-count metadata in the same JSON record.
17. PASS — exact `api_key` value is redacted.
18. PASS — exact `access_token` value is redacted.
19. PASS — exact `refresh_token` value is redacted.
20. PASS — exact credential-bearing `token` value is redacted.
21. PASS — `password` value is redacted.
22. PASS — `authorization` / Bearer credential is redacted.
23. PASS — `secret` / `client_secret` value is redacted.
24. PASS — `sk-...` credential-like value is redacted even when nested.
25. PASS — neighboring non-sensitive JSON fields survive each credential redaction fixture.
26. PASS — plain-text token-usage line survives.
27. PASS — plain-text credential assignment redacts only the credential span/value.
28. PASS — split-read credential marker/value remains protected across OS reads.
29. PASS — record larger than 4096 bytes containing assistant text is not blindly discarded.
30. PASS — hard provider-record upper bound is explicit and tested.
31. PASS — genuinely over-bound record produces truthful bounded truncation/diagnostic behavior.
32. PASS — global output byte/event bounds preserved.
33. PASS — persistence writer durability/retry behavior preserved.
34. PASS — Claude representative stream fixture preserves assistant answer after sanitize→persist→reload.
35. PASS — Codex representative fixture preserves assistant answer after sanitize→persist→reload.
36. PASS — real read-only ScrubBots Claude operation exits `0`.
37. PASS — real operation terminal state is `COMPLETED`.
38. PASS — real operation has meaningful assistant/result evidence before frontend projection.
39. PASS — reloaded SQLite session still contains meaningful assistant/result evidence.
40. PASS — frontend projection renders the real/representative answer and not `No final assistant response was captured`.
41. PASS — no secret values appear in sanitized raw evidence, transcript, logs, or screenshots.
42. PASS — read-only operation creates no Git status delta.
43. PASS — raw event sequence IDs remain durable audit truth.
44. PASS — primary chat shows no sequence IDs or raw stream envelopes.
45. PASS — repeated redaction-only transport chatter does not dominate Timeline UI.
46. PASS — transport/event wording cannot reasonably imply separate AI attempts.
47. PASS — start form field order is Project → Task ID → Prompt → Provider → Start.
48. PASS — Provider selector is visually below Prompt.
49. PASS — new `Current conversation` / `Latest result` area exists directly below start form.
50. PASS — initial Agents navigation auto-selects no historical session.
51. PASS — starting a new session auto-focuses only that newly-created session.
52. PASS — new session user prompt appears immediately in current conversation area.
53. PASS — RUNNING session uses compact progress/activity, not raw JSON dump.
54. PASS — assistant response appears in current conversation area when safely captured.
55. PASS — completed response remains visible without requiring Raw events/Timeline.
56. PASS — historical session list appears below current conversation/result area.
57. PASS — historical rows remain compact and explicit-View only.
58. PASS — Activity/Technical details/Timeline/Raw events/Git evidence closed by default.
59. PASS — permission-model prose is moved out of primary chat unless actionable.
60. PASS — long Markdown wraps vertically with no page-level horizontal scroll.
61. PASS — Markdown rendering remains safe, no untrusted HTML.
62. PASS — HTTPS external-link policy regression tests pass.
63. PASS — exact ACTIVE project confinement tests pass.
64. PASS — cross-project task/session authorization tests pass.
65. PASS — prompt remains absent from argv and stdin stays bounded.
66. PASS — no arbitrary executable/args/PID/shell regression.
67. PASS — no-visible-console regression.
68. PASS — restart/orphan/stop lifecycle regression.
69. PASS — focused Claude backend tests.
70. PASS — focused Codex backend tests.
71. PASS — full serial Rust regression.
72. PASS — focused Agent Session Center frontend tests.
73. PASS — full frontend suite.
74. PASS — `npm run typecheck`.
75. PASS — `npm run build`.
76. PASS — `npm audit --audit-level=high`.
77. PASS — `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`.
78. PASS — `cargo check --manifest-path src-tauri/Cargo.toml --all-targets`.
79. PASS — required `pty-support` cargo check/regression.
80. PASS — `git diff --check`.
81. PASS — publisher rollback/failure harness.
82. PASS — governed production `--no-bundle` publication.
83. PASS — fresh candidate `HIVEAI_FRONTEND_READY`.
84. PASS — stable `HIVEAI_FRONTEND_READY` after swap.
85. PASS — candidate/stable SHA-256 equality.
86. PASS — no forbidden dev listener.
87. PASS — no visible console in stable native smoke.
88. PASS — M15-M20 not activated.
89. PASS — M21 not started.
90. USER ACCEPTANCE — user confirms real Claude answer is visible in the chat result area.
91. USER ACCEPTANCE — user confirms no redaction/event wall in normal use and the new layout matches the requested prompt/provider/result hierarchy.

## Native acceptance evidence

Run at least one new real Claude session against ScrubBots with exactly or materially equivalent harmless read-only prompt:

`Inspect this project read-only and summarize its repository structure. Do not modify any files.`

Capture bounded evidence proving:

- prompt shown under `You`,
- provider shown as Claude,
- actual Claude assistant answer visible in primary conversation,
- state `COMPLETED`, exit `0`,
- assistant answer persists after navigating away/back or reloading session,
- no dozens of redaction markers in primary experience,
- Timeline/Raw events remain optional advanced views,
- no Git status delta,
- no console flash.

Do not treat exit code `0` alone as the acceptance proof.

## Required immutable log

Create:

`H!veAI/docs/H!veAI/codex-logs/M14D_STREAM_SAFE_SELECTIVE_REDACTION_AND_LIVE_CHAT_RESULT_SURFACE_REMEDIATION_LOG.md`

The log must include:

- pre/post implementation commit IDs,
- exact root cause reproduced,
- representative false-positive record examples with synthetic/non-secret values only,
- sanitizer architecture and exact sensitive-key/value policy,
- overlong-record policy and bounds,
- proof assistant evidence survives sanitize→persist→reload→frontend projection,
- real Claude operation result,
- before/after Git evidence,
- focused/full test counts,
- publisher candidate/stable hashes,
- outcomes of all 91 gates,
- remaining user-acceptance gates only.

## Final state

If implementation gates pass, report only:

`M14D REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

Do not close M14.
Do not activate M15.
Do not start M21.
