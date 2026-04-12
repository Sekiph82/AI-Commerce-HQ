"""WebSocket manager for real-time frontend updates."""
import json
import asyncio
from typing import Set
from fastapi import WebSocket, WebSocketDisconnect

# Global set of connected WebSocket clients
_connections: Set[WebSocket] = set()
_lock = asyncio.Lock()

# Message buffer — last N messages replayed to late-connecting clients
# so the operation log is populated immediately on connect
_buffer: list[dict] = []
_BUFFER_MAX = 30
_REPLAY_ON_CONNECT = 20   # how many to send on new connection


async def broadcast_all(message: dict):
    """Send a message to all connected WebSocket clients and buffer it."""
    # Keep recent talking_table and agent_update messages for new clients
    if message.get("type") in ("talking_table", "agent_update", "system_event", "product_update"):
        _buffer.append(message)
        del _buffer[:-_BUFFER_MAX]   # keep last N

    if not _connections:
        return

    payload = json.dumps(message)
    dead: Set[WebSocket] = set()

    async with _lock:
        for ws in _connections:
            try:
                await ws.send_text(payload)
            except Exception:
                dead.add(ws)
        for ws in dead:
            _connections.discard(ws)


async def websocket_endpoint(websocket: WebSocket):
    """WebSocket connection handler."""
    await websocket.accept()

    async with _lock:
        _connections.add(websocket)

    print(f"[WS] Client connected. Total: {len(_connections)}")

    try:
        # Initial heartbeat
        await websocket.send_text(json.dumps({
            "type": "heartbeat",
            "data": {"status": "connected"},
        }))

        # Replay recent messages so the client sees current state immediately
        if _buffer:
            for msg in _buffer[-_REPLAY_ON_CONNECT:]:
                try:
                    await websocket.send_text(json.dumps(msg))
                except Exception:
                    break

        while True:
            try:
                data = await asyncio.wait_for(websocket.receive_text(), timeout=30)
                msg = json.loads(data)
                if msg.get("type") == "ping":
                    await websocket.send_text(json.dumps({"type": "heartbeat", "data": {}}))
            except asyncio.TimeoutError:
                # Periodic keepalive heartbeat
                await websocket.send_text(json.dumps({"type": "heartbeat", "data": {}}))
            except WebSocketDisconnect:
                break
    except Exception as e:
        print(f"[WS] Error: {e}")
    finally:
        async with _lock:
            _connections.discard(websocket)
        print(f"[WS] Client disconnected. Total: {len(_connections)}")
