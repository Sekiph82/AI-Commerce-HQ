"""
TikTok Shop Orchestrator (TTO)

Automated TikTok content and shop automation.

AI MODEL: Gemini 2.0 Flash (primary) → GPT-4o-mini (fallback)

Generates TikTok video concepts, hooks, and product showcases.
Users upload content to TikTok manually or via TikTok API.

Pipeline:
1. TRS — TikTok Research (find trending sounds, hashtags, topics)
2. TSC — Content Script (write viral hooks and scripts)
3. TPU — Product Showcase Planner (plan product-in-video integration)
"""

import asyncio
import json
import time
import uuid
from typing import Callable, Awaitable, Optional
from sqlalchemy import select

from agents.base_agent import BaseAgent
from tools.ai_router import ai_complete
from database.db import SessionFactory
from database.models import AgentRecord


TIKTOK_TEAM = [
    {"label": "TTO", "role": "TikTok Shop Orchestrator", "desk_id": "desk-tto-main"},
    {"label": "TRS", "role": "TikTok Research Agent", "desk_id": "desk-tiktok-trs"},
    {
        "label": "TSC",
        "role": "TikTok Content Script Agent",
        "desk_id": "desk-tiktok-tsc",
    },
    {"label": "TPU", "role": "TikTok Product Planner", "desk_id": "desk-tiktok-tpu"},
]


class FixedTikTokAgent(BaseAgent):
    def __init__(
        self,
        id: str,
        label: str,
        role: str,
        desk_id: str,
        broadcast: Callable,
        config: dict,
    ):
        super().__init__(label, role, "tiktok", broadcast, config)
        self.id = id
        self.desk_id = desk_id

    async def run(self):
        await self.set_status("idle")
        while not self.stopped:
            await asyncio.sleep(10)


