"""
YouTube Master Orchestrator (YMO)

Automated faceless YouTube content pipeline.

AI MODEL: Gemini 2.0 Flash (primary) → GPT-4o-mini (fallback)

Generates faceless YouTube video concepts, scripts, and content plans.
Users upload content to YouTube manually or via API.

Pipeline:
1. YRS — YouTube Research (find trending topics, niches)
2. YSC — Script Writer (write complete faceless video scripts)
3. YED — Video Editor Plan (create editing instructions)
4. YTH — Thumbnail Generator (AI image prompt for thumbnails)
"""

import asyncio
import json
import time
import uuid
from typing import Callable, Awaitable, Optional
from sqlalchemy import select

from agents.base_agent import BaseAgent
from tools.ai_router import ai_complete, ai_image
from database.db import SessionFactory
from database.models import AgentRecord


YOUTUBE_TEAM = [
    {
        "label": "YMO",
        "role": "YouTube Master Orchestrator",
        "desk_id": "desk-ytmo-main",
    },
    {"label": "YRS", "role": "YouTube Research Agent", "desk_id": "desk-youtube-yrs"},
    {"label": "YSC", "role": "YouTube Script Writer", "desk_id": "desk-youtube-ysc"},
    {"label": "YED", "role": "Video Editor Plan Agent", "desk_id": "desk-youtube-yed"},
    {
        "label": "YTH",
        "role": "Thumbnail Generator Agent",
        "desk_id": "desk-youtube-yth",
    },
]


class FixedYouTubeAgent(BaseAgent):
    def __init__(
        self,
        id: str,
        label: str,
        role: str,
        desk_id: str,
        broadcast: Callable,
        config: dict,
    ):
        super().__init__(label, role, "youtube", broadcast, config)
        self.id = id
        self.desk_id = desk_id

    async def run(self):
        await self.set_status("idle")
        while not self.stopped:
            await asyncio.sleep(10)


