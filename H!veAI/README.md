# H!veAI

H!veAI is a local-first AI Development Command Center.

This directory is the dedicated H!veAI application root inside the parent `AI-Commerce-HQ` Git repository. It is intentionally **not** a separate Git repository.

## Canonical roots

Git root:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Application root:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

Canonical repository/branch:
`Sekiph82/AI-Commerce-HQ` / `H!veAI`

## Canonical task tracking

Detailed milestone/task ledger:
`TASKS.md`

Detailed M00-M20 roadmap and dependency path:
`CODEX_ROADMAP.md`

Development protocol / prompts / audits / builder logs:
`docs/H!veAI/README.md`

Package numbering such as `M08.01`, `M08.02`, etc. is used for traceability and audit coverage. It does not imply separate builder prompts. Whole milestones should remain single bounded builder runs unless an independent audit requires a remediation run.

## Current development status

- M00-M09: PASS/CLOSED.
- M09 Task Intelligence Parser: PASS/CLOSED after independent M09D final strict audit.
- M09 original implementation: historical strict-audit FAIL.
- M09A remediation: historical strict re-audit FAIL after two residual findings remained.
- M09B/M09C/M09D remediation and audit history are preserved; M09D final strict audit = PASS.
- Pre-M10 Native UX Hotfix X01/X02: PASS/CLOSED after independent source audit plus user native acceptance.
- X01 terminal/console popup suppression: accepted fixed after approximately 45 minutes of native runtime with no unwanted terminal windows.
- X02 startup intro audio/replay behavior: accepted fixed; audio works and the intro does not replay during same-process route navigation.
- Strict completed roadmap progress remains 10/20 = 50% because the pre-M10 hotfix is not a numbered milestone.
- M10 Workflow State Machine: READY / UNSTARTED.
- M11/M12 remain planned behind M10; Project Dashboard manifest ingestion is reserved for their runtime implementation.

For exact live status, acceptance state, and every completed/planned package from M00 through M20, use `TASKS.md` as the canonical source of truth.
