# H!veAI AI/Codex Operations Protocol

This directory is the canonical, version-controlled operational record for AI-assisted H!veAI development.

## Product naming rule

The canonical product name is **H!veAI**.

- User-visible product copy, documentation headings, reports, prompts, UI labels, release notes, and branding must use exactly `H!veAI`.
- `HiveAI`, `Hive AI`, `HIVEAI`, or other display-name variants are not the product name unless quoted as historical/error text.
- Technical slugs and identifiers that cannot safely contain `!` may use lowercase `hiveai`, for example branch names, package identifiers, directory names such as `docs/hiveai/`, database identifiers, and Rust/TypeScript module names.
- The exclamation mark is the second character of the display name: `H!veAI`.

## Directory structure

- `docs/hiveai/prompts/` — prompts authored by ChatGPT for Codex/Claude or other builders.
- `docs/hiveai/audits/` — independent ChatGPT audits of milestone results, repository state, architecture, and agent output.
- `docs/hiveai/codex-logs/` — Codex chronological milestone logs.

## Naming

Prompts:
- `MXX_<SHORT_NAME>_PROMPT.md`
- intermediate repair prompts may use `MXX_5_<SHORT_NAME>_PROMPT.md`

Audits:
- `MXX_<SHORT_NAME>_AUDIT.md`
- cross-milestone audits may use a date prefix.

Codex logs:
- `MXX_<SHORT_NAME>_CODEX_LOG.md`

## Mandatory workflow

1. ChatGPT writes the next authoritative prompt under `docs/hiveai/prompts/`.
2. Codex reads that prompt from this repository before working.
3. Codex creates or continues the matching file under `docs/hiveai/codex-logs/` at the start of the milestone.
4. Codex appends chronological checkpoints, commands, failures, fixes, tests, commits, and push status to that log.
5. Codex does not erase earlier failures after fixing them.
6. Codex updates the same log across repeated sessions for the same milestone.
7. At milestone end, Codex marks the log `PHASE STATUS: COMPLETE` only when acceptance criteria are actually verified.
8. ChatGPT independently audits the milestone and saves the audit under `docs/hiveai/audits/`.
9. Only after audit approval should the next milestone prompt become active.

## Source-of-truth rule

A chat response alone is not a durable project record. Important prompts, audits, and Codex milestone logs must exist in this repository.

## Safety

- Never commit secrets, tokens, private keys, local databases, or raw credential-bearing environment dumps into logs.
- Redact sensitive values when necessary.
- Do not rewrite old logs to make history cleaner.
- Corrections are appended as new dated entries.
