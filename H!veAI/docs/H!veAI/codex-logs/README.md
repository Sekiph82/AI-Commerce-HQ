# H!veAI Codex Log Standard

Every H!veAI milestone must have one durable chronological Codex log committed to this directory.

Naming:
`MXX_<SHORT_NAME>_CODEX_LOG.md`

At milestone start the log must record:
- milestone name,
- status `IN PROGRESS`,
- repository root,
- branch,
- starting HEAD,
- remotes,
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

Never commit secrets, tokens, `.env` contents, private keys, local DB files, or sensitive credential-bearing output.
