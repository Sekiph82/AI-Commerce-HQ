# M14B Claude Real Operation and Agent Session Center Readability Remediation Prompt

You are working only on H!veAI M14 remediation.

Repository root:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Application root:
`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

Branch:
`H!veAI`

Authoritative audit to read first:
`H!veAI/docs/H!veAI/audits/M14_NATIVE_ACCEPTANCE_CLAUDE_INVOCATION_AND_SESSION_CENTER_UX_AUDIT.md`

Prior M14 authority/evidence to preserve:
- `H!veAI/docs/H!veAI/prompts/M14_AGENT_SESSION_CENTER_CODEX_CLAUDE_IMPLEMENTATION_PROMPT.md`
- `H!veAI/docs/H!veAI/prompts/M14A_NATIVE_TEST_PUBLICATION_AND_ACTIVE_PROJECT_CONFINEMENT_REMEDIATION_PROMPT.md`
- `H!veAI/docs/H!veAI/codex-logs/M14_AGENT_SESSION_CENTER_CODEX_CLAUDE_IMPLEMENTATION_LOG.md`
- `H!veAI/docs/H!veAI/codex-logs/M14A_NATIVE_TEST_PUBLICATION_AND_ACTIVE_PROJECT_CONFINEMENT_REMEDIATION_LOG.md`
- M13/M13A/M13B/M13C/M13D/M13E accepted boundaries and audits.

## Objective

Close only M14-R38, M14-R39, and M14-R40.

Do not activate M15. Do not start M21. Do not rewrite historical audit/log evidence.

## Mandatory preflight

1. `git fetch origin H!veAI`.
2. Fast-forward only to `origin/H!veAI`.
3. Verify exact branch `H!veAI`.
4. Preserve unrelated local/untracked files.
5. Read this prompt and the M14 native-acceptance audit in full before editing.
6. Record exact pre-change HEAD.

## R38: Correct the real Claude invocation from actual CLI evidence

The published native build failed a real Claude operation with:

```text
Error: When using --print, --output-format=stream-json requires --verbose
```

The current process policy evidence shows the fixed invocation lacks `--verbose`:

```text
--print --output-format stream-json --no-session-persistence --permission-mode plan --restricted
```

Requirements:

1. Inspect the actual selected native `claude.exe` on this machine.
2. Record exact `claude --version`.
3. Inspect exact help for `--print`, `--output-format`, `--verbose`, `--permission-mode`, `--restricted`, stdin behavior, and any stream-json requirements.
4. Do not infer compatibility from old docs or tests. The installed binary is authority for this remediation.
5. Correct the fixed governed invocation using only verified flags. If `--verbose` is required and sufficient, add it to the fixed argument policy and prove it.
6. Keep prompt transport via bounded stdin. Do not place user prompt text in argv.
7. Keep direct native executable resolution. No `cmd.exe`, PowerShell, `.cmd`, `.ps1`, arbitrary shell, arbitrary executable, arbitrary args, or frontend-controlled process details.
8. Preserve `CREATE_NO_WINDOW`/background-process behavior.
9. Preserve exact ACTIVE registered-project confinement and canonical registered cwd.
10. Preserve owned PID/process lifecycle, bounded output, redaction-before-persistence, durable event truth, retry provenance, and restart recovery.
11. Update tests so the actual governed Claude argv shape is asserted, including the verified stream-json requirement.
12. Run a harmless **real native Claude operation** after the fix. Preferred target is ScrubBots because the user is explicitly testing it and it is already registered. Use a strictly read-only prompt such as:

```text
Inspect this project read-only and summarize its repository structure. Do not modify any files.
```

13. Capture bounded evidence only. Do not persist secrets.
14. Require one of these truthful outcomes:
    - `COMPLETED`, exit code `0`, readable output, no unrelated file changes; or
    - a genuine auth/billing/provider diagnostic from the external provider after invocation syntax is proven valid.
15. An invocation syntax/flag error is not acceptable.

## R39: Stop auto-expanding persisted sessions on Agents load

The user rejected the current behavior where a long historical session is already expanded before a new prompt is entered.

Requirements:

1. On initial Agents page load, no persisted session detail should be automatically expanded.
2. Persisted sessions must appear as compact session rows/cards only.
3. Each compact row should show at most the essential scan information:
   - provider badge,
   - project name,
   - state,
   - start/end or relative timestamp,
   - short operation label.
4. Add a deliberate `View`/expand/select action.
5. Only one selected-session detail may be open at a time.
6. Add an explicit close/collapse action for selected detail.
7. Do not delete history or alter persisted evidence.
8. If there is an actually active/waiting session, it may be highlighted, but it must not produce a giant automatically expanded raw log surface.
9. A fresh Agents page should prioritize Provider readiness + Start owned session + compact session history.

## R40: Replace raw-event wall with a readable session experience

The current selected-session view exposes escaped JSON, process policy, hashes, long IDs, commands, paths, timestamps, and duplicated error output as the primary UI. This failed user visual acceptance.

Implement a two-layer session reader:

### Default human-readable layer

Keep a compact top summary:
- Provider
- Project display name
- State badge
- Started / ended / elapsed
- Exit code only when relevant
- One concise diagnostic card when failed

Then show **Agent output** as the primary body:
- vertical reading direction,
- wrapped text,
- no horizontal scrolling,
- preserve natural paragraphs/newlines,
- readable Claude/Codex assistant output,
- compact tool/action summaries only when useful,
- do not dump raw JSON by default.

If stderr contains a failure, show the useful diagnostic once. Avoid duplicate copies in `Live output`, `Error output`, and timeline simultaneously.

### Advanced technical layer

Move low-level evidence behind collapsed disclosures, for example:
- `Technical details`
- `Timeline`
- `Raw events`
- `Git evidence`

Advanced details may contain:
- event types,
- prompt hash/reference,
- process policy,
- raw redacted event JSON,
- working directory,
- provider version,
- changed-file authority.

These must be collapsed by default.

### Readability rules

1. Normal user content must not require horizontal scrolling.
2. Long paths and identifiers must wrap or use bounded ellipsis + copy interaction.
3. Do not concatenate provider + UUID + state into one unspaced string.
4. Use labels and spacing that visually separate metadata.
5. Do not render empty giant terminal/output boxes.
6. Do not show Stop/Resume controls when the session state makes them invalid. A terminal `COMPLETED` or `FAILED` session must not prominently offer `Stop owned session`.
7. Resume must remain truthful to provider capability. If unsupported, do not present a misleading active Resume button.
8. Keep Codex and Claude presentation consistent.
9. Preserve redaction markers visibly when they occur.
10. Preserve M13E vertical-reader acceptance rather than regressing to horizontal logs.

## Data and lifecycle truth

- Do not fabricate parsed agent text. Derive readable presentation from persisted stdout/stderr/events.
- Do not discard technical evidence. Hide it behind advanced disclosure, do not delete it.
- Do not trust agent prose for Git changes. Preserve Git Engine authority.
- A session's state, exit code, diagnostics, project identity, provider, and timestamps remain backend truth.

## Tests required

Add/adjust tests for at least:

1. Claude fixed argv includes every installed-CLI-required flag, especially verified `--verbose` if required.
2. Claude prompt remains stdin-bounded and absent from argv.
3. No shell/arbitrary executable/arbitrary args regression.
4. ACTIVE/MISSING/ARCHIVED/unknown project confinement remains exact.
5. Claude success stream parsing into readable agent output.
6. Claude stderr syntax/auth/failure diagnostic presentation.
7. No persisted session auto-selected on initial page load.
8. Compact persisted session list.
9. Deliberate session selection and explicit close.
10. Only one detail expanded at once.
11. Raw events collapsed by default.
12. Timeline collapsed by default.
13. Failed diagnostic shown once in default view.
14. No giant empty output surface.
15. Completed/failed sessions do not expose invalid stop controls.
16. Unsupported resume is not presented as actionable.
17. Long Windows paths/UUIDs wrap or truncate without horizontal page overflow.
18. Existing Codex completed session remains readable.
19. Redaction markers remain visible.
20. Git evidence remains authority-backed.

## Explicit execution gates

All gates below are mandatory unless explicitly marked user acceptance.

1. PASS: fetch + ff-only sync.
2. PASS: exact `H!veAI` branch.
3. PASS: unrelated files preserved.
4. PASS: M14B prompt and audit read in full.
5. PASS: reproduce pre-fix Claude `stream-json requires --verbose` failure or preserve existing native evidence if reproduction would be redundant and costly.
6. PASS: actual `claude.exe` version recorded.
7. PASS: actual installed CLI help inspected for all governed flags.
8. PASS: corrected fixed Claude invocation implemented from evidence.
9. PASS: prompt remains stdin-only and bounded.
10. PASS: no-shell/no-arbitrary-executable/no-arbitrary-args security review.
11. PASS: direct native Claude resolver tests.
12. PASS: exact ACTIVE project confinement tests.
13. PASS: cross-project task/session authorization tests.
14. PASS: redaction-before-persistence tests.
15. PASS: output/event bound tests.
16. PASS: retry/recovery lifecycle tests.
17. PASS: focused Claude backend tests execute, not compile-only.
18. PASS: focused Codex backend regressions execute.
19. PASS: full serial Rust regression with required M14 features executes with 0 failures.
20. PASS: focused Agent Session Center frontend tests.
21. PASS: full frontend suite.
22. PASS: no auto-selected persisted session initial-state test.
23. PASS: compact persisted session list test.
24. PASS: single explicit selected-detail test.
25. PASS: close/collapse selected-detail test.
26. PASS: human-readable output parser/presentation test for Claude.
27. PASS: human-readable output presentation test for Codex.
28. PASS: raw events/timeline advanced sections collapsed by default.
29. PASS: failed error shown once in default view.
30. PASS: invalid Stop/Resume actions hidden/disabled according to state/capability.
31. PASS: long IDs/paths do not cause page-level horizontal overflow.
32. PASS: `npm run typecheck`.
33. PASS: `npm run build`.
34. PASS: `npm audit --audit-level=high` with no high vulnerabilities.
35. PASS: cargo fmt check.
36. PASS: cargo check all targets and required feature set.
37. PASS: `git diff --check`.
38. PASS: real Claude readiness/version in native environment.
39. PASS: harmless real Claude operation gets past argument validation and reaches truthful provider execution.
40. PASS: if provider/auth permits, real Claude operation completes exit 0 with readable output; otherwise document genuine provider/auth blocker with exact bounded diagnostic. Invocation syntax failure is FAIL.
41. PASS: Git diff authority proves harmless read-only operation made no unrelated changes.
42. PASS: no visible console window during readiness or Claude operation.
43. PASS: governed publisher rollback/failure harness.
44. PASS: governed production `--no-bundle` publication.
45. PASS: candidate emits fresh `HIVEAI_FRONTEND_READY`.
46. PASS: stable executable emits fresh `HIVEAI_FRONTEND_READY` after swap.
47. PASS: stable bytes equal accepted candidate bytes.
48. PASS: no forbidden development listener.
49. PASS: M15-M20 not activated.
50. PASS: M21 not started.
51. PENDING USER ACCEPTANCE: user opens stable Agents page and confirms clean compact initial visual.
52. PENDING USER ACCEPTANCE: user starts ScrubBots Claude session and confirms readable vertical output plus truthful final state.

## Publication

Do not claim M14B complete unless the stable `H!veAI/dev-bin/H!veAI.exe` is governed-published and contains the fix.

Preserve startup video/audio/icon acceptance and existing desktop shortcut target.

## Required immutable log

Create:

`H!veAI/docs/H!veAI/codex-logs/M14B_CLAUDE_REAL_OPERATION_AND_SESSION_CENTER_READABILITY_REMEDIATION_LOG.md`

The log must include:
- pre/post commit IDs,
- exact installed Claude version,
- exact relevant CLI help evidence,
- pre-fix failure evidence,
- corrected fixed argv policy,
- real Claude operation result,
- Rust/frontend test counts,
- publication candidate/stable hashes,
- all 52 gate results,
- any genuine remaining user-acceptance-only items.

## Final state

Do not close M14 yourself.

The only acceptable builder terminal statement is:

`M14B REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

M15 and M21 must remain untouched.
