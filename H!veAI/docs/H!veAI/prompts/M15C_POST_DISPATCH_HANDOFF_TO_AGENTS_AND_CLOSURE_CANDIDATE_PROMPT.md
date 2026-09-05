# M15C Post-Dispatch Handoff to Agents and M15 Closure-Candidate Prompt

Date: 2026-09-05
Product: H!veAI
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Milestone: M15C final UX handoff remediation
Authority: authoritative implementation/remediation prompt

## 0. Scope authority

User native acceptance now confirms:

- Prompt Engine ACL generation works.
- Vertical Context -> Review/Approve -> Provider/Dispatch layout is acceptable.
- Bulk Edit task picker is materially improved/acceptable.
- Explicit Codex / Claude provider selection is acceptable.
- Prompt generation, approval, and a real Codex dispatch succeeded.
- The resulting Codex session produced a readable final answer in Agents.

The user explicitly does **not** want to manually re-run the duplicate-dispatch acceptance scenario. Keep the existing M15A durable single-use dispatch implementation and automated race/replay tests as the acceptance evidence for that invariant. Do not weaken or remove those protections.

One UX gap remains:

After a successful Prompt Engine dispatch, the user is told only that a session was dispatched. Add a direct **View result in Agents** / **Open session** handoff that navigates to Agents and automatically opens the exact just-dispatched session.

This is the final M15 closure candidate.

Do not start M16 work in this implementation.
Do not start M21.

## 1. Required user flow

The final primary Prompt Engine flow must be:

`Generate -> Review -> Approve -> Dispatch -> View AI result`

After successful dispatch:

1. keep the success notice;
2. surface a clear primary or secondary action labeled **View result in Agents** (or **Open session** with equivalent clarity);
3. the action must navigate to `/agents`;
4. Agents must automatically select the exact project and exact session returned by the successful dispatch;
5. the selected session detail / Current conversation must open immediately;
6. if the provider session is still RUNNING, the normal Agents polling must continue and the final response should appear when it completes;
7. if it is already COMPLETED, its final response should be visible without the user manually finding the session in history.

Do not auto-navigate immediately after dispatch. The user must choose the handoff action.

## 2. Routing / targeting contract

Use a bounded, inspectable navigation contract.

Preferred implementation:

`/agents?projectId=<registered-project-id>&sessionId=<owned-session-id>`

Equivalent React Router state is allowed only if it remains deterministic and testable. Query params are preferred because they are observable and refresh-safe.

Agents must treat route targeting as a hint, not authority:

- projectId must match a registered project visible to Agents;
- sessionId must be found in that project's persisted owned sessions;
- never open a session belonging to another project;
- never accept arbitrary filesystem paths, process IDs, providers, commands, or prompt text from the URL;
- invalid/missing target IDs must fail safely and leave normal Agents behavior usable.

After a valid target is resolved, it is acceptable to remove/replace the one-shot query params to prevent later polling/navigation from repeatedly forcing selection, as long as the selected session remains open.

## 3. Prompt Engine behavior

In `PromptEnginePage`:

- retain the exact successful `dispatchPrompt` result in bounded React state sufficient to navigate:
  - project ID,
  - session ID,
  - provider,
  - prompt/version reference if useful for display only;
- show **View result in Agents** only after a successful dispatch;
- clear/reset the handoff state when:
  - project changes,
  - a new draft is generated,
  - the active prompt/version changes in a way that makes the previous handoff stale;
- do not show a stale button pointing to an unrelated earlier session;
- do not change dispatch semantics;
- do not auto-dispatch;
- do not create a second session when the handoff button is clicked.

The button is navigation only.

## 4. Agents auto-selection behavior

In `Agents`:

- read the target project/session from the bounded navigation contract;
- if target project differs from current selected project, switch to the target registered project through the existing registry selection path;
- wait for that project's sessions to load;
- select the exact target session when it becomes available;
- open the existing Current conversation/session detail surface;
- preserve existing session polling;
- preserve M14E final-response projection and technical-detail disclosures;
- preserve manual session selection after the initial handoff;
- do not repeatedly override the user's later manual selection.

If the target session cannot be resolved after the normal initial load, show a bounded non-destructive notice such as:
`The dispatched session could not be found for this project.`

Do not crash, redirect loops, or silently open a different session.

## 5. UX details

Prompt Engine success state should read naturally, for example:

`Dispatched Codex session <short-id> with exact version 1.`

Then show:

`View result in Agents`

Use existing H!veAI buttons/icons/styles. Do not introduce a new visual system.

On Agents, the user should land on the exact selected session with the human-readable final answer as the primary content. Technical details, Timeline, Raw events, and Git evidence remain secondary/collapsed as already accepted.

## 6. Preserve M15 guarantees

Do not regress:

- M15A atomic/single-use durable dispatch reservation;
- exact prompt/version/body hash/provider provenance;
- explicit human approval;
- project/task confinement;
- M15B narrow Prompt Engine ACL;
- explicit Codex/Claude provider control;
- bounded/materialized context;
- M14E dedicated final assistant response;
- secret-safe persistence;
- no arbitrary shell/process surface;
- no visible background console popups;
- governed publication.

The user has waived **manual native re-execution** of duplicate-dispatch acceptance. Automated duplicate/race/replay tests remain mandatory and must stay green.

## 7. Required focused tests

