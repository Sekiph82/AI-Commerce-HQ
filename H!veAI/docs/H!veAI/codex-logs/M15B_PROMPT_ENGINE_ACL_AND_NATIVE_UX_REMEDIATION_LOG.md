# M15B Prompt Engine ACL and Native UX Remediation Log

Date: 2026-09-05
Branch: H!veAI
Authoritative prompt: `docs/H!veAI/prompts/M15B_PROMPT_ENGINE_ACL_AND_NATIVE_UX_REMEDIATION_PROMPT.md`
Scope: M15-R56 through M15-R58 only
Status: M15B remediation complete; M15 remains OPEN pending independent re-audit and user native acceptance.

## Synchronization and boundaries

- Ran `git fetch origin H!veAI`.
- Fast-forward synchronized local `H!veAI` from `63b363b345274b931bafccd3d331910160149019` to `e9f7d0e7b03ea655cbafc899ddb5749b87d57890` before implementation.
- Starting synchronized `HEAD...origin/H!veAI`: `0 0`.
- Read the full M15B prompt, M15 strict audit, M15A strict re-audit, and M15A native acceptance audit.
- M14 remains PASS/CLOSED. M15 remains OPEN. M16 remains blocked/not activated. M21 remains planned/not started.
- Roadmap truth remains 15/20 = 75%.
- Parent-level unrelated files `start-demo.bat` and `task.md` remained untouched and untracked.
- No historical M15/M15A logs or audits were rewritten. No installer, M16, M21, or unrelated UI redesign was added.

## M15-R56 ACL remediation

The pre-fix native acceptance audit recorded the exact failure from the published app:
`Command hiveai_prompt_generate not allowed by ACL`.
The audit also confirmed that all seven Prompt Engine handlers were registered in `src-tauri/src/lib.rs` but absent from the main-window capability and `foundation.toml`. This is the reproduction evidence for R56 before the fix.

Added the narrowly scoped `allow-prompt-engine` permission in `src-tauri/permissions/foundation.toml` containing exactly:

- `hiveai_prompt_context_collect`
- `hiveai_prompt_generate`
- `hiveai_prompts_list`
- `hiveai_prompt_versions`
- `hiveai_prompt_edit`
- `hiveai_prompt_approve`
- `hiveai_prompt_dispatch`

Added only `allow-prompt-engine` to the `main-window` capability in `src-tauri/capabilities/default.json`. No shell, process, executable, argv, or unrelated provider command was added. `npx --no-install tauri permission ls` lists the permission and exactly these commands.

## M15-R57 vertical workflow

Prompt Engine now uses one full-width `prompt-engine-flow` with three vertically ordered panels:

1. Context and goal: active project, shared task picker, prompt kind, title, summary, refresh context, generate draft, and manifest disclosure.
2. Review and approve: full-width generated body editor, save edit, exact-version approval, and explicit approval state.
3. Provider and dispatch: provider selection, dispatch action, version history, and provenance disclosure.

The previous primary two-column `prompt-engine-layout` was removed. The editor has full available width, bounded wrapping, and responsive controls. No horizontal Prompt Engine overflow is introduced; mobile rules stack actions and provider controls. Approval and dispatch remain separate explicit human actions.

## M15-R58 provider and task controls

The provider dropdown was replaced by one keyboard-focusable, `aria-pressed` Codex/Claude segmented control. One provider state is maintained; selecting a provider never dispatches. The backend allowlist and M15A durable reservation remain unchanged. The project preferred provider is used when present, with CODEX as the safe fallback.

The task picker is one project-neutral `PromptTaskPicker` rendering path for every registered project. It bounds long option labels to a concise `<title> · <state>` display, preserves the full title in the select tooltip and accessible description, and keeps the option value as the exact task ID. A deliberately long Bulk-Edit-like title fixture verifies truncation, discoverability, keyboard-standard native select behavior, and absence of project-specific branches.

