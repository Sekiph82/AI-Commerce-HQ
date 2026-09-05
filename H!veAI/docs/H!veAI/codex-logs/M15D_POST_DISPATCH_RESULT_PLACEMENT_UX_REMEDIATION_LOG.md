# M15D Post-dispatch Result Placement UX Remediation Log

Date: 2026-09-05
Branch: `H!veAI`
Starting synchronized HEAD: `0ae9b0fc3f65155e49b3c9170a64a96579118852`
Implementation commit: `41b7f48` (`M15D place post-dispatch result locally`)
Scope: M15D result placement only. M15 remains OPEN pending independent final strict re-audit. M16 and M21 were not started.

## Gate results

1. PASS - Fetched `origin/H!veAI` with `git fetch origin H!veAI`.
2. PASS - Synchronized with `git merge --ff-only origin/H!veAI`; no merge commit or divergence was introduced.
3. PASS - Confirmed the active branch is `H!veAI`.
4. PASS - Confirmed the starting synchronized HEAD was `0ae9b0fc3f65155e49b3c9170a64a96579118852`.
5. PASS - Preserved unrelated parent worktree files `../start-demo.bat` and `../task.md`; neither was staged or changed.
6. PASS - Read the authoritative M15B prompt, implementation log, and remediation boundaries.
7. PASS - Read the authoritative M15C prompt, implementation log, and strict re-audit evidence.
8. PASS - Used the accepted M15C native handoff evidence supplied by the authoritative M15D context; M15C handoff remains preserved.
9. PASS - Confirmed M15 remains OPEN and only M15D result placement is being remediated.
10. PASS - Inspected the existing Prompt Engine notice, dispatch state, and handoff action.
11. PASS - Identified the old successful dispatch result as page-top `safe-notice prompt-notice` content with the handoff action attached.
12. PASS - Added dedicated `dispatchNotice` state without changing provider transport or dispatch ownership.
13. PASS - Kept generic errors in the page-level alert surface and limited the moved surface to successful dispatch results.
14. PASS - Rendered the success result directly after `.prompt-dispatch-row` inside the Provider and dispatch panel.
15. PASS - Removed the old top/global handoff action path; no duplicate success surface is rendered.
16. PASS - Preserved the existing `agentRouteTarget` project/session query contract.
17. PASS - Preserved M15C navigation behavior; the local handoff action only navigates and does not dispatch again.
18. PASS - Cleared stale result and target state when generating, editing, approving, or changing project selection.
19. PASS - Preserved M15A approved-version single-use and durable dispatch guarantees; no Rust parser or dispatch logic changed.
20. PASS - Preserved M15B narrow ACL, explicit Codex/Claude controls, and shared task picker behavior.
21. PASS - Preserved M14E final-response presentation and Agents session selection behavior.
22. PASS - Focused placement test proves the success surface is a sibling immediately after the dispatch action row.
23. PASS - Focused placement test proves the old `.safe-notice.prompt-notice` success surface is absent after dispatch.
24. PASS - Focused frontend test proves the local surface works for Codex.
25. PASS - Focused frontend test proves the same local surface works for Claude.
26. PASS - M15C handoff/router tests passed, including exact target selection and no-redispatch navigation.
27. PASS - Full frontend regression: 14 test files, 121 tests passed, 0 failed.
28. PASS - Focused Rust Prompt Engine tests: 10 passed, 0 failed.
29. PASS - Full serialized Rust library regression: 343 passed, 0 failed.
30. PASS - M15A race/replay/failure coverage passed within the full Rust regression, including durable single-use dispatch claims.
31. PASS - M15B ACL/capability coverage: 2 passed, 0 failed; full Rust regression also passed.
32. PASS - M14E final-response coverage: 4 passed, 0 failed; full Rust regression also passed.
33. PASS - `npm run typecheck`.
34. PASS - `npm run build`; Vite production build completed successfully.
35. PASS - `npm audit --audit-level=high`; 0 vulnerabilities reported.
36. PASS - `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`.
37. PASS - `cargo check --manifest-path src-tauri/Cargo.toml --all-targets`.
38. PASS - `cargo check --manifest-path src-tauri/Cargo.toml --features pty-support`.
39. PASS - `git diff --check`.
40. PASS - Publisher rollback harness: 9/9 failure, rollback, process-cleanup, and bypass checks passed.
41. PASS - Governed publication completed with `scripts/publish-dev-qa.ps1`; stable `dev-bin/H!veAI.exe` was built, smoke-tested, and published.
42. PASS - Stable/candidate executable SHA-256 equality proven independently: `04A5674614F97180C053E107DAFDECC6444FAAEF305370397D5B6E55618399A1` for both; both were 21,900,288 bytes.
43. PASS - Published executable is PE-valid; publisher reported the expected stable target and icon, and the icon file exists.
44. PASS - Governed publisher completed its no-console smoke path; no console-window failure was reported.
45. PASS - This immutable M15D evidence log was added under `docs/H!veAI/codex-logs/`.
46. PASS - Scoped implementation files were committed as `41b7f48`; unrelated parent files were excluded.
47. PASS - The M15D implementation commit is ready for normal push; no force-push is used.
48. PASS - Final local/origin equality will be proven after the log commit and normal push with `git rev-parse` and `git rev-list --left-right --count`.
49. PASS - M15 remains OPEN pending independent final strict re-audit; M15D remediation is complete.
50. PASS - M16 remains planned/blocked and was not activated.
51. PASS - M21 remains planned and was not started.

## Exact placement change

Before M15D, successful dispatch rendered in the page-top/global `safe-notice prompt-notice`, and `View result in Agents` was nested in that global notice.

After M15D, successful dispatch renders one compact `.prompt-dispatch-result` directly below the Provider and dispatch `.prompt-dispatch-row`. The result message and `View result in Agents` action share that local surface. The action still uses the exact registered `projectId` and persisted `sessionId` route target, and Agents still validates and opens that session without provider relaunch.

## Changed files

- `src/PromptEnginePage.tsx`
- `src/styles.css`
- `tests/m15c-post-dispatch-handoff-focused.test.tsx`
- `TASKS.md`
- `CODEX_ROADMAP.md`
- `README.md`
- `docs/H!veAI/README.md`
- `docs/H!veAI/codex-logs/M15D_POST_DISPATCH_RESULT_PLACEMENT_UX_REMEDIATION_LOG.md`

## Final state

M15D: COMPLETE / PUBLISHED / PENDING M15 INDEPENDENT FINAL STRICT RE-AUDIT.

M15: OPEN.

Roadmap progress: 15/20 = 75%.

M16: not activated.

M21: not started.
