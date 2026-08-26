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

Application child root:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

The child root is **not** the Git repository root and must never contain its own `.git` repository.

## Canonical tracking files

### `H!veAI/TASKS.md`

The canonical detailed task/progress ledger.

It contains:
- current milestone truth;
- M00-M20 milestone packages such as `M08.01`, `M08.02`, etc.;
- completed/current/planned state;
- remediation history summaries;
- manual acceptance requirements;
- cross-milestone defect queues;
- milestone closure/unlock state.

Subpackage numbering exists for traceability and audit coverage. It does **not** mean each package is a separate builder prompt.

### `H!veAI/CODEX_ROADMAP.md`

The canonical detailed roadmap/dependency view.

It mirrors the milestone package structure at roadmap level and defines:
- purpose;
- major work packages;
- exit condition;
- dependency order;
- builder execution/exit rules.

### `H!veAI/README.md`

The project entry-point status summary. It must point to the canonical task ledger/roadmap and must not contain a stale historical “next milestone” statement.

### `docs/H!veAI/prompts/`

Authoritative prompts authored for Codex/Claude. Prompts are immutable historical evidence once used; remediation uses a new prompt file rather than rewriting the old used prompt.

### `docs/H!veAI/audits/`

Independent ChatGPT audits. Historical FAIL/CONDITIONAL/PASS records are immutable evidence.

### `docs/H!veAI/codex-logs/`

Chronological builder logs. Builder logs are claims/evidence records, not independent acceptance.

## Current roadmap truth

- M00-M10: PASS/CLOSED.
- M09 Task Intelligence Parser: PASS/CLOSED after independent M09D final strict audit.
- Original M09 strict audit: historical FAIL.
- M09A strict re-audit: historical FAIL after residual R01/R02 findings.
- M09B/M09C/M09D remediation and audit history are preserved; M09D final strict audit = PASS.
- Pre-M10 Native UX Hotfix X01/X02: PASS/CLOSED after source-level strict audit plus user native acceptance.
- X01 terminal/console popup suppression: accepted fixed after approximately 45 minutes of native runtime with no unwanted terminal windows.
- X02 startup intro audio/replay behavior: accepted fixed; startup audio works and same-process navigation does not replay the intro.
- M10 original strict audit: historical FAIL with 5 MAJOR findings.
- M10A strict-closure remediation: independent re-audit closed all production MAJOR findings.
- Akilta footer link: PASS/ACCEPTED after native user verification that Chrome opens, H!veAI remains open, and no terminal window appears.
- M10 Workflow State Machine: PASS/CLOSED.
- Strict completed milestone count is now 11/20 = 55%.
- M11 Global Command Center: ACTIVE during this run; final builder state will be IMPLEMENTATION COMPLETE / PENDING INDEPENDENT AUDIT + USER VISUAL ACCEPTANCE.
- M12 remains blocked behind M11. M11/M12 runtime implementation must incorporate the `.hiveai/PROJECT_DASHBOARD.md` authority manifest system, with only M11 active here.

For exact current status, always defer to `H!veAI/TASKS.md`.

## Mandatory workflow

1. ChatGPT inspects/audits the current milestone/state.
2. `TASKS.md` and `CODEX_ROADMAP.md` define current scope/status; detailed subpackages are traceability units, not mandatory prompt splits.
3. ChatGPT writes the next authoritative whole-milestone or bounded remediation prompt under `docs/H!veAI/prompts/`.
4. Codex reads that prompt from the `H!veAI` branch before working.
5. Codex creates the matching immutable log under `docs/H!veAI/codex-logs/`.
6. Codex records starting branch/HEAD/status, commands, decisions, failures, fixes, direct tests, full regression, publication evidence, commits, pushes, and final equality truthfully.
7. Codex never erases or rewrites prior failures after fixing them.
8. Builder completion may update prospective tracking state to “implementation complete / pending independent audit” but may not declare final PASS/CLOSED.
9. ChatGPT independently audits production source, tests, configuration, diff, security, docs/tracker truth, and final branch state.
10. ChatGPT saves the audit under `docs/H!veAI/audits/`.
11. Only an accepted independent audit, plus any required manual/native acceptance, may mark a milestone PASS/CLOSED and unlock the next milestone.

## Prompt design rule

Default to one prompt for one milestone.

Detailed task packages in `TASKS.md` should make the prompt easier to audit, not fragment execution into many tiny prompts. A future prompt should usually group all milestone packages into one bounded implementation contract. If an independent audit finds defects, create one bounded remediation prompt for the open findings only.

Builder-facing prompts should prioritize:
- exact current defect/required behavior;
- exact production boundary;
- exact direct test/state transition;
- PASS only if the test would fail on the pre-fix implementation;
- a short pre-push self-audit.

Avoid drowning implementation instructions in repeated governance prose.

## Tracking update rule

When milestone scope/state changes:
- update `TASKS.md` first as the canonical detailed ledger;
- keep `CODEX_ROADMAP.md` consistent at roadmap/status level;
- keep `H!veAI/README.md` current at high-level status;
- do not rewrite historical prompts/logs/audits to make old failures disappear;
- architecture/governance files should change only when architecture/governance itself changes, not merely because a task checkbox changed.

## Safety

- Never commit secrets, tokens, private keys, `.env` contents, local databases, or credential-bearing dumps.
- Never force-push unless the owner explicitly instructs it for a specific reason.
- Never rewrite history silently.
- Never treat the child `H!veAI` folder as the repository root.
- Never proceed if `git rev-parse --show-toplevel` is not exactly the canonical local root above.
- Never mark future detailed roadmap entries as implemented merely because they are documented.