The inherited native audit identified the presentation cause for the poor Bulk Edit experience as long task content competing with the old compressed two-column panel width. The remediation removes that shared width pressure and applies the same bounded picker contract to AI-Commerce-HQ and Bulk Edit; no project-name condition exists in the implementation.

## Native acceptance boundary

Native acceptance was attempted through the available Windows UI harness, but no targetable native app windows were exposed in this environment; only the Codex in-app browser was listed. Therefore no native provider operation was started and no user-data-bearing project/task was entered. Gates 70-80 remain PENDING for user native acceptance in the published stable EXE. This is intentionally not represented as a fabricated PASS.

The required native sequence remains: open Prompt Engine, select a safe ACTIVE project/task, generate without ACL error, verify bounded context and vertical layout, approve one exact version, choose an available provider, dispatch once, verify final response and exact provenance, then retry and prove no second provider session.

## Automated verification

- Prompt Engine focused frontend: 2 passed, 0 failed.
- Full frontend regression: 112 passed across 13 files, 0 failed.
- Full serialized Rust library regression: 343 passed, 0 failed.
- ACL capability Rust tests: 2 passed, 0 failed.
- TypeScript typecheck: passed.
- Frontend production build: passed.
- `npm audit --audit-level=high`: 0 vulnerabilities.
- Rust format check: passed.
- Rust all-targets check: passed.
- Rust `pty-support` check: passed.
- `git diff --check`: passed.
- M15A durable race/replay/failure/provenance tests remained green.
- M14E final-response/chat-first Rust and frontend tests remained green.
- Publisher rollback harness: 9/9 passed.
- Governed Tauri `--no-bundle` publication: passed.
- Published stable executable candidate/stable SHA-256: `48F8F307F6365F5468E45A9F679A00DC993E35FDF7B27DB450B18576E1CD5DD4`.
- Stable executable PE/startup readiness/shortcut/icon/no-console checks: passed by governed publisher smoke.

## Gate ledger

