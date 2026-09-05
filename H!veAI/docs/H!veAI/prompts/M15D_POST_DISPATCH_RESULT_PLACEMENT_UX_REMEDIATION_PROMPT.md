# M15D Post-Dispatch Result Placement UX Remediation Prompt

Date: 2026-09-05
Product: H!veAI
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Milestone: M15D final visual placement remediation
Authority: authoritative implementation/remediation prompt

## Scope

User native acceptance confirms the M15C handoff works correctly:

- Prompt Engine dispatch succeeds.
- `View result in Agents` works.
- Clicking it opens Agents on the exact just-dispatched session.
- The handoff itself does not create another provider session.

The remaining issue is visual placement only.

Current behavior places the post-dispatch success/result notice and `View result in Agents` action at the very top of Prompt Engine. The user wants that dispatch result to live directly with the dispatch controls.

## Required UX change

Move the post-dispatch success state and `View result in Agents` action out of the page-top/global notice area and place them directly **below the Provider and dispatch section's dispatch action row**.

Desired local structure:

1. Provider buttons: Codex / Claude
2. Dispatch button
3. Immediately below:
   - concise success line such as:
     `Dispatched Claude session <short-id> with exact version <n>.`
   - `View result in Agents` button
4. Version history and provenance may remain below this local result state.

The result should feel like the direct outcome of the dispatch action, not a page-global notification.

## Behavioral requirements

- Do not change dispatch semantics.
- Do not auto-navigate.
- Do not dispatch again when `View result in Agents` is clicked.
- Preserve exact project/session route targeting from M15C.
- Preserve stale-handoff clearing behavior.
- Preserve M15A atomic single-use dispatch.
- Preserve M15B ACL/provider/task-picker behavior.
- Preserve M14E Agents final response presentation.
- Preserve Codex and Claude behavior identically.
- If a general error occurs, page-level error handling may remain where it is; only the successful post-dispatch result/handoff should move locally under Provider and dispatch.

## Visual requirements

- Use existing H!veAI styles/components.
- Keep the local result compact.
- Do not add a new modal, toast system, drawer, or extra page.
- Do not duplicate the same success message at both top and dispatch section.
- Ensure the local result works cleanly at current desktop width and narrow responsive widths.
- Provider section should remain visually balanced after the result block is inserted.

## Required tests

Add/update focused tests proving:

1. successful dispatch renders the success result inside the Provider and dispatch panel;
2. the old page-top success result is absent;
3. `View result in Agents` is rendered inside the Provider and dispatch panel;
4. clicking it still navigates to the exact project/session;
5. clicking it does not invoke dispatch again;
6. Codex and Claude both use the same local result surface;
7. stale handoff state still clears on project change/new draft;
8. M15C route-targeting tests remain green;
9. M15B focused tests remain green;
10. full frontend and Rust regressions remain green.

## Explicit execution gates

1. Fetch `origin/H!veAI`.
2. Fast-forward-only synchronize.
3. Confirm exact `H!veAI` branch.
4. Record starting HEAD/worktree.
5. Preserve unrelated files.
6. Read M15B prompt/log/re-audit.
7. Read M15C prompt/log/re-audit.
8. Confirm user native acceptance of M15C handoff.
9. Confirm M15 remains open for this final visual remediation.
10. Inspect current Prompt Engine notice/result rendering.
11. Identify the exact component/state that places dispatch success at page top.
12. Refactor success rendering into the Provider and dispatch panel.
13. Keep errors separate from local success state.
14. Keep `View result in Agents` adjacent to the local dispatch result.
15. Ensure no duplicate success rendering remains.
16. Preserve exact route-target contract.
17. Preserve no-redispatch navigation behavior.
18. Preserve stale handoff clearing.
19. Preserve M15A dispatch protections.
20. Preserve M15B ACL/provider/task-picker behavior.
21. Preserve M14E Agents result surface.
22. Add/update focused placement test.
23. Add/update no-duplicate-success test.
24. Add/update Codex local-result test.
25. Add/update Claude local-result test.
26. Re-run M15C navigation/no-redispatch test.
27. Run full frontend regression.
28. Run focused Rust M15 tests.
29. Run full serialized Rust regression.
30. Run M15A duplicate/race/replay tests.
31. Run M15B ACL/capability tests.
32. Run M14E final-response regressions.
33. Run TypeScript typecheck.
34. Run frontend production build.
35. Run `npm audit --audit-level=high`.
36. Run Rust fmt check.
37. Run Rust all-targets check.
38. Run Rust pty-support check.
39. Run `git diff --check`.
40. Run publisher rollback harness.
41. Governed-publish stable Tauri EXE.
42. Verify candidate/stable SHA equality.
43. Verify PE/startup/shortcut/icon.
44. Verify no visible console popup.
45. Create immutable M15D implementation log.
46. Commit scoped files only.
47. Push normally, no force.
48. Verify local HEAD equals `origin/H!veAI`.
49. Leave M15 implementation complete pending independent final re-audit.
50. Do not activate M16 in this builder run.
51. Do not start M21.

## Required log

Create:

`H!veAI/docs/H!veAI/codex-logs/M15D_POST_DISPATCH_RESULT_PLACEMENT_UX_REMEDIATION_LOG.md`

Record:

- implementation commit SHA;
- exact old vs new result placement;
- files changed;
- focused test evidence;
- no-redispatch proof;
- regression counts;
- publisher hashes;
- final milestone state.

## Completion boundary

Builder must NOT independently close M15.

Expected state after implementation:

- M15D implementation complete;
- M15 remains OPEN pending independent re-audit;
- M16 remains blocked;
- M21 not started;
- roadmap progress remains `15 / 20 = 75%`.

After independent re-audit, because the user has already confirmed the M15C handoff behavior itself, M15 may close once this local placement change is technically verified and visually accepted.
