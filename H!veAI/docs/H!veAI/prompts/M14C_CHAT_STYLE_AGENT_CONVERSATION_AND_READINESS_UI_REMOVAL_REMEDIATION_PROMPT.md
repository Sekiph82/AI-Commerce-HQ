# M14C Chat-Style Agent Conversation and Readiness UI Removal Remediation Prompt

Date: 2026-09-02
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Milestone: M14
Scope: Native user-acceptance remediation only

## Mission

Remediate the remaining M14 native usability issues in the Agent Session Center without weakening any accepted M13/M14 security, process, persistence, provenance, redaction, project-confinement, or publication guarantees.

The current M14B build successfully executes a real Claude session to `COMPLETED`, but the selected-session experience still exposes too much provider stream/event/tool noise in the primary user-facing output. The user wants the session result to read like a ChatGPT/Claude conversation, not like a debugger or event inspector.

Also remove the large visible `Provider readiness` panel from the top of the Agents page. The readiness checks may remain internally available and may still be used to enable/disable providers or expose a small status only when needed, but the large diagnostic box must not occupy the primary Agents UI.

Do not close M14. Do not activate M15. Do not start M21. M14 remains pending independent strict re-audit and explicit user native/visual acceptance after this remediation.

## Authoritative user acceptance requirements

### R41: Chat-style selected-session experience

Replace the primary selected-session output with a conversation-oriented reader.

The default selected-session surface must look and behave like a modern AI chat transcript:

1. Show the user's submitted prompt as a clearly separated `You` / `User` message.
2. Show the provider's meaningful assistant response as a clearly separated `Claude` or `Codex` assistant message.
3. Render assistant Markdown safely and readably:
   - headings,
   - paragraphs,
   - bullets,
   - numbered lists,
   - inline code,
   - fenced code blocks,
   - tables when present,
   - links only through safe existing external-browser policy.
4. Long content must flow vertically downward. No page-level horizontal scrolling.
5. The assistant answer must be the dominant visual content.
6. Do not display raw provider stream JSON, event envelopes, UUIDs, prompt hashes, rate-limit records, escaped JSON strings, `STREAM_STDOUT`, `PROCESS_POLICY`, `SESSION_STARTED`, or similar internal records in the default conversation.
7. Do not display repeated `[REDACTED SENSITIVE OUTPUT]` records as chat messages.
8. Do not display raw tool-result payloads or giant file lists directly inside the assistant message unless the provider actually emitted them as part of the final assistant answer.
9. Preserve truthful source provenance. Never invent or synthesize an assistant answer that was not present in persisted provider evidence.
10. If the provider produced no final assistant text, show a truthful concise state such as `No final assistant response was captured` and keep technical evidence available under advanced details.

### R42: Tool activity becomes compact secondary activity

Provider tool execution must be summarized separately from the main conversation.

The default conversation may show a small compact activity summary such as:

- `Read project.godot`
- `Inspected scripts/data`
- `Inspected tests`
- `Searched project documentation`

Requirements:

1. Tool activity must be derived from persisted provider/tool evidence only.
2. Do not fabricate friendly labels when the actual tool action cannot be safely identified. Use a truthful generic label such as `Tool action completed`.
3. Collapse repetitive provider events into a small bounded activity list.
4. Do not expose raw tool JSON by default.
5. Provide a user-controlled disclosure such as `View activity` / `Tool activity` for additional human-readable action history.
6. Raw event payloads remain available only in explicit technical/advanced disclosures.

### R43: Remove the large Provider readiness panel from primary Agents UI

Remove the entire large `Provider readiness / Bounded native adapters` card from the top of the Agents page.

The user does not need to see:

- `CODEX VERSION_VERIFIED_AUTH_UNKNOWN`,
- Claude readiness diagnostic prose,
- resolver candidate diagnostics,
- version-probe explanations,
- authentication-unknown prose,

as permanent primary-page content.

Provider readiness must still be enforced before starting a session.

Allowed UX:

- provider dropdown entries may be enabled/disabled based on readiness,
- a tiny inline status icon/text may appear next to a provider only when useful,
- an unavailable provider may show a concise actionable error after selection/start,
- detailed readiness diagnostics may live inside Settings, Technical details, or a collapsible advanced area.

Do not remove backend readiness checks or weaken native executable validation.

### R44: Compact initial Agents page

On initial navigation to Agents:

1. Show the session-start form near the top without the large readiness card.
2. Show compact `Active and persisted sessions` rows beneath it.
3. Do not auto-select or auto-expand a persisted session.
4. No raw session/event content is visible until the user explicitly clicks `View`.
5. The page should fit the normal product visual language already accepted in H!veAI.

### R45: Selected session structure

When the user explicitly clicks `View`, render one selected-session area with this hierarchy:

1. Compact session header:
   - provider,
   - project,
   - state,
   - operation kind,
   - started/ended or elapsed,
   - exit code only if useful.
2. Conversation transcript:
   - User prompt,
   - Assistant response.
3. Optional compact Activity disclosure.
4. Advanced disclosures, closed by default:
   - Technical details,
   - Timeline,
   - Raw events,
   - Git evidence.
