# H!veAI Codex Log Standard

Every H!veAI milestone must have one durable chronological Codex log committed to this directory.

Naming:
`MXX_<SHORT_NAME>_CODEX_LOG.md`

## Mandatory fetch-before-prompt rule

Before reading any milestone prompt files:

```powershell
git fetch origin H!veAI
```

Then compare:

```powershell
git rev-list --left-right --count HEAD...origin/H!veAI
```

If local HEAD is behind `origin/H!veAI` and there are no conflicting local tracked changes:

```powershell
git merge --ff-only origin/H!veAI
```

Then read the authoritative audit and milestone prompt from the updated local checkout.

Never assume missing local prompt/audit files are absent from GitHub before fetching.

The Codex log must record the fetch, ahead/behind result, any fast-forward performed, the synchronized starting HEAD, and any reason synchronization could not be completed safely.

Never use reset, destructive checkout, force-push, or automatic rebase merely to satisfy this preflight. If fast-forward is unsafe, stop and report the exact condition.

At milestone start the log must record:
- milestone name,
- status `IN PROGRESS`,
- repository root,
- branch,
- synchronized starting HEAD,
- remotes,
- fetch/ahead-behind/fast-forward result,
- timestamp.

During work append:
- timestamp,
- action,
- commands,
- relevant outputs,
- files changed,
- decisions and reasons,
- failures,
- fixes,
- tests and results,
- git state,
- commit/push status.

Never remove a failure after it is fixed. Add a later correction entry.

At completion record:
- `PHASE STATUS: COMPLETE` only if acceptance criteria passed,
- final HEAD,
- final verification,
- blockers,
- exact next milestone.

Historical milestone logs must remain separate and unchanged unless a narrowly scoped follow-up entry is required to record that milestone's own final GitHub verification.

Never commit secrets, tokens, `.env` contents, private keys, local DB files, or sensitive credential-bearing output.
