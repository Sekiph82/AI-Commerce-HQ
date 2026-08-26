# M11A REV4 Final Single-Dashboard Integration Strict Re-Audit

Date: 2026-08-26
Product: H!veAI
Branch: `H!veAI`
Audited builder log: `H!veAI/docs/H!veAI/codex-logs/M11A_REV4_FINAL_SINGLE_DASHBOARD_INTEGRATION_CLOSURE_LOG.md`
Audited implementation commit: `25d4b2a0532df8af07c1a5b22062c97fbacf0d11`
Builder-log commit / remote HEAD at audit start: `9ac752cd79f55ffef64494ae8f7c4f7be7e86f56`
Authoritative builder prompt: `H!veAI/docs/H!veAI/prompts/M11A_REV4_FINAL_SINGLE_DASHBOARD_INTEGRATION_CLOSURE_PROMPT.md`

## Verdict

**FAIL / M11 NOT CLOSED**

- BLOCKER: 0
- MAJOR: 2
- MINOR: 2
- NOTE: 2
- Confidence: HIGH
- Regression risk: MEDIUM-HIGH because the remaining defects can make the global Command Center over-report attention and can double-count the same operational failure across provenance layers.

REV4 closes the previously identified parser-front-matter defect, materialized enum validation defect, and the source-level stale watcher-scope defect. Materialized Project Dashboard evidence is now substantially connected to M11. However, two production truthfulness defects remain in the new materialized attention aggregation, so M11 cannot close yet.

Builder-reported local Windows execution results remain builder evidence, not independent acceptance. This audit independently inspected the pushed source, tests, diffs, current branch state, and the REV4 contract.

## Acceptance matrix

| Area | Result | Independent finding |
| --- | --- | --- |
| R15 live watcher scope reconciliation | PASS source-level / NOTE on platform-path evidence | Desired scope is compared with attached scope and a changed mode recreates the watcher. Worker reconciliation runs after dashboard lifecycle signals. The dedicated transition test uses the live manager/worker but manually injects `RawInput`, so actual OS notify delivery for `.hiveai` delete/recreate is still not directly proven by that test. |
| R16 materialized operational evidence | PARTIAL / FAIL | Dashboard blockers, waits, current work, quality facts and undated activity are connected, but WAITING can create false attention without a real wait fact, and provenance-layer de-duplication is too weak to suppress equivalent persisted audit/test/permission attention. |
| R17 front-matter accounting | PASS | Header counting now ends at the first top-level `##` section while Source authorities and materialized sections use separate parsing paths. |
| R18 enum validation | PASS | Project status, Health and Required actor are normalized against the shared contract and invalid values become UNKNOWN with bounded warning evidence. |
| M10 precedence / queue duplicate suppression | PASS for the exercised matching-workflow case | Stronger matching M10 queue evidence suppresses the dashboard Current work duplicate by task identity/title overlap. |
| REV3 UX / Akilta / footer / advanced inventory | PASS source-level, user native visual acceptance still pending | REV4 did not reopen the accepted source implementation. |

---

# Open findings

## R19 / MAJOR - `Project status = WAITING` creates operational attention even when no real wait fact exists

The REV4 contract was explicit: `Project status = WAITING` supports attention when it is paired with a real `Waiting on` value. Required actor alone is also not enough.

Current `materialized_operational_evidence()` computes:

```text
status_attention = Project status is BLOCKED or WAITING
                   OR Health is BLOCKED or ATTENTION
```

and, when no blocker/wait item was already generated, creates a `Project Dashboard status requires attention` row.

Therefore this valid single-dashboard state:

```text
Project status: WAITING
Health: UNKNOWN
Waiting on: NONE
Required actor: NONE
Blockers and waiting: None verified
```

still produces a Needs Your Attention item.

That is false operational escalation. A project may legitimately declare a broad WAITING lifecycle state while having no verified actionable owner/external dependency in the materialized contract. REV4's acceptance rule intentionally required a real wait fact before WAITING becomes attention.

### Required closure

- `Project status = BLOCKED` may independently create attention.
- `Health = BLOCKED` or `ATTENTION` may independently create attention according to the existing health contract.
- `Project status = WAITING` must create attention only when at least one real actionable wait fact exists, such as meaningful `Waiting on` or a verified blockers/waiting entry.
- `Required actor = HUMAN/EXTERNAL` remains supporting severity/context only and must not manufacture attention by itself.
- Add direct negative tests for WAITING + NONE/UNKNOWN/no blockers and positive tests for WAITING + real wait.

## R20 / MAJOR - Materialized attention de-duplication cannot reliably suppress equivalent stronger audit/test/permission evidence

REV4 adds `deduplicate_materialized_attention()`, but it considers a Project Dashboard attention duplicate only when a stronger item's free-text `detail` overlaps the materialized item's free-text `detail`.

Persisted stronger evidence currently uses generic detail strings such as:

- test: `A completed verification/test row has a failed result.`
- audit: `A persisted audit row has a failed result.`
- permission: `<permission kind> requires an explicit decision.`

