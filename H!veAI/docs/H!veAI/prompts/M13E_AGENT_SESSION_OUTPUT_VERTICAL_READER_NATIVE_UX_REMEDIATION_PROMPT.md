# M13E Agent Session Output Vertical Reader Native UX Remediation Prompt

Date: 2026-09-02
Product: H!veAI
Branch: `H!veAI`
Milestone: M13
Scope: native/visual acceptance only

## Authority and purpose

This prompt is the authoritative M13E remediation instruction. It exists because the real Codex operation now completes successfully in native H!veAI, but the persisted session output is difficult to read in the current horizontal, raw-line presentation.

The user has explicitly accepted the successful operation result but has NOT accepted the current session-output UX. The user wants the output to be read as a full-width vertical document that scrolls downward, not sideways.

Do not reinterpret this as M14 work. Do not add PTY/xterm/live-terminal behavior. This is a bounded M13 native UX remediation for already-persisted Codex session evidence.

## Preserve all accepted M13 boundaries

Preserve without weakening:

- M13 provider-neutral adapter contract
- registered-project confinement
- fixed executable/argument policy
- bounded stdin prompt transport
- bounded streaming output
- pre-persistence redaction
- durable event truth
- owned process lifecycle and stop semantics
- Windows native executable resolution
- CREATE_NO_WINDOW background process policy
- M13D real Codex invocation compatibility
- existing session persistence and diagnostic truth
- no arbitrary shell/executable/argument primitive
- no M14 PTY/xterm/live terminal
- no M21 migration

Do not change the successful real Codex execution semantics merely to solve readability.

## User-observed native acceptance defect

The user completed a real ScrubBots Codex operation successfully. The session shows `COMPLETED`, exit code `0`, and the persisted output contains the real Codex JSON/event stream and final repository summary.

However, the expanded session output currently behaves like a very wide raw preformatted line surface. The user must horizontally scroll and still cannot comfortably see the beginning and end of the output. This is not acceptable native UX.

The required interaction model is:

- full-width vertical reader
- text wraps to the available width
- scroll downward through the output
- no mandatory horizontal scrolling for ordinary persisted Codex output
- session metadata remains readable above the output
- long commands/paths may wrap or use controlled break behavior rather than forcing the whole surface wider than the viewport

## M13E required implementation

### 1. Replace horizontal raw-output presentation with a vertical reader

For expanded persisted Codex sessions in Agents:

- render output inside a width-constrained container that never exceeds the usable Agents content width
- use normal downward document scrolling
- enable line wrapping / safe word breaking for long JSON, command, path, and diagnostic strings
- remove the horizontal-reader experience as the primary interaction
- avoid page-level horizontal overflow
- do not require the user to drag a horizontal scrollbar to inspect output

The user must be able to start at the top of the session and read sequentially downward to the end.

### 2. Preserve structured evidence truth

Do not mutate persisted data to make it prettier.

The UI may transform presentation only. Preserve:

- exact session state
- exit code
- diagnostic code/message
- stdout/stderr bounded output
- redaction markers
- chronological/event truth already exposed by the adapter

If the output is rendered from JSON lines, parsing for presentation is allowed only when failure-safe. Never drop unrecognized lines. If parsing fails, show the original redacted persisted line in the vertical reader.

### 3. Prefer readable event presentation over an unreadable text wall

Where safely possible, present persisted Codex JSON-line events as vertically stacked readable rows/cards, for example:

- thread started
- turn started
- agent message
- command started
- command completed
- final agent message

Each event should retain its real content and sequence.

Command strings and aggregated output must wrap inside their own bounded region.

Do not hide important evidence merely because it is verbose.

A compact/raw toggle is acceptable if useful, but the default user-facing mode MUST be the readable vertical mode.

### 4. Full-width session detail behavior

When a persisted session is expanded:

- use the available main-content width rather than a narrow nested pane
- maintain stable padding and readable typography
- keep metadata at the top
- place output immediately below metadata
- vertical scroll should occur naturally with the page or within one deliberate bounded reader, not nested horizontal + vertical scroll traps

