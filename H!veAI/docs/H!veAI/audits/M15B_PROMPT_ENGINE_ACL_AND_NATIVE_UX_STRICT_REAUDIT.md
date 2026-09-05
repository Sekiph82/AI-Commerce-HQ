# M15B Prompt Engine ACL and Native UX Strict Re-Audit

Date: 2026-09-05
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Audited implementation commit: `3382dbdff255de73f7106bf47e364606a2358a8f`
Audited branch HEAD: `48f20321c0b6c6f4bda70e1d72e05a44a4cc4aa6`

## Verdict

**TECHNICAL PASS / USER NATIVE ACCEPTANCE STILL REQUIRED**

- BLOCKER: 0
- MAJOR: 0
- MINOR: 0
- M15-R56: CLOSED
- M15-R57: CLOSED
- M15-R58: CLOSED

M15 remains OPEN pending user native/visual acceptance of the published stable executable.
M16 MUST NOT activate until that acceptance is supplied.
M21 remains planned/not started.
Strict completed roadmap progress remains `15 / 20 = 75%`.

## Evidence independently reviewed

- M15B authoritative remediation prompt.
- M15B remediation log.
- Actual implementation commit `3382dbdff255de73f7106bf47e364606a2358a8f`.
- Current branch HEAD `48f20321c0b6c6f4bda70e1d72e05a44a4cc4aa6`, whose parent is the implementation commit and whose only purpose is finalizing the remediation log.
- `src-tauri/permissions/foundation.toml`.
- `src-tauri/capabilities/default.json`.
- `src-tauri/src/lib.rs`.
- `src/PromptEnginePage.tsx`.
- `tests/m15-prompt-engine-focused.test.tsx`.
- Builder verification/publication evidence from the immutable M15B log.

## M15-R56 closure: Prompt Engine ACL

R56 is CLOSED.

The production Tauri command handlers remain registered and the main-window ACL now contains a dedicated permission:

`allow-prompt-engine`

It allows exactly these seven Prompt Engine commands:

- `hiveai_prompt_context_collect`
- `hiveai_prompt_generate`
- `hiveai_prompts_list`
- `hiveai_prompt_versions`
- `hiveai_prompt_edit`
- `hiveai_prompt_approve`
- `hiveai_prompt_dispatch`

`src-tauri/capabilities/default.json` includes `allow-prompt-engine` for the `main` window.

The Rust capability regression test verifies the permission exists, all seven intended commands are present, and the block does not absorb agent-start, shell, or process permissions.

This directly addresses the accepted native failure:

`Command hiveai_prompt_generate not allowed by ACL`

No broad ACL expansion was introduced.

## M15-R57 closure: full-width vertical Prompt Engine flow

R57 is CLOSED technically.

The old side-by-side primary layout is removed from `PromptEnginePage.tsx`.

The page now renders one vertical `prompt-engine-flow` in this order:

1. Context and goal
2. Review and approve
3. Provider and dispatch

Review/Approve is no longer beside Context. The generated prompt body is rendered in its own lower full-width section. Provider/Dispatch is separated again below approval.

The focused frontend test explicitly verifies the legacy `.prompt-engine-layout` is absent and that all three ordered section headings exist.

This matches the native UX remediation requested by the user.

## M15-R58 closure: provider and task interaction standard

R58 is CLOSED technically, subject to native visual acceptance.

### Provider

The previous provider dropdown is replaced with two explicit native buttons:

- Codex
- Claude

The control:

- uses one shared provider state;
- exposes `aria-pressed`;
- has an obvious selected class;
- does not auto-dispatch;
- changes the dispatch button label to the selected provider;
- preserves backend provider validation and M15A dispatch boundaries.

The selected project's preferred provider is used when available, otherwise CODEX remains the safe fallback. No project-name-specific provider branch was added.

### Task picker

A single `PromptTaskPicker` component is used for all projects.

Its production contract:

- one shared rendering path;
- exact task ID remains the select value;
- visible label is bounded to 72 characters then ellipsized;
- state remains visible as `<title> · <state>`;
- full selected task title is preserved through `title` and an accessible `aria-describedby` detail;
- freeform mode remains available.

The focused frontend fixture includes a deliberately long Bulk-Edit-like task title and verifies bounded rendering, accessible full-title preservation, explicit provider controls, and no project-specific alternate path.

This is sufficient for technical closure. Whether the native Windows select popup is visually satisfactory across the user's real AI-Commerce-HQ and Bulk Edit task sets remains a user acceptance question, not an unresolved source defect at this stage.

## Verification record

Builder evidence reports:

- Prompt Engine focused frontend: 2 passed;
- full frontend: 112 passed across 13 files;
- full serialized Rust: 343 passed;
- ACL/capability Rust tests: 2 passed;
- TypeScript typecheck: PASS;
- frontend production build: PASS;
- npm high-severity audit: 0 vulnerabilities;
- Rust fmt/all-targets/pty-support: PASS;
- M15A dispatch race/replay/failure/provenance regression: PASS;
- M14E final-response/chat-first regressions: PASS;
- publisher rollback harness: 9/9 PASS;
- governed Tauri publication: PASS;
- published stable executable SHA-256:
  `48F8F307F6365F5468E45A9F679A00DC993E35FDF7B27DB450B18576E1CD5DD4`;
- PE/startup/shortcut/icon/no-console publisher smoke: PASS.

The source and commit evidence reviewed are consistent with those claims.

## Implementation provenance

The M15B implementation commit recorded in the log is valid and independently resolved:

`3382dbdff255de73f7106bf47e364606a2358a8f`

Current `H!veAI` branch HEAD is:

`48f20321c0b6c6f4bda70e1d72e05a44a4cc4aa6`

That HEAD is the subsequent documentation commit `docs: finalize M15B remediation log` and directly parents the implementation commit. Provenance is therefore coherent.

## Native acceptance still required

The builder explicitly could not exercise gates 70-80 in a targetable native Windows H!veAI window.

User acceptance must now verify in the published stable executable:

1. Prompt Engine opens with the new vertical layout.
2. Generate draft no longer produces an ACL error.
3. AI-Commerce-HQ task dropdown is visually acceptable.
4. Bulk Edit task dropdown is visually acceptable and follows the same standard.
5. Codex / Claude selection is clear and works.
6. A generated prompt appears in the lower Review and approve section.
7. Exact version approval succeeds.
8. One safe dispatch succeeds.
9. Agent Session Center shows the final assistant response and exact prompt/version/hash/provider provenance.
10. A second dispatch of the same approved version is rejected with no second provider launch.

After those native/visual gates are accepted by the user, M15 may close and roadmap progress may advance to `16 / 20 = 80%`. Only then may M16 activate.

## Final boundary

M14: PASS/CLOSED.

M15A: TECHNICAL PASS.

M15B: TECHNICAL PASS.

M15: OPEN pending user native/visual acceptance only.

M16-M20: blocked/planned.

M21: planned/not started.
