# H!veAI Canonical UI Layout Governance

This document is a durable visual-layout contract for H!veAI. It exists because the M02-era Command Center composition diverged materially from the user's canonical dashboard reference and later manual-QA evidence exposed fixture/live-data mixing, project-identity flicker, duplicated topbar actions, and unwanted nested scrollbars.

## Authority

Canonical visual reference assets are located at:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

The dashboard reference image in that folder is authoritative for the Command Center / Global Overview composition, density, hierarchy, proportions, and dark visual language.

The user has explicitly required that the primary overview fit in one desktop viewport rather than requiring repeated page scrolling.

## Canonical branding assets

Sidebar visible brand source:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI\H!veAI logo.png`

Rules:

- Visible sidebar branding is exactly one combined PNG image from `H!veAI logo.png`.
- Do not show a separate emblem, separate text logo, or `Development command center` subtitle.
- At canonical desktop sizes, the logo should occupy almost the full usable sidebar width with only small horizontal breathing room.
- Size the logo width-first with preserved aspect ratio; fixed-height sizing that makes the combined logo visually tiny is prohibited.
- Preserve the combined image aspect ratio with `object-fit: contain` or equivalent. Do not crop, stretch, recolor, redraw, or distort it.

Desktop shortcut icon source (unchanged):

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI\H!veAI small logo.png`

Use the small logo only for the ICO/Windows shortcut icon. Do not use the
combined sidebar source for the shortcut icon.

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

## Command Center project-selection contract

The secondary `Projects` rail inside `Global Overview` is a **selection control for the central Current Project workspace**, not a route-away navigation list.

Required desktop behavior:

- clicking a project row in the Command Center Projects rail keeps the user on the Command Center route;
- the clicked row becomes the selected/highlighted project;
- the central `CURRENT PROJECT` identity updates immediately to that exact registered project;
- all registry-backed fields in the central panel must update from the same selected project record;
- if task/workflow/metrics evidence for the selected project is not yet available, render neutral unavailable/undiscovered states rather than retaining another project's data;
- selecting `Bulk-Edit` must make the central panel show `Bulk-Edit`; selecting `AI-Commerce-HQ` must make it show `AI-Commerce-HQ`;
- selection must be stored in the shared session-level project state and remain selected when returning to Command Center during the session;
- if the selected project is archived/removed or becomes unavailable under the active registry filter, choose a deterministic valid registry fallback and update the selected state explicitly;
- do not navigate to `/projects/:id` merely because a Command Center rail row was clicked.

The central selected-project workspace must provide a distinct action such as `Open cockpit` that navigates to `/projects/<selectedProjectId>` for the full Project Cockpit route. That action must always target the currently selected project.

The selected row must have a clear but restrained active visual state matching the canonical dark/violet language. Selection must also be keyboard-accessible.

### Projects rail row content

The Global Overview Projects rail is a fast project picker, not a miniature status dashboard.

Each project row must show **only the registered project name**.

Do not show in these rows:

- logo, initials badge, or square project mark;
- ACTIVE/MISSING/ARCHIVED status text;
- Healthy/Watch/Blocked health text;
- progress percentage or progress bar;
- phase/milestone text;
- `Registered...` or other subtitle;
- actor or secondary metadata.

Use compact single-line name rows so the fixed-height selector can display materially more projects at once. The selected row may have a restrained background, border, or accent marker without reintroducing status clutter. Long names may truncate visually only when needed, while retaining the complete name through accessible/title semantics.

The full Projects page is allowed to show richer registry/Git/status metadata. This name-only rule is specific to the Global Overview project-selection rail.

Required QA behavior:

1. start on Command Center with AI-Commerce-HQ selected;
2. confirm project rail rows display project names only;
3. confirm no row contains logo/initial badge, status, health, percentage, progress bar, phase, or registration subtitle;
4. click Bulk-Edit in the Command Center project rail;
5. URL remains the Command Center route;
6. central Current Project changes to Bulk-Edit without reload/restart;
7. click AI-Commerce-HQ and central Current Project changes back;
8. click `Open cockpit` and only then navigate to the currently selected project's cockpit;
9. no stale asynchronous response may revert the selected central project to a previous project.

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
7. Confirm the single combined sidebar logo is not clipped.
8. Confirm the Desktop shortcut icon remains derived from `H!veAI small logo.png`.
9. Confirm the sidebar is slightly narrower while remaining usable.
10. Confirm `Global Overview` is visible without a visible `WORKSPACE OVERVIEW` eyebrow.
11. Confirm the central current-project header is compact and project identity is live Registry data.
12. Confirm Command Center project rail rows display project names only, with no logo/initial badge, status, health, percentage/progress, phase, or registration subtitle.
13. Confirm clicking a project in the Command Center project rail updates the central Current Project in place without leaving Global Overview.
14. Confirm the selected project row is visibly active and `Open cockpit` navigates to exactly that selected project.
15. Confirm a newly registered project appears in the Command Center project rail and sidebar project shortcuts without app restart.
16. Confirm opening that newly registered project's cockpit never flashes FormuLab or any unrelated fixture project.
17. Confirm Command, Assistant, and Notifications topbar icons open distinct surfaces.
18. Confirm the footer is bottom-center/global, heart is red, and the sidebar no longer contains the old footer block.
19. Confirm no giant Runtime/Database/Watcher blocks force the main dashboard downward.
20. Confirm the Projects rail, center Cockpit, and right AI/System column are visible together.
21. If the user has not personally approved the visual result, record `PENDING USER VISUAL ACCEPTANCE`; do not fabricate final visual PASS.

## Future milestones

M11/M12 will add richer Global Command Center and Project Cockpit functionality, but they must preserve this single-viewport composition, live-project identity contract, selection behavior, project-rail name-only density rule, and canonical layout unless the user explicitly changes them.

Future milestones may replace task/workflow placeholders with real data. They must not regress project identity back to fixtures or turn the dashboard back into a long vertically stacked page.

## M08.00 Presentation Bootstrap

The global application background source is:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI\scene 3 starting point.png`

The repository asset is `src/assets/hiveai-app-background.png`. Use one fixed
full-application background layer across every route with preserved aspect ratio,
no tiling, and a dark navy/black readability overlay. The background remains
subordinate to operational content and must not alter the approved viewport,
sidebar, topbar, project rail, footer, or Command Center geometry.

The canonical native opening video source is:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI\videos and gifs\opening video.mp4`

The repository asset is `src/assets/opening-video.mp4`. It plays only in the
native in-window startup overlay, once per native application lifecycle. Native
restart creates a new lifecycle and may play it again; SPA navigation, project
selection, route changes, minimize/restore, and ordinary in-app actions must not
replay it. The React app mounts immediately and `hiveai_frontend_ready` remains
independent of video completion. Media errors and a bounded failsafe must remove
the overlay rather than trap the user.

M08.00 uses a restrained adaptation of the FMCG ERP NEON LIQUID GLASS DESIGN
SYSTEM: dark translucent navy surfaces, bounded blur/saturation, thin
cyan/blue borders, restrained violet glow, readable text, and obvious focus
states. Existing primary/secondary buttons and real table surfaces may receive
controlled glass/glow treatment. Do not apply a competing theme, continuous
glow animation, gaming-HUD styling, heavy nested blur, or opacity that harms
readability or single-viewport geometry.

## M08.00B Background and startup overlay corrections

The canonical application background belongs to the post-sidebar workspace. It
is rendered by `.main-area` and must never sit behind or visually occupy the
sidebar. The background remains centered within the available workspace and
must preserve the approved dashboard geometry on route changes and viewport
resizes.

The native startup video is a true fixed client-viewport overlay outside normal
document flow. It must cover the client viewport, contain the video without
cropping, suppress controls, and never create document overflow or scrollbars.
The React application remains mounted beneath it and frontend readiness is
independent of video completion.

Startup playback authority is native and process-scoped: the first native
claim in a process may play once, a second claim in that process skips, and a
new native process may claim again. Browser previews do not invoke the native
claim. SPA navigation, project selection, route changes, minimize/restore, and
media failure must not replay or trap the application.

## M08 Task Sources workspace

The existing `/tasks` route is the Task Sources workspace until M09. It shows
only bounded source inventory evidence for the selected live Registry project:
relative path, source kind, standard/custom origin, authority/priority,
filesystem freshness evidence, bounded status, and custom-path controls. It
must not display parsed task counts, progress, workflow state, owner,
completion, or next-best-task claims. The table may scroll internally while
the approved Command Center single-viewport composition remains unchanged.
