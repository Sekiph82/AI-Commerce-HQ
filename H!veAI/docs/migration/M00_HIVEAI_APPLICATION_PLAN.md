# M00 H!veAI Application Plan

Date: 2026-08-24

## Canonical Layout

Git root:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Application root:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

This monorepo-style separation is intentional. The parent repository preserves
the old AI-Commerce-HQ source material and Git history. The child `H!veAI`
folder owns the new product workspace, docs, prompts, logs, tests, and future
application code.

## M00 Workspace

The child root contains:

- `README.md`
- `AGENTS.md`
- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`
- `package.json`
- `docs/H!veAI/`
- `docs/migration/`
- `src/`
- `src-tauri/`
- `tests/`

The `src`, `src-tauri`, and `tests` directories are placeholders only in M00.
They exist to reserve the application layout. Product implementation starts in
M01 and later milestones.

## Control Documents

Canonical H!veAI prompts, audits, and Codex logs now live under:

- `H!veAI/docs/H!veAI/prompts/`
- `H!veAI/docs/H!veAI/audits/`
- `H!veAI/docs/H!veAI/codex-logs/`

The branch-level originals remain in place for compatibility and history.

## Next Milestone

The exact next milestone is `M01 - Tauri 2 Foundation`.

Do not begin M01 until M00 is independently audited and approved.
