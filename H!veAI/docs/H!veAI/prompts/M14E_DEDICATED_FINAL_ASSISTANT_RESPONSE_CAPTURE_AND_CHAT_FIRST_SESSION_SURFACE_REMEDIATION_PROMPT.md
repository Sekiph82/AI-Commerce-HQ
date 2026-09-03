# M14E Dedicated Final Assistant Response Capture and Chat-First Session Surface Remediation Prompt

Date: 2026-09-03
Repository: Sekiph82/AI-Commerce-HQ
Branch: H!veAI
Scope: M14 only
Status on entry: M14 open, M14D technically passed, user native acceptance failed because the real final Claude response is still not visible.

## Mission

Fix the remaining M14 native acceptance failure by making the user-facing Agent Session Center behave like a real Claude/ChatGPT conversation surface.

The user does NOT want raw provider stdout, token-estimation chatter, tool envelopes, sequence IDs, transport events, or intermediate assistant planning text to be the primary answer.

The primary result of an owned Codex or Claude session must be:

1. the exact user prompt,
2. the provider's final human-readable assistant response,
3. optional compact activity summary,
4. technical evidence only behind closed advanced disclosures.

A session MUST NOT be considered user-acceptance complete merely because the process exits 0 or the generic stdout capture contains some assistant text.

## Native failure evidence that must be treated as authoritative

The latest native ScrubBots Claude operation completed with exit code 0, but the visible conversation showed only:

`I'll explore the repository's top-level structure and key files directly.`

followed by:

`[bounded output truncated]`

The actual requested repository summary was not visible.

The Timeline showed many Claude stream-json records before completion, including token estimates, rate-limit metadata, system/provider metadata, tool results, directory/file listings, intermediate assistant records, and other transport evidence.

The current backend still uses a shared bounded stdout capture:

- `MAX_OUTPUT_BYTES = 64 * 1024`
- `MAX_OUTPUT_EVENTS = 128`

for the provider stream. That generic bounded capture can fill before the provider's final answer arrives.

Therefore the current architecture conflates two different responsibilities:

- bounded diagnostic / audit transport capture,
- canonical user-facing final assistant response.

M14E must separate them.

## Findings to close

### M14-R50 MAJOR: final assistant answer is not independently captured

The user-facing final answer depends on the same generic stdout/event budget used for verbose provider transport. A long tool-heavy Claude run can exhaust the generic capture before final response text arrives.

### M14-R51 MAJOR: intermediate assistant text is mistaken for the final answer

The current frontend can surface the first or any captured assistant text such as `I'll explore...` even when that is only an intermediate progress/planning message.

### M14-R52 MAJOR: `COMPLETED` can coexist with missing user-visible final answer

A process exit code of 0 plus generic provider evidence is insufficient. The app must distinguish transport success from final-response availability.

### M14-R53 MAJOR: normal UX still exposes transport noise too prominently

Timeline/raw event content can still dominate the visual surface when expanded. Primary normal use must remain chat-first. Advanced evidence may exist, but it must never be required to find the provider's final answer.

## Non-negotiable product behavior

### Primary conversation surface

For both Claude and Codex, the visible result must look conceptually like:

```text
You
Inspect this project read-only and summarize its repository structure. Do not modify any files.

Claude
ScrubBots is a Godot 4.7 project organized around...

## Repository structure
...

## Architecture
...

## Tests
...
```

The final provider answer must be the dominant content.

The user must never need to open Timeline, Raw events, Git evidence, or Technical details to discover what Claude/Codex answered.

### Chat-first hierarchy

Current conversation must display, in order:

- provider + session status summary,
- `You` prompt,
- final assistant answer,
- optional compact activity disclosure,
- optional technical disclosures.

Do not put raw stream text between the prompt and final answer.

### Intermediate provider messages

Intermediate assistant text such as:

- `I'll inspect...`
- `I'll explore...`
- `Let me check...`

must not be presented as the canonical final result.

They may optionally appear in a compact activity/progress surface while RUNNING, but after completion the canonical response must be the actual final provider answer.

## Required architecture

### 1. Dedicated semantic provider parsing

