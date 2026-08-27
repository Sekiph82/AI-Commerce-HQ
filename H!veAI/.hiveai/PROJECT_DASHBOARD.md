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
| Current milestone | M13 |
| Current task | M13A common adapter, streaming, and stop remediation |
| Current task ID | M13 |
| Current workflow state | REMEDIATION_COMPLETE_PENDING_AUDIT |
| Progress | 65% |
| Required actor | CODEX |
| Next action | Independent strict re-audit and user native/visual acceptance |
| Waiting on | Independent strict re-audit and user native/visual acceptance |
| Last meaningful update | M13A R27-R29 remediation and verification gates completed |

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
| M12 | Implement project-scoped Project Cockpit with truthful authority/provenance | CLOSED | CODEX | Accepted M12 strict/audit/native evidence |
| M12A-R26 | Order selected-project workflow history globally before the bounded cockpit cap | CLOSED | CODEX | M12A prompt / strict re-audit |
| M12B-ROUTE-LOAD | Restore native registered-project cockpit snapshot IPC and truthful route errors | CLOSED | CODEX | M12B prompt / strict re-audit / user native acceptance |
| M13 | Codex Adapter implementation | REMEDIATION_COMPLETE_PENDING_AUDIT | CODEX | M13A prompt; common adapter/streaming/stop source/tests; M13A builder log |

## Blockers and waiting

- M12, M12A R26, and M12B are PASS/CLOSED on accepted strict evidence and user native/visual acceptance.
- M13 implementation is complete; independent strict audit and user native/visual acceptance remain pending.
- M13A R27-R29 remediation is complete; independent strict re-audit and user native/visual acceptance remain pending.
- M21 remains planned and was not started.

## Milestone summary

- M00-M10: PASS/CLOSED according to the canonical H!veAI tracker.
- M11 original implementation: historical strict-audit FAIL.
- M11A REV7: PASS/CLOSED; final Projects visual cleanup: PASS/CLOSED.
- M11: PASS/CLOSED.
- M12: PASS/CLOSED on accepted strict evidence and user native/visual acceptance.
- M12A: PASS/CLOSED; R26 independently re-audited.
- M12B: PASS/CLOSED; native Open Cockpit acceptance recorded.
- M13/M13A: REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE.
- Strict completed roadmap count: 13/20 = 65%.

## Quality and verification

| Check | Result | Evidence |
| --- | --- | --- |
| M12A R26 focused tests | PASS | Project-wide starvation, deterministic tie-order, project isolation, and derived-activity tests |
| M12 full regression | PASS | Full native/frontend tests, typecheck/build/audit/checks and governed publication passed |
| M12B route-loading focused tests | PASS | Exact-ID navigation, native ACTIVE snapshot, error classification, permission capability, and M12A isolation coverage |
| M12 closure acceptance | PASS | Accepted M12/M12A/M12B evidence and user native/visual acceptance dated 2026-08-27 |
| M13 focused native adapter tests | PASS | Codex readiness, process boundary, bounded capture, redaction, recovery, and injection assertions |
| M13 focused frontend tests | PASS | Readiness, registered-project scoping, prompt/task request shape, and browser-preview guard |
| M13 full verification | PASS | Native/frontend regression, typecheck, build, audit, formatting, check, and governed publication |
| M13A R27-R29 focused native tests | PASS | Common contract, incremental structured output, pre-persistence redaction, caps, clean-stop limitation, owned-tree escalation, and recovery |
| M13A full verification | PASS | Full native/frontend regression and governed publication after remediation |

## Recent meaningful activity

- M11 closure accepted from the REV7 strict audit and final Projects strict audit; M11 remains PASS/CLOSED.
- M12 Project Cockpit, M12A R26 history remediation, and M12B native route-loading remediation are PASS/CLOSED on accepted evidence; M13 is now the active next implementation milestone.
- Prior M11A R01-R08, E01-E03, and UX01-UX04 source fixes are preserved.
- H!veAI own dashboard contract is now the dogfood single-dashboard watch target; materialized activity remains explicitly undated and REV7 publication evidence is complete.
- M13 Codex adapter is implemented with direct owned-process execution, persisted agent evidence, bounded redacted output, and explicit unsupported resume semantics.
- M13A closes R27-R29 with the provider-neutral lifecycle trait, incremental bounded stream events, and clean-stop-first/owned-tree escalation evidence.

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

M12B remediation prompt: `docs/H!veAI/prompts/M12B_NATIVE_OPEN_COCKPIT_ROUTE_LOADING_REMEDIATION_PROMPT.md`.

M12 closure and M13 activation prompt: `docs/H!veAI/prompts/M12_CLOSURE_AND_M13_ACTIVATION_PROMPT.md`.

M13A remediation is complete; the authoritative M13/M13A prompts, source/test evidence, and immutable builder logs remain available for independent re-audit.

H!veAI actively watches only .hiveai/PROJECT_DASHBOARD.md for project-status changes; the sources above are internal project evidence/provenance and are not independent live-watch targets.
