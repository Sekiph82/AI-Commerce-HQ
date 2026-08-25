---
hiveaiDashboardSchema: hiveai-project-dashboard/v1
projectKey: ai-commerce-hq-hiveai
repository: Sekiph82/AI-Commerce-HQ
branchPolicy: H!veAI branch is the active H!veAI application branch
dashboardMode: source-map
refreshPolicy: watcher-driven source invalidation; no generated status commits
---

# H!veAI Project Dashboard Manifest

This file is a pointer map for H!veAI. It is not a task ledger and must not duplicate task checkboxes.

## Project identity

Project: H!veAI inside AI-Commerce-HQ
Repository: `Sekiph82/AI-Commerce-HQ`
Active branch: `H!veAI`

## Source authorities

Canonical task source: `H!veAI/TASKS.md`
Roadmap source: `H!veAI/CODEX_ROADMAP.md`
Handoff source: none verified
Progress/history sources: `H!veAI/docs/H!veAI/audits/`, `H!veAI/docs/H!veAI/codex-logs/`
Architecture source: `H!veAI/ARCHITECTURE.md`
Decision/governance source: `H!veAI/CONSTITUTION.md`
Agent instruction source: `H!veAI/AGENTS.md`
Security source: no dedicated root security ledger verified
Build/test metadata: `H!veAI/package.json`, `H!veAI/src-tauri/Cargo.toml`

## Authority notes

The repository-root legacy `TASKS.md` belongs to the older AI-Commerce-HQ application history and must not override the active H!veAI child application's canonical `H!veAI/TASKS.md`.

Historical prompts, builder logs, and audits are evidence/history. They are not current task authority unless `H!veAI/TASKS.md` explicitly reopens a finding.

## Refresh model

H!veAI should derive live state from Registry/Git/watcher evidence plus the canonical sources above. This manifest should remain pointer-only and should not be rewritten as a generated status snapshot.
