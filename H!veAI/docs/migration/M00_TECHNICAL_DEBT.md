# M00 Technical Debt

Date: 2026-08-24

## Parent App Debt

- Tauri 1.x must be replaced with Tauri 2 in the H!veAI child application.
- Old shell has broad HTTP and shell-open allowances compared with the desired
  H!veAI permission model.
- Backend starts the Global Master Orchestrator automatically on startup.
- Commerce API wrappers can call external services when credentials are present.
- Config can contain API keys in the old SQLite config table.
- CORS currently allows all origins.
- SQLite migrations are ad hoc and not versioned.
- UI is centered on game/3D/revenue mechanics that H!veAI should not inherit.
- Local ignored artifacts exist: `.env`, `.next/`, `node_modules/`,
  `src-tauri/target/`, `tsconfig.tsbuildinfo`, and `next-env.d.ts`.
- The old parent frontend build currently fails in this local checkout because
  `framer-motion` is absent from `node_modules`.
- The old parent Tauri check currently fails after the frontend failure because
  `src-tauri/tauri.conf.json` expects `../dist`.

## M00 Containment

M00 does not launch inherited commerce workflows and does not copy old runtime
code into H!veAI. Later milestones must use the reuse matrix and this debt list
before adopting any parent code.
