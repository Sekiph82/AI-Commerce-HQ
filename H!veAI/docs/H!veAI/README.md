# H!veAI AI Development Protocol

This directory is the canonical, version-controlled operational record for H!veAI development.

## Canonical product name

The product name is **H!veAI**.

The second character is an exclamation mark.

Do not use `HiveAI`, `Hive AI`, `HIVEAI`, or similar variants in user-visible product naming.

Technical identifiers may use lowercase ASCII-safe forms such as `hiveai` only where punctuation or case is unsafe or unsupported, for example package IDs, app identifiers, environment variables, or internal slugs.

## Canonical repository

GitHub repository:
`Sekiph82/AI-Commerce-HQ`

Canonical development branch:
`H!veAI`

Canonical local repository root:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

The child folder:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

is NOT the Git repository root. It may contain reference/foundation files only.

## Directory structure

- `docs/H!veAI/prompts/` — authoritative prompts authored by ChatGPT for Codex/Claude.
- `docs/H!veAI/audits/` — independent ChatGPT audits.
- `docs/H!veAI/codex-logs/` — Codex chronological milestone logs.

## Mandatory workflow

1. ChatGPT audits the current milestone/state.
2. ChatGPT writes the next authoritative prompt under `docs/H!veAI/prompts/`.
3. Codex reads that prompt from the `H!veAI` branch before working.
4. Codex creates or continues the matching file under `docs/H!veAI/codex-logs/` before making milestone changes.
5. Codex records commands, decisions, failures, fixes, tests, commits, and push status chronologically.
6. Codex never erases prior failures after fixing them.
7. Codex marks a milestone complete only after acceptance criteria are verified.
8. ChatGPT independently audits the result and saves that audit under `docs/H!veAI/audits/`.
9. Only after audit approval is the next milestone prompt activated.

## Safety

- Never commit secrets, tokens, private keys, `.env` contents, local databases, or credential-bearing dumps.
- Never force-push unless the owner explicitly instructs it for a specific reason.
- Never rewrite history silently.
- Never treat the child `H!veAI` folder as the repository root.
- Never proceed if `git rev-parse --show-toplevel` is not exactly the canonical local root above.
