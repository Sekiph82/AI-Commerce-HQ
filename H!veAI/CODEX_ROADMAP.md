# H!veAI 20-Milestone Codex Roadmap

Codex must execute one milestone at a time. Before each milestone: read
AGENTS.md, CONSTITUTION.md, ARCHITECTURE.md and TASKS.md; inspect git; verify
previous milestone; run baseline tests; implement only current scope; add
tests; verify; update docs/tasks; review diff; commit focused changes.

M01 — Tauri 2 Foundation: modernize shell, capabilities, identity, native lifecycle.
M02 — UI Shell: replace game UI with professional H!veAI navigation/design system.
M03 — Runtime Refactor: remove commerce auto-runtime; decide Rust vs sidecar boundary.
M04 — Database: versioned H!veAI SQLite schema and migration recovery.
M05 — Project Registry: safely register local repos and GitHub identity.
M06 — Git Engine: local status, diff, commits, remote divergence and safe writes.
M07 — Watcher: reactive repo/task changes and durable snapshots.
M08 — Task Discovery: find authoritative planning/progress sources.
M09 — Task Parser: normalize FormuLab, Scrubbots, FMCG ERP and generic repos.
M10 — Workflow: canonical state machine, gates, evidence and overrides.
M11 — Command Center: single-screen portfolio operations and attention queue.
M12 — Project Cockpit: end-to-end per-project operational view.
M13 — Codex Adapter: real project-scoped start/resume/stop/stream sessions.
M14 — Session Center: PTY/xterm, live terminal, diff, permissions and recovery.
M15 — Prompt Engine: versioned prompts with provenance and dispatch.
M16 — GPT Audit: independent PASS/FAIL audit and remediation loop.
M17 — Claude Adapter: second builder through same agent contract.
M18 — GitHub: PR/issues/Actions/releases and local/remote reconciliation.
M19 — Next Best Task + Brief: explainable priority and engineering summary.
M20 — Chat + Hardening + Release: action-capable AI chat, security, packaging, v1.0.

## Dependency path
M01→M02→M03→M04→M05→M06→M07→M08→M09→M10→M11/M12→M13→M14→M15→M16→M17→M18→M19→M20

## Exit rule
Codex may not mark a milestone complete because code was generated. Completion
requires tests/evidence and an updated TASKS.md.