Materialized quality attention uses content such as `Native tests: FAIL`, and blockers/waits use their dashboard text. These strings normally do not overlap even when they describe the same task/test/audit gate.

So a single logical failure can appear twice in Needs Your Attention and inflate `needs_attention`, directly violating the REV4 no-double-counting contract across M10, agent, audit/test/permission and materialized evidence.

The focused REV4 test proves M10 queue suppression, but there is no equivalent direct test proving dashboard attention is replaced by a matching persisted test/audit/permission item.

### Required closure

Implement provenance-aware deterministic equivalence instead of generic-detail substring matching.

At minimum:

- carry/derive a normalized materialized source identity for blocker/wait/quality items;
- prefer project + task ID when the dashboard row/fact can be tied to a task;
- for Quality/verification use a deterministic normalized check identity and accepted evidence class;
- compare persisted audit/test/permission evidence using project/task/evidence class before falling back to conservative normalized text;
- never suppress unrelated failures merely because both contain words such as `FAIL` or `blocked`;
- prove direct dashboard + persisted TEST_RUN duplicate suppression;
- prove direct dashboard + persisted AUDIT duplicate suppression where an identity match is available;
- prove unrelated dashboard and persisted failures remain separate;
- KPI attention count must reflect the de-duplicated set.

## R21 / MINOR - Quality table header is parsed as a factual quality row

`parse_bounded_facts()` excludes labels such as `Field`, `Role`, and `Source`, but does not exclude the standard Quality/verification table header `Check`.

For the standard shape:

```text
| Check | Result | Evidence |
| --- | --- | --- |
| Native tests | PASS | ... |
```

`Check: Result` becomes a `MaterializedFact`. It consumes one of the bounded quality slots and can surface as an Engineering Brief fact.

### Required closure

- Exclude the known Quality/verification table header row (`Check` / `Result`, case-insensitive) without weakening real custom fact labels.
- Add a parser test asserting the header is not materialized and the first real quality item is the first fact.

## R22 / MINOR - Some Project Dashboard operational IDs are order-based instead of content/source-stable

Blocker attention IDs currently use the list index:

`PROJECT_DASHBOARD:BLOCKER:<project>:<index>`

and undated activity IDs similarly use `<index>`.

Adding one item at the top changes the identity of every later item even when their content is unchanged. The REV4 contract asked for deterministic IDs based on project identity plus materialized source identity/content so stable evidence does not churn across dashboard edits.

### Required closure

- Use a bounded deterministic digest or normalized source identity for materialized blocker/wait/quality/activity IDs.
- Keep project identity in the key.
- Do not use unbounded raw content as an ID.
- Preserve duplicate protection and stable ordering.
- Add a test proving insertion of an unrelated earlier item does not change the stable ID of an unchanged later materialized fact.

---

# Evidence notes

## E09 / NOTE - R15 transition test bypasses actual OS notify delivery

`live_dashboard_contract_changes_reconcile_watcher_scope_without_restart` uses the real manager and worker but calls the manager's internal sender with manually constructed `RawInput` events. This proves worker/reconciliation logic, not Windows notify behavior for the transition itself.

A separate generic notify-backend test proves the backend can observe a temporary file modification, but it does not exercise the exact legacy -> single -> legacy dashboard lifecycle through the operating-system event path.

This does not reopen R15 as a known production defect because the pushed watcher source now contains the required scope comparison/recreation logic. Still, before final release hardening, a native acceptance/integration path should exercise actual dashboard create/delete/recreate or atomic replacement without direct sender injection.

## E10 / NOTE - Builder test/publication totals are claims, not independent execution

The immutable builder log reports 264 Rust tests, 86 frontend tests, typecheck/build/audit/fmt/check, governed publication and all 9 failure-harness cases as PASS. The pushed source contains the named regression tests and the remote contains the implementation/log commits. This audit cannot independently execute the user's Windows-local publisher, shortcut, audio or native WebView geometry, so those execution results remain builder evidence and user native/visual acceptance remains pending.

---

# Confirmed REV4 closures

The following REV3 findings are closed by the pushed source and are not reopened here:

- R15 stale watch mode caused by same-root early return: desired scope is now compared and a changed scope recreates the watcher.
- R17 materialized colon lines no longer consume the front-matter field budget.
- R18 invalid Project status / Health / Required actor no longer become arbitrary runtime states.
- Materialized Current work reaches Work Queue with conservative status mapping.
- Materialized blockers/waits and explicit failed Quality values reach attention.
- Materialized Recent meaningful activity remains undated instead of receiving a fabricated timestamp.
- Matching stronger M10 workflow queue evidence suppresses the tested weaker dashboard queue row.
- M09 authoritative task totals are not increased by materialized Current work rows.
- REV3 topbar/footer/Akilta changes remain preserved in the audited REV4 diff.

# Required next action

Do not start M12.

Run one final bounded M11A REV5 micro-closure for R19-R22 only, plus the smallest direct regression set necessary to prove the fixes and preserve R15-R18/REV3 behavior.

M11 may close only when the independent REV5 re-audit finds no BLOCKER/MAJOR defect and the user completes native visual acceptance of the published Command Center shell.
