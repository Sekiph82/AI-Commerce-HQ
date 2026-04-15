"""
Global Master Orchestrator (GMO)
- Arrives first at HQ
- Activates ALL platform orchestrators
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
    Activates: Etsy, Fiverr, YouTube, Trading
    """

    def __init__(self, broadcast: Callable[[dict], Awaitable[None]], config: dict):
        super().__init__(
            "GMO", "Global Master Orchestrator", "global", broadcast, config
        )
        self.platform_masters: dict = {}

    async def narrate(self, message: str):
        """Broadcast a talking table narration message."""
        await self.broadcast(
            {
                "type": "talking_table",
                "data": {
                    "message": message,
                    "timestamp": int(time.time() * 1000),
                },
            }
        )

    async def run(self):
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

        await self.set_status("entering", "Arriving at HQ")
        await self.narrate("GMO is arriving at the AI Commerce HQ building...")
        await asyncio.sleep(3)

        await self.set_status("working", "System initialization")
        await self.narrate("GMO is online. Initializing all systems.")
        await self.emit_event("Global Master Orchestrator online", "system")

        await asyncio.sleep(2)
        await self.narrate("GMO is reviewing platform status and preparing the team...")
        await asyncio.sleep(3)

        platform_tasks = []

        await self.set_status("working", "Activating Etsy operations")
        await self.narrate(
            "GMO is assigning ETMO to open the Etsy Operations Center..."
        )

        from orchestrator.etsy_master import EtsyMasterOrchestrator

        etmo = EtsyMasterOrchestrator(broadcast=self.broadcast, config=self.config)
        etmo.desk_id = "desk-etmo-main"
        self.platform_masters["etsy"] = etmo

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
        platform_tasks.append(asyncio.create_task(etmo.safe_run()))

        await asyncio.sleep(2)

        await self.set_status("working", "Activating Fiverr operations")
        await self.narrate("GMO is assigning FMO to open the Fiverr Station...")

        from orchestrator.fiverr_master import FiverrMasterOrchestrator

        fmo = FiverrMasterOrchestrator(broadcast=self.broadcast, config=self.config)
        fmo.desk_id = "desk-fmo-main"
        self.platform_masters["fiverr"] = fmo

        async with SessionFactory() as session:
            result = await session.execute(
                select(AgentRecord).where(
                    AgentRecord.label == "FMO",
                    AgentRecord.platform == "fiverr",
                )
            )
            existing = result.scalar_one_or_none()
            if existing:
                fmo.id = existing.id
                fmo.created_at = existing.created_at
            else:
                record = AgentRecord(
                    id=fmo.id,
                    label="FMO",
                    role="Fiverr Master Orchestrator",
                    status="idle",
                    platform="fiverr",
                    desk_id="desk-fmo-main",
                    created_at=fmo.created_at,
                    task_count=0,
                )
                session.add(record)
                await session.commit()

        await self.broadcast({"type": "agent_update", "data": fmo.to_dict()})
        await self.emit_event("Fiverr Master Orchestrator activated", "agent_created")
        await self.narrate("FMO is arriving at the Fiverr Station...")
        platform_tasks.append(asyncio.create_task(fmo.safe_run()))

        await asyncio.sleep(2)

        await self.set_status("working", "Activating Trading Lab")
        await self.narrate("GMO is assigning TMO to open the Trading Lab...")

        from orchestrator.trading_master import TradingMasterOrchestrator

        tmo = TradingMasterOrchestrator(broadcast=self.broadcast, config=self.config)
        tmo.desk_id = "desk-tmo-main"
        self.platform_masters["trading"] = tmo

        async with SessionFactory() as session:
            result = await session.execute(
                select(AgentRecord).where(
                    AgentRecord.label == "TMO",
                    AgentRecord.platform == "trading",
                )
            )
            existing = result.scalar_one_or_none()
            if existing:
                tmo.id = existing.id
                tmo.created_at = existing.created_at
            else:
                record = AgentRecord(
                    id=tmo.id,
                    label="TMO",
                    role="Trading Master Orchestrator",
                    status="idle",
                    platform="trading",
                    desk_id="desk-tmo-main",
                    created_at=tmo.created_at,
                    task_count=0,
                )
                session.add(record)
                await session.commit()

        await self.broadcast({"type": "agent_update", "data": tmo.to_dict()})
        await self.emit_event("Trading Master Orchestrator activated", "agent_created")
        await self.narrate("TMO is arriving at the Trading Lab...")
        platform_tasks.append(asyncio.create_task(tmo.safe_run()))

        await asyncio.sleep(2)

        await self.set_status("working", "Activating YouTube Content Factory")
        await self.narrate("GMO is assigning YMO to open the Content Factory...")

        from orchestrator.youtube_master import YouTubeMasterOrchestrator

        ymo = YouTubeMasterOrchestrator(broadcast=self.broadcast, config=self.config)
        ymo.desk_id = "desk-ytmo-main"
        self.platform_masters["youtube"] = ymo

        async with SessionFactory() as session:
            result = await session.execute(
                select(AgentRecord).where(
                    AgentRecord.label == "YMO",
                    AgentRecord.platform == "youtube",
                )
            )
            existing = result.scalar_one_or_none()
            if existing:
                ymo.id = existing.id
                ymo.created_at = existing.created_at
            else:
                record = AgentRecord(
                    id=ymo.id,
                    label="YMO",
                    role="YouTube Master Orchestrator",
                    status="idle",
                    platform="youtube",
                    desk_id="desk-ytmo-main",
                    created_at=ymo.created_at,
                    task_count=0,
                )
                session.add(record)
                await session.commit()

        await self.broadcast({"type": "agent_update", "data": ymo.to_dict()})
        await self.emit_event("YouTube Master Orchestrator activated", "agent_created")
        await self.narrate("YMO is arriving at the Content Factory...")
        platform_tasks.append(asyncio.create_task(ymo.safe_run()))

        await asyncio.sleep(2)

        await self.set_status("working", "Activating TikTok Viral Engine")
        await self.narrate("GMO is assigning TTO to open the Viral Engine...")

        from orchestrator.tiktok_master import TikTokMasterOrchestrator

        tto = TikTokMasterOrchestrator(broadcast=self.broadcast, config=self.config)
        tto.desk_id = "desk-tto-main"
        self.platform_masters["tiktok"] = tto

        async with SessionFactory() as session:
            result = await session.execute(
                select(AgentRecord).where(
                    AgentRecord.label == "TTO",
                    AgentRecord.platform == "tiktok",
                )
            )
            existing = result.scalar_one_or_none()
            if existing:
                tto.id = existing.id
                tto.created_at = existing.created_at
            else:
                record = AgentRecord(
                    id=tto.id,
                    label="TTO",
                    role="TikTok Shop Orchestrator",
                    status="idle",
                    platform="tiktok",
                    desk_id="desk-tto-main",
                    created_at=tto.created_at,
                    task_count=0,
                )
                session.add(record)
                await session.commit()

        await self.broadcast({"type": "agent_update", "data": tto.to_dict()})
        await self.emit_event("TikTok Shop Orchestrator activated", "agent_created")
        await self.narrate("TTO is arriving at the Viral Engine...")
        platform_tasks.append(asyncio.create_task(tto.safe_run()))

        await asyncio.sleep(2)
        await self.set_status("idle", "Supervising operations")
        await self.narrate(
            f"GMO is at the HQ command desk. {len(self.platform_masters)} platforms active. Office is open."
        )

        await asyncio.sleep(5)

        while not self.stopped:
            await asyncio.sleep(60)
            active_count = sum(
                1
                for m in self.platform_masters.values()
                if m.status not in ("blocked", "idle")
            )
            if active_count > 0:
                await self.set_status(
                    "working", f"Supervising {active_count} active platform(s)"
                )
                await asyncio.sleep(5)
                await self.set_status("idle", "Monitoring all platforms")

            for name, master in self.platform_masters.items():
                if master.status == "blocked":
                    await self.emit_event(
                        f"Platform {name} is blocked — monitoring", "error"
                    )
                    await self.narrate(
                        f"GMO is monitoring a blocked state in {name} operations..."
                    )

        for task in platform_tasks:
            task.cancel()
        for task in platform_tasks:
            try:
                await task
            except asyncio.CancelledError:
                pass
