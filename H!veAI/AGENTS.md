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

## Canonical UI Assets

Canonical UI reference assets are located at:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Use the assets in this folder as authoritative visual references:

- dashboard reference image
- Akilta logo
- H!veAI logo

Do not redesign these assets unless explicitly instructed.

For any milestone that creates, changes, or visually touches the H!veAI UI, use the dashboard reference to reproduce the layout, spacing, visual hierarchy, panels, cards, navigation, right-side assistant/status columns, typography density, and overall dark visual language as closely as practical.

Use the H!veAI logo in product branding.

Use the Akilta logo in the footer with:

`Built with ♥ for maximum productivity by Akilta`

Preserve these UI rules across future milestones unless the user explicitly changes them.

## Development Manual QA Launcher

During active development, the user must be able to launch the latest validated H!veAI desktop build by double-clicking one stable Windows shortcut without reinstalling the application.

Canonical stable development executable:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI\dev-bin\H!veAI.exe`

Canonical Desktop shortcut:

`Desktop\H!veAI.lnk`

Canonical shortcut icon source image:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI\H!veAI small logo.png`

Use this exact image as the authoritative visual source for the Desktop shortcut icon. Do not substitute, redesign, recolor, crop, or replace it unless the user explicitly changes this rule.

Because Windows `.lnk` icon locations should use a Windows-compatible icon resource rather than depend directly on PNG rendering, create or refresh a deterministic `.ico` derivative from this exact PNG when needed. Prefer the stable local derivative:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI\dev-bin\H!veAI.ico`

The `.ico` derivative must preserve the source logo and should include standard Windows icon sizes where tooling permits, including 16, 32, 48, 64, 128, and 256 pixels. The Desktop `H!veAI.lnk` IconLocation must point to this stable `.ico` derivative, or to an equivalent Windows icon resource generated from the same canonical PNG if technically required.

Launcher rules:

- `H!veAI.lnk` must target the stable `H!veAI.exe` directly.
- The shortcut must never target `.bat`, `cmd.exe`, PowerShell, npm, cargo, Vite, or an installer.
- After each future milestone passes its required full verification, publish the new validated desktop executable to the same stable path using the repository's safe development-QA publication helper.
- Publish through staging/validation so a failed build never overwrites the last known-good executable.
- Keep the Desktop shortcut target and icon location stable across milestones.
- If the canonical shortcut icon source PNG changes, regenerate the `.ico` derivative from that source before validating the shortcut.
- Do not require setup or reinstallation during active development.
- The final Windows installer, Program Files installation, Start Menu shortcut, uninstaller, and release packaging remain a final-release/M20 concern, not the active development launcher model.

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

## Strict Audit Governance Standard

H!veAI milestone closure uses an evidence-first audit model inspired by the strict
FormuLab audit workflow. A builder/Codex completion statement is never sufficient
proof of milestone completion.

The independent auditor must recover the real milestone contract from the prompt,
architecture, TASKS, prior audits, migration docs, and acceptance criteria, then
verify repository truth against that contract.

Every milestone audit must include these sections:

1. `VERDICT` — exactly one of `PASS`, `CONDITIONAL`, or `FAIL`.
2. `CONTRACT RECOVERY` — what the milestone was actually required to deliver.
3. `BRANCH / HEAD / DIFF SCOPE` — audited branch, commit range, final HEAD, and changed-file scope.
4. `ACCEPTANCE CRITERIA MATRIX` — every acceptance criterion marked individually as `PASS`, `PARTIAL`, `FAIL`, or `UNVERIFIED`.
5. `BUILDER CLAIMS VS REPOSITORY TRUTH` — compare Codex-log claims against actual implementation.
6. `FILE / SYMBOL EVIDENCE` — inspect the real implementation paths, symbols, configuration, permissions, and runtime boundaries.
7. `FOCUSED TEST EVIDENCE` — verify tests that directly exercise the changed behavior.
8. `REGRESSION EVIDENCE` — verify relevant previously completed behavior still works.
9. `SECURITY / SAFETY REVIEW` — permissions, secrets, filesystem/process/network boundaries, destructive actions, and unsafe fallbacks.
10. `ARCHITECTURE CONSISTENCY` — check the implementation against H!veAI architectural decisions and cross-milestone contracts.
11. `TRACKER / LOG / DOCUMENTATION TRUTHFULNESS` — TASKS, milestone log, migration docs, and final report must match repository reality.
12. `FINAL REPOSITORY STATE` — verify final commit/push state, remote visibility, historical-log preservation, and user-state preservation.
13. `OPEN CROSS-MILESTONE FINDINGS` — carry forward defects or technical debt discovered in earlier milestones; do not silently forget them.
14. `DEFECTS BY SEVERITY` — classify findings as `BLOCKER`, `MAJOR`, `MINOR`, or `NOTE`.
15. `TECHNICAL DEBT / UPGRADE OPPORTUNITIES` — production-hardening or maintainability improvements even when not blocking.
16. `UNVERIFIED ITEMS` — anything lacking sufficient evidence must remain explicitly unverified; never convert missing evidence into PASS.
17. `REGRESSION RISK` — `LOW`, `MEDIUM`, or `HIGH`, with rationale.
18. `AUDIT CONFIDENCE` — `LOW`, `MEDIUM`, or `HIGH`, with rationale.
19. `FINAL VERDICT` — concise closure statement.
20. `REQUIRED REMEDIATION` — exact fixes required before progression when verdict is not an unconditional PASS.

### Audit evidence rules

- Treat Codex logs as claims to verify, not as proof.
- Prefer source code, configuration, tests, committed artifacts, Git history, and runtime evidence over summaries.
- A passing test suite does not override a direct specification violation.
- A feature that exists only in a mock/test path but not in production code is not complete.
- If a manual acceptance step is required and was not performed, mark it `UNVERIFIED` or `PENDING MANUAL ACCEPTANCE`; never fabricate a PASS.
- If environment limitations prevent verification, record the limitation explicitly.
- Cross-milestone regressions or previously missed defects may reopen an earlier milestone finding.
- Historical milestone logs must remain immutable; corrections belong in new audit/remediation files.
- Do not approve solely because implementation compiles, tests pass, or Codex reports `COMPLETE`.

### Verdict semantics

- `PASS`: all blocking requirements are satisfied with sufficient evidence; only clearly non-blocking notes may remain.
- `CONDITIONAL`: core implementation is substantially correct, but one or more bounded required follow-ups remain before the next quality gate or release boundary.
- `FAIL`: any blocker, specification violation, unsafe behavior, missing required evidence, or materially incomplete acceptance criterion exists.

### Remediation prompt rule

When an audit finds required fixes, create a bounded remediation prompt rather than a vague cleanup request.

A remediation prompt must enumerate every finding separately and, for each one, specify:

- originating milestone/finding
- severity
- exact file/symbol or subsystem where known
- current incorrect behavior
- required target behavior
- required code/config/documentation changes
- focused tests to add or update
- regression tests that must remain green
- security/safety constraints
- acceptance criteria for closure
- prohibited shortcuts or scope expansion

For cross-milestone remediation after M07, use milestone code `M07.01` and include:

- every finding from the strict M07 audit
- every still-open finding carried from M01-M06
- regression verification across M01-M07
- final tracker/log/documentation truthfulness review
- final local/remote HEAD verification

Do not start the next normal milestone until required remediation has been audited and closed when the audit verdict or explicit project governance requires that gate.

## Safety

- Do not overwrite user changes.
- Do not force-push.
- Do not commit secrets, `.env`, local databases, caches, build output, or
  user-specific runtime data.
- Do not run inherited commerce, trading, social media, marketplace, or
  publishing operations from the old parent application during migration work.
- Do not replace the old parent application in place.