Add frontend/router tests covering at least:

1. successful Prompt Engine dispatch reveals **View result in Agents**;
2. the button carries the exact returned project/session target;
3. clicking the button performs navigation only and does not dispatch again;
4. changing project/new draft clears stale handoff state;
5. Agents with a valid project/session route target selects the exact session;
6. Agents never selects a session from the wrong project;
7. invalid session target fails safely;
8. after initial target selection, user can manually select another session without route logic snapping back;
9. a RUNNING targeted session remains selected while polling updates it;
10. M14E final response rendering remains unchanged;
11. existing M15A duplicate-dispatch/race/replay tests remain green;
12. existing M15B ACL/provider/task-picker tests remain green.

## 8. Native acceptance scenario

If a safe already-dispatched session exists in the test environment, reuse it for route-targeting validation without launching another provider.

Otherwise perform one bounded safe dispatch using an available provider.

Required native UX evidence:

1. Prompt Engine successful dispatch state displays **View result in Agents**.
2. Click it.
3. H!veAI navigates to Agents.
4. Exact just-dispatched session is selected automatically.
5. Human-readable agent result is visible or, if still running, its live Current conversation is visible and later completes in place.
6. No manual history hunting is required.
7. No second provider session is created by the navigation action.

Do not require the user to manually test duplicate dispatch again.

## 9. Explicit execution gates

1. Fetch `origin/H!veAI`.
2. Fast-forward-only synchronize.
3. Confirm exact `H!veAI` branch.
4. Record starting HEAD/worktree.
5. Preserve unrelated files.
6. Read M15 authoritative prompt.
7. Read M15 strict audit.
8. Read M15A prompt/log/re-audit.
9. Read M15B prompt/log/re-audit.
10. Record user native acceptance evidence from M15B.
11. Confirm M15 remains OPEN before M15C.
12. Confirm M16 not activated.
13. Confirm M21 not started.
14. Inspect current Prompt Engine dispatch result type.
15. Inspect current Prompt Engine successful-dispatch state handling.
16. Inspect App/React Router routes.
17. Inspect Agents project/session selection lifecycle.
18. Define bounded handoff target contract.
19. Implement post-dispatch handoff state.
20. Add **View result in Agents** action.
21. Ensure handoff click performs navigation only.
22. Clear stale handoff on project change.
23. Clear stale handoff on new draft/version replacement.
24. Add Agents route-target parsing.
25. Validate target project against registry records.
26. Load/switch exact target project through existing selection path.
27. Resolve exact session only inside target project.
28. Auto-select exact target session.
29. Preserve polling for RUNNING target.
30. Prevent repeated route logic from overriding manual later selection.
31. Handle invalid target safely.
32. Preserve normal direct `/agents` behavior with no params.
33. Preserve M14E Current conversation UI.
34. Preserve advanced disclosures.
35. Preserve Prompt Engine ACL.
36. Preserve M15A atomic/single-use dispatch.
37. Preserve provider selection.
38. Add focused Prompt Engine handoff test.
39. Add navigation-does-not-redispatch test.
40. Add stale-handoff reset test.
41. Add valid Agents target-selection test.
42. Add wrong-project target rejection test.
43. Add invalid-session safe-failure test.
44. Add manual-selection-after-handoff test.
45. Add RUNNING polling target test.
46. Run focused M15/M15C frontend tests.
47. Run full frontend regression.
48. Run focused Rust M15 tests.
49. Run full serialized Rust regression.
50. Run M15A duplicate/race/replay tests.
51. Run M15B ACL/capability tests.
52. Run M14E final-response regressions.
53. Run TypeScript typecheck.
54. Run frontend production build.
55. Run `npm audit --audit-level=high`.
56. Run Rust fmt check.
57. Run Rust all-targets check.
58. Run Rust pty-support check.
59. Run `git diff --check`.
60. Run security review for route-target confinement.
61. Verify URL/state never carries prompt body/secrets/filesystem paths.
62. Run publisher rollback harness.
63. Governed-publish stable Tauri `--no-bundle` EXE.
64. Verify candidate/stable SHA equality.
65. Verify PE/startup/shortcut/icon.
66. Verify no visible console popup.
67. Perform native post-dispatch handoff acceptance if safely feasible.
68. Verify no second session is created by handoff navigation.
69. Create immutable M15C implementation log.
70. Commit scoped files only.
71. Push normally, no force.
72. Verify local HEAD equals `origin/H!veAI`.
73. Leave M15 implementation complete pending independent final re-audit.
74. Do not activate M16 in this builder run.
75. Do not start M21.

## 10. Required log

Create:

`H!veAI/docs/H!veAI/codex-logs/M15C_POST_DISPATCH_HANDOFF_TO_AGENTS_LOG.md`

Record:

- implementation commit SHA;
- files changed;
- exact navigation contract;
- target-validation behavior;
- tests and counts;
- proof handoff navigation does not dispatch again;
- native handoff evidence if available;
- publisher hashes;
- final milestone state.

## 11. Completion boundary

Builder must NOT independently declare M15 closed.

Expected builder final state:

- M15C implementation complete;
- M15 remains OPEN pending independent final re-audit;
- M16 remains blocked;
- M21 not started;
- progress remains `15 / 20 = 75%`.

After independent re-audit and user confirmation of the **View result in Agents** handoff, M15 may be closed and M16 activated.