Do not continue treating Claude/Codex stdout as an opaque blob for the primary conversation.

Parse provider records semantically after the existing pre-persistence sanitizer.

For Claude stream-json, classify records at minimum into:

- assistant intermediate/progress text,
- final assistant/result text,
- tool use,
- tool result,
- system/provider metadata,
- rate-limit/usage/token telemetry,
- diagnostic/error.

For Codex structured output, implement equivalent semantic classification from the actual accepted Codex output shapes already used by the adapter.

Do not invent undocumented provider schemas. Use observed/fixture/native evidence and provider output already available in the repository.

### 2. Dedicated final assistant response storage

Introduce a durable final-response channel independent from the generic 64 KiB/128-event stdout capture.

Preferred approaches include either:

- dedicated columns on `agent_sessions`, or
- a dedicated normalized `agent_messages` / `agent_session_messages` table.

Choose the smallest safe schema that supports:

- user prompt,
- provider final assistant text,
- optional provider message role/type metadata,
- timestamps,
- provider/session identity,
- bounded size,
- migration from the current schema.

The final assistant response MUST NOT share the generic stdout byte/event budget.

### 3. Independent bounded final-response budget

Define a dedicated bound for final user-facing assistant text.

The bound must be large enough for realistic agent answers and independent from diagnostic stdout.

Requirements:

- explicit constant,
- explicit truncation semantics,
- no silent loss,
- UTF-8 safe truncation,
- no secret leakage,
- no dependence on the generic `MAX_OUTPUT_BYTES` or `MAX_OUTPUT_EVENTS`.

If a provider final answer itself exceeds this dedicated bound, the UI must truthfully show a final-response-specific truncation indicator. Do not reuse generic `[bounded output truncated]` as though the entire provider stream were the answer.

### 4. Final-response authority rules

For Claude, determine the authoritative final answer from the actual terminal/final result semantics of the installed Claude Code stream-json output.

For Codex, determine the authoritative final assistant response from the actual accepted Codex structured output semantics.

Do not choose "first assistant record".

Do not choose "last arbitrary stdout line".

Do not infer finality from elapsed time.

Use provider-semantic terminal/result evidence.

### 5. Completion state contract

Separate process lifecycle completion from answer availability.

A terminal session may have:

- process completed successfully,
- final response captured successfully,
- final response unavailable,
- final response truncated,
- final response persistence degraded.

The canonical session model must truthfully expose these distinctions.

For a successful user-facing completion, require durable final assistant text unless the operation type explicitly has no assistant response by design. Current FREEFORM/TASK Agent operations do require one.

If process exit is 0 but no final assistant response was durably captured, surface a bounded diagnostic such as:

`CLAUDE_FINAL_RESPONSE_UNAVAILABLE`

or

`CODEX_FINAL_RESPONSE_UNAVAILABLE`

and do not silently present the session as a normal completed chat answer.

### 6. Persistence-before-UI truth

The final response shown after reload must come from durable state, not only in-memory stream state.

Prove this exact chain:

`provider final record -> sanitize -> semantic parse -> dedicated final-response persist -> SQLite reload -> frontend Current conversation`

for both Claude and Codex representative fixtures.

### 7. Preserve existing generic transport evidence

Do NOT remove the existing bounded raw/technical evidence.

Timeline, raw events, tool activity, Git evidence, diagnostics, process policy, and transport captures remain useful audit truth.

They must simply be secondary.

The generic stdout capture may continue to truncate at its accepted bounds without affecting the final assistant response.

### 8. Frontend conversation projection

The frontend must consume the dedicated final-response field/message as the canonical assistant answer.

Do not reconstruct the primary answer from generic stdout when dedicated final response exists.

Historical sessions without the new field may use a clearly bounded compatibility projection if safe, but no migration may fabricate a final answer.

For historical sessions where no reliable final answer exists, say that clearly.

### 9. Current conversation UX

Preserve the accepted M14D requested form order:

1. Project
2. Task ID
3. Prompt
4. Provider
5. Start session

Keep Current conversation directly beneath the form.

When a newly started session is RUNNING:

- show the user prompt immediately,
- show restrained progress/activity,
- do not dump raw JSON.

