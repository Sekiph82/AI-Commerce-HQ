# M03 Runtime Architecture Refactor

## Scope and decision

M03 establishes the active H!veAI runtime boundary. The selected architecture is **Rust-native H!veAI core with no always-on Python sidecar**. H!veAI starts its Tauri plugins and an in-process `RuntimeSupervisor`; it does not start, depend on, or probe the legacy AI-Commerce-HQ backend.

The supervisor is intentionally dormant infrastructure for future approved adapters. M03 does not add SQLite, Project Registry, Git Engine, filesystem watching, task parsing, agent execution, PTY execution, audits, GitHub integration, or arbitrary process execution.

## Legacy runtime inventory

Inventory was read-only. The legacy application was not launched.

| Responsibility | Evidence | Startup trigger | Side-effect / credential risk | Reuse decision |
| --- | --- | --- | --- | --- |
| FastAPI and WebSocket server | Parent `backend/main.py:68-124`, `backend/api/websocket.py:18-81` | `python backend/main.py` / uvicorn | Opens HTTP and WebSocket service; wildcard CORS; event replay | Archive-only source material; future event-ledger concepts are reimplemented under H!veAI |
| Database initialization | Parent `backend/main.py:43-48`, `backend/database/db.py:7-34` | FastAPI lifespan | Creates `hq.db`, applies ad-hoc product migrations, reads `HQ_DATA_DIR` | Excluded from M03; versioned H!veAI SQLite begins in M04 |
| GMO lifecycle and restart loop | Parent `backend/main.py:26-35`, `49-65` | FastAPI lifespan | Starts all commerce orchestration and restarts failures every 10 seconds | Permanently excluded from H!veAI startup |
| Marketplace/social/trading orchestrators | Parent `backend/orchestrator/gmo.py:80-285`, plus Etsy/Fiverr/Trading/YouTube/TikTok modules | GMO imports and creates platform masters | Network/API credentials and automated commerce/trading/social side effects | Archive-only; later adapters require explicit H!veAI milestones |
| BaseAgent lifecycle | Parent `backend/agents/base_agent.py:8-90` | Orchestrator-created agents | Async loops, broadcast events, broad string states | Pattern reference only; future H!veAI agent contract belongs to later milestones |
| Development launcher | Parent `dev.py:124-175` | `python dev.py` | Installs Python dependencies, starts backend, waits on port 8765, then starts Tauri | Not used by H!veAI; parent launcher remains untouched |
| Legacy Tauri process lifecycle | Parent `src-tauri/src/main.rs:9-190` | Parent Tauri startup | Spawns bundled `backend.exe` or Python, polls `localhost:8765`, restarts and kills child | Not copied; child H!veAI has no equivalent spawn path |
| Health endpoint / port | Parent `backend/main.py:127-134` and `dev.py:25`, `111-122` | Backend startup | Requires `127.0.0.1:8765` | H!veAI does not open or require port 8765 |
| Python dependencies and env | Parent `backend/requirements.txt:5-27`, `backend/database/db.py:8` | Parent launcher/backend import | FastAPI, uvicorn, AI clients, SQLite, `HQ_DATA_DIR` | Not a child dependency; H!veAI starts without Python |

## Component ownership and startup model

| Component | Owner | M03 behavior |
| --- | --- | --- |
| Native lifecycle and plugins | `H!veAI/src-tauri/src/lib.rs` | Starts in Tauri; no child process |
| Runtime state domain | `H!veAI/src-tauri/src/runtime.rs` | Owns explicit states, health, disabled legacy representation, transitions, backoff helper, and error sanitization |
| Runtime status presentation | `H!veAI/src/components/RuntimeStatusPanel.tsx` | Reads only `hiveai_runtime_status`; browser preview truthfully reports native runtime unavailable |
| Legacy commerce runtime | Parent `backend/` and parent `src-tauri/` | Excluded from H!veAI startup and not imported by child code |
| Future adapters/sidecars | Future milestones | Must be added through an explicit allowlisted boundary; none exists in M03 |

Startup sequence: Tauri initializes logging/notifications, registers `RuntimeSupervisor`, and opens the H!veAI window. The frontend loads the existing M02 shell and requests native/runtime status. Shutdown closes the window without a child-process stop path because no child process exists.

## Runtime state and recovery model

`RuntimeState` includes `STOPPED`, `STARTING`, `HEALTHY`, `DEGRADED`, `STOPPING`, `FAILED`, and `DISABLED`. `RuntimeHealth` includes `HEALTHY`, `DEGRADED`, `FAILED`, and `DISABLED`. Components expose id, display name, kind, state, health, timestamps, restart count, sanitized error, and ownership.

M03 reports the native core as `HEALTHY` and the legacy commerce component as `DISABLED`. `sidecarEnabled` is false. Valid transition rules and a bounded exponential restart-backoff helper are tested as dormant supervisor infrastructure. No process is spawned merely to satisfy the recovery model.

## IPC and security boundary

M01 commands remain unchanged: `hiveai_native_status` and `hiveai_request_restart`. M03 adds only `hiveai_runtime_status`, which returns structured H!veAI-owned state. The new capability permission is `allow-runtime-status`. No shell, filesystem, HTTP, generic process, or unrestricted network permissions were added. The frontend cannot supply executable paths or commands.

## Legacy containment proof

- Child `H!veAI/src-tauri/src/lib.rs` contains no `Command`, `Child`, Python path, port 8765 probe, or backend health poll.
- Child Tauri startup manages only `RuntimeSupervisor`; no legacy module is imported or launched.
- Child `package.json` and Tauri config do not reference the parent backend or `8765`.
- Bounded smoke verifies H!veAI starts with no parent Python dependency, no legacy child process, and zero port 8765 listeners.
- Parent application files remain source material and were not modified.

## Dependency advisory disposition

The initial M02 audit reported three findings: two high and one critical. The scoped child command `npm audit --json` identified:

- `react-router-dom@7.8.2` direct runtime dependency, with vulnerable transitive `react-router` ranges. The affected server/SSR/open-redirect surfaces are not used by this client-only BrowserRouter shell, but the package was upgraded to compatible `7.18.2` rather than relying on non-applicability.
- `vitest@3.2.4` direct dev dependency, critical advisory for the Vitest UI server file-read/execute path below `3.2.6`. H!veAI runs `vitest run` without the UI server and does not package Vitest, but it was upgraded to compatible `3.2.7`.

After targeted upgrades, `npm audit` reports zero vulnerabilities. No `npm audit fix --force` or unrelated major-version churn was used.

## CSP and technical debt

The M01 localhost Vite HTTP/WebSocket CSP allowances remain so the verified Tauri development flow continues to work. M03 does not broaden CSP. A production-only tightened policy remains a pre-release hardening task once dev and production configuration are separated safely.

Remaining debt: versioned H!veAI SQLite migrations, a real project/task model, approved adapter supervision, and production CSP separation belong to later milestones. The parent backend remains intentionally present as archived source material and is not a runtime dependency of H!veAI.

## Verification evidence

- `npm audit`: zero vulnerabilities.
- `npm run typecheck`: passed.
- `npm test`: passed with 8 frontend tests.
- `npm run build`: passed.
- `cargo fmt --manifest-path H!veAI/src-tauri/Cargo.toml -- --check`: passed after formatting.
- `cargo check`, `cargo test`, and `cargo build`: passed; 5 meaningful Rust runtime tests.
- Bounded `npm run tauri:dev` smoke: H!veAI title, native status log, runtime status, zero port 8765 listeners, no legacy process, clean shutdown.
