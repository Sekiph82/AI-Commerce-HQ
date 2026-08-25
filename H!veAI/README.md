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

- M00-M08: PASS/CLOSED.
- M09 Task Intelligence Parser: ACTIVE / NOT CLOSED.
- M09 original implementation: historical strict-audit FAIL.
- M09A remediation: historical strict re-audit FAIL after two residual findings remained.
- M09B bounded-identity micro-fix: implementation present; independent strict re-audit/final closure pending.
- Strict completed roadmap progress remains 9/20 = 45% until M09 closes.
- M10+ remain blocked/unstarted.
- Before M10, two queued native UX defects must be closed: repeated visible Git child-process console windows and muted startup-video audio.

For exact live status, acceptance state, and every completed/planned package from M00 through M20, use `TASKS.md` as the canonical source of truth.
