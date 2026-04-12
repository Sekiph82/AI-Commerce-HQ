"""
AI Commerce HQ — Python Backend
FastAPI + WebSocket server that runs the AI agent system.
"""
import asyncio
import sys
import os
from contextlib import asynccontextmanager

# Add backend dir to path
sys.path.insert(0, os.path.dirname(__file__))

from fastapi import FastAPI, Request
from fastapi.responses import Response
from starlette.websockets import WebSocket
import uvicorn

from database.db import init_db, get_config
from api.routes import router
from api.websocket import websocket_endpoint, broadcast_all


_gmo_task: asyncio.Task | None = None


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


@asynccontextmanager
async def lifespan(app: FastAPI):
    # ── Startup ──────────────────────────────────────────────────────────────
    global _gmo_task

    print("[HQ] Initializing database...")
    await init_db()
    print("[HQ] Database ready.")

    config = await get_config()

    print("[HQ] Starting Global Master Orchestrator...")
    from orchestrator.gmo import GlobalMasterOrchestrator
    gmo = GlobalMasterOrchestrator(broadcast=broadcast_all, config=config)
    _gmo_task = asyncio.create_task(_run_gmo(gmo))

    print("[HQ] Runtime started. Listening on http://localhost:8765")

    yield  # app is running

    # ── Shutdown ─────────────────────────────────────────────────────────────
    if _gmo_task:
        _gmo_task.cancel()
        try:
            await _gmo_task
        except asyncio.CancelledError:
            pass
    print("[HQ] Runtime shutdown.")


# ── FastAPI app (HTTP only) ───────────────────────────────────────────────────
_fastapi = FastAPI(title="AI Commerce HQ Runtime", version="1.0.0", lifespan=lifespan)


@_fastapi.middleware("http")
async def _cors(request: Request, call_next):
    """Add CORS headers to all HTTP responses."""
    if request.method == "OPTIONS":
        return Response(
            status_code=200,
            headers={
                "Access-Control-Allow-Origin": "*",
                "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, OPTIONS, PATCH",
                "Access-Control-Allow-Headers": "*",
            },
        )
    response = await call_next(request)
    response.headers["Access-Control-Allow-Origin"] = "*"
    response.headers["Access-Control-Allow-Methods"] = "GET, POST, PUT, DELETE, OPTIONS, PATCH"
    response.headers["Access-Control-Allow-Headers"] = "*"
    return response


_fastapi.include_router(router)


@_fastapi.post("/api/config/reload")
async def reload_config():
    """Reload config and push to agents. Called after settings save."""
    await get_config()
    if _gmo_task and not _gmo_task.done():
        await broadcast_all({
            "type": "system_event",
            "data": {"eventType": "system", "message": "Configuration updated — agents will use new settings on next cycle."},
        })
    return {"status": "reloaded"}


# ── Outer ASGI dispatcher ─────────────────────────────────────────────────────
#
# WebSocket upgrade requests are caught here — BEFORE the FastAPI app and ALL
# its middleware.  No middleware can return 403 because the WebSocket scope
# never enters the FastAPI stack at all.
#
# HTTP and lifespan scopes pass straight through to FastAPI.

class _App:
    async def __call__(self, scope, receive, send):
        if scope["type"] == "websocket":
            # Handle directly — zero middleware involvement
            ws = WebSocket(scope, receive, send)
            await websocket_endpoint(ws)
        else:
            await _fastapi(scope, receive, send)


app = _App()


if __name__ == "__main__":
    uvicorn.run(
        "main:app",
        host="127.0.0.1",
        port=8765,
        reload=False,
        log_level="info",
    )
