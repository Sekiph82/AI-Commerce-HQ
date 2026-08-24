# M00 Reuse Matrix

Date: 2026-08-24

## Classification

| Component | Classification | Notes |
| --- | --- | --- |
| React/Vite tooling | A. REUSE WITH MINOR CHANGES | Good basis for H!veAI UI after moving under `H!veAI`. |
| Tailwind | A. REUSE WITH MINOR CHANGES | Keep utility workflow; replace old commerce/game palette later. |
| Zustand | B. REUSE AFTER REFACTOR | Pattern is useful, but state shape is commerce/product specific. |
| Framer Motion | A. REUSE WITH MINOR CHANGES | Suitable for dashboard transitions in later UI milestone. |
| Tauri shell concepts | B. REUSE AFTER REFACTOR | Lifecycle and desktop packaging are useful; implementation is Tauri 1.x. |
| Rust lifecycle/process management | B. REUSE AFTER REFACTOR | Backend spawning/health/restart concepts are reusable with stricter policy. |
| FastAPI backend | C. ARCHIVE / REFERENCE ONLY | Useful boundary reference, but H!veAI target favors Rust native core first. |
| WebSocket manager | B. REUSE AFTER REFACTOR | Broadcast/replay/heartbeat pattern is useful for agent events. |
| BaseAgent | B. REUSE AFTER REFACTOR | Abstract agent lifecycle is useful; product domain and status strings differ. |
| async SQLite | B. REUSE AFTER REFACTOR | Local-first persistence pattern is useful; schema must be replaced. |
| migrations | C. ARCHIVE / REFERENCE ONLY | Current ad hoc migration pattern is not sufficient. |
| 3D UI | D. DO NOT COPY INTO H!veAI | H!veAI requires a professional command center, not a game world. |
| commerce orchestrators | D. DO NOT COPY INTO H!veAI | Etsy/Fiverr/trading/YouTube/TikTok product automation is out of scope. |
| revenue/XP/gamification | D. DO NOT COPY INTO H!veAI | Not aligned with H!veAI command-center workflow. |
| installer/build scripts | B. REUSE AFTER REFACTOR | Windows packaging knowledge is useful after Tauri 2 migration. |

## M00 Decision

No old parent runtime code is copied into H!veAI during M00. The reusable pieces
are recorded here for later milestones.
