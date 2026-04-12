"""
Global Master Orchestrator (GMO)
- Arrives first at HQ
- Activates ETMO second
- Supervises all platform operations
- Narrates the arrival sequence for the Talking Table
"""
import asyncio
import time
from typing import Callable, Awaitable

from agents.base_agent import BaseAgent
from database.db import SessionFactory
from database.models import AgentRecord
from sqlalchemy import select


class GlobalMasterOrchestrator(BaseAgent):
    """
    The GMO supervises all platform operations.
    In v1, only Etsy is active.
    """

    def __init__(self, broadcast: Callable[[dict], Awaitable[None]], config: dict):
        super().__init__("GMO", "Global Master Orchestrator", "global", broadcast, config)
        self.platform_masters: dict = {}

    async def narrate(self, message: str):
        """Broadcast a talking table narration message."""
        await self.broadcast({
            "type": "talking_table",
            "data": {
                "message": message,
                "timestamp": int(time.time() * 1000),
            },
        })

    async def run(self):
        # ── Resolve stable ID from DB BEFORE any broadcasts ──────────────
        async with SessionFactory() as session:
            result = await session.execute(
                select(AgentRecord).where(
                    AgentRecord.label == "GMO",
                    AgentRecord.platform == "global",
                )
            )
            existing = result.scalar_one_or_none()
            if existing:
                self.id = existing.id
                self.created_at = existing.created_at
            else:
                record = AgentRecord(
                    id=self.id,
                    label="GMO",
                    role="Global Master Orchestrator",
                    status="idle",
                    platform="global",
                    created_at=self.created_at,
                    task_count=0,
                )
                session.add(record)
                await session.commit()

        # ── Arrival sequence (all broadcasts now use stable ID) ───────────
        await self.set_status("entering", "Arriving at HQ")
        await self.narrate("GMO is arriving at the AI Commerce HQ building...")
        await asyncio.sleep(3)

        await self.set_status("working", "System initialization")
        await self.narrate("GMO is online. Initializing all systems.")
        await self.emit_event("Global Master Orchestrator online", "system")

        await asyncio.sleep(2)
        await self.narrate("GMO is reviewing platform status and preparing the team...")
        await asyncio.sleep(3)
        await self.set_status("working", "Activating Etsy operations")
        await self.narrate("GMO is assigning ETMO to open the Etsy Operations Center...")

        # ── Launch ETMO ───────────────────────────────────────────────────
        from orchestrator.etsy_master import EtsyMasterOrchestrator
        etmo = EtsyMasterOrchestrator(broadcast=self.broadcast, config=self.config)
        etmo.desk_id = "desk-etmo-main"
        self.platform_masters["etsy"] = etmo

        # Persist ETMO (check DB first)
        async with SessionFactory() as session:
            result = await session.execute(
                select(AgentRecord).where(
                    AgentRecord.label == "ETMO",
                    AgentRecord.platform == "etsy",
                )
            )
            existing = result.scalar_one_or_none()
            if existing:
                etmo.id = existing.id
                etmo.created_at = existing.created_at
            else:
                record = AgentRecord(
                    id=etmo.id,
                    label="ETMO",
                    role="Etsy Master Orchestrator",
                    status="idle",
                    platform="etsy",
                    desk_id="desk-etmo-main",
                    created_at=etmo.created_at,
                    task_count=0,
                )
                session.add(record)
                await session.commit()

        await self.broadcast({"type": "agent_update", "data": etmo.to_dict()})
        await self.emit_event("Etsy Master Orchestrator activated", "agent_created")
        await self.narrate("ETMO is arriving at the Etsy Operations Center...")

        await asyncio.sleep(2)
        await self.set_status("idle", "Supervising operations")
        await self.narrate("GMO is at the HQ command desk. Office is open.")

        # Run ETMO in background
        etmo_task = asyncio.create_task(etmo.safe_run())

        # ── GMO supervision loop ──────────────────────────────────────────
        while not self.stopped:
            await asyncio.sleep(60)
            active_count = sum(
                1 for m in self.platform_masters.values()
                if m.status not in ("blocked", "idle")
            )
            if active_count > 0:
                await self.set_status("working", f"Supervising {active_count} active platform(s)")
                await asyncio.sleep(5)
                await self.set_status("idle", "Monitoring all platforms")

            for name, master in self.platform_masters.items():
                if master.status == "blocked":
                    await self.emit_event(f"Platform {name} is blocked — monitoring", "error")
                    await self.narrate(f"GMO is monitoring a blocked state in {name} operations...")

        etmo_task.cancel()