When the session completes:

- replace progress emphasis with the final assistant answer,
- render Markdown safely,
- preserve headings, lists, code blocks, and paragraphs,
- vertical reading only,
- no page-level horizontal scroll.

### 10. Activity and technical evidence

Use separate closed disclosures such as:

- View activity
- Technical details
- Timeline
- Raw events
- Git evidence

All must be closed by default after completion.

Normal users should never need these to understand the result.

## Security and architecture invariants that MUST remain unchanged

Preserve every accepted M13/M13A/M13B/M13C/M13D/M13E/M14/M14A/M14B/M14C/M14D boundary, including:

- ACTIVE registered project confinement,
- cross-project task/session authorization,
- prompt absent from argv,
- bounded stdin prompt transport,
- no arbitrary executable selection,
- no arbitrary args,
- no arbitrary PID control,
- no shell primitive exposure,
- native direct executable resolution,
- Windows no-visible-console behavior,
- shared selective pre-persistence sanitization,
- durable event truth,
- process ownership,
- stop/recovery/orphan semantics,
- no generic permission/approval shell surface,
- M15 not activated,
- M21 not started.

Do not weaken redaction to rescue final-response capture.

Final assistant text must still pass through the shared selective sanitizer before persistence.

## Required database work

If schema migration is required:

- use the existing migration/versioning mechanism,
- preserve all existing user sessions,
- make migration idempotent under the project's migration model,
- prove old databases open successfully,
- prove new sessions persist final answers,
- prove historical rows with no final answer remain readable,
- do not destructively rewrite old raw events.

## Required backend tests

Add focused tests that prove at least all of the following:

### Claude

1. many telemetry/tool records can exceed the generic stdout capture budget while a later final Claude answer still persists completely within the dedicated final-response bound.
2. first intermediate assistant text is NOT selected as final.
3. terminal/final Claude result text IS selected as final.
4. sanitized credential values never survive into the final answer storage.
5. ordinary words such as `token`, `secret.txt`, token counters, rate limits, and model metadata do not erase the answer.
6. final answer survives SQLite reload.
7. generic stdout may be `truncated=true` while final response remains available.
8. exit 0 with no final response produces the truthful final-response-unavailable diagnostic.

### Codex

9. representative Codex intermediate/progress records are not selected as final.
10. canonical terminal/final Codex assistant output is selected.
11. generic stream truncation does not erase the dedicated final response.
12. final answer survives SQLite reload.
13. secret sanitization still applies before final-response persistence.
14. exit 0 with no canonical final answer produces a truthful diagnostic.

### Shared persistence

15. final response persistence failure cannot be silently reported as normal success.
16. historical sessions without dedicated final response remain loadable.
17. schema migration leaves prior M14 session/event rows intact.

## Required frontend tests

Add focused tests proving:

1. Current conversation shows the exact user prompt.
2. Current conversation shows the dedicated final Claude answer.
3. Current conversation shows the dedicated final Codex answer.
4. intermediate assistant messages are not rendered as the final answer after completion.
5. generic `[bounded output truncated]` does not replace a valid dedicated final answer.
6. transport sequence IDs are absent from the primary conversation.
7. rate-limit/token-estimation records are absent from primary conversation.
8. raw provider JSON is absent from primary conversation.
9. technical evidence disclosures are closed by default.
10. Markdown answer renders vertically and safely.
11. very long final Markdown wraps without horizontal page scrolling.
12. historical session without reliable final answer displays an explicit compatibility/unavailable state, not fabricated text.
13. a RUNNING session can show restrained progress without treating progress text as final.

## Required real native acceptance exercise before publication

Use the real installed Claude Code with registered ScrubBots and this exact harmless prompt:

`Inspect this project read-only and summarize its repository structure. Do not modify any files.`

The builder must prove all of the following before declaring M14E implementation complete:

- operation starts from H!veAI,
- no terminal/console flash,
- process exits 0,
- no ScrubBots Git delta,
- generic transport may contain substantial tool/telemetry traffic,
- final Claude answer is non-empty and meaningfully longer than the intermediate `I'll explore...` style progress line,
- final answer is persisted in the dedicated final-response storage,
- app is closed/reloaded or equivalent durable reload is performed,
- the same final answer remains available after reload,
- Current conversation visibly shows the final Claude answer without opening Timeline/Raw events,
- no raw JSON wall appears in normal use.

