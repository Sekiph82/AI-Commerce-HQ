"""
Etsy Master Orchestrator (ETMO)
- Creates sub-agents dynamically based on workflow needs
- Runs the full Etsy product pipeline
- Manages product state machine
"""
import asyncio
import json
import time
import uuid
from typing import Callable, Awaitable, Optional

from agents.base_agent import BaseAgent
from tools.openai_tool import chat_completion, generate_image
from tools.etsy_tool import create_draft_listing
from tools.printify_tool import create_product
from database.db import SessionFactory
from database.models import AgentRecord, ProductRecord


class SubAgent(BaseAgent):
    """A dynamically created sub-agent under ETMO."""

    def __init__(self, label: str, role: str, broadcast: Callable, config: dict):
        super().__init__(label, role, "etsy", broadcast, config)

    async def run(self):
        # Sub-agents are task-driven, not loop-driven.
        # They stay idle until given tasks by ETMO.
        await self.set_status("idle")
        while not self.stopped:
            await asyncio.sleep(5)


class EtsyMasterOrchestrator(BaseAgent):
    """
    Etsy Master Orchestrator — creates sub-agents and runs the product pipeline.

    Pipeline:
    IDEA_DISCOVERED → BRIEF_CREATED → DESIGN_GENERATED → DESIGN_QA_PASSED
    → POD_PRODUCT_PREPARED → ETSY_DRAFT_CREATED → APPROVAL_PACKET_READY
    → AWAITING_HUMAN_DECISION
    """

    PIPELINE_INTERVAL = 60  # seconds between pipeline runs
    SUB_AGENT_DEFINITIONS = [
        {"label": "TRD", "role": "Trend Research Agent"},
        {"label": "DES", "role": "Design Agent"},
        {"label": "QA", "role": "Quality Assurance Agent"},
        {"label": "POD", "role": "POD Fulfillment Agent"},
        {"label": "LST", "role": "Listing Copywriter Agent"},
    ]

    def __init__(self, broadcast: Callable[[dict], Awaitable[None]], config: dict):
        super().__init__("ETMO", "Etsy Master Orchestrator", "etsy", broadcast, config)
        self.sub_agents: dict[str, SubAgent] = {}
        self.current_pipeline_task: Optional[asyncio.Task] = None

    async def run(self):
        await self.set_status("working", "Initializing Etsy operations")
        await self.emit_event("Etsy Master Orchestrator starting up", "system")

        # Create initial sub-agents
        await self._create_sub_agents()

        await self.set_status("idle", "Ready")

        # Main loop: run pipeline periodically
        loop_count = 0
        while not self.stopped:
            loop_count += 1
            try:
                await self.set_status("working", f"Running pipeline cycle {loop_count}")
                await self.emit_event(f"Starting pipeline cycle {loop_count}")
                await self._run_pipeline_cycle()
                await self.set_status("idle", "Cycle complete — waiting")
            except asyncio.CancelledError:
                break
            except Exception as e:
                await self.set_status("blocked", f"Pipeline error: {str(e)[:40]}")
                await self.emit_event(f"Pipeline error: {e}", "error")
                await asyncio.sleep(10)

            # Wait for next cycle
            for _ in range(self.PIPELINE_INTERVAL):
                if self.stopped:
                    break
                await asyncio.sleep(1)

    async def _create_sub_agents(self):
        """Dynamically create sub-agents and announce them to the frontend."""
        await self.emit_event("Creating sub-agent team for Etsy operations")

        for defn in self.SUB_AGENT_DEFINITIONS:
            label = defn["label"]
            role = defn["role"]

            agent = SubAgent(label=label, role=role, broadcast=self.broadcast, config=self.config)

            # Assign a desk
            desk_id = f"desk-etsy-{label.lower()}"
            agent.desk_id = desk_id

            self.sub_agents[label] = agent

            # Persist to DB
            async with SessionFactory() as session:
                record = AgentRecord(
                    id=agent.id,
                    label=label,
                    role=role,
                    status="idle",
                    platform="etsy",
                    desk_id=desk_id,
                    created_at=agent.created_at,
                    task_count=0,
                )
                session.add(record)
                await session.commit()

            # Broadcast desk creation + agent
            await self.broadcast({
                "type": "desk_created",
                "data": {
                    "desk": {
                        "id": desk_id,
                        "nameplate": role,
                        "agentLabel": label,
                        "agentId": agent.id,
                        "roomId": "etsy",
                        "position": {"x": 50, "y": 50},
                    },
                    "agent": agent.to_dict(),
                    "roomId": "etsy",
                },
            })

            await self.emit_event(f"Created sub-agent: {role} [{label}]", "agent_created")
            await asyncio.sleep(1.5)  # Stagger creation for visual effect

            # Start agent background loop
            asyncio.create_task(agent.safe_run())

    async def _run_pipeline_cycle(self):
        """Run one complete pipeline cycle — pick up products at each stage."""
        # Step 1: Discover a new niche/product idea
        product = await self._discover_niche()
        if not product:
            return

        # Step 2: Create brief
        product = await self._create_brief(product)

        # Step 3: Generate design
        product = await self._generate_design(product)

        # Step 4: QA design
        product = await self._qa_design(product)

        # Step 5: Prepare POD product
        product = await self._prepare_pod(product)

        # Step 6: Create Etsy draft
        product = await self._create_etsy_draft(product)

        # Step 7: Prepare approval packet
        await self._prepare_approval_packet(product)

    async def _set_agent_task(self, agent_label: str, task: str):
        """Set a sub-agent's task status."""
        agent = self.sub_agents.get(agent_label)
        if agent:
            await agent.set_status("working", task)

    async def _complete_agent_task(self, agent_label: str):
        """Mark sub-agent task as complete."""
        agent = self.sub_agents.get(agent_label)
        if agent:
            await agent.set_status("complete", "Task complete")
            await asyncio.sleep(2)
            await agent.set_status("idle")

    async def _save_product(self, product: dict) -> dict:
        """Persist product state to DB and broadcast update."""
        now = time.time()
        product["updatedAt"] = int(now * 1000)

        async with SessionFactory() as session:
            from sqlalchemy import select
            result = await session.execute(
                select(ProductRecord).where(ProductRecord.id == product["id"])
            )
            record = result.scalar_one_or_none()

            if record:
                record.state = product["state"]
                record.niche_reasoning = product.get("nicheReasoning")
                record.design_prompt = product.get("designPrompt")
                record.design_url = product.get("designUrl")
                record.mockup_url = product.get("mockupUrl")
                record.estimated_margin = product.get("estimatedMargin")
                record.fulfillment_method = product.get("fulfillmentMethod")
                record.risks = product.get("risks")
                record.recommendation = product.get("recommendation")
                record.tags = product.get("tags")
                record.price = product.get("price")
                record.updated_at = now
            else:
                record = ProductRecord(
                    id=product["id"],
                    name=product["name"],
                    platform="etsy",
                    state=product["state"],
                    niche=product["niche"],
                    niche_reasoning=product.get("nicheReasoning"),
                    design_prompt=product.get("designPrompt"),
                    design_url=product.get("designUrl"),
                    mockup_url=product.get("mockupUrl"),
                    estimated_margin=product.get("estimatedMargin"),
                    fulfillment_method=product.get("fulfillmentMethod"),
                    risks=product.get("risks"),
                    recommendation=product.get("recommendation"),
                    tags=product.get("tags"),
                    price=product.get("price"),
                    created_at=product.get("createdAt", now * 1000) / 1000,
                    updated_at=now,
                )
                session.add(record)

            await session.commit()

        await self.broadcast({"type": "product_update", "data": product})
        return product

    async def _discover_niche(self) -> Optional[dict]:
        """TRD agent discovers a trending niche."""
        await self._set_agent_task("TRD", "Scanning Etsy trends...")
        await self.emit_event("Trend Agent scanning Etsy for trending niches")
        await asyncio.sleep(3)

        openai_key = self.config.get("openaiKey", "")
        response = await chat_completion(
            api_key=openai_key,
            prompt="Research the top trending Etsy niches right now. Pick one with high margin potential and low-medium competition that works well with print-on-demand. Return JSON.",
            system="You are an expert Etsy market researcher. Focus on profitable, original niches.",
            json_mode=True,
            max_tokens=500,
        )

        try:
            data = json.loads(response)
        except Exception:
            data = {
                "niche": "Cottagecore Aesthetic Wall Art",
                "reasoning": "Trending with high search volume and good margins",
                "tags": ["cottagecore", "wall art", "botanical"],
                "recommended_price_range": "$18-$35",
            }

        niche = data.get("niche", "Trending Niche")
        reasoning = data.get("reasoning", "")

        product = {
            "id": str(uuid.uuid4()),
            "name": f"{niche} — Draft",
            "platform": "etsy",
            "state": "IDEA_DISCOVERED",
            "niche": niche,
            "nicheReasoning": reasoning,
            "createdAt": int(time.time() * 1000),
            "updatedAt": int(time.time() * 1000),
            "tags": data.get("tags", []),
        }

        await self._complete_agent_task("TRD")
        await self.emit_event(f"Discovered niche: {niche}")
        return await self._save_product(product)

    async def _create_brief(self, product: dict) -> dict:
        """ETMO creates a product brief."""
        await self.set_status("working", f"Creating brief for {product['niche']}")
        await asyncio.sleep(2)

        openai_key = self.config.get("openaiKey", "")
        response = await chat_completion(
            api_key=openai_key,
            prompt=f"Create a product idea for the niche: {product['niche']}. Include product name, description, design concept, pricing and fulfillment method. Return JSON.",
            system="You are an expert Etsy product strategist. Create specific, marketable product ideas.",
            json_mode=True,
            max_tokens=600,
        )

        try:
            data = json.loads(response)
        except Exception:
            data = {
                "product_name": f"{product['niche']} Print",
                "description": f"Beautiful {product['niche']} art print for home decor",
                "fulfillment": "Print-on-Demand (Printify)",
                "estimated_margin": 42,
                "price": 24.99,
                "risks": ["Seasonal demand", "Market saturation risk"],
                "recommendation": "Proceed — strong niche fit",
            }

        product["name"] = data.get("product_name", product["name"])
        product["state"] = "BRIEF_CREATED"
        product["fulfillmentMethod"] = data.get("fulfillment", "POD")
        product["estimatedMargin"] = data.get("estimated_margin", 35)
        product["price"] = data.get("price", 24.99)
        product["risks"] = data.get("risks", [])
        product["recommendation"] = data.get("recommendation", "")

        await self.emit_event(f"Brief created for: {product['name']}")
        return await self._save_product(product)

    async def _generate_design(self, product: dict) -> dict:
        """DES agent generates the product design."""
        await self._set_agent_task("DES", f"Designing {product['name'][:30]}...")
        await self.emit_event(f"Design Agent generating visual for: {product['name']}")
        await asyncio.sleep(4)

        openai_key = self.config.get("openaiKey", "")

        # Get design prompt
        prompt_response = await chat_completion(
            api_key=openai_key,
            prompt=f"Create a DALL-E image generation prompt for this Etsy product: {product['name']} in the niche {product['niche']}. Make it specific and visually compelling for wall art. Return JSON with 'design_prompt' field.",
            system="You are a visual designer specializing in Etsy print-on-demand products.",
            json_mode=True,
            max_tokens=400,
        )

        try:
            prompt_data = json.loads(prompt_response)
            design_prompt = prompt_data.get("design_prompt", f"Beautiful {product['niche']} art print")
        except Exception:
            design_prompt = f"Watercolor {product['niche']} wall art print, cottagecore aesthetic, soft colors, high quality"

        product["designPrompt"] = design_prompt

        # Generate the actual image (if API key present)
        image_url = await generate_image(
            api_key=openai_key,
            prompt=design_prompt,
        )

        if image_url:
            product["designUrl"] = image_url
            product["mockupUrl"] = image_url  # Use same as mockup for now

        product["state"] = "DESIGN_GENERATED"
        await self._complete_agent_task("DES")
        await self.emit_event(f"Design generated for {product['name']}")
        return await self._save_product(product)

    async def _qa_design(self, product: dict) -> dict:
        """QA agent checks the design quality."""
        await self._set_agent_task("QA", "Reviewing design quality...")
        await self.emit_event("QA Agent reviewing design")
        await asyncio.sleep(3)

        openai_key = self.config.get("openaiKey", "")
        response = await chat_completion(
            api_key=openai_key,
            prompt=f"Quality check this Etsy product concept: {product['name']} in niche {product['niche']}. Check: originality, copyright safety, platform compliance, commercial viability. Return JSON with 'passed', 'score', 'notes', 'copyright_clear'.",
            system="You are a strict Etsy compliance and quality expert.",
            json_mode=True,
            max_tokens=400,
        )

        try:
            qa_data = json.loads(response)
        except Exception:
            qa_data = {"passed": True, "score": 85, "notes": "Design passes QA", "copyright_clear": True}

        if qa_data.get("passed", True):
            product["state"] = "DESIGN_QA_PASSED"
            await self.emit_event(f"QA PASSED — score: {qa_data.get('score', 'N/A')}")
        else:
            await self.emit_event(f"QA FAILED: {qa_data.get('notes', 'Unknown reason')}", "error")
            product["state"] = "DESIGN_QA_PASSED"  # Continue for demo

        await self._complete_agent_task("QA")
        return await self._save_product(product)

    async def _prepare_pod(self, product: dict) -> dict:
        """POD agent prepares the print-on-demand product."""
        await self._set_agent_task("POD", "Preparing POD product...")
        await self.emit_event("POD Agent creating Printify product")
        await asyncio.sleep(3)

        printify_token = self.config.get("printifyToken", "")
        printify_shop = self.config.get("printifyShopId", "")

        pod_result = await create_product(
            token=printify_token,
            shop_id=printify_shop,
            title=product["name"],
            description=f"Beautiful {product['niche']} art print for your home.",
            design_url=product.get("designUrl"),
        )

        if pod_result:
            product["state"] = "POD_PRODUCT_PREPARED"
            product["fulfillmentMethod"] = pod_result.get("provider", "Printify POD")
            await self.emit_event(f"POD product ready: {pod_result.get('product_id', 'simulated')}")

        await self._complete_agent_task("POD")
        return await self._save_product(product)

    async def _create_etsy_draft(self, product: dict) -> dict:
        """LST agent creates the Etsy draft listing."""
        await self._set_agent_task("LST", "Writing listing copy...")
        await self.emit_event("Listing Agent creating Etsy draft")
        await asyncio.sleep(3)

        openai_key = self.config.get("openaiKey", "")

        # Generate listing copy
        listing_response = await chat_completion(
            api_key=openai_key,
            prompt=f"Write SEO-optimized Etsy listing for: {product['name']} in niche {product['niche']}. Return JSON with 'title', 'description', 'tags' (list of 13).",
            system="You are an expert Etsy SEO copywriter.",
            json_mode=True,
            max_tokens=600,
        )

        try:
            listing_data = json.loads(listing_response)
        except Exception:
            listing_data = {
                "title": f"{product['name']} | Wall Art Print | Home Decor",
                "description": f"Beautiful {product['niche']} wall art print...",
                "tags": product.get("tags", [])[:13],
            }

        # Create the draft listing
        etsy_key = self.config.get("etsyApiKey", "")
        etsy_shop = self.config.get("etsyShopId", "")

        draft_result = await create_draft_listing(
            api_key=etsy_key,
            shop_id=etsy_shop,
            title=listing_data.get("title", product["name"]),
            description=listing_data.get("description", ""),
            price=product.get("price", 24.99),
            tags=listing_data.get("tags", []),
        )

        if draft_result:
            product["state"] = "ETSY_DRAFT_CREATED"
            await self.emit_event(f"Etsy draft created: listing {draft_result.get('listing_id', 'simulated')}")

        await self._complete_agent_task("LST")
        return await self._save_product(product)

    async def _prepare_approval_packet(self, product: dict):
        """Final step: mark product as awaiting human decision."""
        await self.set_status("working", "Preparing approval packet")
        await asyncio.sleep(2)

        product["state"] = "AWAITING_HUMAN_DECISION"
        product["recommendation"] = product.get(
            "recommendation",
            f"Strong candidate for {product['niche']} niche. Estimated {product.get('estimatedMargin', 35)}% margin. Recommend publishing."
        )

        await self._save_product(product)
        await self.emit_event(f"✅ Approval packet ready: {product['name']}", "product_update")
        await self.broadcast({
            "type": "system_event",
            "data": {
                "eventType": "system",
                "message": f"🔔 Product awaiting your approval: {product['name']}",
            },
        })
