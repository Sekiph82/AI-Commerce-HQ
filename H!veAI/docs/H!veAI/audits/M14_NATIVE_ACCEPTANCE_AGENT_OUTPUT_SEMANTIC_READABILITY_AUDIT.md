# M14 Native Acceptance Agent Output Semantic Readability Audit

Date: 2026-09-02
Product: H!veAI
Branch: `H!veAI`
Status: **FAIL / M14 remains open**

## Native evidence

The user tested the governed stable H!veAI build after M14B and provided native screenshots.

Positive acceptance evidence:

- Agents opens with a compact persisted-session list rather than auto-expanding the oldest/latest session.
- ScrubBots defaults to Claude.
- Claude readiness is visible and the real Claude session reaches `COMPLETED` with exit code `0`.
- The selected session has collapsible `Technical details`, `Timeline`, `Raw events`, and `Git evidence` sections.
- The user-visible session list is materially cleaner than the pre-M14B surface.

## Finding M14-R41 — MAJOR

**The default Agent Output is still not semantically human-readable.**

Native screenshots show that the primary `AGENT OUTPUT` surface still promotes transport/tool-stream artifacts as if they were user-facing agent content. Examples visible in the accepted stable application include:

- `rate_limit_event`
- `[REDACTED SENSITIVE OUTPUT]` records repeated as primary output
- serialized Claude `user/message/tool_result` JSON payloads
- long escaped file lists and tool-result payloads
- full source/config file dumps
- repeated generic `OUTPUT` cards that do not distinguish assistant response, tool activity, and raw evidence

The user must still scroll through a large amount of machine-oriented material before reaching anything resembling a concise agent response. This does not satisfy the M14B requirement that raw protocol/event material remain advanced evidence rather than the default reading experience.

### Important distinction

Raw JSON inside explicitly opened `Timeline` or `Raw events` disclosures is acceptable. Those sections are technical evidence surfaces and may remain verbose.

The defect is specifically that the **primary Agent Output** still includes transport/protocol/tool-result records and large raw dumps.

## Required remediation behavior

The primary reader must be semantic, provider-aware, and compact:

1. Show the final assistant response prominently when one exists.
2. Show assistant text/thinking summaries only when safe and user-facing.
3. Represent tool activity as compact summaries such as `Read project.godot`, `Listed 16 files under scripts/data`, or `Inspected tests/run_tests.gd`, not raw provider JSON.
4. Do not surface `rate_limit_event`, `system/init`, `thinking_tokens`, raw stream envelopes, UUIDs, session IDs, tool-use IDs, prompt hashes, or provider transport metadata in primary output.
5. Do not show `[REDACTED SENSITIVE OUTPUT]` as repeated primary cards. If redaction removed user-facing content, show at most one compact notice such as `Some provider output was hidden for safety.`
6. Large file contents/tool results must be collapsed behind an explicit `View tool output` / `View file content` disclosure.
7. The primary reader must not become a raw source-code/file browser by default.
8. `Timeline`, `Raw events`, `Technical details`, and `Git evidence` remain collapsed by default and retain full bounded evidence.
9. Codex and Claude must both use the same semantic presentation contract, with provider-specific parsers only where necessary.
10. No stored evidence may be deleted or weakened merely to simplify the UI.

## Acceptance target

For the same harmless ScrubBots Claude inspection operation, the selected session should fit into a compact vertical flow:

- provider/state/project/timing metadata
- one concise `Agent response` section
- a short `Activity` list of meaningful tool actions
- optional collapsed tool outputs
- collapsed technical evidence sections

The user should be able to understand what Claude concluded without reading JSON, escaped transport records, redaction spam, hashes, or raw full-file dumps.

## Verdict

- M14B technical implementation: previously PASS on strict evidence.
- Claude real operation: PASS.
- Persisted-session initial compactness: PASS.
- Primary selected-session readability: **FAIL**.

`M14-R41` is opened as a MAJOR native UX finding.

M14 must remain open. M15 must not be activated. M21 must not be started.