Do not make the main navigation/sidebar disappear unless the existing H!veAI UX convention already supports that. "Full-width" here means the usable main content area, not necessarily OS fullscreen.

### 5. COMPLETED session must remain truthfully COMPLETED

The real ScrubBots operation is successful. Do not regress lifecycle classification.

Tests must include:

- COMPLETED exit code 0 session renders as COMPLETED
- failed session remains FAILED
- diagnostic metadata remains present
- redacted output remains redacted
- output reader wraps long content and does not require horizontal page scrolling

### 6. Preserve no-visible-console behavior

M13D R33 remains accepted. Do not regress it.

No code in this UI remediation may reintroduce visible terminal/console windows during:

- app post-startup readiness
- Agents readiness
- Codex operation launch
- operation completion
- stop/escalation

## Explicit execution gates

Run and record every gate below. Do not compress them into "all tests pass".

1. Safely `git fetch origin H!veAI` and synchronize using fast-forward only.
2. Confirm exact branch `H!veAI` and clean scoped worktree, preserving unrelated user files.
3. Read this authoritative prompt from the synchronized checkout.
4. Inspect current Agents session rendering source and identify the exact horizontal-overflow mechanism before editing.
5. Add focused frontend tests proving long persisted output wraps vertically and does not require horizontal scrolling.
6. Add focused frontend test for COMPLETED session metadata and output visibility.
7. Add focused frontend test for FAILED session diagnostic visibility.
8. Add focused frontend test proving redaction markers remain visible and secret markers are not reintroduced.
9. Run the full existing M13 frontend focused suite.
10. Run the full frontend test suite.
11. Run TypeScript typecheck.
12. Run frontend production build.
13. Run `npm audit --audit-level=high`.
14. Run focused Rust Codex adapter tests to prove no backend lifecycle regression.
15. Run full Rust library regression.
16. Run `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`.
17. Run `cargo check --manifest-path src-tauri/Cargo.toml --all-targets`.
18. Run `git diff --check`.
19. Run the governed publisher failure/rollback harness.
20. Run governed production Tauri `--no-bundle` publication.
21. Run stable executable smoke and shortcut/icon validation.
22. Run a native post-startup no-visible-console smoke.
23. If safely feasible, run one harmless real Codex operation against a disposable or explicitly safe registered fixture and confirm session state/output rendering remains truthful.
24. Manually inspect the published native Agents session detail at a normal laptop viewport and prove no page-level horizontal scrollbar is required to read ordinary session output.
25. Confirm M14 and M21 were not started.
26. Confirm strict roadmap progress remains `13 / 20 = 65%` until independent re-audit and user native/visual acceptance.

## Required native acceptance shape

The published H!veAI.exe must show a persisted completed Codex session such that:

- the session expands vertically
- the user can see the top metadata
- the output begins below it
- the user scrolls downward to read the operation
- long lines wrap or safely break
- no sideways scrolling is necessary to read the complete ordinary output
- final agent response is reachable by scrolling downward

## Scope boundaries

Do NOT:

- start M14
- add PTY/xterm/live shell UI
- redesign unrelated pages
- alter project registry semantics
- alter task/workflow authority
- change Codex model/process policy unless strictly necessary to preserve current working behavior
- weaken redaction or persistence truth
- create installer work
- start M21

## Tracking and log

Update only the current M13/M13E tracking surfaces truthfully.

Strict progress remains `13 / 20 = 65%`.

Create immutable builder log:

`H!veAI/docs/H!veAI/codex-logs/M13E_AGENT_SESSION_OUTPUT_VERTICAL_READER_NATIVE_UX_REMEDIATION_LOG.md`

The log must include:

- exact root cause of the horizontal overflow
- exact files changed
- screenshots/manual native observations if available locally
- each explicit execution gate above with command/result
- exact implementation commit SHA
- publication evidence
- confirmation that M14/M21 were untouched

Final builder state must be exactly:

`M13E REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

Stop after M13E. Do not activate M14.
