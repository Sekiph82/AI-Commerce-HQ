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
| Current task | M12 Project Cockpit implementation prompt preparation |
| Current task ID | M12 |
| Current workflow state | READY_FOR_NEXT_IMPLEMENTATION_RUN |
| Progress | 60% |
| Required actor | CODEX |
| Next action | Prepare the authoritative M12 implementation prompt in a separate run; no M12 implementation has started |
| Waiting on | Authoritative M12 implementation prompt; none currently exists |
| Last meaningful update | M11 PASS/CLOSED; M12 activated for the next implementation run |

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
| M12 | Prepare the next authoritative Project Cockpit implementation prompt | READY | CODEX | M11 closure + M12 activation prompt |

## Blockers and waiting

- M12 is no longer blocked by M11; its implementation prompt must be prepared in a separate run.
- M21 remains planned and was not started.

## Milestone summary

- M00-M10: PASS/CLOSED according to the canonical H!veAI tracker.
- M11 original implementation: historical strict-audit FAIL.
- M11A REV7: PASS/CLOSED; final Projects visual cleanup: PASS/CLOSED.
- M11: PASS/CLOSED.
- M12: READY / ACTIVE FOR NEXT IMPLEMENTATION RUN; implementation not started.
- Strict completed roadmap count: 12/20 = 60%.

## Quality and verification

| Check | Result | Evidence |
| --- | --- | --- |
| REV7 focused tests | PASS | R24/R25 adversarial tests, prior R19-R23 tests, parser and watcher tests passed |
| REV7 full regression | PASS | 278 native tests, 87 frontend tests, typecheck/build/audit/checks and governed publication passed |

## Recent meaningful activity

- M11 closure accepted from the REV7 strict audit and final Projects strict audit; M11 is PASS/CLOSED and M12 is activated for the next implementation run.
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

M12 implementation prompt: no separate authoritative prompt currently exists; it must be prepared in a separate run before implementation.

H!veAI actively watches only .hiveai/PROJECT_DASHBOARD.md for project-status changes; the sources above are internal project evidence/provenance and are not independent live-watch targets.
