"""
AI Commerce HQ — Python Backend
FastAPI + WebSocket server that runs the AI agent system.
"""
import asyncio
import sys
import os

# Add backend dir to path
sys.path.insert(0, os.path.dirname(__file__))

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
import uvicorn

from database.db import init_db, get_config
from api.routes import router
from api.websocket import websocket_endpoint, broadcast_all


app = FastAPI(title="AI Commerce HQ Runtime", version="1.0.0")

# Allow frontend (Tauri webview + dev server)
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

app.include_router(router)


@app.websocket("/ws")
async def ws_endpoint(websocket):
    await websocket_endpoint(websocket)


# --- Startup ---

_gmo_task: asyncio.Task | None = None


@app.on_event("startup")
async def startup():
    global _gmo_task
    print("[HQ] Initializing database...")
    await init_db()
    print("[HQ] Database ready.")

    # Load config
    config = await get_config()

    # Start the Global Master Orchestrator
    print("[HQ] Starting Global Master Orchestrator...")
    from orchestrator.gmo import GlobalMasterOrchestrator

    gmo = GlobalMasterOrchestrator(broadcast=broadcast_all, config=config)
    _gmo_task = asyncio.create_task(_run_gmo(gmo))

    print("[HQ] Runtime started. Listening on http://localhost:8765")


async def _run_gmo(gmo):
    """Run GMO with auto-restart on failure."""
    while True:
        try:
            await gmo.safe_run()
        except asyncio.CancelledError:
            break
        except Exception as e:
            print(f"[GMO] Crashed: {e} — restarting in 10s")
            await asyncio.sleep(10)


@app.on_event("shutdown")
async def shutdown():
    global _gmo_task
    if _gmo_task:
        _gmo_task.cancel()
        try:
            await _gmo_task
        except asyncio.CancelledError:
            pass
    print("[HQ] Runtime shutdown.")


# --- Config hot-reload ---

@app.post("/api/config/reload")
async def reload_config():
    """Reload config and push to agents. Called after settings save."""
    config = await get_config()
    if _gmo_task and not _gmo_task.done():
        # Agents will pick up new config on next cycle via DB
        await broadcast_all({
            "type": "system_event",
            "data": {"eventType": "system", "message": "Configuration updated — agents will use new settings on next cycle."},
        })
    return {"status": "reloaded"}


if __name__ == "__main__":
    uvicorn.run(
        "main:app",
        host="127.0.0.1",
        port=8765,
        reload=False,
        log_level="info",
    )
