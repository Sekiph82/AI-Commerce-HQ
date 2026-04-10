"""
Global Master Orchestrator (GMO)
- Top-level system controller
- Spawns platform master agents
- Supervises all operations
"""
import asyncio
import time
from typing import Callable, Awaitable

from agents.base_agent import BaseAgent
from database.db import SessionFactory
from database.models import AgentRecord


class GlobalMasterOrchestrator(BaseAgent):
    """
    The GMO supervises all platform operations.
    In v1, only Etsy is active.
    """

    def __init__(self, broadcast: Callable[[dict], Awaitable[None]], config: dict):
        super().__init__("GMO", "Global Master Orchestrator", "global", broadcast, config)
        self.platform_masters: dict = {}

    async def run(self):
        await self.set_status("working", "System initialization")
        await self.emit_event("Global Master Orchestrator online", "system")

        # Persist GMO agent to DB
        async with SessionFactory() as session:
            record = AgentRecord(
                id=self.id,
                label="GMO",
                role="Global Master Orchestrator",
                status="working",
                platform="global",
                created_at=self.created_at,
                task_count=0,
            )
            session.add(record)
            await session.commit()

        await asyncio.sleep(2)
        await self.emit_event("Initializing platform operations")
        await self.set_status("working", "Activating Etsy operations")

        # Launch Etsy Master Orchestrator
        from orchestrator.etsy_master import EtsyMasterOrchestrator
        etmo = EtsyMasterOrchestrator(broadcast=self.broadcast, config=self.config)
        etmo.desk_id = "desk-etmo-main"
        self.platform_masters["etsy"] = etmo

        # Persist ETMO
        async with SessionFactory() as session:
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

        # Broadcast ETMO agent
        await self.broadcast({"type": "agent_update", "data": etmo.to_dict()})
        await self.emit_event("Etsy Master Orchestrator activated", "agent_created")

        await asyncio.sleep(1)
        await self.set_status("idle", "Supervising all operations")

        # Run ETMO in background
        etmo_task = asyncio.create_task(etmo.safe_run())

        # GMO supervision loop
        while not self.stopped:
            await asyncio.sleep(30)
            # Check on platform masters
            active_count = sum(1 for m in self.platform_masters.values() if m.status != "blocked")
            await self.set_status("idle", f"Supervising {active_count} platform(s)")

            # Restart blocked masters
            for name, master in self.platform_masters.items():
                if master.status == "blocked":
                    await self.emit_event(f"Platform {name} is blocked — monitoring", "error")

        etmo_task.cancel()
