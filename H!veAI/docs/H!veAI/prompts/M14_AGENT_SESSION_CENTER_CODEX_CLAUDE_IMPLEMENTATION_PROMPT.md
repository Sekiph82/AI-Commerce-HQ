# M14 Agent Session Center - Codex + Claude Implementation Prompt

Date: 2026-09-02
Product: H!veAI
Branch: `H!veAI`
Milestone: M14 - Agent Session Center
Authority: authoritative implementation prompt

## 0. Milestone transition authority

The user has now personally accepted the published M13 native behavior, including:

- Codex readiness/version resolution
- no post-startup visible console flashes
- one real ScrubBots Codex operation completing successfully with exit code 0
- truthful persisted session state/output
- the M13E full-width vertical persisted-session reader with no required sideways scrolling

Independent strict re-audits for the accepted M13 remediation chain are PASS. Therefore this M14 implementation run is authorized to:

1. mark M13 PASS/CLOSED using the already-existing audit/native-acceptance evidence,
2. advance strict completed roadmap progress from `13 / 20 = 65%` to `14 / 20 = 70%`,
3. activate M14,
4. implement M14 only,
5. leave M15-M20 blocked/planned and M21 planned/not started.

Do not rewrite historical M13/M13A/M13B/M13C/M13D/M13E evidence. Preserve it as immutable provenance.

## 1. Product goal

Build the H!veAI Agent Session Center as the native live-session surface for BOTH:

- `CODEX`
- `CLAUDE`

The user has been developing ScrubBots with Claude and explicitly requires Claude to remain a first-class builder option rather than forcing ScrubBots onto Codex.

M14 must extend the provider-neutral native adapter foundation created in M13. Do not create a separate Claude-only lifecycle system or a competing session truth store.

The end state should let the user choose Codex or Claude for a registered project, start and observe a real provider-owned session, inspect a live terminal/event stream, view changed files and Git evidence, handle waiting/permission states, stop/retry safely, and recover truthfully after H!veAI restarts.

## 2. Non-negotiable inherited boundaries

Preserve all accepted M13 boundaries unless this prompt explicitly extends them for M14:

- registered-project confinement
- canonical cwd validation
- exact project/task ownership validation
- provider-neutral native lifecycle contract
- no generic arbitrary executable launcher
- no generic arbitrary argument launcher
- no shell-wrapper execution primitive
- no arbitrary PID control endpoint
- native process ownership
- bounded capture/history
- pre-persistence redaction
- durable event truth
- truthful terminal states
- deterministic truncation evidence
- clean-stop-first / bounded owned-tree escalation semantics
- Windows native executable resolution
- `CREATE_NO_WINDOW` for non-PTY background/helper processes
- existing `agent_sessions` / `agent_events` authority unless a narrow additive schema migration is genuinely required
- M12 Project Cockpit behavior
- M13 persisted-session vertical reader
- startup media/icon behavior
- project dashboard authority
- current Git Engine authority
- governed publication workflow

A PTY is introduced in M14, but it MUST NOT become a generic shell feature. The PTY may only be attached to an H!veAI-owned, allowlisted agent provider session created through the provider contract.

## 3. Provider model: Codex + Claude

### 3.1 Common provider contract

Extend the existing provider-neutral adapter contract rather than replacing it.

The common native provider/session semantics must cover at minimum:

- provider identity
- availability/readiness
- version
- authentication/readiness state when truthfully knowable
- capability metadata
- start
- list
- read/status
- live stream subscription/event transport
- resize when PTY-backed
- permission/waiting state
- stop
- retry with provenance
- resume only if genuinely supported by that provider
- recovery/reconciliation after H!veAI restart
- terminal/final state normalization

Provider-specific details may remain behind provider implementations.

### 3.2 Codex provider

Preserve the accepted working Codex implementation and real-operation behavior.

Do not regress:

- native `codex.exe` resolver
- bounded safe invocation
- compatible model/user-config policy established in M13D
- successful real session completion
- session output persistence/redaction
- no visible console flashes

If M14 PTY support requires a different Codex process mode, first prove why. Prefer adapting the live session surface around the already-working Codex execution path rather than destabilizing it.

### 3.3 Claude provider

Add a real native Claude provider implementation.

Before coding provider flags, inspect the actually installed Claude CLI on the target Windows host. Discover and record:

