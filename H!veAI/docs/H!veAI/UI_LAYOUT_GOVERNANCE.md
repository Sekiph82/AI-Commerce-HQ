# H!veAI Canonical UI Layout Governance

This document is a durable visual-layout contract for H!veAI. It exists because the M02-era Command Center composition diverged materially from the user's canonical dashboard reference and later manual-QA evidence exposed fixture/live-data mixing, project-identity flicker, duplicated topbar actions, and unwanted nested scrollbars.

## Authority

Canonical visual reference assets are located at:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

The dashboard reference image in that folder is authoritative for the Command Center / Global Overview composition, density, hierarchy, proportions, and dark visual language.

The user has explicitly required that the primary overview fit in one desktop viewport rather than requiring repeated page scrolling.

## Canonical branding assets

Sidebar brand icon source:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI\H!veAI small logo.png`

Sidebar wordmark source:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI\H!veAI text logo.png`

Rules:

- Use `H!veAI small logo.png` only for the small square/emblem position at the top-left of the sidebar.
- Use `H!veAI text logo.png` immediately beside the small logo for the H!veAI wordmark.
- Do not use the large combined logo image in the sidebar header.
- Do not reproduce the H!veAI wordmark as ordinary CSS/text beside the image when the canonical text-logo asset is available.
- Do not crop, stretch, recolor, redraw, or distort either image.
- Render both with preserved aspect ratio and `object-fit: contain` or equivalent.
- The small logo must remain completely visible inside its box with breathing room. No part of the emblem may be clipped.
- The text logo must fit naturally beside it without overflowing the sidebar.

## Single-viewport Command Center contract

The Command Center / Global Overview is a command-center dashboard, not a vertically stacked report page.

At normal desktop QA sizes, especially the canonical reference size `1536x1024` and the user's maximized `2048x1280` desktop, the complete primary overview must fit inside one application viewport.

Required behavior:

- No outer/body vertical scrolling is allowed on the primary Command Center at the canonical desktop QA sizes.
- No outer/body horizontal scrolling is allowed.
- The application shell, sidebar, topbar, and Command Center use the available viewport height.
- The core cockpit panels visible in the canonical primary overview must not show visible internal scrollbars at the canonical desktop QA sizes. This specifically includes Current Task, Workflow Status, Recent Activity, Project Metrics, and compact System Status.
- Fit those core panels by compacting typography, row spacing, visible row counts, or summarizing secondary detail. Do not solve the canonical desktop layout by adding nested scrollbars.
- A genuinely long variable-length collection may use a deliberately bounded internal overflow only when there is no compact summary alternative, but the main Command Center must remain a one-screen operational console and visible scrollbars are not acceptable in the core cockpit blocks named above.
- For substantially smaller windows, responsive reflow or bounded overflow is allowed, but avoid turning the overview into several stacked full-width screens.

## Canonical composition

Reproduce the dashboard reference as closely as practical without pulling future business logic forward.

The desktop composition is:

1. Primary left navigation sidebar.
2. Compact top header / Global Overview area.
3. Compact KPI/status strip near the top.
4. A secondary Projects rail/list on the left side of the main dashboard content.
5. A large central Project Cockpit / current-project workspace.
6. A right-side vertical column containing AI Engineering Brief, AI Assistant, and System Status.
7. Recent Activity and Project Metrics are compact sections integrated into the central/bottom dashboard composition, not full-width page-length blocks.
8. Footer branding remains compact and global.

Do not replace this with a two-column collection of large project cards plus several full-width infrastructure panels.

### Command Center heading

- The visible page title is `Global Overview`.
- Do not render an additional visible `WORKSPACE OVERVIEW` / `Workspace overview` eyebrow above it.
- Keep hidden accessibility labels if needed, but the extra visible heading is not part of the canonical composition.

### Current-project header

Keep the central cockpit header compact. At canonical desktop sizes place the current-project identity and state on one compact line or equivalent dense composition, for example:

`CURRENT PROJECT   <Project Name>   <status badge>`

The project name must not consume a second oversized hero row when the same information can fit beside the label.

## Live project identity is the single source of truth

In the Tauri desktop application, real Project Registry data is authoritative for project identity.

- Command Center project rail must derive from the persisted Project Registry, not `fixtures.ts` project identity rows.
- Sidebar Project Shortcuts must derive from the same persisted Project Registry view/model.
- A project successfully registered on the Projects page must appear on the Command Center and relevant shortcut surface without requiring application restart.
- Project counts that represent registered projects must derive from the real registry. Do not display fixture counts as if they are live registry truth.
- Static fixtures may remain only for explicitly labeled browser-preview/demo surfaces or future task/workflow placeholders. They must never override or masquerade as a real registered project's identity in the Tauri desktop app.
- Prefer one shared registry provider/store/hook/event model so Projects, Command Center, sidebar shortcuts, and Project Cockpit observe the same project set and refresh signal.

## Project Cockpit route identity contract

A route `/projects/:id` must never briefly render another project's cockpit while the requested registered project is loading.

Required behavior in Tauri desktop mode:

- resolve the route ID against the real Project Registry;
- while resolution is pending, show a loading/skeleton state carrying no other project's identity;
- when resolved, keep that project identity stable for the lifetime of that route unless the user explicitly navigates elsewhere;
- if the registered project does not exist, show a clear not-found/error state;
- do not use `projects[0]`, FormuLab, or any other fixture as an implicit fallback for an unknown registered ID;
- do not flash FormuLab and then replace it with AI-Commerce-HQ or another registered project after an asynchronous request finishes;
- opening a newly registered project's `Open cockpit` action must open that exact project directly.

