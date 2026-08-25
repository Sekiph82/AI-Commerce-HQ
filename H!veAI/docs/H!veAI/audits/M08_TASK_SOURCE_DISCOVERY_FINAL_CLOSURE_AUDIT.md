# M08 Task Source Discovery Final Closure Audit

Date: 2026-08-25

## Verdict

`PASS / CLOSED`

M08 Task Source Discovery is fully closed.

The complete M08 chain was independently audited across original implementation, M08A, M08B, and M08C remediation passes. The final M08C strict re-audit found no BLOCKER or MAJOR production findings and left only the native `/tasks` manual visual gate open. That final manual gate is now PASS by direct user inspection documented in `M08_TASK_SOURCE_DISCOVERY_MANUAL_ACCEPTANCE.md`.

M09 Task Intelligence Parser is now the next authorized milestone. It remains unstarted until a dedicated M09 implementation prompt is issued.

## Closure basis

### Original M08 strict audit

Historical verdict: `FAIL`.

The original M08 implementation established the source-discovery foundation but had production/evidence gaps in filesystem bounds, custom-source operations/order, project-source reconciliation, stale frontend transitions, persistence evidence, and log completeness.

Historical audit remains immutable:

`docs/H!veAI/audits/M08_TASK_SOURCE_DISCOVERY_STRICT_AUDIT.md`

### M08A strict re-audit

Historical verdict: `FAIL`.

M08A closed the broad filesystem, status-boundary, handoff, stale-race, containment, ownership/schema, and evidence gaps, but left positional reorder correctness, pre-version adoption strictness, mounted add/reorder visibility evidence, and direct persisted SQL/order evidence incomplete.

Historical audit remains immutable:

`docs/H!veAI/audits/M08A_TASK_SOURCE_DISCOVERY_STRICT_REAUDIT.md`

### M08B strict re-audit

Historical verdict: `FAIL`.

M08B closed the remaining M08A findings for current-format settings, including true positional reorder, narrow legacy source adoption, direct SQL hash/deletion evidence, mounted add/reorder transitions, and table metadata evidence. It exposed one final backward-compatibility defect for original-M08 custom settings without explicit `order`, plus one small three-CUSTOM evidence mismatch.

Historical audit remains immutable:

`docs/H!veAI/audits/M08B_TASK_SOURCE_DISCOVERY_FINAL_STRICT_REAUDIT.md`

### M08C strict re-audit

Verdict before manual gate: `CONDITIONAL PASS`.

M08C closed the final production compatibility defect by distinguishing missing legacy order metadata, normalizing legacy custom paths by persisted vector position, preserving path-only rename position, and repairing explicit contiguous order on H!veAI-owned mutation. It also completed the three-CUSTOM plus STANDARD ordering evidence.

No BLOCKER or MAJOR production finding remained.

Audit:

`docs/H!veAI/audits/M08C_CUSTOM_ORDER_BACKCOMPAT_STRICT_REAUDIT.md`

### Final manual acceptance

Verdict: `PASS`.

The user inspected the refreshed native Task Sources workspace across `AI-Commerce-HQ`, `Bulk-Edit`, and `ScrubBots`, confirmed correct project-specific source inventory and accepted the presentation with the explicit statement `bence OK`.

Acceptance record:

`docs/H!veAI/audits/M08_TASK_SOURCE_DISCOVERY_MANUAL_ACCEPTANCE.md`

## Final accepted M08 production contract

M08 now provides a bounded, local-first Task Source Discovery layer that:

- discovers approved standard task/planning/handoff/instruction source files and bounded source directories;
- supports safe project-scoped custom source paths with add/remove/update and deterministic explicit ordering;
- preserves backward compatibility with original-M08 custom settings that lacked `order` metadata;
- enforces registered-project status and physical containment boundaries;
- produces bounded structured warning evidence for candidate/work/depth limits;
- computes source metadata including kind, origin, authority/priority, modification evidence, status, size, and SHA-256 where available;
- persists M08-owned, versioned source inventory non-destructively in `project_sources` while preserving unrelated legacy rows;
- prevents stale project-list and custom-mutation completions from reclaiming the currently selected project UI;
- keeps browser preview isolated from native filesystem discovery;
- presents truthful native Task Sources inventory without parsing tasks or inventing workflow/completion state;
- preserves the accepted H!veAI presentation, startup intro, sidebar/logo, post-sidebar background, Registry behavior, publisher, and no-bundle launcher contracts.

## Residual notes

- Windows physical symlink/junction creation remains `UNVERIFIED` in the automated fixture because the environment denies link creation with OS error 1314. The production containment logic remains unchanged and previously accepted; this environment limitation does not reopen M08.
- One M08C builder-log focused-test name had a documentation-only mismatch. The source-level test and production behavior were independently inspected; no remediation is required.
- Historical FAIL audits remain evidence history and do not reopen the closed milestone.

## Final milestone state

`M08 = PASS / CLOSED`

Strict completed milestone count: `9 / 20 = 45%`.

Next milestone: `M09 Task Intelligence Parser = READY / UNSTARTED`.
