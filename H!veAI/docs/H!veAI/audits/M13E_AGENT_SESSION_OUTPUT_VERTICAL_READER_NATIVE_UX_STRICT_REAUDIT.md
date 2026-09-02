# M13E Agent Session Output Vertical Reader Native UX Strict Re-Audit

Date: 2026-09-02
Product: H!veAI
Branch: `H!veAI`
Milestone: M13
Scope: M13E native/visual UX remediation
Builder log: `docs/H!veAI/codex-logs/M13E_AGENT_SESSION_OUTPUT_VERTICAL_READER_NATIVE_UX_REMEDIATION_LOG.md`
Implementation commit: `6185893b995ec0b640bb14cdcbe9a64335d080bc`

## Verdict

**PASS TECHNICALLY / USER NATIVE-VISUAL ACCEPTANCE STILL REQUIRED**

- BLOCKER: 0
- MAJOR: 0
- MINOR: 0
- NOTE: 2
- Confidence: HIGH for source/test compliance, pending direct user visual acceptance for the published native viewport.

M13 remains open. M14 and M21 must remain unstarted until the user accepts the native M13E viewport.

## Scope audited

The re-audit reviewed the authoritative M13E prompt, immutable builder log, implementation commit, actual `src/pages.tsx` diff, `src/project-cockpit.css`, and focused M13 frontend tests. The builder log was treated as claimed evidence only and was not accepted without source/test inspection.

## Requirement findings

### E01 - Replace horizontal raw persisted-session output with a vertical reader: PASS

The previous Agents persisted output used raw `<pre className="cockpit-code">` rendering. M13E introduces the scoped `CodexSessionOutput` reader for stdout/stderr. The reader parses JSON lines only for presentation, preserves parse failures as original persisted lines, and renders rows vertically.

The scoped CSS constrains width with `max-width: 100%`, `min-width: 0`, and `width: 100%`; removes horizontal reader behavior with `overflow-x: hidden`; provides downward scrolling with `overflow-y: auto`; and wraps long content with `overflow-wrap: anywhere`, `word-break: break-word`, and `white-space: pre-wrap`.

This satisfies the authoritative prompt's requirement that ordinary persisted Codex output no longer require sideways scrolling.

### E02 - Preserve structured evidence truth: PASS

The UI transformation does not modify persisted adapter data. `codexOutputRows()` consumes the session text, attempts per-line JSON parsing only for display, pretty-prints recognized JSON, and falls back to the original persisted line when parsing fails. Session state, exit code, diagnostic code/message, stdout/stderr, truncation markers, and redaction markers remain sourced from the existing session object.

### E03 - Readable event presentation: PASS

Recognized JSON-line events are rendered as numbered vertically stacked event rows with the event `type` used as a label when present. The original event object remains visible through pretty-printed JSON content. Unrecognized lines remain visible and are not dropped.

### E04 - Full-width session detail behavior: PASS

The implementation uses the available Agents content width rather than allowing unbounded preformatted content to widen the page. Metadata remains above the output reader. The reader uses one deliberate bounded vertical scroll region (`max-height: 420px`) and does not introduce nested horizontal scrolling.

The prompt explicitly allowed vertical scrolling either naturally with the page or within one deliberate bounded reader, so the 420px bounded reader is compliant.

### E05 - COMPLETED/FAILED lifecycle truth: PASS

Focused tests preserve both successful and failed semantics. A completed fixture asserts `COMPLETED` plus output visibility. A failed fixture asserts `FAILED`, `CODEX_PROCESS_FAILED`, exit code `1`, diagnostic message, redacted stderr, and absence of a protected `password=` marker.

No backend lifecycle code was changed by M13E.

### E06 - Long-line wrapping and no raw `<pre>` reader: PASS

The focused long-output test includes a deeply nested Windows path inside JSON plus a 400-character unrecognized line. It verifies the new scoped reader is used, the long content remains present, and the reader contains no `<pre>` element. The CSS independently enforces wrapping and hidden horizontal overflow.

### E07 - M13 security/process boundaries preserved: PASS

The M13E implementation commit is UI/tracking scoped. It does not alter Codex process launch, executable resolution, stdin transport, persistence, redaction, stop/recovery, CREATE_NO_WINDOW policy, registered-project confinement, or model/process policy. M14 PTY/xterm/live-terminal behavior and M21 migration are not introduced.

### E08 - Execution gates: PASS WITH USER-ONLY VISUAL GATE PENDING

The builder log records all 26 explicit gates. Focused frontend 6/6, full frontend 101/101, focused Rust 24/24, full Rust 312/312, typecheck, build, dependency audit, fmt/check/diff, publisher rollback harness, governed no-bundle publication, stable artifact validation, and no-visible-console smoke are all recorded PASS.

Gate 24 is correctly recorded as pending user acceptance rather than falsely claimed by Codex. This is the only remaining acceptance boundary.

## Notes

### N01 - JSON event presentation remains technical rather than semantic

The reader pretty-prints the full JSON object and labels the row using its `type`. This is materially more readable than the former horizontal raw wall and satisfies M13E. A future product polish pass could extract fields such as agent message text, command, status, and aggregated output into richer semantic cards, but that is not required to close M13E and should not expand M13 into M14-style terminal UX.

### N02 - Bounded reader height is acceptable but must be judged natively

`max-height: 420px` creates one intentional vertical reader inside the session detail. This is explicitly allowed by the prompt. Native user acceptance should confirm that this feels comfortable at the user's laptop viewport and that no page-level or reader-level horizontal scrollbar is required.

## Required user native acceptance

Before M13 can close, the user must verify the newly published `H!veAI/dev-bin/H!veAI.exe` in the real native viewport:

1. Expand the completed ScrubBots persisted Codex session.
2. Confirm metadata is readable at the top.
3. Confirm output begins below metadata and reads top-to-bottom.
4. Confirm long JSON, Windows paths, commands, and aggregated output wrap within the main content width.
5. Confirm no sideways scrolling is needed to read ordinary session output.
6. Confirm the final agent response is reachable by scrolling downward.
7. Confirm the previously accepted no-visible-console behavior remains intact.

If these checks pass, M13E native/visual acceptance is satisfied and M13 can proceed to closure plus M14 activation. If the native viewport still requires horizontal scrolling or the bounded reader is materially unusable, M13 remains open and a targeted UX remediation is required.

## Milestone state

- M00-M12: PASS/CLOSED.
- M13/M13A/M13B/M13C/M13D: accepted technical boundaries preserved.
- M13E: **TECHNICAL STRICT RE-AUDIT PASS / PENDING USER NATIVE-VISUAL ACCEPTANCE**.
- Strict roadmap progress remains `13 / 20 = 65%` until M13 closure.
- M14: BLOCKED / NOT STARTED.
- M21: PLANNED / NOT STARTED.