Browser-preview fixtures may use explicit fixture routes, but they must be isolated from Tauri registered-project resolution.

## Sidebar width

The sidebar should be slightly narrower than the earlier 244 px implementation so more horizontal space remains for the main cockpit.

- Target approximately `216–224px` at the canonical desktop size, unless a nearby value is required to avoid logo/text clipping.
- Do not compress the H!veAI logo/wordmark or primary navigation labels to achieve the narrower width.
- Keep comfortable hit targets and preserve responsive behavior.

## Topbar action identity

Distinct icons must not all open the same UI surface.

- Search field / `Ctrl+K` and Command icon may open the Command Palette.
- Sparkles/assistant icon must open or focus an AI Assistant surface distinct from the Command Palette.
- Bell icon must open a Notifications surface distinct from both Command Palette and AI Assistant.
- If assistant/notification business logic is not implemented yet, use honest bounded placeholder drawers/popovers with correct titles and future-state messaging. Do not route all three to the same modal.
- Keep each button's accessible label accurate to the surface it opens.

## Infrastructure status placement

Runtime, Database, Filesystem Watcher, Git, and similar infrastructure health are important but must not consume multiple full-width rows in the primary overview.

On the Command Center:

- show them as compact status summaries in the right-side System Status area or another compact reference-aligned surface;
- preserve access to detailed status elsewhere or via an expandable/detail surface;
- do not render large full-width Runtime Status, Database Status, and Filesystem Watcher blocks sequentially above the project dashboard.

This rule is about layout and density, not deleting infrastructure functionality.

## Density and spacing

Use the canonical dashboard's dense professional command-center treatment:

- smaller, tighter cards;
- restrained vertical margins;
- compact headers;
- compact KPI tiles;
- information-dense project rows;
- consistent thin borders;
- dark navy/black surfaces;
- violet/blue accents with green/yellow/red state colors;
- avoid oversized hero headings and large empty vertical gaps.

The dashboard must feel like a single operational console.

## Footer placement

Footer branding is global, not a large sidebar block.

At canonical desktop sizes:

- remove the `Built with ... by Akilta` footer from the left sidebar bottom area;
- render the footer compactly at the bottom center of the application/main workspace, matching the reference composition as closely as practical;
- keep `Built with` and `for maximum productivity by Akilta` readable but visually quiet;
- render the heart `♥` visibly red;
- use the canonical Akilta logo/wordmark where practical and ensure it remains visible on the dark background;
- footer placement must not introduce page scrolling or obscure interactive content.

## Scroll policy

Preferred implementation model:

- `html`, `body`, `#root`, and `.app-shell` occupy the viewport;
- the AppShell itself does not create document-length vertical overflow;
- the Command Center route uses an explicit viewport-height grid/flex layout with `min-height: 0` on nested grid/flex children;
- compact core panels fit without visible nested scrollbars at canonical desktop QA sizes;
- overflow is assigned only to deliberately bounded variable-length surfaces when truly necessary;
- other routes may keep their own route-appropriate scrolling behavior.

Do not globally break the Projects, Cockpit, Settings, or other routes merely to remove Command Center scrolling.

## Visual acceptance

A Command Center visual change is not accepted merely because the frontend compiles.

Before claiming visual PASS:

1. Launch the stable `Desktop\H!veAI.lnk` development build.
2. Confirm the real production-mode Tauri frontend renders.
3. Compare side-by-side with the canonical dashboard reference and the latest user-annotated QA screenshots.
4. At `1536x1024`, confirm the complete primary overview is visible without outer vertical/horizontal scrolling.
5. At the user's maximized desktop size (`2048x1280` evidence), confirm the complete primary overview is visible without outer vertical/horizontal scrolling.
6. Confirm Current Task, Workflow Status, Recent Activity, Project Metrics, and System Status have no visible internal scrollbars at the canonical sizes.
7. Confirm the sidebar small logo is not clipped.
8. Confirm the sidebar wordmark uses `H!veAI text logo.png`.
9. Confirm the sidebar is slightly narrower while remaining usable.
10. Confirm `Global Overview` is visible without a visible `WORKSPACE OVERVIEW` eyebrow.
11. Confirm the central current-project header is compact and project identity is live Registry data.
12. Confirm a newly registered project appears in the Command Center project rail and sidebar project shortcuts without app restart.
13. Confirm opening that newly registered project's cockpit never flashes FormuLab or any unrelated fixture project.
14. Confirm Command, Assistant, and Notifications topbar icons open distinct surfaces.
15. Confirm the footer is bottom-center/global, heart is red, and the sidebar no longer contains the old footer block.
16. Confirm no giant Runtime/Database/Watcher blocks force the main dashboard downward.
17. Confirm the Projects rail, center Cockpit, and right AI/System column are visible together.
18. If the user has not personally approved the visual result, record `PENDING USER VISUAL ACCEPTANCE`; do not fabricate final visual PASS.

## Future milestones

M11/M12 will add richer Global Command Center and Project Cockpit functionality, but they must preserve this single-viewport composition, live-project identity contract, and canonical layout unless the user explicitly changes them.

Future milestones may replace task/workflow placeholders with real data. They must not regress project identity back to fixtures or turn the dashboard back into a long vertically stacked page.