5. Close button to collapse the selected session.

Completed/failed terminal sessions must not expose meaningless Stop controls. Resume must remain capability-gated and unsupported providers must not expose an actionable Resume button.

## Provider-specific parsing requirements

### Claude

The installed native Claude Code version is currently `2.1.248 (Claude Code)` and the accepted fixed invocation includes:

`--print --output-format stream-json --verbose --no-session-persistence --permission-mode plan --restricted`

Do not change these accepted fixed args unless installed CLI evidence proves a compatibility requirement.

Implement a deterministic parser/presentation projection from persisted Claude stream evidence.

The parser must distinguish at minimum:

- assistant response text,
- tool-use/tool-result events,
- status/rate-limit/system records,
- redacted records,
- final result/terminal records.

Only actual assistant response text belongs in the main assistant message.

If Claude's stream contains multiple assistant text segments, preserve their original order and combine them without duplicating content.

### Codex

Preserve the accepted Codex adapter invocation and M13 security contract.

Implement the same conversation projection for Codex persisted output:

- extract actual agent/assistant message text,
- exclude raw item/event envelopes from the default chat,
- separate command/tool activity from the assistant answer,
- preserve provider ordering,
- never fabricate missing text.

The same visual conversation component should support both providers where practical.

## Prompt persistence / transcript truth

The chat-style transcript must show the user's original prompt text.

If current persistence stores only a prompt hash/reference and not the prompt body, do not fake it and do not recover it from logs by heuristics.

Instead, add the minimum safe persistence required for future sessions so the original bounded prompt body can be rendered in the conversation transcript.

Security constraints:

1. Preserve the 64 KiB prompt bound.
2. Store only the user's explicit session prompt, not environment variables or shell-derived content.
3. Do not place prompt content into process argv.
4. Keep stdin-bounded transport.
5. Existing historical sessions without persisted prompt body must display a truthful placeholder such as `Original prompt text was not persisted for this session`.
6. If schema migration is required, make it bounded, backward-compatible, idempotent, and covered by tests.

## Redaction requirements

Do not weaken M13B redaction guarantees.

However, the chat projection must not treat every redacted stream record as a user-visible assistant message.

- Raw evidence retains redaction markers where required.
- Primary conversation ignores redacted-only transport/event records unless redaction itself is the only meaningful terminal evidence.
- Never reconstruct or infer redacted content.

## UI design requirements

The result should feel like a focused AI conversation surface, not a database/event viewer.

Use existing H!veAI visual language. Do not copy external product branding.

Recommended structure:

```text
CLAUDE · ScrubBots · COMPLETED · 41s

You
Inspect this project and give me a detailed report.
Do not modify any files.

Claude
## Repository overview
ScrubBots is a Godot 4.7 project ...

### Architecture
- ...

Activity · 12 actions
[View activity]

[Technical details] [Timeline] [Raw events] [Git evidence]
```

Do not render one card per raw stream event.
Do not render UUID/event sequence/provider transport details in primary conversation.
Do not make the user scroll through dozens of redacted or tool-result records to find the answer.

## Security and architecture boundaries that must remain unchanged

Preserve all accepted M13/M14 boundaries:

- registered project authority,
- exact ACTIVE project requirement,
- canonical registered project cwd,
- cross-project task/session rejection,
- no arbitrary cwd from frontend,
- no arbitrary executable,
- no arbitrary process arguments,
- no arbitrary PID control,
- no shell wrappers,
- prompt absent from argv,
- bounded stdin prompt transport,
- native executable validation,
- no-visible-console process policy,
- redaction before persistence,
- bounded stream/output persistence,
- durable terminal state truth,
- owned-process stop semantics,
- restart/orphan recovery,
- provider-neutral session model,
- governed Git evidence authority,
- M14 PTY/capability boundaries,
- no M15/M21 implementation.

## Required implementation evidence

Before changing code:

1. Safely fetch `origin/H!veAI`.
2. Fast-forward only.
3. Confirm exact branch `H!veAI`.
4. Read in full:
   - M14 implementation prompt,
   - M14 implementation log,
   - M14A prompt/log/audit,
   - M14B prompt/log/strict re-audit,
   - current `agent_session_center.rs`,
   - current `agentSessionCenter.ts`,
   - current Agents page implementation and CSS,
   - current relevant frontend tests.
5. Preserve unrelated user-owned files.

## Explicit execution gates

All gates below are mandatory unless explicitly marked user acceptance.

