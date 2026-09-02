# M14 Native Acceptance Claude Invocation and Session Center UX Audit

Date: 2026-09-02
Product: H!veAI
Branch: `H!veAI`
Authority: user native/visual acceptance evidence after M14A strict re-audit

## Verdict

**FAIL / M14 MUST REMAIN OPEN**

M13 remains PASS/CLOSED. M15-M20 remain blocked. M21 remains not started.

The M14A technical re-audit closed R35-R37, but native user acceptance exposed one functional MAJOR defect and two native UX MAJOR defects in the newly published Agent Session Center.

## User evidence

The user opened the newly published stable H!veAI executable and confirmed:

- Codex and Claude readiness are visible.
- ScrubBots selects Claude as the preferred provider.
- Claude Code version `2.1.248` is visible.
- A real Claude session was started against registered project ScrubBots.
- The Claude session failed immediately with exit code `1` and visible stderr:

```text
Error: When using --print, --output-format=stream-json requires --verbose
```

The session timeline records the fixed Claude argument policy as:

```text
--print --output-format stream-json --no-session-persistence --permission-mode plan --restricted
```

The required `--verbose` flag is absent.

The user also reported that before entering a new prompt, an old persisted session was already expanded into a very long output/timeline surface. After running the failed Claude prompt, a second large session detail surface appeared. The user found both views visually poor and difficult to read.

## Findings

### M14-R38 MAJOR: Real Claude invocation is incompatible with the installed Claude Code CLI

The installed Claude Code `2.1.248` rejects the governed fixed invocation because `--print` plus `--output-format stream-json` requires `--verbose`.

This is a real native acceptance failure, not a hypothetical compatibility concern. The CLI itself produced the diagnostic and H!veAI correctly persisted it.

Required correction:

- Re-inspect the actual installed Claude CLI help/version on the user's machine.
- Correct the fixed governed invocation using only documented flags supported by that exact CLI.
- At minimum, prove whether adding `--verbose` is the correct bounded fix for `--print --output-format stream-json`.
- Do not guess or introduce shell wrappers.
- Preserve bounded stdin prompt transport, registered ACTIVE project confinement, direct native executable resolution, `CREATE_NO_WINDOW`, redaction, owned process semantics, and provider-neutral contracts.
- Run a harmless real Claude operation against ScrubBots or a disposable registered fixture and require truthful `COMPLETED` plus usable output.

### M14-R39 MAJOR: Persisted sessions auto-dominate the Agents page before the user starts/selects a session

On opening Agents, a previously persisted session is already rendered as a very large selected-session detail/timeline/output surface. This makes the page feel as if a session is already running or being forced on the user and pushes the primary operation controls away from the useful viewport.

Required correction:

- Do not auto-expand a persisted session on initial Agents page load.
- Initial selected-session state must be empty unless there is an actually active session that requires attention, and even then the UI must remain compact.
- Render persisted sessions as compact rows/cards with provider, project, status, time, and a deliberate `View`/expand action.
- Selecting one session must not expand every other session.
- Provide explicit close/collapse behavior for the selected session detail.
- Preserve all persisted history and provenance. This is presentation only, not data deletion.

### M14-R40 MAJOR: Session details expose internal raw event JSON as the primary reading experience

The current selected-session UI shows raw timestamped event records, JSON payloads, process-policy details, escaped paths, command metadata, and long identifiers as a giant text wall. The user explicitly rejected this visual experience.

Required correction:

- The default selected-session view must be human-readable and vertically structured.
- Keep a concise header: provider, project name, state, elapsed time, exit code, diagnostic.
- Primary body should show **agent output/conversation** in a readable vertical stream.
- Internal process events, raw JSON, process policy, prompt hashes, executable details, and low-level timeline data must move behind collapsed advanced sections such as `Technical details`, `Timeline`, or `Raw events`.
- No horizontal scrolling for normal content. Long paths/IDs must wrap or be truncated with copy affordance.
- Errors should appear as a compact diagnostic card rather than duplicating the same error in Live output + Error output + raw timeline.
- Completed Codex and Claude sessions must share the same readable presentation model.
- Preserve redaction markers and bounded evidence truth. Do not hide genuine failure diagnostics.

## Acceptance target

M14 cannot close until all of the following are true in the stable native build:

1. Agents opens with a clean compact page and no old session automatically expanded into a giant log.
2. ScrubBots defaults to Claude.
3. Claude readiness/version remains truthful.
4. A harmless real Claude operation starts without CLI argument rejection.
5. The operation reaches `COMPLETED` with exit code `0`, or if external auth/provider genuinely blocks it, H!veAI shows the precise bounded provider diagnostic rather than an invocation syntax error.
6. Agent output is readable vertically.
7. Raw event JSON is hidden behind an explicit advanced disclosure.
8. Persisted sessions remain compact until the user deliberately opens one.
9. No visible console/terminal flash occurs.
10. Existing Codex M13 behavior and M14A R35-R37 boundaries remain intact.

## Current milestone state

**M14: FAIL / NATIVE ACCEPTANCE BLOCKED BY R38-R40**

Strict completed roadmap count remains `14 / 20 = 70%` until M14 receives independent re-audit and user native/visual acceptance.
