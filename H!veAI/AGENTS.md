# H!veAI Codex Instructions

## Product Name

The product name is H!veAI. The second character is `!`.

Use `hiveai` only for technical identifiers where punctuation is unsafe.

## Roots

Run Git commands from:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Put new H!veAI application code, product docs, prompts, audits, Codex logs,
tests, desktop shell files, and application configuration under:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

Do not treat `H!veAI` as a separate Git repository. It must not contain a `.git`
directory.

## Mandatory Fetch-Before-Prompt Preflight

Before reading milestone prompt files:

```powershell
git fetch origin H!veAI
```

Then compare:

```powershell
git rev-list --left-right --count HEAD...origin/H!veAI
```

If local HEAD is behind `origin/H!veAI` and there are no conflicting local
tracked changes:

```powershell
git merge --ff-only origin/H!veAI
```

Then read the authoritative audit and milestone prompt from the updated local
checkout.

Never assume missing local prompt/audit files are absent from GitHub before
fetching.

Do not use reset, force-push, destructive checkout, or automatic rebase to make
this synchronization succeed. If a fast-forward cannot be performed safely,
stop and report the exact divergence or conflicting tracked changes.

## Session Start

At the start of each milestone:

1. Run the mandatory fetch-before-prompt preflight above.
2. Read `H!veAI/AGENTS.md`, `H!veAI/CONSTITUTION.md`, `H!veAI/ARCHITECTURE.md`,
   and `H!veAI/TASKS.md` from the synchronized checkout.
3. Inspect branch, HEAD, remotes, status, tags, and worktrees from the Git root.
4. Read the authoritative prior milestone audit and current milestone prompt.
5. Create or continue the milestone Codex log under
   `H!veAI/docs/H!veAI/codex-logs/`.
6. Work only on the active milestone.

## Safety

- Do not overwrite user changes.
- Do not force-push.
- Do not commit secrets, `.env`, local databases, caches, build output, or
  user-specific runtime data.
- Do not run inherited commerce, trading, social media, marketplace, or
  publishing operations from the old parent application during migration work.
- Do not replace the old parent application in place.