- exact executable selected
- exact version output
- supported noninteractive/interactive invocation shapes
- stdin behavior
- project/cwd behavior
- permission model
- session/resume capabilities
- output format options if any
- whether PTY is required for the desired interactive behavior
- whether user configuration changes invocation behavior
- available safe ways to avoid command-line prompt leakage

Do NOT invent Claude CLI flags or model names.

Use only a bounded, allowlisted Claude invocation shape proven against the installed CLI.

Do not expose credentials, API keys, auth tokens, config secrets, or environment secrets in persisted output, diagnostics, command lines, UI, or logs.

If Claude authentication cannot be safely preflighted, report an explicit `AUTH_UNKNOWN`-style state and let the real bounded operation determine it truthfully.

### 3.4 Provider selection in UI

Agents / Agent Session Center must offer a clear provider selector:

- Codex
- Claude

The selected provider must be visible on every live/persisted session row/card.

Use distinct provider badges, but keep session state vocabulary consistent.

### 3.5 Per-project preferred provider

Add a persisted per-project preferred agent provider if this can be done as a narrow extension of the existing registry/settings model.

Allowed values initially:

- `CODEX`
- `CLAUDE`
- unset / ask each time

Do not hardcode project names into runtime provider logic.

For the user's current local registered ScrubBots project, if it exists and has no explicit agent-provider preference, set its persisted preference to `CLAUDE` through the same normal settings/storage path used by the UI. This is a one-time user-intent migration/configuration action, NOT name-based runtime branching.

The user must still be able to choose Codex manually for ScrubBots per session.

## 4. M14.01 - PTY foundation

Implement a native Rust PTY/process manager suitable for provider-owned interactive sessions.

Requirements:

- Windows support first
- process ownership remains native
- provider session owns its PTY/process tree
- no arbitrary shell creation API
- no arbitrary executable/args from frontend
- bounded read loop
- deterministic shutdown
- terminal resize support
- explicit session association
- explicit project association
- explicit provider association
- no detached/orphaned provider process after normal finalization

Use a mature Rust PTY library if appropriate, but keep the dependency footprint justified and audited.

If a provider does not need PTY, it may keep a non-PTY owned-process transport while still feeding the common live session center.

## 5. M14.02 - Session list and status

Build a real Agent Session Center view that shows active and persisted sessions across providers.

Each session must expose at least:

- provider
- project
- optional task
- operation kind
- state
- started time
- elapsed timer while active
- ended time
- exit code if terminal
- waiting/permission/crash state
- diagnostic code/message when relevant
- prompt/reference provenance without leaking full protected content where unsafe
- provider version reference

Required normalized states should cover at minimum:

- STARTING
- RUNNING
- WAITING_PERMISSION
- STOPPING
- COMPLETED
- FAILED
- STOPPED
- CRASHED

Do not fake support for a provider state the provider cannot expose.

## 6. M14.03 - Live terminal

Add an xterm.js terminal surface for PTY-backed provider sessions.

Critical security boundary:

This is an agent-session terminal, NOT a general-purpose terminal page.

Frontend may NOT supply:

- arbitrary executable paths
- arbitrary shell names
- arbitrary startup command vectors
- arbitrary PIDs

The terminal attaches only to a session that the native provider manager already owns.

Requirements:

- live provider output
- user input only where the provider/session contract permits it
- bounded retained terminal history
- terminal resize
- automatic follow-tail with user-controlled scrollback
- clear provider/project/session identity
- no page-level horizontal overflow
- appropriate text wrapping/terminal behavior
- redaction/persistence boundaries clearly separated from transient display where needed

Do not persist raw secrets merely because they appeared in a PTY stream.

## 7. M14.04 - Session timeline

Add a structured session timeline adjacent to or below the terminal.

Timeline should combine durable evidence such as:

- session started
- provider readiness/version
- prompt reference/hash/version
- agent messages when safely represented
- tool/command events
- permission requested
- permission approved/denied
- Git snapshot/diff events
- test events
- stop/retry/recovery events
- terminal state

Preserve event ordering and provenance.

Do not infer success from agent prose when native/Git/test evidence disagrees.

## 8. M14.05 - Diff and changed files

Add project/session changed-file evidence.

Rules:

- reuse the existing H!veAI Git Engine as authority
- compare pre-session and post/current Git evidence where possible
- show changed file paths and diff access
- distinguish staged, unstaged, untracked, and conflicted evidence when available
- do not trust the agent saying "I changed X" as proof
- avoid mutating Git solely to produce the view

For long diffs, preserve the existing safe bounded/readable presentation conventions.

