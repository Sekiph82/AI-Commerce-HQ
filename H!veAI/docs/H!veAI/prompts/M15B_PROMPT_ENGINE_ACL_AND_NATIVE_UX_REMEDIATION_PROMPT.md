# M15B Prompt Engine ACL and Native UX Remediation Prompt

Date: 2026-09-05
Product: H!veAI
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Milestone: M15B remediation
Authority: authoritative remediation prompt

## 0. Scope authority

The M15A technical strict re-audit passed R54/R55, but user native acceptance failed with three new findings:

- **M15-R56 BLOCKER:** Prompt Engine native commands are registered but not allowed by the main-window Tauri ACL.
- **M15-R57 MAJOR:** current two-column Prompt Engine layout is visually unacceptable; Review and approve must move below Context as a full-width vertical step.
- **M15-R58 MAJOR:** provider selection and task picker need one consistent native interaction standard.

Fix R56-R58 only.

M14 remains PASS/CLOSED.
M15 remains OPEN.
M16-M20 remain blocked/planned.
M21 remains planned/not started.
Progress remains `15 / 20 = 75%`.

Do not rewrite historical M15/M15A logs or audits.

## 1. Inherited non-negotiable boundaries

Preserve:

- ACTIVE registered-project confinement;
- task/project ownership checks;
- immutable prompt versions;
- explicit review/edit/approve before dispatch;
- M15A single-use durable dispatch reservation;
- exact prompt/version/hash/provider provenance;
- accepted M14E dedicated final-response capture;
- no arbitrary executable/shell/argv/PID surface;
- prompt content never in provider argv;
- bounded context/materialization;
- secret-safe persistence;
- no background console popup;
- governed stable publication.

## 2. R56 ACL remediation

### Reproduce first

Using the current stable/native app or controlled Tauri capability test, reproduce:

`Command hiveai_prompt_generate not allowed by ACL`

Confirm this is the exact native failure before changing ACL.

### Required permission

Add a narrowly scoped permission, suggested identifier:

`allow-prompt-engine`

It must allow only:

- `hiveai_prompt_context_collect`
- `hiveai_prompt_generate`
- `hiveai_prompts_list`
- `hiveai_prompt_versions`
- `hiveai_prompt_edit`
- `hiveai_prompt_approve`
- `hiveai_prompt_dispatch`

Add the permission to the `main` window capability in `src-tauri/capabilities/default.json`.

Do not attach Prompt Engine calls to a broader unrelated permission.

### ACL tests

Add tests that assert:

- the Prompt Engine permission exists;
- all seven intended commands are present;
- main-window capability includes the Prompt Engine permission;
- no unrelated shell/process commands were added.

Run generated Tauri ACL/schema validation if the project supports it.

## 3. R57 vertical Prompt Engine workflow

Redesign only the Prompt Engine page into a full-width vertical workflow.

### Required order

**Section 1: Context & goal**
- Project
- Task
- Prompt kind
- Title
- Summary
- Refresh context
- Generate draft
- Context manifest disclosure

**Section 2: Review & approve**
- full-width below Section 1
- generated prompt body uses the available content width
- readable multiline editor
- Save edit
- Approve exact version
- approval/version state

**Section 3: Provider & dispatch**
- full-width below approval
- explicit provider control
- Dispatch approved prompt
- version/provenance disclosure

Do not place Review beside Context.

At the user's normal 1100x760 native window, there must be no overlapping text/panels, no horizontal page scrolling caused by the Prompt Engine, and no clipped action controls.

Responsive behavior at minimum width must remain usable.

## 4. R58 explicit provider control

Replace the current provider dropdown in the Prompt Engine dispatch section with a clearly visible two-choice control:

- **Codex**
- **Claude**

Preferred shape: segmented buttons/tabs/radio cards using existing H!veAI visual language.

Requirements:

- one selected state is obvious;
- keyboard accessible;
- selection alone does not dispatch;
- provider is still validated against the backend allowlist;
- dispatch button text may reflect selection, e.g. `Dispatch to Claude`;
- do not duplicate provider state in multiple controls.

Default selection may follow the registered project's preferred provider if available; otherwise use the existing safe default. Do not hardcode project names.

## 5. R58 task picker standardization

### Reproduce both projects

Native-test Prompt Engine task selection with at least:

- `AI-Commerce-HQ`
- `Bulk Edit`

Record why Bulk Edit currently renders poorly. Do not guess. Check actual task titles/states and CSS/layout interaction.

### Required task-picker standard

Use one rendering/component path for all projects.

The selected field must:

- fit the panel width;
- show a concise selected label;
- avoid multi-line/native overflow;
- ellipsize long titles safely;
- preserve the full task title via accessible title/tooltip or detail text.

Dropdown/options must remain readable for long tasks. A recommended display contract is:

`<task title> · <state>`

with milestone/actor as secondary metadata only if the chosen component supports it cleanly.

Do not create project-specific branches for Bulk Edit or AI-Commerce-HQ.

If the accepted-looking control used elsewhere in H!veAI is reusable, standardize on it rather than inventing a second visual grammar.

Add a frontend fixture with a deliberately long Bulk-Edit-like task title.

## 6. Native Prompt Engine acceptance flow