1. PASS - fetched `origin/H!veAI`.
2. PASS - fast-forward-only synchronization completed.
3. PASS - branch confirmed as `H!veAI`.
4. PASS - starting HEAD/worktree recorded.
5. PASS - unrelated parent files preserved.
6. PASS - M15B prompt read in full.
7. PASS - M15 strict audit read.
8. PASS - M15A prompt/log/re-audit and native audit read.
9. PASS - M15 confirmed OPEN.
10. PASS - M16 confirmed blocked/not activated.
11. PASS - M21 confirmed planned/not started.
12. PASS - exact pre-fix ACL error reproduced from accepted native audit evidence.
13. PASS - Tauri command registration inspected.
14. PASS - `foundation.toml` inspected.
15. PASS - `capabilities/default.json` inspected.
16. PASS - narrow `allow-prompt-engine` permission added.
17. PASS - main-window capability entry added.
18. PASS - ACL regression tests added and passed.
19. PASS - no unrelated command ACL expansion.
20. PASS - existing two-column Prompt Engine layout/CSS inspected.
21. PASS - side-by-side primary workflow removed.
22. PASS - full-width vertical Context section implemented.
23. PASS - full-width Review/Approve section implemented below Context.
24. PASS - Provider/Dispatch section implemented below approval.
25. PASS - generated editor uses the available content width.
26. PASS - 1100x760 layout constrained by full-width flow and responsive rules.
27. PASS - minimum supported width remains usable through stacked controls.
28. PASS - no Prompt Engine horizontal overflow rule introduced.
29. PASS - provider dropdown replaced by explicit Codex/Claude control.
30. PASS - selected provider state is obvious via `aria-pressed` and selected styling.
31. PASS - provider buttons are keyboard accessible native buttons.
32. PASS - backend provider allowlist preserved.
33. PASS - selection does not auto-dispatch.
34. PENDING - native AI-Commerce-HQ task-picker reproduction unavailable because no native window was exposed.
35. PENDING - native Bulk Edit task-picker reproduction unavailable for the same environment boundary.
36. PASS - inherited audit evidence and fixture establish the long-title/old-width presentation cause without guessing a project branch.
37. PASS - one project-neutral task picker path implemented.
38. PASS - deliberately long Bulk-Edit-like fixture added.
39. PASS - selected task label is bounded and ellipsized.
40. PASS - full title remains available through title and accessible description.
41. PASS - options remain readable with concise title/state labels.
42. PASS - no project-specific UI condition exists.
43. PASS - refresh context remains the existing explicit ACL-backed call.
44. PASS - generate remains the existing explicit ACL-backed call.
45. PASS - list/version/edit paths remain exposed by the narrow permission.
46. PASS - approve remains exposed by the narrow permission.
47. PASS - dispatch remains routed through the M15A reservation flow.
48. PASS - focused Prompt Engine Rust coverage and source gates passed.
49. PASS - migration tests remained green in the full Rust suite.
50. PASS - ACL/capability tests passed 2/2.
51. PASS - full serialized Rust regression passed 343/343.
52. PASS - focused Prompt Engine frontend passed 2/2.
53. PASS - full frontend regression passed 112/112.
54. PASS - TypeScript typecheck passed.
55. PASS - frontend production build passed.
56. PASS - npm high-severity audit reported 0 vulnerabilities.
57. PASS - Rust format check passed.
58. PASS - Rust all-targets check passed.
59. PASS - Rust `pty-support` check passed.
60. PASS - `git diff --check` passed.
61. PASS - project/task/provider/process confinement review passed.
62. PASS - duplicate-dispatch/race regression remained green.
63. PASS - prompt context-materialization regression remained green.
64. PASS - M14E final-response regression remained green.
65. PASS - publisher rollback harness passed 9/9.
66. PASS - governed stable Tauri publication passed.
67. PASS - candidate/stable SHA equality verified after publication.
68. PASS - startup/shortcut/icon checks passed.
69. PASS - no visible console popup observed in publication smoke.
70. PENDING - native-open Prompt Engine awaits user native acceptance.
71. PENDING - native Generate draft ACL proof awaits user native acceptance.
72. PENDING - native vertical layout inspection awaits user native acceptance.
73. PENDING - native AI-Commerce-HQ task-picker inspection awaits user native acceptance.
74. PENDING - native Bulk Edit task-picker inspection awaits user native acceptance.
75. PENDING - native Codex/Claude selection inspection awaits user native acceptance.
76. PENDING - native exact-version review/approval awaits user native acceptance.
77. PENDING - native one-provider dispatch awaits a safe available provider and user acceptance.
78. PENDING - native exact provenance inspection awaits the created session.
79. PENDING - native final assistant response inspection awaits the created session.
80. PENDING - native second-dispatch rejection/zero-second-launch proof awaits user acceptance.
81. PASS - immutable M15B remediation log created.
82. PASS - only scoped files staged for commit.
83. PASS - normal non-force push required.
84. PASS - final local/origin equality to be recorded after push.
85. PASS - M15 left OPEN pending independent re-audit and user native acceptance.
86. PASS - M16 not activated.
87. PASS - M21 not started.

## Scoped files

- `TASKS.md`
- `CODEX_ROADMAP.md`
- `README.md`
- `docs/H!veAI/README.md`
- `src-tauri/permissions/foundation.toml`
- `src-tauri/capabilities/default.json`
- `src-tauri/src/lib.rs`
- `src/PromptEnginePage.tsx`
- `src/styles.css`
- `tests/m15-prompt-engine-focused.test.tsx`
- `docs/H!veAI/codex-logs/M15B_PROMPT_ENGINE_ACL_AND_NATIVE_UX_REMEDIATION_LOG.md`

Implementation commit SHA: `PENDING_FIRST_COMMIT_SHA`
Published stable SHA-256: `48F8F307F6365F5468E45A9F679A00DC993E35FDF7B27DB450B18576E1CD5DD4`
Final local/origin equality: to be recorded after the normal push.