Do not count the exercise as PASS if the only visible Claude text is an intermediate planning/progress sentence.

## Required Codex acceptance fixture

Run a bounded representative Codex operation/fixture proving the same semantic final-response projection path.

A real user-repo write operation is not required. Use a disposable registered fixture where appropriate.

## Explicit execution gates

1. PASS: fetch origin and fast-forward only; no merge commits.
2. PASS: confirm exact `H!veAI` branch.
3. PASS: preserve unrelated user-owned parent files.
4. PASS: read M14D audit, prompt, log, and current source before changes.
5. PASS: reproduce generic stdout truncation causing final Claude answer loss.
6. PASS: prove current first/intermediate assistant selection bug.
7. PASS: inspect real installed Claude stream-json final/terminal record semantics.
8. PASS: inspect accepted Codex final structured-output semantics.
9. PASS: document semantic classification rules before implementation.
10. PASS: introduce dedicated final-response persistence independent of generic stdout cap.
11. PASS: dedicated final-response size bound is explicit.
12. PASS: dedicated truncation marker/status is explicit and truthful.
13. PASS: final-response truncation is UTF-8 safe.
14. PASS: shared sanitizer runs before final-response persistence.
15. PASS: credential values remain absent from final-response storage.
16. PASS: token counters remain allowed.
17. PASS: rate-limit metadata remains allowed in technical evidence.
18. PASS: ordinary natural-language `token` remains allowed.
19. PASS: ordinary filename/prose containing `secret` remains allowed unless it is an actual credential field/value.
20. PASS: Claude intermediate assistant record is classified as non-final.
21. PASS: Claude canonical terminal/final result is classified as final.
22. PASS: Codex intermediate/progress record is classified as non-final.
23. PASS: Codex canonical terminal/final result is classified as final.
24. PASS: generic stdout 64 KiB truncation can occur without losing final Claude answer.
25. PASS: generic stdout event cap can occur without losing final Claude answer.
26. PASS: generic stdout truncation can occur without losing final Codex answer.
27. PASS: final answer survives dedicated persistence and SQLite reload.
28. PASS: final-response persistence error cannot be silently swallowed.
29. PASS: exit 0 + missing Claude final answer yields truthful diagnostic.
30. PASS: exit 0 + missing Codex final answer yields truthful diagnostic.
31. PASS: historical session rows remain readable.
32. PASS: schema migration preserves old agent events.
33. PASS: migration regression from existing user DB shape passes.
34. PASS: Current conversation uses dedicated final response as canonical answer.
35. PASS: user prompt is displayed above the answer.
36. PASS: Claude label is displayed for Claude answer.
37. PASS: Codex label is displayed for Codex answer.
38. PASS: intermediate `I'll explore...` style text is not final after completion.
39. PASS: valid dedicated answer suppresses generic `[bounded output truncated]` from the primary chat.
40. PASS: token-estimate records absent from primary chat.
41. PASS: rate-limit events absent from primary chat.
42. PASS: tool JSON absent from primary chat.
43. PASS: sequence IDs absent from primary chat.
44. PASS: raw stream envelopes absent from primary chat.
45. PASS: activity disclosure closed by default.
46. PASS: Technical details closed by default.
47. PASS: Timeline closed by default.
48. PASS: Raw events closed by default.
49. PASS: Git evidence closed by default.
50. PASS: primary final answer does not require opening any disclosure.
51. PASS: Markdown headings render.
52. PASS: Markdown lists render.
53. PASS: Markdown code blocks render safely.
54. PASS: untrusted HTML is not executed.
55. PASS: long final answer wraps vertically.
56. PASS: no page-level horizontal scroll from final answer.
57. PASS: Project -> Task ID -> Prompt -> Provider -> Start order preserved.
58. PASS: Current conversation remains directly below form.
59. PASS: history remains below Current conversation.
60. PASS: no historical session auto-opens on Agents navigation.
61. PASS: newly started session is the only auto-focused session.
62. PASS: ACTIVE project confinement tests pass.
63. PASS: cross-project task authorization tests pass.
64. PASS: cross-project session authorization tests pass.
65. PASS: prompt absent from argv.
66. PASS: stdin prompt remains bounded.
67. PASS: arbitrary executable control remains rejected.
68. PASS: arbitrary args remain rejected.
69. PASS: arbitrary PID remains rejected.
70. PASS: shell control remains rejected.
71. PASS: no-visible-console tests pass.
72. PASS: stop/recovery/orphan lifecycle tests pass.
73. PASS: shared redaction regression tests pass.
74. PASS: Claude focused backend tests pass.
75. PASS: Codex focused backend tests pass.
76. PASS: persistence/migration focused tests pass.
77. PASS: full serial Rust regression passes.
78. PASS: focused Agent Session Center frontend tests pass.
79. PASS: full frontend suite passes.
80. PASS: `npm run typecheck` passes.
81. PASS: `npm run build` passes.
82. PASS: `npm audit --audit-level=high` passes.
83. PASS: Rust fmt check passes.
84. PASS: all-target Rust check passes.
85. PASS: `pty-support` Rust check passes.
86. PASS: `git diff --check` passes.
87. PASS: publisher rollback/failure harness passes 9/9.
88. PASS: governed production no-bundle publication passes.
89. PASS: fresh candidate `HIVEAI_FRONTEND_READY` passes.
90. PASS: stable `HIVEAI_FRONTEND_READY` after swap passes.
91. PASS: candidate/stable SHA-256 equality passes.
92. PASS: no forbidden development listener.
93. PASS: stable native no-visible-console smoke passes.
94. PASS: real ScrubBots Claude operation exits 0 with no Git delta.
95. PASS: real ScrubBots Claude final answer is meaningfully complete, not only an intermediate planning sentence.
96. PASS: real Claude final answer persists in dedicated storage.
97. PASS: real Claude final answer survives application/database reload.
98. PASS: real Claude final answer is visible in Current conversation without advanced disclosures.
99. PASS: representative Codex final answer follows the same dedicated persistence/projection contract.
100. PASS: generic transport can truncate while dedicated final answer remains visible.
101. PASS: normal primary chat contains no raw JSON wall.
102. PASS: M15-M20 remain inactive.
103. PASS: M21 remains not started.
104. USER ACCEPTANCE PENDING: user confirms Claude's full final answer is visible in the native Current conversation area.
105. USER ACCEPTANCE PENDING: user confirms normal use shows prompt + final answer first, with technical evidence secondary.

