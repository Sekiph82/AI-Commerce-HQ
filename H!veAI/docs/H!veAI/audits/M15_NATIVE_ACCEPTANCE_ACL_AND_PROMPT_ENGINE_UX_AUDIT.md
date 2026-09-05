# M15 Native Acceptance ACL and Prompt Engine UX Audit

Date: 2026-09-05
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Current branch HEAD reviewed: `b32dcdddfe362d930d39a7c7883ab33de744389f`

## Verdict

**M15 NATIVE ACCEPTANCE: FAIL**

- BLOCKER: 1
- MAJOR: 2
- MINOR: 0

M15 MUST remain OPEN.
M16 MUST NOT activate.
M21 MUST NOT start.
Strict completed roadmap progress remains `15 / 20 = 75%`.

## User-native evidence

The published Prompt Engine loads, fields can be populated, and the Generate draft button is enabled. Pressing it produces the native error:

`Command hiveai_prompt_generate not allowed by ACL`

The user also rejected the current Prompt Engine UX:

1. two-column Context / Review layout causes visual collision and cramped text;
2. Review and approve should be moved below the generation area as a full-width next step;
3. provider choice should be an explicit visible Codex / Claude control in the main flow;
4. Task dropdown behavior/presentation is inconsistent between projects, with Bulk Edit producing a poor dropdown while AI-Commerce-HQ appears acceptable.

## M15-R56 BLOCKER — Prompt Engine native commands are not exposed through the Tauri ACL

### Evidence

Production command handlers exist in `src-tauri/src/lib.rs` for:

- `hiveai_prompt_context_collect`
- `hiveai_prompt_generate`
- `hiveai_prompts_list`
- `hiveai_prompt_versions`
- `hiveai_prompt_edit`
- `hiveai_prompt_approve`
- `hiveai_prompt_dispatch`

But `src-tauri/permissions/foundation.toml` contains no Prompt Engine permission and `src-tauri/capabilities/default.json` contains no corresponding capability identifier.

The native error is therefore expected: Tauri registers the command but the main window lacks ACL permission to invoke it.

### Required closure

Add one narrowly scoped Prompt Engine permission containing exactly the Prompt Engine commands required by the M15 UI, add that permission to the main-window capability, and add regression coverage that would have caught this omission before publication.

Do not widen ACL beyond M15 commands.

## M15-R57 MAJOR — Prompt Engine layout is not native-user acceptable

### Evidence

The current page uses a side-by-side `.prompt-engine-layout` with Context and Review panels. On the user's published desktop viewport the panels feel compressed and their content visually competes.

### Required closure

Replace the two-column primary workflow with one clear vertical flow:

1. Context and goal
2. Generated prompt preview / review
3. Approval
4. Provider selection
5. Dispatch / result state

The Review and approve section must be full-width below Context rather than beside it.

Preserve the underlying explicit human approval boundary. This is presentation remediation only, not automatic approval/dispatch.

## M15-R58 MAJOR — provider/task controls lack a consistent interaction standard

### Provider

Provider selection must be visibly available as a two-choice Codex / Claude segmented control or equivalent explicit buttons. Do not hide provider selection in a narrow dropdown.

Selection remains a dispatch-time choice and must not trigger an operation by itself.

### Task picker

The task selector must render consistently across all registered projects regardless of task-title length/content.

Reproduce with at least:

- AI-Commerce-HQ
- Bulk Edit

Normalize:

- width and height;
- selected-value clipping/truncation;
- dropdown option readability;
- long task names;
- state/milestone metadata presentation;
- keyboard operation;
- no panel overflow;
- no project-specific CSS/data special case.

If a reusable select component already exists and matches the accepted AI-Commerce-HQ experience, reuse it. Otherwise introduce one bounded reusable Prompt Engine task-picker component.

## Acceptance boundary

M15 cannot close until:

- ACL generation works in the stable EXE;
- generated bounded prompt appears;
- review/approve vertical layout is native-readable;
- Codex/Claude control is explicit;
- Bulk Edit and AI-Commerce-HQ task selectors are visually consistent;
- one safe Prompt Engine dispatch succeeds;
- same approved version cannot dispatch twice;
- resulting Agent session retains M14E final-response presentation and exact prompt provenance.
