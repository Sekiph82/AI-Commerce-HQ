# M02 UI Shell and Design System

## Scope

M02 establishes the production-quality visual shell for H!veAI. It is UI architecture with static fixtures only. No project registry, repository scanning, Git engine, filesystem watcher, task intelligence, agent execution, audit service, GitHub integration, persistence, or arbitrary process execution is present.

## Frontend architecture

- React + TypeScript + Vite remain the child frontend stack established in M01.
- `src/App.tsx` owns route composition and `src/components/Shell.tsx` owns the persistent desktop frame.
- `src/components/ui.tsx` contains reusable visual primitives and `src/pages.tsx` contains route-level surfaces.
- `src/types.ts` defines the canonical UI state and actor vocabulary.
- `src/fixtures.ts` is the explicit static-data boundary; fixture provenance is visible in the UI and comments.

## Design tokens and components

Tokens live in `src/styles.css` and cover graphite surfaces, borders, text hierarchy, accent, success, warning, danger, running, audit, human-required, and external-wait states. The file also defines typography, spacing rhythm, radius, elevation, motion timings, focus treatment, responsive breakpoints, and reduced-motion behavior.

Reusable pieces include `PageHeader`, `SectionHeader`, `StatusBadge`, `ActorBadge`, `MetricCard`, `ProgressIndicator`, `ProjectOperationCard`, `ActivityRow`, `PrimaryActionButton`, and empty/loading/error states. Navigation and command palette are reusable shell primitives.

## Routes

The desktop SPA provides `/`, `/projects`, `/projects/:id`, `/tasks`, `/agents`, `/audits`, `/activity`, and `/settings`. The Project Cockpit has the requested Overview, Tasks, Workflow, Agents, Audit, Git, Tests, Activity, Files, and Settings tabs. Non-Overview tabs are polished placeholders until their runtime milestones.

## Accessibility and motion

Semantic links and buttons, visible keyboard focus, labels for icon-only controls, explicit status text plus icons, responsive layouts, command palette keyboard shortcuts, and a reduced-motion media policy are included. Motion is limited to page/card entrance, progress interpolation, active navigation, and palette transitions; there is no continuous decorative animation.

## Dependencies

Added `framer-motion` for intentional transitions, `lucide-react` for consistent accessible iconography, `react-router-dom` for the desktop SPA, and Vitest/Testing Library/jsdom for child frontend tests. `motion-dom` and `motion-utils` are pinned to compatible `12.23.28` releases because the initial Framer Motion install resolved an incompatible transitive minor.

## CSP decision

The M01 CSP localhost HTTP/WebSocket origins remain because Tauri development uses the Vite server. M02 does not broaden capabilities or add network access. A production-only tightened CSP remains a release-hardening decision for M20 because changing the current config without a separate production/dev config would risk breaking the verified development flow.

## Known limitations

All project, task, agent, audit, activity, and engineering brief values are static fixtures. Buttons that imply future operations show a safe later-milestone message or only navigate within the UI. Native status IPC and M01 logging/notification foundations remain in Rust and are not removed.
