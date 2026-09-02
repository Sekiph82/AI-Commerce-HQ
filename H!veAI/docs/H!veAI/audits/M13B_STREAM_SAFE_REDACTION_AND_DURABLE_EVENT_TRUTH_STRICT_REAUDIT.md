# M13B Stream-Safe Redaction and Durable Event Truth — Strict Re-Audit

Date: 2026-09-02
Product: H!veAI
Branch: `H!veAI`
Audited builder log: `H!veAI/docs/H!veAI/codex-logs/M13B_STREAM_SAFE_REDACTION_AND_DURABLE_EVENT_TRUTH_REMEDIATION_LOG.md`
Audited implementation commit: `61493b01d8fc9cce72c5e7d5495df0a1814d6991`
Historical findings under review: R30, R31

## Verdict

**PASS / R30 CLOSED / R31 CLOSED / M13 TECHNICAL STRICT AUDIT PASS / USER NATIVE-VISUAL ACCEPTANCE STILL REQUIRED**

- BLOCKER: 0
- MAJOR: 0
- MINOR: 0
- NOTE: 2
- Confidence: HIGH

M13B closes the two residual production defects identified by the M13A strict re-audit. No new production defect requiring another remediation run was found in the audited scope.

M13 is not yet milestone-closed because user-owned native/visual acceptance of the M13 UI/interaction surface remains outstanding.

## R30 — Stream-safe redaction across arbitrary pipe reads

**Result: PASS / CLOSED**

The previous defect redacted independent OS pipe chunks, allowing a protected marker to cross a read boundary and escape classification.

The implementation now introduces a stateful `StreamRedactor` before capture or persistence:

- raw pipe bytes are accumulated across reads;
- complete newline-delimited records are classified before they are handed to `Capture`;
- an unterminated final record is classified at EOF;
- the carry is explicitly bounded by `MAX_REDACTION_CARRY_BYTES = 4096`;
- an overlong unterminated record is conservatively replaced with the redaction marker and its remainder is discarded until newline;
- `Capture::append` accepts only already-classified/redacted text;
- protected marker classes remain `api_key`, `apikey`, `token`, `password`, `secret`, `authorization`, and `sk-`.

This architecture removes the original chunk-boundary leak rather than merely adding a test-specific overlap heuristic.

The builder log reports adversarial one-byte-read coverage for every protected marker, unterminated sensitive final records, Unicode split across read boundaries, direct inspection of persisted `STREAM_OUTPUT` payloads, and carry/capture bound tests. This is appropriate direct evidence for R30.

## R31 — Durable stream-event truth

**Result: PASS / CLOSED**

The previous defect allowed stdout/stderr capture counters to advance while incremental SQLite writes could fail silently, causing final evidence to overstate durable output.

The implementation now separates capture truth from persistence truth:

- stdout/stderr reader threads send already-redacted bounded events through a bounded `sync_channel`;
- a single `EventWriter` owns the incremental event-store path;
- stream writes use exactly three bounded attempts with bounded backoff;
- durable byte/event counters advance only after `STREAM_OUTPUT` persistence succeeds;
- failed persistence marks explicit degraded state and bounded diagnostic data;
- the implementation attempts a `PERSISTENCE_DEGRADED` evidence row on first terminal stream persistence failure;
- `SESSION_FINISHED` records distinct captured and persisted counters and preserves legacy flat counters as durable counts;
- session reconstruction sorts stream evidence by channel-local sequence instead of depending on timestamp/UUID tie ordering.

The builder log reports deterministic injected transient and terminal persistence failures, dual-channel concurrency, direct durable-row/count equality, and terminal-state/count checks. These tests directly target the R31 failure mode.

## Preservation of accepted M13/M13A boundaries

Independent source review of implementation commit `61493b01...` shows the M13B production change is concentrated in the Codex adapter stream redaction/persistence path plus canonical status documentation. The previously accepted provider-neutral adapter contract, project/task validation, fixed direct Codex launch, bounded prompt, stdin prompt transport, owned-process lifecycle, explicit unsupported resume semantics, and owned-tree stop escalation remain intact.

M14 PTY/xterm work and M21 standalone migration were not started.

## Verification evidence

Builder evidence reports:

- M13B focused native adapter tests: 19 PASS;
- full Rust regression: 306 PASS;
- focused M13 frontend tests: 3 PASS;
- full frontend regression: 98 PASS;
- typecheck/build/audit/fmt/check/diff hygiene: PASS;
- publisher failure harness: 9/9 PASS;
- governed publication: final elevated unchanged run PASS;
- Codex readiness probe: `codex-cli 0.130.0-alpha.5`.

The two earlier non-elevated publication attempts failed at candidate smoke without replacing the stable executable. The same unchanged publication flow later passed under the required Windows process context. This is not treated as a production defect because rollback/safe-publication behavior was preserved and the final governed publication completed successfully.

## Notes

### NOTE N01 — User native interaction acceptance remains mandatory

M13 includes user-facing Codex adapter controls/readiness/session evidence. Source, test, and publication evidence cannot substitute for the user's real native interaction check. The user should validate the published `H!veAI/dev-bin/H!veAI.exe` before M13 is closed.

### NOTE N02 — No real coding operation was run against a user project

The builder deliberately limited real Codex evidence to a harmless version/readiness probe and used deterministic disposable fixtures for lifecycle/security tests. This is acceptable for strict source/security audit, but the final native acceptance should include one deliberately harmless bounded Codex session in a safe registered project if the UI exposes Start/Stop controls cleanly.

## Required native acceptance before closure

Using the newly published `H!veAI/dev-bin/H!veAI.exe`, verify at minimum:

1. Agents / Project Cockpit Agents shows Codex as detected, including version/readiness truthfully.
2. A selected registered ACTIVE project is shown correctly.
3. Starting one harmless bounded Codex operation works from the intended M13 UI surface.
4. The session appears with truthful RUNNING/completed state and bounded output/evidence.
5. Stop works for an owned running session if a long-enough harmless operation is available.
6. Resume is shown as unsupported rather than pretending to resume.
7. No terminal/console window flashes unexpectedly.
8. Existing Command Center / Projects / Tasks / Project Cockpit navigation remains healthy.

If the native checks pass, M13 can proceed to canonical closure and M14 activation. If a concrete native defect appears, open a narrowly scoped remediation instead.

## Closure state

**M13 TECHNICAL STRICT AUDIT: PASS**

**M13 MILESTONE: PENDING USER NATIVE/VISUAL ACCEPTANCE**

**Strict completed roadmap count remains 13/20 = 65% until M13 canonical closure.**