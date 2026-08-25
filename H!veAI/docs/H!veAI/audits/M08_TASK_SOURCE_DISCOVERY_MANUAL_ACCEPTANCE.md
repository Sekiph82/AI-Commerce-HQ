# M08 Task Source Discovery Manual Acceptance

Date: 2026-08-25

## Result

`PASS`

The user directly inspected the refreshed native H!veAI `/tasks` Task Sources workspace and explicitly accepted the presentation and behavior as OK.

This manual acceptance closes the only remaining gate from `M08C_CUSTOM_ORDER_BACKCOMPAT_STRICT_REAUDIT.md`.

## Evidence supplied by the user

The user supplied native H!veAI screenshots of the Task Sources workspace for three different registered projects:

1. `AI-Commerce-HQ`
   - selected project identity and local project path render correctly;
   - 2 available sources are shown;
   - `TASKS.md` and `task.md` appear as STANDARD TASKS sources;
   - authority/priority, modified evidence, and AVAILABLE status are visible.

2. `Bulk-Edit`
   - the selected project changes in-place to `Bulk-Edit`;
   - 4 available sources are shown;
   - `TASKS.md`, `HANDOFF.md`, `ROADMAP.md`, and `CLAUDE.md` render with distinct kinds/authority classes;
   - no stale `AI-Commerce-HQ` source inventory remains visible after the project switch.

3. `ScrubBots`
   - the selected project changes in-place to `ScrubBots`;
   - 2 available sources are shown;
   - `tasks.md` and `CLAUDE.md` render with the expected STANDARD origin and AVAILABLE status;
   - no stale `Bulk-Edit` inventory remains visible.

Across the supplied screenshots:

- the `Task Sources` heading and selected-project identity are readable;
- the source table exposes Path, Kind, Origin, Authority, Modified, and Status columns;
- the `Rescan sources` control is visible and correctly placed;
- the Custom source paths panel is present and visually contained;
- the approved H!veAI sidebar, enlarged one-piece logo, topbar, post-sidebar hive background, and restrained liquid-glass treatment remain intact;
- there is no visible horizontal overflow or layout breakage;
- the single right-side page scrollbar is visually acceptable for this route;
- no fake parsed-task/workflow/completion claims are shown.

## User statement

The user explicitly stated: `bence OK`.

This is treated as direct manual acceptance of the remediated native Task Sources workspace.

## Final manual gate

`M08 native /tasks visual acceptance = PASS`
