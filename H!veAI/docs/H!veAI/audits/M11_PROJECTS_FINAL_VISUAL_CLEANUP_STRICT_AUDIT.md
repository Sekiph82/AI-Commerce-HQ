# M11 Projects Final Visual Cleanup - Strict Audit

Date: 2026-08-27
Product: H!veAI
Branch: `H!veAI`
Audited builder log: `H!veAI/docs/H!veAI/codex-logs/M11_PROJECTS_FINAL_VISUAL_CLEANUP_LOG.md`
Audited implementation commit: `650eabea6b0d4170f02bd019b466aa4f7e1eaad6`
Authoritative prompt: `H!veAI/docs/H!veAI/prompts/M11_PROJECTS_FINAL_VISUAL_CLEANUP_PROMPT.md`

## Verdict

**CONDITIONAL PASS / SOURCE + AUTOMATED VERIFICATION PASS / USER VISUAL ACCEPTANCE PENDING**

- BLOCKER: 0
- MAJOR: 0
- MINOR: 0 production defects
- NOTE: 2
- Confidence: HIGH
- Regression risk: LOW

The implementation matches the requested bounded visual cleanup. The permanent right-side `Registry Boundary` and `Current View` cards are removed from the normal Projects page; the main registry area reclaims the full available width; the Registry Boundary safety contract is moved into the Add Project flow; and search/filter/sort/card behavior remains covered by focused tests.

No source-level defect requiring another implementation run was found.

## Acceptance matrix

| Area | Result | Independent finding |
| --- | --- | --- |
| Remove Registry Boundary side panel | PASS | `registry-aside` and its Registry Boundary panel are removed from the Projects page. |
| Remove Current View card | PASS | The registered-project summary card is removed and not recreated elsewhere in the normal page flow. |
| Reclaim layout width | PASS | `registry-layout` is overridden to block layout and `registry-grid` expands to a 3-column responsive grid, falling to 2 and 1 columns at bounded breakpoints. |
| Move safety contract into Add Project | PASS | Register-mode dialog now shows `Registry Boundary`, `Read-only by design`, explicit-user-action, no branch/file/remote mutation, and live Git metadata/status statements. |
| Preserve non-register dialog behavior | PASS source-level | The new boundary section renders only when `mode === "register"`; relocation flow remains outside this added block. |
| Preserve search/filter/sort/cards | PASS | Focused test asserts Add Project, search textbox, status filter, sort control, and project articles remain present. |
| Remove permanent side content | PASS | Focused test asserts `Registry Boundary`, `Current view`, and `.registry-aside` are absent before opening Add Project. |
| Add Project functionality | PASS | Existing registration-flow test continues through folder path/name and submission after asserting the moved safety block. |
| Full frontend regression | PASS by builder evidence | 9 files / 87 tests reported PASS. |
| Native/Rust regression | PASS by builder evidence | 278 Rust tests reported PASS; typecheck/build/cargo check/fmt/audit/diff checks passed. |
| Governed QA publication | PASS by builder evidence | Failure harness 9/9 and governed QA publication reported PASS. |
| Scope discipline | PASS | Implementation commit changes only `main.tsx`, `pages.tsx`, new `projects.css`, and the focused frontend test. |

## Independent source findings

### Layout

The previous permanent `<aside className="registry-aside">` block is deleted entirely. No empty placeholder column remains in the JSX. The new Projects-specific stylesheet is imported after the existing global/Command Center styles so its bounded registry layout rules take effect for this page.

The grid is explicitly:

- 3 columns at normal desktop width;
- 2 columns below 1050px;
- 1 column below 720px.

This is consistent with the requested reclaimed workspace and responsive card behavior.

### Add Project safety content

The safety information is rendered only for `mode === "register"`, immediately inside the existing Project Registry dialog before the registration form. This places the information at the requested decision point without adding another persistent Projects-page panel.

### Tests

The focused tests directly exercise the two critical acceptance rules:

1. the safety copy is absent from the normal Projects page but present after clicking `Add project`;
2. search, status filtering, sorting, and project cards remain present while the old side column remains absent.

The registration test continues beyond the new assertions into the existing register-folder workflow, reducing the risk that the added dialog section blocks or breaks form behavior.

## Notes

### NOTE P01 - final visual composition remains user-owned

Automated tests prove structure and presence/absence, not aesthetic balance at the user's actual Windows window size. Final acceptance should confirm that the reclaimed width looks correct and the new Add Project safety block is not visually cramped or oversized.

### NOTE P02 - new `projects.css` is intentionally page-specific

The implementation adds a dedicated stylesheet imported globally from `main.tsx`, but its selectors are Projects-specific. No conflicting generic selector was introduced in the reviewed patch. This is acceptable for the bounded cleanup, though future UI consolidation may fold these rules back into the main design-system stylesheet if desired.

## Required user visual check

Open the published H!veAI QA executable and verify:

1. Projects no longer shows the permanent Registry Boundary / Current View right column;
2. project cards use the reclaimed width cleanly;
3. at the user's normal window size the grid looks balanced and does not create unwanted horizontal/vertical overflow;
4. `+ Add project` opens the dialog with the compact Registry Boundary safety section;
5. the safety section does not obscure or crowd the folder path / display-name controls;
6. search, status filter, sorting, project cards and navigation remain visually intact.

If these pass, this visual cleanup can be accepted without another Codex remediation run.

## Closure

**SOURCE/AUTOMATED AUDIT: PASS**

**FINAL STATE: PENDING USER VISUAL ACCEPTANCE ONLY**

Do not start M12 until the user explicitly accepts the M11 native visual state.