class YouTubeMasterOrchestrator(BaseAgent):
    """
    Automated faceless YouTube content pipeline.
    Generates video concepts, scripts, editing plans, and thumbnails.
    Does NOT upload to YouTube — creates complete content packages.
    """

    PIPELINE_INTERVAL = 2400  # 40 minutes between cycles

    NICHES = [
        "AI Tools & Tutorials",
        "History Facts",
        "Mind-Blowing Facts",
        "Space & Astronomy",
        "Technology Explained",
        "True Crime Stories",
        "DIY & Life Hacks",
        "Mystery & Unexplained",
        "Science Experiments",
        "Top 10 Lists",
        "Urban Legends",
        "Health & Wellness Tips",
    ]

    def __init__(self, broadcast: Callable[[dict], Awaitable[None]], config: dict):
        super().__init__(
            "YMO", "YouTube Master Orchestrator", "youtube", broadcast, config
        )
        self.team: dict[str, FixedYouTubeAgent] = {}
        self.video_count = 0

    async def narrate(self, message: str):
        await self.broadcast(
            {
                "type": "talking_table",
                "data": {"message": message, "timestamp": int(time.time() * 1000)},
            }
        )

    async def run(self):
        await self.set_status("working", "Initializing YouTube operations")
        await self.narrate("YMO is arriving at the Content Factory...")
        await asyncio.sleep(3)

        await self.emit_event("YouTube Master Orchestrator online")
        await self.narrate("YMO is calling in the video production team...")
        await asyncio.sleep(2)

        await self._init_team()

        await self.set_status("idle", "Team ready")
        await self.narrate("Content Factory is operational. Video pipeline active.")

        loop_count = 0
        while not self.stopped:
            loop_count += 1
            try:
                await self.set_status("working", f"Video cycle {loop_count}")
                await self.narrate(
                    f"YMO is starting YouTube video production cycle {loop_count}..."
                )
                await self._run_pipeline_cycle(loop_count)
                await self.set_status("idle", "Awaiting next cycle")
            except asyncio.CancelledError:
                break
            except Exception as e:
                await self.set_status("blocked", f"Error: {str(e)[:40]}")
                await self.emit_event(f"YouTube error: {e}", "error")
                await asyncio.sleep(30)

            for _ in range(self.PIPELINE_INTERVAL):
                if self.stopped:
                    break
                await asyncio.sleep(1)

    async def _init_team(self):
        for defn in YOUTUBE_TEAM:
            label, role, desk_id = defn["label"], defn["role"], defn["desk_id"]

            async with SessionFactory() as session:
                result = await session.execute(
                    select(AgentRecord).where(
                        AgentRecord.label == label,
                        AgentRecord.platform == "youtube",
                    )
                )
                existing = result.scalar_one_or_none()

            if existing:
                agent = FixedYouTubeAgent(
                    id=existing.id,
                    label=label,
                    role=role,
                    desk_id=desk_id,
                    broadcast=self.broadcast,
                    config=self.config,
                )
                agent.created_at = existing.created_at
            else:
                agent = FixedYouTubeAgent(
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
                            platform="youtube",
                            desk_id=desk_id,
                            created_at=agent.created_at,
                            task_count=0,
                        )
                    )
                    await session.commit()
                await self.emit_event(f"YouTube agent joined: {role}", "agent_created")

            self.team[label] = agent
            await self.broadcast({"type": "agent_update", "data": agent.to_dict()})
            await asyncio.sleep(1.5)
            asyncio.create_task(agent.safe_run())

    async def _run_pipeline_cycle(self, cycle: int):
        self.video_count += 1
        video = await self._step_research()
        if not video:
            return

        video = await self._step_script(video)
        video = await self._step_editor_plan(video)
        video = await self._step_thumbnail(video)

        await self._save_video(video)
        await self.narrate(f"YouTube video package complete: {video['title']}")
        await self.emit_event(
            f"YOUTUBE VIDEO READY: {video['title']}", "product_update"
        )

    async def _step_research(self) -> Optional[dict]:
        await self.narrate("YRS is researching trending YouTube topics...")
        yrs = self.team.get("YRS")
        if yrs:
            await yrs.set_status("working", "Researching YouTube trends")
        await asyncio.sleep(6)

        niche = self.NICHES[self.video_count % len(self.NICHES)]

        response = await ai_complete(
            config=self.config,
            prompt=(
                f"You are a YouTube content researcher. Find a high-performing video topic "
                f"in the niche: {niche}. Consider: search volume, competition level, "
                "audience engagement potential, and trending interest. "
                "Return JSON: {title, niche, hook_angle: string, target_keywords: [], "
                "estimated_views: string, competition: low/medium/high, "
                "video_length_recommendation: string, ctr_hook: string}"
            ),
            system=(
                "You are YRS — YouTube Research Agent. "
                "You discover viral-worthy content topics for faceless YouTube channels. "
                "Focus on evergreen and trending topics with high search intent."
            ),
            json_mode=True,
            max_tokens=500,
        )

        try:
            data = json.loads(response)
        except Exception:
            data = {
                "title": f"10 Mind-Blowing Facts About {niche} You Didn't Know",
                "niche": niche,
                "hook_angle": "Amazing secrets revealed",
                "target_keywords": [niche.lower(), "facts", "amazing", "secrets"],
                "estimated_views": "50K-500K",
                "competition": "medium",
                "video_length_recommendation": "8-12 minutes",
                "ctr_hook": "What if I told you...",
            }

        if yrs:
            await yrs.set_status("complete", "Research complete")
            await asyncio.sleep(2)
            await yrs.set_status("idle")

        return {
            "id": str(uuid.uuid4()),
            "title": data.get("title", "Amazing Video"),
            "niche": data.get("niche", niche),
            "hookAngle": data.get("hook_angle", ""),
            "hook": data.get("ctr_hook", ""),
            "targetKeywords": data.get("target_keywords", []),
            "estimatedViews": data.get("estimated_views", ""),
            "competition": data.get("competition", "medium"),
            "videoLength": data.get("video_length_recommendation", "10 minutes"),
            "platform": "youtube",
            "state": "IDEA_DISCOVERED",
            "script": "",
            "editorPlan": "",
            "thumbnailPrompt": "",
            "thumbnailUrl": "",
            "estimatedEarnings": 0,
            "createdAt": int(time.time() * 1000),
        }

    async def _step_script(self, video: dict) -> dict:
        await self.narrate("YSC is writing a complete faceless video script...")
        ysc = self.team.get("YSC")
        if ysc:
            await ysc.set_status("working", "Writing video script")

        await asyncio.sleep(12)

        response = await ai_complete(
            config=self.config,
            prompt=(
                f"Write a complete faceless YouTube video script for: '{video['title']}'. "
                f"Niche: {video['niche']}. Hook angle: {video.get('hookAngle', '')}. "
                "Target length: {video['videoLength']}. "
                "Format: [HOOK_OPENING] [SECTION_1 with narration script] "
                "[SECTION_2 with narration script] [SECTION_3 with narration script] "
                "[SECTION_4 with narration script] [CTA_CLOSING]. "
                "Each section should be 60-120 words of actual narration. "
                "Include B-ROLL suggestions for each section in parentheses. "
                "Add caption/SUBTITLE cues in brackets. "
                "Total script should fill the target video length. "
                "Return JSON: {script_sections: [{section_title, narration, broll_suggestion}], "
                "total_duration_minutes: number, narrator_style: string}"
            ),
            system=(
                "You are YSC — YouTube Script Writer. "
                "You write engaging, fast-paced scripts for faceless YouTube videos. "
                "Use short sentences, high energy, and curiosity gaps. "
                "Write like a native speaker with excellent pacing."
            ),
            json_mode=True,
            max_tokens=2000,
        )

        try:
            data = json.loads(response)
        except Exception:
            data = {
                "script_sections": [
                    {
                        "section_title": "The Hook",
                        "narration": f" What if I told you that everything you know about {video['niche']} is completely wrong? Stick around because I'm about to reveal things that will blow your mind.",
                        "broll_suggestion": "Dramatic close-up, fast cuts",
                    },
                    {
                        "section_title": "The Revelation",
                        "narration": f" Most people have no idea about the hidden secrets in {video['niche']}. Here's what the experts don't want you to know.",
                        "broll_suggestion": "Animated graphics, stock footage",
                    },
                    {
                        "section_title": "Deep Dive",
                        "narration": f" Let's break down exactly how this works, step by step. Pay attention because this is where most people get it wrong.",
                        "broll_suggestion": "Screen recordings, diagrams",
                    },
                    {
                        "section_title": "The Mind-Blowing Facts",
                        "narration": f" And now for the part you've been waiting for — the facts that will change everything you thought you knew.",
                        "broll_suggestion": "Dynamic visuals, fast transitions",
                    },
                    {
                        "section_title": "Call to Action",
                        "narration": " If this video opened your eyes, smash that like button and subscribe. Hit the bell so you never miss an amazing fact.",
                        "broll_suggestion": "Channel trailer clips",
                    },
                ],
                "total_duration_minutes": 10,
                "narrator_style": "Energetic, conversational, curious",
            }

        script_text = "\n\n".join(
            f"[{s['section_title']}]\n{s['narration']}\n(B-Roll: {s['broll_suggestion']})"
            for s in data.get("script_sections", [])
        )

        video["script"] = script_text
        video["scriptSections"] = data.get("script_sections", [])
        video["videoLength"] = f"{data.get('total_duration_minutes', 10)} minutes"
        video["narratorStyle"] = data.get("narrator_style", "energetic")
        video["state"] = "DESIGN_GENERATED"

        if ysc:
            await ysc.set_status("complete", "Script complete")
            await asyncio.sleep(2)
            await ysc.set_status("idle")

        return video

    async def _step_editor_plan(self, video: dict) -> dict:
        await self.narrate("YED is creating the video editing instructions...")
        yed = self.team.get("YED")
        if yed:
            await yed.set_status("working", "Creating editing plan")
        await asyncio.sleep(6)

        response = await ai_complete(
            config=self.config,
            prompt=(
                f"Create a detailed video editing plan for: '{video['title']}'. "
                f"Video length: {video['videoLength']}. "
                f"Script sections: {len(video.get('scriptSections', []))}. "
                "Include: intro sequence, transition styles, text overlay instructions, "
                "background music suggestions, pacing/tempo, color grading notes, "
                "subtitle style, thumbnail timing. "
                "Return JSON: {intro_notes: string, transition_style: string, "
                "music_style: string, color_grade: string, subtitle_style: string, "
                "pacing_notes: string, recommended_tools: []}"
            ),
            system=(
                "You are YED — Video Editor Plan Agent. "
                "You create detailed editing blueprints for faceless YouTube videos. "
                "Think about pacing, visual interest, and viewer retention."
            ),
            json_mode=True,
            max_tokens=500,
        )

        try:
            data = json.loads(response)
        except Exception:
            data = {
                "intro_notes": "3-second logo reveal + dramatic title animation",
                "transition_style": "Quick whip pans and match cuts between sections",
                "music_style": "Upbeat lo-fi or cinematic ambient",
                "color_grade": "Vibrant, high contrast, slightly warm tones",
                "subtitle_style": "Large white text with black outline, centered",
                "pacing_notes": "Fast cuts in sections 1-2, slower in section 3, fast in 4",
                "recommended_tools": ["CapCut", "DaVinci Resolve", "Premiere Pro"],
            }

        video["editorPlan"] = data

        if yed:
            await yed.set_status("complete", "Editing plan ready")
            await asyncio.sleep(2)
            await yed.set_status("idle")

        return video

    async def _step_thumbnail(self, video: dict) -> dict:
        await self.narrate("YTH is generating a click-worthy thumbnail...")
        yth = self.team.get("YTH")
        if yth:
            await yth.set_status("working", "Creating thumbnail concept")

        await asyncio.sleep(5)

        response = await ai_complete(
            config=self.config,
            prompt=(
                f"Create a detailed thumbnail generation prompt for: '{video['title']}'. "
                f"Niche: {video['niche']}. "
                "The thumbnail must be eye-catching, click-worthy, and suitable for YouTube. "
                "Include: main subject, expression, text overlay idea, background, color scheme. "
                "Return JSON: {image_prompt: string, text_overlay: string, color_scheme: [], "
                "composition_notes: string}"
            ),
            system=(
                "You are YTH — Thumbnail Generator Agent. "
                "You create thumbnail concepts that maximize CTR. "
                "Use contrast, faces with expressions, and readable text."
            ),
            json_mode=True,
            max_tokens=400,
        )

        try:
            data = json.loads(response)
        except Exception:
            data = {
                "image_prompt": f"Dramatic wide shot of {video['niche']}, bright colors, cinematic lighting, hyperrealistic, 4K",
                "text_overlay": f"MIND = BLOWN",
                "color_scheme": ["#FF4444", "#FFD700", "#000000"],
                "composition_notes": "Centered subject, bold text in upper third",
            }

        video["thumbnailPrompt"] = data.get("image_prompt", "")
        video["thumbnailText"] = data.get("text_overlay", "")
        video["thumbnailColors"] = data.get("color_scheme", [])
        video["state"] = "AWAITING_HUMAN_DECISION"

        thumbnail_url = await ai_image(
            config=self.config, prompt=data.get("image_prompt", "")
        )
        if thumbnail_url:
            video["thumbnailUrl"] = thumbnail_url

        estimated_views = video.get("estimatedViews", "50K-500K")
        estimated_cpm = 2.5
        video["estimatedEarnings"] = 50 * estimated_cpm / 1000

        if yth:
            await yth.set_status("complete", "Thumbnail ready")
            await asyncio.sleep(2)
            await yth.set_status("idle")

        return video

    async def _save_video(self, video: dict):
        video["updatedAt"] = int(time.time() * 1000)
        async with SessionFactory() as session:
            result = await session.execute(
                select(ProductRecord).where(ProductRecord.id == video["id"])
            )
            record = result.scalar_one_or_none()
            if record:
                record.state = video.get("state", "AWAITING_HUMAN_DECISION")
                record.updated_at = time.time()
            else:
                session.add(
                    ProductRecord(
                        id=video["id"],
                        name=video.get("title", ""),
                        platform="youtube",
                        state=video.get("state", "AWAITING_HUMAN_DECISION"),
                        niche=video.get("niche", ""),
                        created_at=video.get("createdAt", time.time() * 1000) / 1000,
                        updated_at=time.time(),
                    )
                )
            await session.commit()
        await self.broadcast({"type": "product_update", "data": video})