After ACL and UI remediation, perform the following in the published stable EXE if safe:

1. Open Prompt Engine.
2. Select an ACTIVE project.
3. Select a task or freeform operation.
4. Fill title and summary.
5. Click Generate draft.
6. Prove no ACL error occurs.
7. Prove the generated prompt contains materialized bounded context.
8. Review the prompt in the full-width lower section.
9. Approve the exact version.
10. Select Codex or Claude using the new explicit control.
11. Dispatch exactly once.
12. Verify Agent Session Center shows the final assistant response.
13. Verify exact prompt/version/hash/provider provenance.
14. Attempt second dispatch of the same approved version.
15. Prove no second provider process/session starts.

If provider quota prevents a real Claude operation, use Codex for native acceptance. Do not burn quota merely to prove the UI. Record the provider used.

## 7. Explicit execution gates

1. Fetch `origin/H!veAI`.
2. Fast-forward-only sync.
3. Confirm branch.
4. Record starting HEAD/worktree.
5. Preserve unrelated files.
6. Read M15 prompt.
7. Read M15 strict audit.
8. Read M15A prompt/log/re-audit.
9. Confirm M15 OPEN.
10. Confirm M16 blocked.
11. Confirm M21 not started.
12. Reproduce native ACL error.
13. Inspect Tauri command registration.
14. Inspect `foundation.toml`.
15. Inspect `capabilities/default.json`.
16. Add narrow `allow-prompt-engine` permission.
17. Add main-window capability entry.
18. Add ACL regression tests.
19. Verify no unrelated command ACL expansion.
20. Inspect Prompt Engine current two-column CSS/layout.
21. Remove side-by-side primary workflow.
22. Implement full-width vertical Context section.
23. Implement full-width Review/Approve section below.
24. Implement Provider/Dispatch section below approval.
25. Verify generated prompt editor uses usable width.
26. Verify 1100x760 native layout.
27. Verify minimum supported window layout.
28. Verify no horizontal page overflow.
29. Replace provider dropdown with Codex/Claude explicit control.
30. Verify obvious selected state.
31. Verify keyboard accessibility.
32. Preserve backend provider allowlist.
33. Preserve no-auto-dispatch behavior.
34. Reproduce task picker with AI-Commerce-HQ.
35. Reproduce task picker with Bulk Edit.
36. Document the actual presentation cause.
37. Implement one project-neutral task picker standard.
38. Add long-title fixture.
39. Verify selected task truncation.
40. Verify full title remains discoverable/accessibly exposed.
41. Verify options remain readable.
42. Verify no project-specific UI condition.
43. Verify Context refresh works through ACL.
44. Verify Generate works through ACL.
45. Verify list/version/edit works through ACL.
46. Verify Approve works through ACL.
47. Verify Dispatch reaches M15A reservation flow.
48. Run Prompt Engine focused Rust tests.
49. Run migration tests.
50. Run ACL/capability tests.
51. Run full serialized Rust regression.
52. Run focused Prompt Engine frontend tests.
53. Run full frontend regression.
54. Run TypeScript typecheck.
55. Run frontend production build.
56. Run npm audit high.
57. Run Rust fmt check.
58. Run Rust all-targets check.
59. Run Rust pty-support check.
60. Run `git diff --check`.
61. Run project/task/provider/process confinement review.
62. Run duplicate-dispatch/race regression.
63. Run prompt context-materialization regression.
64. Run M14E final-response regression.
65. Run publisher rollback harness.
66. Governed-publish stable Tauri EXE.
67. Verify candidate/stable SHA equality.
68. Verify startup/shortcut/icon.
69. Verify no visible console popup.
70. Native-open Prompt Engine.
71. Generate draft without ACL error.
72. Native-check vertical layout.
73. Native-check AI-Commerce-HQ task picker.
74. Native-check Bulk Edit task picker.
75. Native-check Codex/Claude selection.
76. Native review/approve one exact prompt.
77. Native dispatch once using available provider.
78. Verify exact provenance.
79. Verify final assistant response.
80. Verify second dispatch is rejected with zero second launch.
81. Create immutable M15B log.
82. Commit scoped files only.
83. Push normally, no force.
84. Verify local/origin HEAD equality.
85. Leave M15 OPEN pending independent re-audit and user native/visual acceptance.
86. Do not activate M16.
87. Do not start M21.

## 8. Required log

Create:

`H!veAI/docs/H!veAI/codex-logs/M15B_PROMPT_ENGINE_ACL_AND_NATIVE_UX_REMEDIATION_LOG.md`

Record:

- actual implementation commit SHA;
- ACL reproduction evidence before fix;
- exact permission/capability changes;
- task-picker cause found for Bulk Edit;
- UI files/components changed;
- test counts;
- native acceptance evidence;
- chosen provider for dispatch acceptance;
- duplicate-dispatch evidence;
- publication hashes;
- final milestone state.

## 9. Completion boundary

Do NOT close M15.

Expected builder state:

- R56 remediation complete, pending independent re-audit/user acceptance;
- R57 remediation complete, pending independent re-audit/user acceptance;
- R58 remediation complete, pending independent re-audit/user acceptance;
- M15 remains OPEN;
- M16 blocked;
- M21 not started;
- progress remains `15 / 20 = 75%`.