## 9. M14.06 - Stop, retry, resume, recovery

### Stop

Preserve the M13 clean-stop-first / bounded escalation rules.

For PTY-backed providers, use the provider/session-appropriate graceful signal if technically supported, then bounded escalation on the owned process tree only.

### Retry

Add retry for failed/stopped operations with explicit provenance:

- source session ID
- source prompt/version reference
- provider
- project
- task
- retry timestamp

Retry must create a NEW session. Never mutate historical session identity.

### Resume

Only expose Resume when the selected provider has a real, verified resume capability that can be safely bound to the original project/session.

If unsupported, keep it explicitly unsupported. Do not leave a dead Resume button that always errors.

### Restart recovery

On H!veAI restart:

- reconcile persisted transient sessions
- truthfully distinguish recoverable, orphaned, crashed, and externally-gone processes
- never attach to arbitrary unrelated PIDs
- do not silently convert unknown state into COMPLETED

## 10. M14.07 - Permission UI

Implement explicit provider permission/request attention UI where the provider exposes such a state.

Requirements:

- show what action is requesting permission at a bounded/safe level
- show provider/project/session identity
- explicit Approve / Deny
- persist the user's decision as session evidence
- native notification when a background session is waiting for attention, if notifications are available and permitted
- no auto-approve of broad/dangerous permission requests

Do not invent a fake permission layer over providers that do not expose a controllable permission mechanism. In that case show truthful provider limitations.

## 11. User experience

The M14 Agent Session Center should feel like one coherent operations room, not separate Codex and Claude pages.

Recommended structure:

- top: provider readiness cards for Codex and Claude
- start operation: Project, Provider, optional Task, Prompt
- active sessions section
- persisted/recent sessions section
- selected session detail
- live terminal where applicable
- event timeline
- changed files / diff
- diagnostics and recovery actions

Keep the M13E vertical persisted reader available for completed/non-PTY evidence.

Provider badges should make Codex vs Claude obvious without duplicating the entire screen.

## 12. ScrubBots acceptance scenario

A mandatory native acceptance scenario for M14 is ScrubBots with Claude.

Using the registered local ScrubBots project:

1. provider preference displays `CLAUDE` after the one-time persisted preference setup,
2. user can still switch provider to `CODEX`,
3. start a harmless Claude operation scoped to ScrubBots,
4. H!veAI shows the Claude session as owned by ScrubBots,
5. live output/timeline becomes visible,
6. no unrelated project is accessible through the session contract,
7. no unexpected console window flashes,
8. final status is truthful,
9. output remains readable vertically,
10. Git changed-file evidence truthfully reports no changes for a read-only inspection prompt.

Use a harmless prompt for acceptance, for example a read-only repository structure summary. Do not modify ScrubBots merely to prove Claude connectivity.

## 13. Security tests required

Add adversarial coverage for at least:

- frontend cannot choose executable path
- frontend cannot supply raw provider argument vector
- frontend cannot request arbitrary shell
- frontend cannot control arbitrary PID
- provider mismatch rejection
- session/project mismatch rejection
- task/project mismatch rejection
- cwd escape rejection
- symlink/canonical-path escape where relevant
- cross-project session access rejection
- stale/replayed permission decision rejection
- secret-like output redaction before persistence
- secret split across stream chunks
- bounded event/history limits
- terminal resize only for owned PTY session
- stop only for owned provider session
- retry creates new provenance-linked session
- recovery cannot adopt unrelated processes
- Claude resolver rejects unsafe shim/wrapper candidates when direct native execution is required
- Codex M13 resolver/security tests remain green

## 14. Explicit execution gates

Run and record every gate individually. Do not summarize them as "all tests pass".

