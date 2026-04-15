"""
Fiverr Master Orchestrator (FMO)

Manages a fixed 6-agent team for Fiverr freelance automation.

AI MODEL: Gemini 2.0 Flash (primary) → GPT-4o-mini (fallback)

Pipeline:
1. FRD — Market Research (find in-demand Fiverr gigs)
2. FPR — Profile Optimizer (set up optimized Fiverr profile)
3. FSP — Service Pack Builder (create gig packages)
4. FAQ — FAQ & Response Generator
5. FDL — Deliverable Generator (create actual work outputs)
6. FMO — Coordination & Quality

NOTE: Fiverr doesn't have a public API for automation.
This system generates deliverables, drafts, and templates that users can
manually upload to Fiverr. It acts as a WORK GENERATOR.
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
from database.models import AgentRecord, ProductRecord


FIXED_FIVERR_TEAM = [
    {"label": "FMO", "role": "Fiverr Master Orchestrator", "desk_id": "desk-fmo-main"},
    {
        "label": "FRD",
        "role": "Fiverr Market Research Agent",
        "desk_id": "desk-fiverr-frd",
    },
    {"label": "FPR", "role": "Fiverr Profile Optimizer", "desk_id": "desk-fiverr-fpr"},
    {
        "label": "FSP",
        "role": "Fiverr Service Pack Builder",
        "desk_id": "desk-fiverr-fsp",
    },
    {"label": "FAQ", "role": "Fiverr FAQ Agent", "desk_id": "desk-fiverr-faq"},
    {
        "label": "FDL",
        "role": "Fiverr Deliverable Generator",
        "desk_id": "desk-fiverr-fdl",
    },
]


class FixedFiverrAgent(BaseAgent):
    def __init__(
        self,
        id: str,
        label: str,
        role: str,
        desk_id: str,
        broadcast: Callable,
        config: dict,
    ):
        super().__init__(label, role, "fiverr", broadcast, config)
        self.id = id
        self.desk_id = desk_id

    async def run(self):
        await self.set_status("idle")
        while not self.stopped:
            await asyncio.sleep(10)


class FiverrMasterOrchestrator(BaseAgent):
    """
    Manages Fiverr freelance automation.
    Generates deliverables, gig descriptions, and service packages
    that users can manually upload to Fiverr.
    """

    PIPELINE_INTERVAL = 1800  # 30 minutes between cycles

    def __init__(self, broadcast: Callable[[dict], Awaitable[None]], config: dict):
        super().__init__(
            "FMO", "Fiverr Master Orchestrator", "fiverr", broadcast, config
        )
        self.team: dict[str, FixedFiverrAgent] = {}

    async def narrate(self, message: str):
        await self.broadcast(
            {
                "type": "talking_table",
                "data": {"message": message, "timestamp": int(time.time() * 1000)},
            }
        )

    async def run(self):
        await self.set_status("working", "Initializing Fiverr operations")
        await self.narrate("FMO is arriving at the Fiverr Operations Center...")
        await asyncio.sleep(3)

        await self.emit_event("Fiverr Master Orchestrator online")
        await self.narrate("FMO is calling in the freelance team...")
        await asyncio.sleep(2)

        await self._init_team()

        await self.set_status("idle", "Team ready")
        await self.narrate("Fiverr team is at their desks. Operations open.")

        loop_count = 0
        while not self.stopped:
            loop_count += 1
            try:
                await self.set_status("working", f"Fiverr cycle {loop_count}")
                await self.narrate(
                    f"FMO is starting Fiverr freelance cycle {loop_count}..."
                )
                await self._run_pipeline_cycle(loop_count)
                await self.set_status("idle", "Awaiting next cycle")
            except asyncio.CancelledError:
                break
            except Exception as e:
                await self.set_status("blocked", f"Error: {str(e)[:40]}")
                await self.emit_event(f"Fiverr error: {e}", "error")
                await asyncio.sleep(30)

            for _ in range(self.PIPELINE_INTERVAL):
                if self.stopped:
                    break
                await asyncio.sleep(1)

    async def _init_team(self):
        for defn in FIXED_FIVERR_TEAM:
            label, role, desk_id = defn["label"], defn["role"], defn["desk_id"]

            async with SessionFactory() as session:
                result = await session.execute(
                    select(AgentRecord).where(
                        AgentRecord.label == label,
                        AgentRecord.platform == "fiverr",
                    )
                )
                existing = result.scalar_one_or_none()

            if existing:
                agent = FixedFiverrAgent(
                    id=existing.id,
                    label=label,
                    role=role,
                    desk_id=desk_id,
                    broadcast=self.broadcast,
                    config=self.config,
                )
                agent.created_at = existing.created_at
            else:
                agent = FixedFiverrAgent(
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
                            platform="fiverr",
                            desk_id=desk_id,
                            created_at=agent.created_at,
                            task_count=0,
                        )
                    )
                    await session.commit()
                await self.emit_event(f"Fiverr agent joined: {role}", "agent_created")

            self.team[label] = agent
            await self.broadcast({"type": "agent_update", "data": agent.to_dict()})
            await asyncio.sleep(1.5)
            asyncio.create_task(agent.safe_run())

    async def _run_pipeline_cycle(self, cycle: int):
        service = await self._step_market_research()
        if not service:
            return

        service = await self._step_profile_optimization(service)
        service = await self._step_service_pack(service)
        service = await self._step_faq_generation(service)
        service = await self._step_deliverable(service)

        await self._save_service(service)
        await self.narrate(f"Fiverr service package complete: {service['name']}")
        await self.emit_event(
            f"FIVERR SERVICE READY: {service['name']}", "product_update"
        )

    async def _step_market_research(self) -> Optional[dict]:
        await self.narrate("FRD is researching top-performing Fiverr gig categories...")
        frd = self.team.get("FRD")
        if frd:
            await frd.set_status("working", "Analyzing Fiverr marketplace trends")

        await asyncio.sleep(8)

        response = await ai_complete(
            config=self.config,
            prompt=(
                "You are an expert Fiverr marketplace researcher. "
                "Find the top 3 most profitable freelance service categories on Fiverr right now. "
                "Consider: high demand, good pricing, low competition, fast delivery options. "
                "Return JSON: {category, subcategory, service_name, description, "
                "starting_price, estimated_earnings_per_order, competition_level: low/medium/high, "
                "delivery_time_days, tags: []}"
            ),
            system=(
                "You are FRD — Fiverr Market Research Agent. "
                "You discover profitable freelance niches on Fiverr. "
                "Think about AI-related services, content creation, and digital services."
            ),
            json_mode=True,
            max_tokens=600,
        )

        try:
            data = json.loads(response)
        except Exception:
            data = {
                "category": "Graphics & Design",
                "subcategory": "AI Art & Illustration",
                "service_name": "Custom AI-Generated Artwork",
                "description": "I will create unique AI-generated artwork for your project, brand, or personal use.",
                "starting_price": 15,
                "estimated_earnings_per_order": 12,
                "competition_level": "medium",
                "delivery_time_days": 1,
                "tags": [
                    "ai art",
                    "custom artwork",
                    "digital illustration",
                    "ai portrait",
                ],
            }

        await self.narrate(f"FRD found opportunity: {data.get('service_name')}")
        if frd:
            await frd.set_status("complete", "Research complete")
            await asyncio.sleep(2)
            await frd.set_status("idle")

        return {
            "id": str(uuid.uuid4()),
            "name": data.get("service_name", "Custom Service"),
            "platform": "fiverr",
            "state": "IDEA_DISCOVERED",
            "category": data.get("category", ""),
            "description": data.get("description", ""),
            "startingPrice": data.get("starting_price", 15),
            "earningsPerOrder": data.get("estimated_earnings_per_order", 12),
            "deliveryDays": data.get("delivery_time_days", 1),
            "tags": data.get("tags", []),
            "createdAt": int(time.time() * 1000),
        }

    async def _step_profile_optimization(self, service: dict) -> dict:
        await self.narrate("FPR is optimizing the Fiverr seller profile...")
        fpr = self.team.get("FPR")
        if fpr:
            await fpr.set_status("working", "Creating optimized seller profile")
        await asyncio.sleep(6)

        response = await ai_complete(
            config=self.config,
            prompt=(
                f"Create an optimized Fiverr seller profile for a freelancer offering: "
                f"{service['name']}. "
                "Return JSON: {seller_title, overview, skills: [], "
                "years_experience: number, languages: [], response_time}"
            ),
            system=(
                "You are FPR — Fiverr Profile Optimizer. "
                "You create compelling seller profiles that convert buyers. "
                "Focus on professional presentation and keyword optimization."
            ),
            json_mode=True,
            max_tokens=400,
        )

        try:
            data = json.loads(response)
        except Exception:
            data = {
                "seller_title": f"Expert {service['name']} Provider",
                "overview": f"Professional {service['category']} specialist with years of experience delivering high-quality {service['name']} to clients worldwide.",
                "skills": ["AI Art Generation", "Creative Design", "Fast Delivery"],
                "years_experience": 3,
                "languages": ["English"],
                "response_time": "1 hour",
            }

        service["profileTitle"] = data.get("seller_title", "")
        service["profileOverview"] = data.get("overview", "")
        service["profileSkills"] = data.get("skills", [])
        service["responseTime"] = data.get("response_time", "1 hour")

        if fpr:
            await fpr.set_status("complete", "Profile optimized")
            await asyncio.sleep(2)
            await fpr.set_status("idle")

        await self.narrate("Fiverr seller profile optimized and ready.")
        return service

    async def _step_service_pack(self, service: dict) -> dict:
        await self.narrate("FSP is building the service tier packages...")
        fsp = self.team.get("FSP")
        if fsp:
            await fsp.set_status("working", "Creating gig packages")
        await asyncio.sleep(6)

        response = await ai_complete(
            config=self.config,
            prompt=(
                f"Create 3 pricing tiers for a Fiverr gig offering: {service['name']}. "
                f"Base price: ${service.get('startingPrice', 15)}. "
                "Return JSON: {basic: {price, delivery_days, revisions, description, includes: []}, "
                "standard: {price, delivery_days, revisions, description, includes: []}, "
                "premium: {price, delivery_days, revisions, description, includes: []}}"
            ),
            system=(
                "You are FSP — Fiverr Service Pack Builder. "
                "Create compelling 3-tier pricing that maximizes order value. "
                "Use anchoring and value stacking to drive standard tier purchases."
            ),
            json_mode=True,
            max_tokens=500,
        )

        try:
            data = json.loads(response)
        except Exception:
            base = service.get("startingPrice", 15)
            data = {
                "basic": {
                    "price": base,
                    "delivery_days": 1,
                    "revisions": 1,
                    "description": "Basic package",
                    "includes": ["1 design", "PNG file"],
                },
                "standard": {
                    "price": base * 2.5,
                    "delivery_days": 2,
                    "revisions": 3,
                    "description": "Standard package",
                    "includes": ["3 designs", "PNG + JPG", "Source file"],
                },
                "premium": {
                    "price": base * 5,
                    "delivery_days": 3,
                    "revisions": 5,
                    "description": "Premium package",
                    "includes": [
                        "5 designs",
                        "All formats",
                        "Source files",
                        "Commercial license",
                    ],
                },
            }

        service["pricingTiers"] = data
        service["state"] = "DESIGN_GENERATED"

        if fsp:
            await fsp.set_status("complete", "Packages built")
            await asyncio.sleep(2)
            await fsp.set_status("idle")

        await self.narrate("Service tier packages created: Basic, Standard, Premium")
        return service

    async def _step_faq_generation(self, service: dict) -> dict:
        await self.narrate("FAQ is generating common buyer questions and responses...")
        faq = self.team.get("FAQ")
        if faq:
            await faq.set_status("working", "Creating FAQ responses")
        await asyncio.sleep(5)

        response = await ai_complete(
            config=self.config,
            prompt=(
                f"Create 5 common buyer questions and answers for a Fiverr gig: {service['name']}. "
                "Make them realistic and helpful. "
                "Return JSON: {faqs: [{question, answer}]}"
            ),
            system=(
                "You are FAQ — Fiverr FAQ Agent. "
                "You create helpful FAQ sections that reduce buyer friction and increase conversions. "
                "Anticipate real buyer concerns and address them clearly."
            ),
            json_mode=True,
            max_tokens=400,
        )

        try:
            data = json.loads(response)
        except Exception:
            data = {
                "faqs": [
                    {
                        "question": "What file formats do you deliver?",
                        "answer": "I deliver PNG, JPG, and source files (PSD/AI) depending on your tier.",
                    },
                    {
                        "question": "Can I request revisions?",
                        "answer": "Yes! All packages include at least 1 revision. Standard and Premium include more.",
                    },
                    {
                        "question": "How do I provide reference images?",
                        "answer": "You can upload reference images directly to the Fiverr order after purchase.",
                    },
                    {
                        "question": "Do you offer commercial use?",
                        "answer": "Premium tier includes full commercial license for business use.",
                    },
                    {
                        "question": "What is your turnaround time?",
                        "answer": "Delivery starts within 24 hours for all orders, faster for rush requests.",
                    },
                ]
            }

        service["faqs"] = data.get("faqs", [])

        if faq:
            await faq.set_status("complete", "FAQs generated")
            await asyncio.sleep(2)
            await faq.set_status("idle")

        return service

    async def _step_deliverable(self, service: dict) -> dict:
        await self.narrate("FDL is generating template deliverables...")
        fdl = self.team.get("FDL")
        if fdl:
            await fdl.set_status("working", "Creating deliverable templates")
        await asyncio.sleep(8)

        response = await ai_complete(
            config=self.config,
            prompt=(
                f"Create a deliverable template and workflow document for a Fiverr gig: "
                f"{service['name']}. "
                "Include: order workflow, file naming convention, communication template, "
                "delivery checklist. "
                "Return JSON: {workflow_steps: [], file_template: string, "
                "delivery_checklist: [], communication_template: string}"
            ),
            system=(
                "You are FDL — Fiverr Deliverable Generator. "
                "You create systematic workflows that ensure consistent, high-quality delivery. "
                "Think about efficiency, client satisfaction, and repeat orders."
            ),
            json_mode=True,
            max_tokens=500,
        )

        try:
            data = json.loads(response)
        except Exception:
            data = {
                "workflow_steps": [
                    "Receive order details",
                    "Create initial draft",
                    "Submit for review",
                    "Make revisions",
                    "Deliver final files",
                ],
                "file_template": f"[ClientName]_[Service]_[Date]_[Version]",
                "delivery_checklist": [
                    "All files exported",
                    "Preview images created",
                    "Source files organized",
                    "Order marked complete",
                ],
                "communication_template": "Hi! Thank you for your order. I've received your details and will start working immediately. You'll receive your first draft within [TIME]. Please let me know if you have any questions!",
            }

        service["workflow"] = data
        service["state"] = "AWAITING_HUMAN_DECISION"

        if fdl:
            await fdl.set_status("complete", "Deliverables ready")
            await asyncio.sleep(2)
            await fdl.set_status("idle")

        return service

    async def _save_service(self, service: dict) -> dict:
        service["updatedAt"] = int(time.time() * 1000)
        async with SessionFactory() as session:
            result = await session.execute(
                select(ProductRecord).where(ProductRecord.id == service["id"])
            )
            record = result.scalar_one_or_none()
            if record:
                record.state = service["state"]
                record.niche = service.get("name", "")
                record.updated_at = time.time()
            else:
                session.add(
                    ProductRecord(
                        id=service["id"],
                        name=service.get("name", ""),
                        platform="fiverr",
                        state=service.get("state", "IDEA_DISCOVERED"),
                        niche=service.get("category", ""),
                        created_at=service.get("createdAt", time.time() * 1000) / 1000,
                        updated_at=time.time(),
                    )
                )
            await session.commit()

        await self.broadcast({"type": "product_update", "data": service})
        return service
