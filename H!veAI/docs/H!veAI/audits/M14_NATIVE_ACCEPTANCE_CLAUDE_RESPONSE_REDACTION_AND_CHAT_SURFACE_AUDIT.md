# M14 Native Acceptance Claude Response Redaction and Chat Surface Audit

Date: 2026-09-03
Branch: H!veAI
Reviewed implementation: `810857676dfea13e33aa681f93c00cc920f35ba2`
Scope: user native acceptance after M14C

## Verdict

**FAIL / M14 MUST REMAIN OPEN**

The user confirmed that the large Provider readiness card is gone and that real Claude sessions can reach `COMPLETED` with exit code `0`. However, the actual assistant answer is not reliably retained or rendered. The selected session shows `No final assistant response was captured`, while Timeline/Raw events contain dozens of `[REDACTED SENSITIVE OUTPUT]` stream records. The user also requests a tighter chat-first layout.

## R46 MAJOR — Claude stream redaction destroys non-secret provider records

Production source in `src-tauri/src/agent_session_center.rs` applies whole-record redaction whenever the lowercased record contains any substring in:

- `api_key`
- `apikey`
- `token`
- `password`
- `secret`
- `authorization`
- `sk-`

This is too broad for Claude `stream-json`. Normal provider records legitimately contain non-secret metadata such as token counts, thinking-token/rate-limit fields, tool/session metadata, and other strings containing `token`. Because the implementation replaces the entire newline-delimited JSON record with `[REDACTED SENSITIVE OUTPUT]`, meaningful assistant content in the same record can be destroyed before persistence. The native evidence supplied by the user shows this repeatedly and the final chat projection has no assistant response to render.

Additionally, the same stream redactor replaces an unterminated record once the carry exceeds `4096` bytes and discards the remainder until newline. Claude stream-json records containing tool results or assistant payloads can exceed this bound. This can destroy valid provider evidence even when no secret exists.

The redaction guarantee must remain pre-persistence, but it must become structure-aware and value-selective. Ordinary words/metadata containing `token` must never cause the entire record to disappear.

## R47 MAJOR — `COMPLETED` is not sufficient if no usable assistant response survives persistence

A Claude process exiting `0` currently yields terminal state `COMPLETED` even when the user-facing transcript cannot recover any assistant answer. Native acceptance requires stronger truthful success evidence for normal freeform sessions.

A completed session must preserve at least one meaningful provider assistant/result segment when the provider emitted one. If the provider genuinely emits no assistant/result text, the UI may truthfully say so, but the implementation must distinguish that case from local evidence loss caused by redaction/truncation/parsing.

For the exact native acceptance prompt shown by the user, the provider did real work and the session completed, yet the answer is absent. This is a product failure even though exit code is zero.

## R48 MAJOR — Timeline/raw event sequencing is transport evidence, not repeated AI attempts

The user's observation of roughly 73 sequences does not mean Claude attempted the prompt 73 times. These are persisted event sequence numbers generated for stream/provider events. The current advanced view makes transport chatter look like repeated agent attempts.

The primary conversation must never expose this sequence noise. Advanced Timeline/Raw events may retain bounded evidence, but repeated redaction-only stream events should be compacted or summarized where safe and must not dominate the surface.

## R49 MAJOR — Chat response surface and start-form hierarchy still need native UX remediation

The user requests the following exact visual hierarchy:

1. Start form order should be:
   - Project
   - Task ID
   - Prompt
   - Provider
   - Start session button
2. Move the Provider selector below the Prompt field.
3. Immediately below the start form, reserve a large conversation/result area where the newly started Claude/Codex session is shown live and remains visible after completion.
4. Starting a new session may auto-focus/select **that newly-created session only**. Initial navigation must still never auto-open an old persisted session.
5. Historical `Active and persisted sessions` should remain a compact history section below the current conversation/result area.
6. The current response surface must show `You` and `Claude`/`Codex` with the actual assistant answer as dominant content.
7. Technical details, Timeline, Raw events, and Git evidence stay closed by default.

## Required remediation direction

Do not weaken M13B security. Replace blanket line-substring redaction with a bounded, provider-safe sanitizer that preserves non-secret JSON structure and assistant text while redacting actual secret values before persistence.

At minimum:

- Parse complete Claude NDJSON records within an explicit bounded record size.
- Recursively sanitize sensitive JSON fields by exact/controlled key classification rather than substring-matching the entire serialized record.
- `input_tokens`, `output_tokens`, `thinking_tokens`, rate-limit counters, model metadata, and natural-language use of the word `token` are not secrets by themselves.
- Exact credential-bearing fields such as `api_key`, `apikey`, `access_token`, `refresh_token`, exact `token` where semantically credential-bearing, `password`, `authorization`, `secret`, `client_secret`, and equivalent proven credential fields must have values redacted.
- Credential-like values such as OpenAI-style `sk-...`, bearer credentials, or other existing proven secret patterns must remain redacted even when nested.
- Never persist an unredacted secret first and sanitize later.
- Preserve assistant/result text in a record even if unrelated metadata in that same record requires redaction.
- Replace the 4096-byte destructive whole-record fallback with an explicitly bounded non-destructive strategy for normal Claude stream-json. Over-bound records must produce truthful truncation evidence, not silently erase valid assistant content.
- Preserve global output/event bounds.

## Final status

**BLOCKER: 0**
**MAJOR: 4 (R46-R49)**

M14 remains open. M15 and M21 remain blocked/not started.