1. PASS: fetch and fast-forward-only synchronization.
2. PASS: exact `H!veAI` branch confirmed.
3. PASS: unrelated user files preserved.
4. PASS: authoritative M14/M14A/M14B evidence read in full.
5. PASS: current Agents initial-load behavior reproduced.
6. PASS: current selected-session raw/event-heavy output reproduced.
7. PASS: current large Provider readiness card reproduced before removal.
8. PASS: backend provider readiness checks preserved.
9. PASS: large Provider readiness card removed from primary Agents page.
10. PASS: provider availability still truthfully gates start actions.
11. PASS: initial Agents page auto-selects no persisted session.
12. PASS: persisted session list remains compact.
13. PASS: explicit `View` selects exactly one session.
14. PASS: explicit close/collapse clears selected session detail.
15. PASS: user prompt renders as a distinct conversation message for new sessions.
16. PASS: historical session without stored prompt text renders truthful placeholder.
17. PASS: Claude assistant text parser test with real/representative stream-json fixture.
18. PASS: Claude rate-limit/system records excluded from primary assistant message.
19. PASS: Claude raw tool-result JSON excluded from primary assistant message.
20. PASS: Claude repeated redaction markers excluded from primary assistant message.
21. PASS: Claude multiple assistant text segments preserve order without duplication.
22. PASS: Codex assistant text parser fixture.
23. PASS: Codex event envelopes excluded from primary assistant message.
24. PASS: Codex command/tool output separated from primary assistant answer.
25. PASS: tool activity summary is bounded and evidence-derived.
26. PASS: raw tool/event payloads are hidden by default.
27. PASS: Technical details closed by default.
28. PASS: Timeline closed by default.
29. PASS: Raw events closed by default.
30. PASS: Git evidence closed by default.
31. PASS: completed sessions expose no invalid Stop action.
32. PASS: Resume remains capability-gated.
33. PASS: failed session diagnostic remains concise and visible without raw-event expansion.
34. PASS: long Markdown paragraphs wrap vertically.
35. PASS: long paths and code blocks do not create page-level horizontal scrolling.
36. PASS: Markdown headings/lists/code render safely.
37. PASS: no untrusted HTML execution from provider Markdown.
38. PASS: no unsafe direct external navigation bypassing existing browser policy.
39. PASS: prompt remains absent from provider process argv.
40. PASS: stdin prompt transport remains bounded.
41. PASS: exact ACTIVE project confinement tests.
42. PASS: cross-project task/session authorization tests.
43. PASS: redaction-before-persistence tests.
44. PASS: output/event bounds tests.
45. PASS: restart/orphan recovery regression.
46. PASS: focused Claude backend tests.
47. PASS: focused Codex backend tests.
48. PASS: full serial Rust regression with required M14 features.
49. PASS: focused Agent Session Center frontend tests.
50. PASS: full frontend suite.
51. PASS: `npm run typecheck`.
52. PASS: `npm run build`.
53. PASS: `npm audit --audit-level=high`.
54. PASS: `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`.
55. PASS: `cargo check --manifest-path src-tauri/Cargo.toml --all-targets` plus required PTY feature check.
56. PASS: `git diff --check`.
57. PASS: publisher rollback/failure harness.
58. PASS: governed production `--no-bundle` publication.
59. PASS: fresh candidate `HIVEAI_FRONTEND_READY`.
60. PASS: stable executable `HIVEAI_FRONTEND_READY` after swap.
61. PASS: stable bytes equal accepted candidate bytes by SHA-256.
62. PASS: no forbidden development listener.
63. PASS: no visible console during startup/readiness/operation/native smoke.
64. PASS: real Claude readiness/version remains valid.
65. PASS: harmless real ScrubBots Claude session reaches provider execution and exits successfully.
66. PASS: real Claude session shows a chat-style user message and meaningful assistant response without raw event noise in the default view.
67. PASS: real Claude session produces no Git status delta for the read-only acceptance prompt.
68. PASS: M15-M20 not activated.
69. PASS: M21 not started.
70. PENDING USER ACCEPTANCE: user confirms the top Provider readiness card is gone and the initial Agents page is visually clean.
71. PENDING USER ACCEPTANCE: user confirms a selected Claude/Codex session reads like a normal AI chat conversation and no raw event wall is visible by default.

## Native acceptance prompt

For the final harmless real Claude smoke, use a read-only prompt such as:

`Inspect this project read-only and give me a concise architecture summary. Do not modify, create, delete, rename, or commit any files.`

The acceptance evidence must show:

- Claude selected,
- ScrubBots selected,
- session completes successfully,
- user prompt visible as a conversation message,
- Claude's meaningful response visible as the main assistant message,
- raw events/timeline collapsed,
- no large Provider readiness card,
- no terminal/console flash,
- no Git status delta.

## Publication

Use only the governed H!veAI publisher flow already established.

Do not replace the stable executable unless candidate readiness passes.
Do not bypass rollback semantics.
Do not create an installer.
Preserve governed shortcut target/icon.

## Required immutable log

Create:

`H!veAI/docs/H!veAI/codex-logs/M14C_CHAT_STYLE_AGENT_CONVERSATION_AND_READINESS_UI_REMOVAL_REMEDIATION_LOG.md`

The log must include:

- pre/post commit IDs,
- exact files changed,
- parser/projection strategy for Claude and Codex,
- prompt persistence/schema decision,
- evidence that Provider readiness backend remains enforced while the large card is removed,
- focused/full test counts,
- real native Claude operation result,
- Git before/after evidence,
- publisher candidate/stable SHA-256,
- all 71 gate outcomes,
- any remaining user-acceptance-only gates.

## Final state

If all implementation gates pass, report:

`M14C REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

Do not declare M14 closed.
Do not activate M15.
Do not start M21.
