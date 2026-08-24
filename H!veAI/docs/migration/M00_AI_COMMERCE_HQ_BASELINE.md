# M00 AI-Commerce-HQ Baseline

Date: 2026-08-24

Git root: `C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Application root: `C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

## Repository Identity

- Official repository: `https://github.com/Sekiph82/AI-Commerce-HQ.git`
- Canonical branch: `H!veAI`
- Starting official branch HEAD: `f5e311e81435b0252420fde9609c96ea3fe25144`
- `H!veAI` is not a nested Git repository.

## Parent Application Summary

The parent root contains the old AI-Commerce-HQ application. M00 treats it as
source material only and does not convert it in place.

Frontend:
- React 18 + TypeScript + Vite.
- Entry point: `src/main.tsx`.
- Root screen: `src/App.tsx`.
- State management: Zustand in `src/store/useAppStore.ts`.
- Styling: Tailwind via `tailwind.config.js` and `src/index.css`.
- Primary UI: Three.js/React Three Fiber gamified office world under
  `src/components/Game/`.

Desktop/Tauri:
- Tauri 1.x stack in `src-tauri`.
- Rust entry point: `src-tauri/src/main.rs`.
- Native shell starts or reuses a Python backend on port `8765`.
- Tauri allowlist includes window controls, shell open, and HTTP access scoped
  to localhost.

Backend:
- Python FastAPI runtime in `backend/main.py`.
- WebSocket manager in `backend/api/websocket.py`.
- REST API in `backend/api/routes.py`.
- Global Master Orchestrator starts automatically on backend startup.
- Commerce orchestrators exist for Etsy, Fiverr, trading, YouTube, and TikTok.

Database:
- Async SQLAlchemy + SQLite.
- Default data directory: `~/.ai-commerce-hq`.
- Database file: `hq.db`.
- Schema includes agents, products, config, and events.
- Migration behavior is ad hoc column-add SQL with swallowed exceptions.

Build/test:
- Root scripts include `npm run build`, `npm run tauri:dev`,
  `npm run tauri:build`, and `python dev.py`.
- Python launcher can install dependencies and start backend/Tauri development.
- There is no tracked `tests/` directory on the official `H!veAI` branch.
- M00 validation result: old parent `npm run build` failed because the current
  local `node_modules` lacks `framer-motion` / type declarations.
- M00 validation result: `python -m compileall -q backend` passed.
- M00 validation result: old parent `cargo check --manifest-path
  src-tauri/Cargo.toml` failed because Tauri 1 expects `../dist`, and the
  failed frontend build did not produce `dist`.

Security:
- Root `.env` exists locally and is ignored. It was not read.
- API key fields are stored through the old config route and old SQLite config
  table.
- Backend CORS allows all origins.
- Old code can call external commerce/AI APIs when configured.
- Old runtime must not be launched casually during H!veAI migration.
