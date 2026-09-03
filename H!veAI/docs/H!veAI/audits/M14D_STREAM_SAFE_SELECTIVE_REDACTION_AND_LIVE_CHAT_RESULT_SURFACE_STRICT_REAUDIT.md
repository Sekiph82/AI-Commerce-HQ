# M14D Stream-Safe Selective Redaction and Live Chat Result Surface Strict Re-Audit

Date: 2026-09-03
Branch: `H!veAI`
Scope: M14D only
Implementation commits reviewed: `7e06ac952b2c7b84f336145dce967703e48c2844`, `402643bb7837c249cef95a415589ac15bbbeb651`
Builder log reviewed: `H!veAI/docs/H!veAI/codex-logs/M14D_STREAM_SAFE_SELECTIVE_REDACTION_AND_LIVE_CHAT_RESULT_SURFACE_REMEDIATION_LOG.md`

## Verdict

**PASS / PENDING USER NATIVE-VISUAL ACCEPTANCE**

No open BLOCKER, MAJOR, or MINOR finding was found in the reviewed M14D technical scope. M14 remains open until the user confirms the real native Claude answer is visible in the Current conversation surface and that ordinary use no longer exposes a redaction/event wall.

## Findings

### R46 — Destructive whole-record redaction

**CLOSED.** The old provider-local redactors were removed and both Claude and Codex now use the shared `stream_sanitizer` contract. Valid JSON records are parsed, recursively sanitized by exact normalized sensitive keys, and reserialized before capture and durable persistence. Ordinary `input_tokens`, `output_tokens`, `thinking_tokens`, rate-limit metadata, model metadata, ordinary prose containing `token`, and filenames such as `secret.txt` are not whole-record redaction triggers.

Sensitive values under bounded credential-bearing keys remain replaced with `[REDACTED SENSITIVE VALUE]`, including nested values. Plain-text assignment and Bearer / `sk-` markers remain selectively protected rather than deleting the entire provider record.

### R47 — Long provider record destruction

**CLOSED.** The previous 4096-byte destructive carry behavior was replaced by an explicit 256 KiB provider-record bound. Normal large records survive. Records exceeding the hard bound retain a bounded prefix and append `[PROVIDER RECORD TRUNCATED]` instead of silently discarding the assistant record.

### R48 — Successful terminal state without durable assistant evidence

**CLOSED.** Provider success now has an assistant/result-evidence guard. A zero exit without meaningful durable provider answer evidence receives provider-specific diagnostics instead of silently presenting a normal answerless success. M14D also adds SQLite sanitize→persist→reload proof for Claude assistant text.

### R49 — Primary chat/result surface and requested form layout

**CLOSED TECHNICALLY.** The implementation changes the start-form order to Project → Task ID → Prompt → Provider → Start. A dedicated `Current conversation` surface appears before session history, auto-focuses only a newly started or explicitly viewed session, and keeps raw activity/technical evidence behind advanced disclosures. Repeated redaction-only transport markers are explicitly excluded from the primary conversation projection.

## Source review notes

The shared sanitizer defines a 256 KiB per-record bound, exact sensitive-key normalization, JSON field-level sanitization, selective plain-text assignment masking, split-read protection, and meaningful provider-output detection. The reviewed tests include preservation of assistant text beside token counters, credential removal, large-record survival, and explicit truncation evidence.

The frontend focused test was strengthened after implementation to include repeated `[REDACTED SENSITIVE VALUE]` transport records and asserts that they do not appear in the primary reader.

## Test and publication evidence reviewed

Builder evidence reports:

- frontend full suite: `108 passed, 0 failed`
- Agent Session Center focused frontend: `7 passed`
- Codex focused frontend: `6 passed`
- serial Rust library regression: `327 passed, 0 failed, 0 ignored`
- sanitizer/provider targeted tests: PASS
- `npm run typecheck`: PASS
- `npm run build`: PASS
- `npm audit --audit-level=high`: PASS, `0 vulnerabilities`
- cargo format/check + `pty-support`: PASS
- `git diff --check`: PASS
- publisher rollback harness: `9/9` PASS
- governed production publication: PASS
- candidate/stable SHA-256: `CA98036D58151895CAFFAAE3EC8A2F6DE5DC84D7660850B86146BF4230DD21C7`
- no visible console and no forbidden development listener in native smoke
- harmless real ScrubBots Claude operation: exit `0`, assistant/result evidence present, extracted answer non-empty, no project Git delta

## Final status

**BLOCKER: 0**
**MAJOR: 0**
**MINOR: 0**

**M14D technical status: PASS**

Remaining acceptance gates:

1. User confirms the real Claude answer is visible in the native `Current conversation` surface.
2. User confirms normal use does not show a redaction/event wall and the Project → Task ID → Prompt → Provider layout is visually correct.

Do not close M14 until both user-native gates pass. Do not activate M15 or start M21.
