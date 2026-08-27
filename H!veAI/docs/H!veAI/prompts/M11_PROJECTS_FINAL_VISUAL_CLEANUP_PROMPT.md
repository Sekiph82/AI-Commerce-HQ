# M11 Projects Final Visual Cleanup

## Authority

This is a bounded visual cleanup task for H!veAI before final M11 native visual acceptance.

Work only on the `H!veAI` branch.

Do not start M12 or M21. Do not modify M11A identity behavior.

## User-requested Projects page change

The user has reviewed Command Center / Projects / Tasks and considers the current Projects page's permanent right-side information column to be the remaining visual issue.

Remove these two permanent right-side cards from the normal Projects page:

1. `REGISTRY BOUNDARY`
   - `Read-only by design`
   - registry safety explanation

2. `CURRENT VIEW`
   - `<n> registered projects`
   - persisted-registry explanation

They must no longer occupy a permanent right-side column.

After removal:

- reclaim the entire right-side width for the main Projects content;
- allow the project registry grid/content area to expand naturally;
- do not leave an empty placeholder column;
- preserve the existing page max-width, spacing rhythm and card design;
- preserve search, status filter, sorting and `+ Add project`;
- preserve existing responsive behavior;
- do not redesign project cards.

## Move Registry Boundary information into Add Project

The Registry Boundary information is useful at registration time. Move its safety meaning into the existing Add Project flow.

When the user clicks `+ Add project`, the existing dialog/modal/panel must include a compact information section with this meaning:

### Registry Boundary
**Read-only by design**

H!veAI records project identity and cached Git metadata without changing the selected folder.

- Explicit user action required
- No branch, file, or remote mutation
- Live Git metadata and status available

Reuse the current wording/semantics where practical.

This information must be visible inside the Add Project flow before/while the user registers the folder.

Do not create another permanent Registry Boundary card elsewhere on the Projects page.

## Current View

Remove the `CURRENT VIEW / registered projects` card completely.

Do not move it into another standalone card.

The registered-project count may remain wherever it already appears naturally in normal registry UI, but do not introduce redundant decorative summary UI.

## Layout target

The resulting Projects page should visually consist of:

- page title/description;
- `+ Add project`;
- search/filter/sort toolbar;
- expanded project-card registry grid.

The registry grid should use the reclaimed horizontal space and remain visually balanced with the Command Center and Tasks pages.

## Scope protection

Do not change:

- project registration semantics;
- read-only registry guarantees;
- Git operations;
- persistence/database behavior;
- Command Center logic;
- Tasks logic;
- M11A REV7 identity implementation;
- startup video;
- application/native icons;
- Akilta attribution;
- external registered projects;
- Bulk Edit;
- M12;
- M21.

## Verification

Run:

- Projects-focused frontend tests;
- complete frontend test suite;
- typecheck;
- production frontend build;
- existing native/publisher QA gate required by the project.

Add/update focused tests proving:

1. Registry Boundary is not rendered as a permanent Projects-page side card.
2. Current View card is not rendered.
3. Add Project flow contains the Registry Boundary safety information.
4. Add Project behavior remains functional.
5. Search/filter/sort/project cards remain present.

Publish the normal H!veAI dev QA executable.

Create an immutable log:

`H!veAI/docs/H!veAI/codex-logs/M11_PROJECTS_FINAL_VISUAL_CLEANUP_LOG.md`

Commit and push to `origin/H!veAI`.

Final builder state:

`M11 PROJECTS FINAL VISUAL CLEANUP COMPLETE / PENDING USER VISUAL ACCEPTANCE`

Stop after this task. Do not start M12.
