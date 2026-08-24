# H!veAI Correct Local Root Reset Audit

Date: 2026-08-24
Auditor: ChatGPT
Status: RESET REQUIRED BEFORE FURTHER DEVELOPMENT

## Canonical local paths

The user has now explicitly confirmed the intended local repository root is:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

The H!veAI-specific local folder is only a child workspace/foundation folder:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

The child `H!veAI` folder is NOT the Git repository root and must never be treated as the repository root unless `git rev-parse --show-toplevel` proves otherwise.

## Consequence

Earlier Codex M00/M01 reports are no longer accepted as authoritative because Codex appears to have inspected or operated on a different codebase / wrong working context. Those milestone results are therefore marked:

`SUPERSEDED / INVALID BASELINE`

They must not be used as evidence that M00 or M01 is complete for the intended H!veAI transformation of `Sekiph82/AI-Commerce-HQ`.

Do not delete the earlier work. Preserve it for forensic/reference purposes, but do not build future milestones on top of it until the correct local repository root has been re-audited.

## Canonical product naming

The product name is exactly:

`H!veAI`

The second character is an exclamation mark.

User-visible naming, documentation titles, milestone names, release names, window titles and product branding must use `H!veAI`.

Technical slugs may remain `hiveai` where punctuation is unsafe or unsupported, for example:

- branch names
- package IDs
- folder slugs such as `docs/hiveai/`
- application identifiers

## Required reset

Before any further milestone:

1. Start Codex from the exact canonical local repository root.
2. Run `git rev-parse --show-toplevel` and require exact match with the canonical path.
3. Inspect `git remote -v`, branch, HEAD, status, tree and representative files.
4. Compare the local repository to `https://github.com/Sekiph82/AI-Commerce-HQ`.
5. Do not read sibling folders as project source unless explicitly instructed.
6. Treat the child `H!veAI` folder only as source material for H!veAI foundation documents.
7. Re-run M00 from scratch against the correct repository root.
8. Only after a clean M00 audit may M01 be attempted again.

## Verdict

`M00 MUST BE RE-RUN FROM THE CORRECT LOCAL REPOSITORY ROOT.`

`M01 AND M02 ARE BLOCKED UNTIL THE RESET M00 PASSES.`
