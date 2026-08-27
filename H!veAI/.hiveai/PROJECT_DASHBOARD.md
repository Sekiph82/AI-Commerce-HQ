hiveaiDashboardSchema: hiveai-project-dashboard/v1
dashboardMode: source-map
trackingMode: single-dashboard-watch
refreshPolicy: project-agent-maintained; H!veAI watches only .hiveai/PROJECT_DASHBOARD.md
projectKey: H!veAI
repository: Sekiph82/AI-Commerce-HQ
branchPolicy: H!veAI

## Source authorities

- Canonical task source: `TASKS.md`
- Agent instruction source: `AGENTS.md`
- Architecture source: `ARCHITECTURE.md`
- Build/test metadata: `src-tauri/Cargo.toml`

## H!veAI live status

| Field | Value |
| --- | --- |
| Project status | ACTIVE |
| Health | UNKNOWN |
| Current milestone | M12 |
| Current task | M12 Project Cockpit implementation |
| Current task ID | M12 |
| Current workflow state | IMPLEMENTATION_COMPLETE_PENDING_AUDIT |
| Progress | 60% |
| Required actor | CODEX |
| Next action | Independent strict audit and user native/visual acceptance |
| Waiting on | Independent strict audit and user native/visual acceptance |
| Last meaningful update | M12 implementation complete; strict audit and user native/visual acceptance remain pending |

## Current work

| ID | Item | Status | Owner/actor | Evidence/source |
| --- | --- | --- | --- | --- |
| M11A.REV5-R19 | Keep WAITING without a real wait fact out of attention | CLOSED | CODEX | REV5 prompt / command_center.rs |
| M11A.REV5-R20 | Deduplicate attention only with conservative provenance identity | CLOSED | CODEX | REV5 prompt / command_center.rs |
| M11A.REV5-R21 | Ignore Quality table headers as facts | CLOSED | CODEX | REV5 prompt / project_dashboard.rs |
| M11A.REV5-R22 | Keep materialized operational IDs stable across row insertion | CLOSED | CODEX | REV5 prompt / command_center.rs |
| M11A.REV6-R23 | Preserve full bounded scalar identity before hashing | CLOSED | CODEX | REV6 prompt / command_center.rs |
| M11A.REV7-R24 | Preserve Unicode operational identity | CLOSED | CODEX | REV7 prompt / command_center.rs |
| M11A.REV7-R25 | Preserve structured Quality identity | CLOSED | CODEX | REV7 prompt / command_center.rs |
| M12 | Implement project-scoped Project Cockpit with truthful authority/provenance | IMPLEMENTATION_COMPLETE_PENDING_AUDIT | CODEX | M12 implementation prompt / source and test evidence |

## Blockers and waiting

- M12 implementation is complete; independent strict audit and user native/visual acceptance remain pending.
- M21 remains planned and was not started.

## Milestone summary

- M00-M10: PASS/CLOSED according to the canonical H!veAI tracker.
- M11 original implementation: historical strict-audit FAIL.
- M11A REV7: PASS/CLOSED; final Projects visual cleanup: PASS/CLOSED.
- M11: PASS/CLOSED.
- M12: IMPLEMENTATION COMPLETE / PENDING INDEPENDENT STRICT AUDIT + USER NATIVE/VISUAL ACCEPTANCE.
- Strict completed roadmap count: 12/20 = 60%.

## Quality and verification

| Check | Result | Evidence |
| --- | --- | --- |
| M12 focused tests | PASS | Project Cockpit route/race/containment/unknown/correction tests |
| M12 full regression | PASS | 282 native tests, 92 frontend tests, typecheck/build/audit/checks and governed publication passed |

## Recent meaningful activity

- M11 closure accepted from the REV7 strict audit and final Projects strict audit; M11 remains PASS/CLOSED.
- M12 Project Cockpit implementation, full regression, and governed publication completed; independent strict audit and user native/visual acceptance remain pending.
- Prior M11A R01-R08, E01-E03, and UX01-UX04 source fixes are preserved.
- H!veAI own dashboard contract is now the dogfood single-dashboard watch target; materialized activity remains explicitly undated and REV7 publication evidence is complete.

## Provenance

- Task authority: `TASKS.md`
- Roadmap context: `CODEX_ROADMAP.md`
- Governance: `AGENTS.md`
- Architecture: `ARCHITECTURE.md`
- Constitution: `CONSTITUTION.md`
- Historical M11A evidence: `docs/H!veAI/codex-logs/M11A_GLOBAL_COMMAND_CENTER_STRICT_CLOSURE_LOG.md`
- Independent decision: `docs/H!veAI/audits/M11A_POST_LOG_STRICT_REAUDIT_AND_PRODUCT_DELTA.md`
- Independent deep audit: `docs/H!veAI/audits/M11A_REV6_DEEP_IDENTITY_STRICT_REAUDIT.md`
- Build/test evidence: `docs/H!veAI/codex-logs/M11A_REV7_UNICODE_AND_STRUCTURED_IDENTITY_FINAL_CLOSURE_LOG.md`

M12 implementation prompt: `docs/H!veAI/prompts/M12_PROJECT_COCKPIT_IMPLEMENTATION_PROMPT.md`.

H!veAI actively watches only .hiveai/PROJECT_DASHBOARD.md for project-status changes; the sources above are internal project evidence/provenance and are not independent live-watch targets.