1. `git fetch origin H!veAI`.
2. Synchronize using fast-forward only.
3. Confirm exact branch `H!veAI`.
4. Confirm scoped worktree and preserve unrelated user files.
5. Read this authoritative prompt from the synchronized checkout in full.
6. Verify M13 strict audits and recorded user native acceptance before marking M13 closed.
7. Update canonical tracking: M13 PASS/CLOSED, M14 ACTIVE/IMPLEMENTING, strict progress `14 / 20 = 70%`.
8. Inspect current provider-neutral adapter/session schema before editing.
9. Inspect the installed Codex CLI and preserve current accepted invocation behavior.
10. Discover the installed Claude CLI executable(s), version, invocation help, stdin behavior, auth/readiness behavior, and safe session capabilities without leaking credentials.
11. Prove provider resolver behavior with disposable fixtures.
12. Add/finalize common provider contract tests for Codex + Claude.
13. Add PTY manager lifecycle tests.
14. Add owned-process/PTY resize tests.
15. Add provider/session/project/task confinement adversarial tests.
16. Add Claude start/status/stop/recovery focused tests using a controlled helper fixture where real external behavior is nondeterministic.
17. Add at least one real Claude readiness/version probe on the target host when Claude CLI is installed.
18. If safely feasible, run one harmless real Claude operation in a disposable or explicitly safe registered fixture; otherwise record the exact external blocker and do not fake PASS.
19. Preserve and run focused Codex M13/M13A/M13B/M13C/M13D backend tests.
20. Run focused M14 Rust tests.
21. Run full Rust library regression serially.
22. Run frontend focused Agent Session Center tests.
23. Run full frontend test suite.
24. Run TypeScript typecheck.
25. Run frontend production build.
26. Run `npm audit --audit-level=high`.
27. Run `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`.
28. Run `cargo check --manifest-path src-tauri/Cargo.toml --all-targets`.
29. Run `git diff --check`.
30. Run security review for arbitrary executable/shell/PID/args surfaces.
31. Run secret/redaction adversarial tests for both providers.
32. Run bounded terminal/history/event retention tests.
33. Run retry provenance tests.
34. Run restart/orphan recovery tests.
35. Run Git changed-file/diff authority tests.
36. Run permission UI/decision persistence tests for any provider permission mechanism actually supported.
37. Run no-visible-console native smoke for background helper/readiness paths.
38. Run PTY live-session native smoke where technically feasible.
39. Run governed publisher failure/rollback harness.
40. Run governed production Tauri `--no-bundle` publication.
41. Validate stable executable PE, shortcut target, shortcut icon, startup smoke, and candidate/stable SHA equality.
42. Manually inspect normal laptop viewport for provider selector, active session list, live terminal, timeline, diff/changed files, and persisted-session readability when native tooling permits.
43. Execute the ScrubBots + Claude native acceptance scenario when safely feasible; record exactly which portions require user acceptance.
44. Confirm M15-M20 were not activated.
45. Confirm M21 was not started.
46. Confirm final builder state remains M14 implementation complete but pending independent strict audit and user native/visual acceptance.

## 15. Required tests/audit evidence

At minimum create/extend focused evidence for:

- provider-neutral Codex + Claude dispatch
- provider selector UI
- preferred provider persistence
- ScrubBots preference behavior without runtime name hardcoding
- Claude readiness/version
- Claude bounded invocation
- PTY lifecycle
- live output
- terminal resize
- active session timers/status
- structured timeline
- changed files/diff
- permission decision evidence where supported
- stop
- retry provenance
- resume truthfulness
- restart recovery
- no arbitrary process surface
- redaction
- bounded retention
- existing Codex behavior regression

## 16. Tracking and completion state

Update only truthful current milestone/tracking surfaces.

At the beginning of this run, after confirming evidence:

- M13 = PASS/CLOSED
- strict completed progress = `14 / 20 = 70%`
- M14 = ACTIVE / IMPLEMENTING

At the end of a successful builder run:

- M13 remains PASS/CLOSED
- M14 = `IMPLEMENTATION COMPLETE / PENDING INDEPENDENT STRICT AUDIT + USER NATIVE/VISUAL ACCEPTANCE`
- strict completed progress remains `14 / 20 = 70%`
- M15-M20 remain planned/blocked
- M21 remains planned/not started

Do NOT mark M14 PASS/CLOSED yourself.

## 17. Immutable builder log

Create:

`H!veAI/docs/H!veAI/codex-logs/M14_AGENT_SESSION_CENTER_CODEX_CLAUDE_IMPLEMENTATION_LOG.md`

The log must include:

- synchronized preflight
- M13 closure evidence references
- exact files/schema changes
- Codex discovery/invocation preservation evidence
- Claude executable/version/help/discovery evidence
- Claude invocation/auth/PTY decisions and why
- provider contract design
- PTY design
- permission model truth
- ScrubBots provider preference implementation
- exact commands/results for all 46 gates
- real provider probe evidence separated from controlled fixture evidence
- native publication evidence
- exact implementation commit SHA
- explicit statement that M15-M21 were not started

Final builder state must be exactly:

`M14 IMPLEMENTATION COMPLETE / PENDING INDEPENDENT STRICT AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

Stop after M14. Do not activate M15 or M21.
