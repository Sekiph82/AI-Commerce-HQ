"""WebSocket manager for real-time frontend updates."""
import json
import asyncio
from typing import Set
from fastapi import WebSocket, WebSocketDisconnect

# Global set of connected WebSocket clients
_connections: Set[WebSocket] = set()
_lock = asyncio.Lock()


async def broadcast_all(message: dict):
    """Send a message to all connected WebSocket clients."""
    if not _connections:
        return

    payload = json.dumps(message)
    dead = set()

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
        # Send initial heartbeat
        await websocket.send_text(json.dumps({"type": "heartbeat", "data": {"status": "connected"}}))

        while True:
            try:
                # Receive messages (clients don't send much, but keep connection alive)
                data = await asyncio.wait_for(websocket.receive_text(), timeout=30)
                # Handle client messages if needed
                msg = json.loads(data)
                if msg.get("type") == "ping":
                    await websocket.send_text(json.dumps({"type": "heartbeat", "data": {}}))
            except asyncio.TimeoutError:
                # Send periodic heartbeat
                await websocket.send_text(json.dumps({"type": "heartbeat", "data": {}}))
            except WebSocketDisconnect:
                break
    except Exception as e:
        print(f"[WS] Error: {e}")
    finally:
        async with _lock:
            _connections.discard(websocket)
        print(f"[WS] Client disconnected. Total: {len(_connections)}")
