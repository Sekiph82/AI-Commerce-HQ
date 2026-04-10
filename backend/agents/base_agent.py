import asyncio
import time
import uuid
from abc import ABC, abstractmethod
from typing import Optional, Callable, Awaitable, Any


class BaseAgent(ABC):
    """Base class for all AI agents in the system."""

    def __init__(
        self,
        label: str,
        role: str,
        platform: str,
        broadcast: Callable[[dict], Awaitable[None]],
        config: dict,
    ):
        self.id = str(uuid.uuid4())
        self.label = label
        self.role = role
        self.platform = platform
        self.broadcast = broadcast
        self.config = config
        self.status = "idle"
        self.current_task: Optional[str] = None
        self.desk_id: Optional[str] = None
        self.created_at = time.time()
        self.task_count = 0
        self._stop_event = asyncio.Event()

    def to_dict(self) -> dict:
        return {
            "id": self.id,
            "label": self.label,
            "role": self.role,
            "status": self.status,
            "platform": self.platform,
            "currentTask": self.current_task,
            "deskId": self.desk_id,
            "createdAt": int(self.created_at * 1000),
            "taskCount": self.task_count,
        }

    async def set_status(self, status: str, task: Optional[str] = None):
        self.status = status
        self.current_task = task
        if status == "working":
            self.task_count += 1
        await self.broadcast({
            "type": "agent_update",
            "data": self.to_dict(),
        })

    async def emit_event(self, message: str, event_type: str = "system"):
        await self.broadcast({
            "type": "system_event",
            "data": {
                "eventType": event_type,
                "message": f"[{self.label}] {message}",
                "agentId": self.id,
                "timestamp": int(time.time() * 1000),
            },
        })

    async def think(self, seconds: float = 1.0):
        """Simulate thinking/processing time."""
        await asyncio.sleep(seconds)

    def stop(self):
        self._stop_event.set()

    @property
    def stopped(self) -> bool:
        return self._stop_event.is_set()

    @abstractmethod
    async def run(self):
        """Main agent loop."""
        pass

    async def safe_run(self):
        """Wrapper that catches and logs errors."""
        try:
            await self.run()
        except asyncio.CancelledError:
            pass
        except Exception as e:
            await self.set_status("blocked", f"Error: {str(e)[:50]}")
            await self.emit_event(f"Error: {e}", "error")