## Publication requirements

After all implementation/test gates above pass:

1. run the governed publisher,
2. confirm stable `dev-bin\H!veAI.exe` contains M14E,
3. prove candidate/stable SHA equality,
4. verify startup/video/icon/audio remain accepted,
5. verify no terminal flashes,
6. remove no evidence required by previous accepted milestones,
7. keep M14 open pending independent strict re-audit + user native acceptance.

## Required immutable remediation log

Create:

`H!veAI/docs/H!veAI/codex-logs/M14E_DEDICATED_FINAL_ASSISTANT_RESPONSE_CAPTURE_AND_CHAT_FIRST_SESSION_SURFACE_REMEDIATION_LOG.md`

The log must include:

- exact pre-remediation reproduction,
- exact root cause,
- observed Claude/Codex semantic final-record rules,
- schema/migration changes,
- dedicated final-response bound,
- sanitize -> semantic parse -> persist -> reload -> frontend proof,
- real ScrubBots Claude native evidence,
- generic stdout truncation evidence coexisting with preserved final response,
- all test counts,
- publisher evidence,
- stable SHA,
- explicit 1-105 gate ledger,
- implementation commit SHA(s),
- final statement that M14 remains open pending independent strict re-audit + user acceptance.

## Commit and push

Commit all scoped M14E changes and the immutable log to `H!veAI`, push to `origin/H!veAI`, and leave the working tree clean except for pre-existing unrelated user-owned files that were never part of this repository scope.

Do NOT activate M15.
Do NOT start M21.
Do NOT close M14 yourself.