class TikTokMasterOrchestrator(BaseAgent):
    """
    Automated TikTok content pipeline.
    Generates viral hooks, scripts, and product showcase plans.
    """

    PIPELINE_INTERVAL = 1200  # 20 minutes

    CONTENT_TYPES = [
        "Product Showcase",
        "Unboxing",
        "Before/After",
        "Tutorial",
        "Storytime",
        "POV",
        "Day in Life",
        "Hot Take",
        "Listicle",
        "Trend Recap",
        "Behind the Scenes",
        "Customer Reaction",
    ]

    def __init__(self, broadcast: Callable[[dict], Awaitable[None]], config: dict):
        super().__init__("TTO", "TikTok Shop Orchestrator", "tiktok", broadcast, config)
        self.team: dict[str, FixedTikTokAgent] = {}

    async def narrate(self, message: str):
        await self.broadcast(
            {
                "type": "talking_table",
                "data": {"message": message, "timestamp": int(time.time() * 1000)},
            }
        )

    async def run(self):
        await self.set_status("working", "Initializing TikTok operations")
        await self.narrate("TTO is arriving at the Viral Engine...")
        await asyncio.sleep(3)

        await self.emit_event("TikTok Shop Orchestrator online")
        await self.narrate("TTO is calling the content team...")
        await asyncio.sleep(2)

        await self._init_team()

        await self.set_status("idle", "Team ready")
        await self.narrate("Viral Engine is operational. Content pipeline active.")

        loop_count = 0
        while not self.stopped:
            loop_count += 1
            try:
                await self.set_status("working", f"TikTok cycle {loop_count}")
                await self.narrate(
                    f"TTO is starting TikTok content cycle {loop_count}..."
                )
                await self._run_pipeline_cycle(loop_count)
                await self.set_status("idle", "Awaiting next cycle")
            except asyncio.CancelledError:
                break
            except Exception as e:
                await self.set_status("blocked", f"Error: {str(e)[:40]}")
                await self.emit_event(f"TikTok error: {e}", "error")
                await asyncio.sleep(30)

            for _ in range(self.PIPELINE_INTERVAL):
                if self.stopped:
                    break
                await asyncio.sleep(1)

    async def _init_team(self):
        for defn in TIKTOK_TEAM:
            label, role, desk_id = defn["label"], defn["role"], defn["desk_id"]

            async with SessionFactory() as session:
                result = await session.execute(
                    select(AgentRecord).where(
                        AgentRecord.label == label,
                        AgentRecord.platform == "tiktok",
                    )
                )
                existing = result.scalar_one_or_none()

            if existing:
                agent = FixedTikTokAgent(
                    id=existing.id,
                    label=label,
                    role=role,
                    desk_id=desk_id,
                    broadcast=self.broadcast,
                    config=self.config,
                )
                agent.created_at = existing.created_at
            else:
                agent = FixedTikTokAgent(
                    id=str(uuid.uuid4()),
                    label=label,
                    role=role,
                    desk_id=desk_id,
                    broadcast=self.broadcast,
                    config=self.config,
                )
                async with SessionFactory() as session:
                    session.add(
                        AgentRecord(
                            id=agent.id,
                            label=label,
                            role=role,
                            status="idle",
                            platform="tiktok",
                            desk_id=desk_id,
                            created_at=agent.created_at,
                            task_count=0,
                        )
                    )
                    await session.commit()
                await self.emit_event(f"TikTok agent joined: {role}", "agent_created")

            self.team[label] = agent
            await self.broadcast({"type": "agent_update", "data": agent.to_dict()})
            await asyncio.sleep(1.5)
            asyncio.create_task(agent.safe_run())

    async def _run_pipeline_cycle(self, cycle: int):
        content = await self._step_research()
        if not content:
            return

        content = await self._step_script(content)
        content = await self._step_product_planner(content)

        await self._save_content(content)
        await self.narrate(f"TikTok content ready: {content['title']}")
        await self.emit_event(
            f"TIKTOK CONTENT READY: {content['title']}", "product_update"
        )

    async def _step_research(self) -> Optional[dict]:
        await self.narrate("TRS is researching trending TikTok topics...")
        trs = self.team.get("TRS")
        if trs:
            await trs.set_status("working", "Researching TikTok trends")
        await asyncio.sleep(5)

        content_type = self.CONTENT_TYPES[cycle % len(self.CONTENT_TYPES)]

        response = await ai_complete(
            config=self.config,
            prompt=(
                f"You are a TikTok trend researcher. Find a viral-worthy content topic "
                f"for TikTok Shop in the content format: {content_type}. "
                "Consider: trending sounds, hashtags, engagement hooks, product tie-ins. "
                "Return JSON: {topic, content_type, hook_angle: string, "
                "target_hashtags: [], trending_sound_idea: string, "
                "viral_potential: high/medium/low, target_audience: string}"
            ),
            system=(
                "You are TRS — TikTok Research Agent. "
                "You discover viral-worthy content trends for TikTok. "
                "Think about what gets shared, duetted, and stitched."
            ),
            json_mode=True,
            max_tokens=500,
        )

        try:
            data = json.loads(response)
        except Exception:
            data = {
                "topic": f"Amazing {content_type} that will blow your mind",
                "content_type": content_type,
                "hook_angle": "Wait for the ending...",
                "target_hashtags": ["#fyp", "#viral", "#foryou", "#trending"],
                "trending_sound_idea": "Upbeat lo-fi with build-up drop",
                "viral_potential": "high",
                "target_audience": "Gen Z shoppers, 18-34",
            }

        if trs:
            await trs.set_status("complete", "Research complete")
            await asyncio.sleep(2)
            await trs.set_status("idle")

        return {
            "id": str(uuid.uuid4()),
            "title": data.get("topic", "Viral TikTok Content"),
            "contentType": data.get("content_type", content_type),
            "hookAngle": data.get("hook_angle", ""),
            "hashtags": data.get("target_hashtags", []),
            "soundIdea": data.get("trending_sound_idea", ""),
            "viralPotential": data.get("viral_potential", "medium"),
            "targetAudience": data.get("target_audience", ""),
            "platform": "tiktok",
            "state": "IDEA_DISCOVERED",
            "script": "",
            "productPlan": "",
            "estimatedReach": 0,
            "createdAt": int(time.time() * 1000),
        }

    async def _step_script(self, content: dict) -> dict:
        await self.narrate("TSC is writing a viral TikTok script...")
        tsc = self.team.get("TSC")
        if tsc:
            await tsc.set_status("working", "Writing TikTok script")
        await asyncio.sleep(8)

        response = await ai_complete(
            config=self.config,
            prompt=(
                f"Write a viral TikTok script for: '{content['title']}'. "
                f"Content type: {content['contentType']}. "
                f"Hook: {content.get('hookAngle', '')}. "
                "Format for 15-60 second video. "
                "Structure: HOOK (first 3 seconds) → BODY (middle) → CTA (last 5 seconds). "
                "Return JSON: {hook_text, hook_action: string, body_scenes: [], "
                "cta_text: string, cta_action: string, estimated_engagement_rate: string}"
            ),
            system=(
                "You are TSC — TikTok Content Script Agent. "
                "You write addictive, viral TikTok scripts. "
                "The first 3 seconds MUST hook or the viewer scrolls past. "
                "Use pattern interrupts, curiosity gaps, and strong CTAs."
            ),
            json_mode=True,
            max_tokens=600,
        )

        try:
            data = json.loads(response)
        except Exception:
            data = {
                "hook_text": f"Wait... {content.get('hookAngle', 'this is incredible')}",
                "hook_action": "Eye contact, fast zoom",
                "body_scenes": [
                    {
                        "timestamp": "3-15s",
                        "action": "Main reveal",
                        "text_overlay": "Wow!",
                    },
                    {
                        "timestamp": "15-40s",
                        "action": "Deep dive",
                        "text_overlay": "Here's why...",
                    },
                ],
                "cta_text": "Follow for more!",
                "cta_action": "Follow button tap",
                "estimated_engagement_rate": "8-15%",
            }

        content["script"] = data
        content["state"] = "DESIGN_GENERATED"

        if tsc:
            await tsc.set_status("complete", "Script complete")
            await asyncio.sleep(2)
            await tsc.set_status("idle")

        return content

    async def _step_product_planner(self, content: dict) -> dict:
        await self.narrate("TPU is planning the product showcase integration...")
        tpu = self.team.get("TPU")
        if tpu:
            await tpu.set_status("working", "Planning product showcase")
        await asyncio.sleep(6)

        response = await ai_complete(
            config=self.config,
            prompt=(
                f"Create a product showcase plan for TikTok content: '{content['title']}'. "
                "Include: product to feature, placement in video, "
                "shoppable link strategy, estimated reach and conversion. "
                "Return JSON: {product_name: string, placement_timing: string, "
                "shoppable_strategy: string, estimated_reach: number, "
                "conversion_rate_estimate: string}"
            ),
            system=(
                "You are TPU — TikTok Product Planner. "
                "You integrate products naturally into viral TikTok content. "
                "Think about authentic placement, not pushy sales."
            ),
            json_mode=True,
            max_tokens=400,
        )

        try:
            data = json.loads(response)
        except Exception:
            data = {
                "product_name": "Featured Product",
                "placement_timing": "15-20 second mark (natural integration)",
                "shoppable_strategy": "Product link in bio + mention in comments",
                "estimated_reach": 50000,
                "conversion_rate_estimate": "1-3%",
            }

        content["productPlan"] = data
        content["estimatedReach"] = data.get("estimated_reach", 50000)
        content["state"] = "AWAITING_HUMAN_DECISION"

        if tpu:
            await tpu.set_status("complete", "Plan complete")
            await asyncio.sleep(2)
            await tpu.set_status("idle")

        return content

    async def _save_content(self, content: dict):
        content["updatedAt"] = int(time.time() * 1000)
        async with SessionFactory() as session:
            result = await session.execute(
                select(ProductRecord).where(ProductRecord.id == content["id"])
            )
            record = result.scalar_one_or_none()
            if record:
                record.state = content.get("state", "AWAITING_HUMAN_DECISION")
                record.updated_at = time.time()
            else:
                session.add(
                    ProductRecord(
                        id=content["id"],
                        name=content.get("title", ""),
                        platform="tiktok",
                        state=content.get("state", "AWAITING_HUMAN_DECISION"),
                        niche=content.get("contentType", ""),
                        created_at=content.get("createdAt", time.time() * 1000) / 1000,
                        updated_at=time.time(),
                    )
                )
            await session.commit()
        await self.broadcast({"type": "product_update", "data": content})
