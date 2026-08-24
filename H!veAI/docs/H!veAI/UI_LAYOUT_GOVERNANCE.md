# H!veAI Canonical UI Layout Governance

This document is a durable visual-layout contract for H!veAI. It exists because the current M02-era Command Center composition diverged materially from the user's canonical dashboard reference and expanded into multiple vertical screens.

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
- If a data list has more rows than fit, that panel may have its own bounded internal scrolling area. The whole dashboard must not turn into a multi-page vertical document.
- For substantially smaller windows, responsive reflow or bounded internal panel scrolling is allowed, but avoid turning the overview into several stacked full-width screens.

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
8. Footer branding remains compact.

Do not replace this with a two-column collection of large project cards plus several full-width infrastructure panels.

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

## Scroll policy

Preferred implementation model:

- `html`, `body`, `#root`, and `.app-shell` occupy the viewport;
- the AppShell itself does not create document-length vertical overflow;
- the Command Center route uses an explicit viewport-height grid/flex layout with `min-height: 0` on nested grid/flex children;
- overflow is assigned only to specific bounded panels that need it;
- other routes may keep their own route-appropriate scrolling behavior.

Do not globally break the Projects, Cockpit, Settings, or other routes merely to remove Command Center scrolling.

## Visual acceptance

A Command Center visual change is not accepted merely because the frontend compiles.

Before claiming visual PASS:

1. Launch the stable `Desktop\H!veAI.lnk` development build.
2. Confirm the real production-mode Tauri frontend renders.
3. Compare side-by-side with the canonical dashboard reference.
4. At `1536x1024`, confirm the complete primary overview is visible without outer vertical/horizontal scrolling.
5. At the user's maximized desktop size (`2048x1280` evidence), confirm the complete primary overview is visible without outer vertical/horizontal scrolling.
6. Confirm the sidebar small logo is not clipped.
7. Confirm the sidebar wordmark uses `H!veAI text logo.png`.
8. Confirm no giant Runtime/Database/Watcher blocks force the main dashboard downward.
9. Confirm the Projects rail, center Cockpit, and right AI/System column are visible together.
10. If the user has not personally approved the visual result, record `PENDING USER VISUAL ACCEPTANCE`; do not fabricate final visual PASS.

## Future milestones

M11/M12 will add richer Global Command Center and Project Cockpit functionality, but they must preserve this single-viewport composition and canonical layout unless the user explicitly changes it.

Future milestones may replace placeholders with real data. They must not regress the dashboard back into a long vertically stacked page.
