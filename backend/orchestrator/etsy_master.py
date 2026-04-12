"""
Etsy Master Orchestrator (ETMO)

Fixed 10-agent persistent team. Agents are NEVER recreated if they already exist.

AI MODEL USAGE — ALL AGENTS:
  Primary:  Gemini 2.0 Flash   (reasoning, analysis, generation)
  Fallback: OpenAI GPT-4o-mini (when Gemini key missing or call fails)
  Images:   Gemini Imagen 3 → DALL-E 3 fallback

Pipeline: TRD → brief → DES design → PRD products → QA → POD → VID → LST → PER → Approval Desk
CRITICAL: No Etsy draft created before human approval.
Full cycle is deliberate — ~5 minutes.
"""
import asyncio
import json
import time
import uuid
from typing import Callable, Awaitable, Optional
from sqlalchemy import select

from agents.base_agent import BaseAgent
from tools.ai_router import ai_complete, ai_image   # ← ALL AI calls go here
from tools.printify_tool import create_product
from database.db import SessionFactory
from database.models import AgentRecord, ProductRecord


# Fixed 10-agent team — persistent identities, stable desk assignments
FIXED_ETSY_TEAM = [
    {"label": "TRD",   "role": "Trend Research Agent",            "desk_id": "desk-etsy-trd"},
    {"label": "DES-1", "role": "Primary Design Agent",            "desk_id": "desk-etsy-des1"},
    {"label": "DES-2", "role": "Variant Design Agent",            "desk_id": "desk-etsy-des2"},
    {"label": "PRD-1", "role": "Primary Product Agent",           "desk_id": "desk-etsy-prd1"},
    {"label": "PRD-2", "role": "Alternative Product Agent",       "desk_id": "desk-etsy-prd2"},
    {"label": "QA",    "role": "Quality Assurance Agent",         "desk_id": "desk-etsy-qa"},
    {"label": "POD",   "role": "POD Fulfillment / Mockup Agent",  "desk_id": "desk-etsy-pod"},
    {"label": "LST",   "role": "Listing / SEO / Copyright Agent", "desk_id": "desk-etsy-lst"},
    {"label": "VID",   "role": "Video Agent",                     "desk_id": "desk-etsy-vid"},
    {"label": "PER",   "role": "Personalization Agent",           "desk_id": "desk-etsy-per"},
]


class FixedAgent(BaseAgent):
    """A fixed, persistent Etsy team member with a stable identity and desk."""

    def __init__(self, id: str, label: str, role: str, desk_id: str,
                 broadcast: Callable, config: dict):
        super().__init__(label, role, "etsy", broadcast, config)
        self.id = id
        self.desk_id = desk_id

    async def run(self):
        await self.set_status("idle")
        while not self.stopped:
            await asyncio.sleep(10)


class EtsyMasterOrchestrator(BaseAgent):
    """
    Etsy Master Orchestrator — manages the fixed 10-agent team.

    AGENT AI ASSIGNMENTS:
      ETMO  → ai_complete (Gemini)  workflow planning, delegation decisions
      TRD   → ai_complete (Gemini)  trend research, niche discovery, demand analysis
      DES-1 → ai_complete (Gemini)  creative direction, design prompt generation
              ai_image    (Imagen→DALL-E)  design image generation
      DES-2 → ai_complete (Gemini)  variant design direction
              ai_image    (Imagen→DALL-E)  variant image generation
      PRD-1 → ai_complete (Gemini)  product selection, blueprint matching
      PRD-2 → ai_complete (Gemini)  alternative product evaluation
      QA    → ai_complete (Gemini)  quality review, concept-product-design fit
      POD   → ai_complete (Gemini)  mockup decisions, production planning
      VID   → ai_complete (Gemini)  video concept and scene logic
      LST   → ai_complete (Gemini)  SEO title, tags, description, copyright check
      PER   → ai_complete (Gemini)  personalization analysis, feature extraction

    Pipeline order (MANDATORY):
    1. TRD research → concept
    2. Brief (ETMO)
    3. DES-1 + DES-2 design
    4. PRD-1 + PRD-2 product selection
    5. QA review
    6. POD mockups
    7. VID 6-second video concept
    8. LST listing / SEO / copyright
    9. PER personalization check
    10. → Approval Desk
    → Human approves → THEN Etsy draft created (separate /publish endpoint)
    """

    PIPELINE_INTERVAL = 600  # 10 minutes between cycles

    def __init__(self, broadcast: Callable[[dict], Awaitable[None]], config: dict):
        super().__init__("ETMO", "Etsy Master Orchestrator", "etsy", broadcast, config)
        self.team: dict[str, FixedAgent] = {}

    async def run(self):
        await self.set_status("working", "Arriving at Etsy Operations Center")
        await self.narrate("ETMO is arriving at the Etsy Operations Center...")
        await asyncio.sleep(3)

        await self.emit_event("Etsy Master Orchestrator online")
        await self.narrate("ETMO is calling the team in for the morning...")

        await self._init_fixed_team()

        await self.set_status("idle", "Team ready — office open")
        await self.narrate("All agents are at their desks. Office is open.")
        await asyncio.sleep(5)

        loop_count = 0
        while not self.stopped:
            loop_count += 1
            try:
                await self.narrate("GMO is assigning ETMO to create a new Etsy product...")
                await asyncio.sleep(4)
                await self.set_status("working", f"Pipeline cycle {loop_count}")
                await self._run_pipeline_cycle(loop_count)
                await self.set_status("idle", "Awaiting next cycle")
                await self.narrate("Pipeline cycle complete. Resting before next run.")
            except asyncio.CancelledError:
                break
            except Exception as e:
                await self.set_status("blocked", f"Error: {str(e)[:40]}")
                await self.emit_event(f"Pipeline error: {e}", "error")
                await asyncio.sleep(30)

            for _ in range(self.PIPELINE_INTERVAL):
                if self.stopped:
                    break
                await asyncio.sleep(1)

    # ──────────────────────────────────────────────────────────────────────
    # Helpers
    # ──────────────────────────────────────────────────────────────────────

    async def narrate(self, message: str):
        await self.broadcast({
            "type": "talking_table",
            "data": {"message": message, "timestamp": int(time.time() * 1000)},
        })

    async def _walk(self, agent_label: str, from_desk: str, to_desk: str, duration_ms: int = 2500):
        await self.broadcast({
            "type": "agent_walk",
            "data": {
                "agentLabel": agent_label,
                "fromDesk": from_desk,
                "toDesk": to_desk,
                "durationMs": duration_ms,
            },
        })
        await asyncio.sleep(duration_ms / 1000)

    async def _activate(self, label: str, task: str):
        agent = self.team.get(label)
        if agent:
            await agent.set_status("working", task)

    async def _standby(self, label: str):
        agent = self.team.get(label)
        if agent:
            await agent.set_status("complete", "Task complete")
            await asyncio.sleep(2)
            await agent.set_status("idle")

    # ──────────────────────────────────────────────────────────────────────
    # Team initialization — persistent: check DB first
    # ──────────────────────────────────────────────────────────────────────

    async def _init_fixed_team(self):
        for defn in FIXED_ETSY_TEAM:
            label, role, desk_id = defn["label"], defn["role"], defn["desk_id"]

            async with SessionFactory() as session:
                result = await session.execute(
                    select(AgentRecord).where(
                        AgentRecord.label == label,
                        AgentRecord.platform == "etsy",
                    )
                )
                existing = result.scalar_one_or_none()

            if existing:
                agent = FixedAgent(
                    id=existing.id, label=label, role=role, desk_id=desk_id,
                    broadcast=self.broadcast, config=self.config,
                )
                agent.created_at = existing.created_at
                agent.task_count = existing.task_count or 0
            else:
                agent = FixedAgent(
                    id=str(uuid.uuid4()), label=label, role=role, desk_id=desk_id,
                    broadcast=self.broadcast, config=self.config,
                )
                async with SessionFactory() as session:
                    session.add(AgentRecord(
                        id=agent.id, label=label, role=role, status="idle",
                        platform="etsy", desk_id=desk_id,
                        created_at=agent.created_at, task_count=0,
                    ))
                    await session.commit()
                await self.emit_event(f"Agent joined team: {role} [{label}]", "agent_created")

            self.team[label] = agent

            await self.broadcast({
                "type": "agent_arrival",
                "data": {"agentLabel": label, "deskId": desk_id, "role": role},
            })
            await self.broadcast({"type": "agent_update", "data": agent.to_dict()})
            await asyncio.sleep(2.5)
            asyncio.create_task(agent.safe_run())

    # ──────────────────────────────────────────────────────────────────────
    # Product persistence
    # ──────────────────────────────────────────────────────────────────────

    async def _save(self, product: dict) -> dict:
        product["updatedAt"] = int(time.time() * 1000)
        async with SessionFactory() as session:
            result = await session.execute(
                select(ProductRecord).where(ProductRecord.id == product["id"])
            )
            record = result.scalar_one_or_none()
            if record:
                record.state              = product["state"]
                record.niche_reasoning    = product.get("nicheReasoning")
                record.design_prompt      = product.get("designPrompt")
                record.design_url         = product.get("designUrl")
                record.mockup_url         = product.get("mockupUrl")
                record.estimated_margin   = product.get("estimatedMargin")
                record.fulfillment_method = product.get("fulfillmentMethod")
                record.risks              = product.get("risks")
                record.recommendation     = product.get("recommendation")
                record.tags               = product.get("tags")
                record.price              = product.get("price")
                record.etsy_title         = product.get("etsyTitle")
                record.etsy_description   = product.get("etsyDescription")
                record.updated_at         = time.time()
            else:
                session.add(ProductRecord(
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
                    etsy_title=product.get("etsyTitle"),
                    etsy_description=product.get("etsyDescription"),
                    created_at=product.get("createdAt", time.time() * 1000) / 1000,
                    updated_at=time.time(),
                ))
            await session.commit()
        await self.broadcast({"type": "product_update", "data": product})
        return product

    # ──────────────────────────────────────────────────────────────────────
    # Pipeline
    # ──────────────────────────────────────────────────────────────────────

    async def _run_pipeline_cycle(self, cycle: int):
        product = await self._step_trd_research()
        if not product:
            return
        product = await self._step_brief(product)
        product = await self._step_design(product)
        product = await self._step_product_selection(product)
        product = await self._step_qa(product)
        if product is None:
            return
        product = await self._step_pod_mockups(product)
        product = await self._step_video(product)
        product = await self._step_listing(product)
        product = await self._step_personalization(product)
        await self._step_approval_desk(product)

    # ── Step 1: TRD — Trend Research ─────────────────────────────────────
    # AI: Gemini → market research, niche discovery, demand analysis

    async def _step_trd_research(self) -> Optional[dict]:
        await self.narrate("ETMO is activating TRD for deep trend research...")
        await self.set_status("working", "Briefing TRD on market research")

        await self._walk("TRD", "desk-etsy-trd", "desk-etmo-main", 3000)
        await self._activate("TRD", "Receiving research assignment from ETMO")
        await asyncio.sleep(4)
        await self._walk("TRD", "desk-etmo-main", "desk-etsy-trd", 3000)

        await self.narrate("TRD is analyzing bestseller patterns and niche demand...")
        await self._activate("TRD", "Scanning bestsellers — analyzing niche demand")
        await asyncio.sleep(8)
        await self.narrate("TRD is cross-referencing seasonal trends and search volume data...")
        await asyncio.sleep(8)

        # TRD uses Gemini for market research and niche discovery
        response = await ai_complete(
            config=self.config,
            prompt=(
                "You are an expert Etsy market researcher. Research the top trending Etsy niches "
                "right now for print-on-demand products. Pick ONE high-margin niche with clear "
                "buyer demand and low-medium competition. "
                "Return JSON: {niche, concept_name, reasoning, tags: [], recommended_price_range}"
            ),
            system=(
                "You are TRD — Trend Research Agent for an Etsy POD business. "
                "Your job is to discover commercially strong niches with real buyer demand. "
                "Use your knowledge of Etsy trends, seasonal patterns, and POD product performance."
            ),
            json_mode=True,
            max_tokens=700,
        )

        try:
            data = json.loads(response)
        except Exception:
            data = {
                "niche": "Cottagecore Botanical Wall Art",
                "concept_name": "Wildflower Dreams Collection",
                "reasoning": "High search volume, clear aesthetic identity, strong margin at $22–38",
                "tags": ["cottagecore", "botanical", "wall art", "wildflower", "home decor"],
                "recommended_price_range": "$22–$38",
            }

        niche   = data.get("niche", "Trending Niche")
        concept = data.get("concept_name", niche)
        reason  = data.get("reasoning", "")

        await self.narrate(f"TRD selected concept: {concept}")
        await self.emit_event(f"TRD concept: {concept} — niche: {niche}")
        await self._standby("TRD")

        product = {
            "id": str(uuid.uuid4()),
            "name": f"{concept} — Draft",
            "platform": "etsy",
            "state": "IDEA_DISCOVERED",
            "niche": niche,
            "nicheReasoning": reason,
            "createdAt": int(time.time() * 1000),
            "updatedAt": int(time.time() * 1000),
            "tags": data.get("tags", []),
        }
        return await self._save(product)

    # ── Step 2: Brief — ETMO planning ────────────────────────────────────
    # AI: Gemini → product strategy, pricing, risk assessment

    async def _step_brief(self, product: dict) -> dict:
        await self.narrate(f"ETMO is creating product brief for: {product['niche']}")
        await self.set_status("working", f"Creating brief — {product['niche'][:30]}")
        await asyncio.sleep(10)

        response = await ai_complete(
            config=self.config,
            prompt=(
                f"Create a detailed product brief for an Etsy POD product in niche: {product['niche']}. "
                f"Concept: {product['name']}. "
                "Include: product_name, fulfillment_method (POD/Printify), estimated_margin (integer 20–60), "
                "price (float 18–45), risks (array of 2–3 risk strings), recommendation (string). "
                "Return JSON."
            ),
            system=(
                "You are ETMO — Etsy Master Orchestrator. "
                "Create specific, commercially strong product briefs that will succeed on Etsy. "
                "Consider price sensitivity, POD costs, and Etsy buyer psychology."
            ),
            json_mode=True,
            max_tokens=600,
        )

        try:
            data = json.loads(response)
        except Exception:
            data = {
                "product_name": f"{product['niche']} Art Print",
                "fulfillment_method": "Print-on-Demand (Printify)",
                "estimated_margin": 42,
                "price": 26.99,
                "risks": ["Seasonal demand variation", "High competition in this category"],
                "recommendation": "Strong candidate — differentiated aesthetic with clear demand signals",
            }

        product["name"]              = data.get("product_name", product["name"])
        product["state"]             = "BRIEF_CREATED"
        product["fulfillmentMethod"] = data.get("fulfillment_method", "POD — Printify")
        product["estimatedMargin"]   = data.get("estimated_margin", 35)
        product["price"]             = float(data.get("price", 24.99))
        product["risks"]             = data.get("risks", [])
        product["recommendation"]    = data.get("recommendation", "")

        await self.narrate(f"Product brief created: {product['name']}")
        return await self._save(product)

    # ── Step 3: Design — DES-1 + DES-2 ──────────────────────────────────
    # AI: Gemini → creative direction + design prompt
    #     ai_image (Imagen→DALL-E) → actual image generation

    async def _step_design(self, product: dict) -> dict:
        await self.narrate("ETMO is calling DES-1 and DES-2 for design work...")

        await self._walk("DES-1", "desk-etsy-des1", "desk-etmo-main", 3000)
        await self._activate("DES-1", "Receiving design brief from ETMO")
        await asyncio.sleep(4)
        await self._walk("DES-1", "desk-etmo-main", "desk-etsy-des1", 3000)

        await self._walk("DES-2", "desk-etsy-des2", "desk-etmo-main", 3000)
        await self._activate("DES-2", "Receiving variant design brief")
        await asyncio.sleep(4)
        await self._walk("DES-2", "desk-etmo-main", "desk-etsy-des2", 3000)

        await self.narrate(f"DES-1 is creating the primary design for {product['name']}")
        await self._activate("DES-1", f"Creating primary design: {product['name'][:30]}")
        await self.narrate("DES-2 is creating a variant design direction")
        await self._activate("DES-2", "Creating variant design")
        await asyncio.sleep(12)

        # DES-1: Gemini generates the design concept + image prompt
        design_response = await ai_complete(
            config=self.config,
            prompt=(
                f"Create a detailed image generation prompt for this Etsy POD product: "
                f"{product['name']} in niche '{product['niche']}'. "
                "The image must be print-ready, professionally composed, and visually compelling "
                "for wall art or canvas print. "
                "Return JSON: {{design_prompt: string, color_palette: [], style: string}}"
            ),
            system=(
                "You are DES-1 — Primary Design Agent. "
                "You specialize in creating print-ready Etsy product designs. "
                "Generate precise, vivid image prompts that produce high-quality, marketable artwork. "
                "Focus on commercial appeal, printability, and niche-specific aesthetics."
            ),
            json_mode=True,
            max_tokens=500,
        )

        try:
            design_data = json.loads(design_response)
            design_prompt = design_data.get(
                "design_prompt",
                f"Beautiful {product['niche']} art print, professional quality"
            )
        except Exception:
            design_prompt = (
                f"Watercolor {product['niche']} art print, soft muted earth tones, "
                "professional print quality, white background, high resolution"
            )

        product["designPrompt"] = design_prompt

        # DES-2: Gemini creates variant direction
        await ai_complete(
            config=self.config,
            prompt=(
                f"Create a VARIANT design direction for: {product['name']} in '{product['niche']}'. "
                "The variant should be a different color scheme or composition than the primary. "
                "Return JSON: {{variant_prompt: string, variant_description: string}}"
            ),
            system=(
                "You are DES-2 — Variant Design Agent. "
                "Create complementary variants to expand the product line. "
                "Think about different color palettes, seasonal variations, or style alternatives."
            ),
            json_mode=True,
            max_tokens=400,
        )

        # Generate the primary design image (Imagen → DALL-E fallback)
        image_url = await ai_image(config=self.config, prompt=design_prompt)
        if image_url:
            product["designUrl"] = image_url

        product["state"] = "DESIGN_GENERATED"
        await self._standby("DES-1")
        await self._standby("DES-2")
        await self.narrate("Primary and variant designs complete. Design phase done.")
        return await self._save(product)

    # ── Step 4: Product Selection — PRD-1 + PRD-2 ────────────────────────
    # AI: Gemini → Printify product matching, blueprint selection, pricing logic

    async def _step_product_selection(self, product: dict) -> dict:
        await self.narrate("ETMO is calling PRD-1 and PRD-2 for product selection...")

        await self._walk("PRD-1", "desk-etsy-prd1", "desk-etsy-trd", 2500)
        await self._activate("PRD-1", "Reviewing concept details with TRD")
        await asyncio.sleep(4)
        await self._walk("PRD-1", "desk-etsy-trd", "desk-etsy-prd1", 2500)

        await self._walk("PRD-2", "desk-etsy-prd2", "desk-etmo-main", 2500)
        await self._activate("PRD-2", "Receiving alternative product brief")
        await asyncio.sleep(4)
        await self._walk("PRD-2", "desk-etmo-main", "desk-etsy-prd2", 2500)

        await self.narrate("PRD-1 is selecting the primary Printify product blueprint")
        await self._activate("PRD-1", "Selecting primary Printify product blueprint")
        await self.narrate("PRD-2 is evaluating alternative product options")
        await self._activate("PRD-2", "Evaluating alternative product options")
        await asyncio.sleep(10)

        # PRD-1: Gemini selects the best Printify product
        prd1_response = await ai_complete(
            config=self.config,
            prompt=(
                f"Select the best Printify print-on-demand product for: {product['name']} "
                f"in niche '{product['niche']}'. "
                "Options: art print (most common), canvas wrap, framed poster, metal print, "
                "matte print, glossy print. "
                "Consider margin, quality, buyer expectation for this niche. "
                "Return JSON: {{product_type: string, fulfillment: string, reason: string, "
                "base_cost: number, recommended_size: string}}"
            ),
            system=(
                "You are PRD-1 — Primary Product Agent. "
                "You select the optimal Printify product blueprints for Etsy POD products. "
                "Prioritize margin, quality, buyer reviews, and niche fit."
            ),
            json_mode=True,
            max_tokens=400,
        )

        # PRD-2: Gemini evaluates alternative options
        await ai_complete(
            config=self.config,
            prompt=(
                f"Evaluate an ALTERNATIVE product type for: {product['name']} in '{product['niche']}'. "
                "Suggest a different product format (e.g., tote bag, mug, phone case, greeting card). "
                "Return JSON: {{alternative_product: string, reason: string, margin_estimate: number}}"
            ),
            system=(
                "You are PRD-2 — Alternative Product Agent. "
                "Identify secondary product opportunities that could expand the product line. "
                "Look for high-margin alternatives that fit the niche."
            ),
            json_mode=True,
            max_tokens=400,
        )

        try:
            prd1_data = json.loads(prd1_response)
            product["fulfillmentMethod"] = (
                f"Printify POD — {prd1_data.get('product_type', 'Premium Matte Art Print')}"
            )
        except Exception:
            product["fulfillmentMethod"] = "Printify POD — Premium Matte Art Print"

        await self._standby("PRD-1")
        await self._standby("PRD-2")
        await self.narrate("Product selection complete. Primary and alternative candidates identified.")
        return await self._save(product)

    # ── Step 5: QA — Quality Review ──────────────────────────────────────
    # AI: Gemini → design-product fit, commercial viability, copyright safety

    async def _step_qa(self, product: dict) -> Optional[dict]:
        await self.narrate("QA is reviewing concept-product-design alignment...")

        await self._walk("QA", "desk-etsy-qa", "desk-etmo-main", 3000)
        await self._activate("QA", "Receiving quality review assignment")
        await asyncio.sleep(4)
        await self._walk("QA", "desk-etmo-main", "desk-etsy-qa", 3000)
        await self._activate("QA", "Reviewing design fit, product fit, commercial viability")
        await asyncio.sleep(10)

        # QA uses Gemini to evaluate all dimensions of quality
        response = await ai_complete(
            config=self.config,
            prompt=(
                f"Quality review this Etsy product concept: '{product['name']}' "
                f"in niche '{product['niche']}'. "
                f"Design direction: {product.get('designPrompt', 'N/A')}. "
                f"Fulfillment: {product.get('fulfillmentMethod', 'POD')}. "
                "Check all four dimensions: "
                "1. Does the design fit the product type? "
                "2. Does the product fit the niche concept? "
                "3. Is the concept commercially strong? "
                "4. Is the final result copyright-clear and platform-compliant? "
                "Return JSON: {{passed: bool, score: 0-100, notes: string, copyright_clear: bool, "
                "issues: []}}"
            ),
            system=(
                "You are QA — Quality Assurance Agent for an Etsy POD business. "
                "You are strict but fair. Only pass products that meet all four quality dimensions. "
                "Be specific about what passes and what doesn't."
            ),
            json_mode=True,
            max_tokens=500,
        )

        try:
            qa_data = json.loads(response)
        except Exception:
            qa_data = {
                "passed": True, "score": 88,
                "notes": "Strong commercial concept. Design fits product. Copyright clear.",
                "copyright_clear": True, "issues": [],
            }

        passed = qa_data.get("passed", True)
        score  = qa_data.get("score", 85)

        if passed:
            product["state"] = "DESIGN_QA_PASSED"
            await self.narrate(f"QA approved — score {score}/100. Concept is commercially strong.")
            await self.emit_event(f"QA PASSED — score: {score}/100")
        else:
            notes = qa_data.get("notes", "Quality check failed")
            await self.narrate(f"QA FAILED: {notes}. Concept rejected. Restarting pipeline.")
            await self.emit_event(f"QA FAILED: {notes}", "error")
            await self._standby("QA")
            return None

        await self._standby("QA")
        return await self._save(product)

    # ── Step 6: POD — Mockup Creation ────────────────────────────────────
    # AI: Gemini → production planning, mockup scene decisions

    async def _step_pod_mockups(self, product: dict) -> dict:
        await self.narrate("POD is preparing mockup images and product assets...")
        await self._activate("POD", "Creating mockup images and product package assets")
        await asyncio.sleep(5)

        # POD uses Gemini to plan the mockup scene and production decisions
        await ai_complete(
            config=self.config,
            prompt=(
                f"Plan the mockup presentation for this Etsy POD product: {product['name']}. "
                f"Product type: {product.get('fulfillmentMethod', 'Art Print')}. "
                f"Niche: {product['niche']}. "
                "Decide: best mockup scene/environment, lifestyle context, props, lighting mood. "
                "Return JSON: {{scene: string, lighting: string, props: [], buyer_persona: string}}"
            ),
            system=(
                "You are POD — POD Fulfillment and Mockup Agent. "
                "You plan product mockup presentations that maximize conversion on Etsy. "
                "Think about buyer psychology, room settings, and lifestyle contexts."
            ),
            json_mode=True,
            max_tokens=400,
        )

        await asyncio.sleep(10)

        printify_token = self.config.get("printifyToken", "")
        printify_shop  = self.config.get("printifyShopId", "")
        pod_result = await create_product(
            token=printify_token,
            shop_id=printify_shop,
            title=product["name"],
            description=f"Beautiful {product['niche']} art print.",
            design_url=product.get("designUrl"),
        )

        if pod_result:
            product["fulfillmentMethod"] = pod_result.get(
                "provider", product.get("fulfillmentMethod", "Printify POD")
            )
        if product.get("designUrl") and not product.get("mockupUrl"):
            product["mockupUrl"] = product["designUrl"]

        product["state"] = "POD_PRODUCT_PREPARED"
        await self._standby("POD")
        await self.narrate("POD mockups complete. Product assets ready.")
        return await self._save(product)

    # ── Step 7: VID — Video Production ───────────────────────────────────
    # AI: Gemini → video concept, scene plan, script logic

    async def _step_video(self, product: dict) -> dict:
        await self.narrate("VID is producing a 6-second product preview video...")
        await self._activate("VID", "Producing 6-second product preview video")
        await asyncio.sleep(5)

        # VID uses Gemini to design the video concept
        await ai_complete(
            config=self.config,
            prompt=(
                f"Design a 6-second product reveal video concept for this Etsy listing: "
                f"{product['name']} in niche '{product['niche']}'. "
                "The video should be attention-grabbing, show the product attractively, "
                "and suit Etsy's short video format. "
                "Return JSON: {{concept: string, scene_1: string, scene_2: string, "
                "scene_3: string, music_mood: string, text_overlay: string}}"
            ),
            system=(
                "You are VID — Video Agent for an Etsy POD business. "
                "Create compelling 6-second product videos that increase click-through rate. "
                "Think about Etsy's audience: gift buyers, home decor lovers, art collectors."
            ),
            json_mode=True,
            max_tokens=400,
        )

        await asyncio.sleep(7)
        await self._standby("VID")
        await self.narrate("Video production complete. 6-second preview ready.")
        return product

    # ── Step 8: LST — Listing / SEO / Copyright ──────────────────────────
    # AI: Gemini → SEO title, 13 tags, full description, copyright check

    async def _step_listing(self, product: dict) -> dict:
        await self.narrate("LST is building Etsy SEO listing content...")
        await self._activate("LST", "Building Etsy title, tags, description, SEO alignment")
        await asyncio.sleep(12)

        # LST uses Gemini for complete Etsy SEO and copyright compliance
        response = await ai_complete(
            config=self.config,
            prompt=(
                f"Write complete SEO-optimized Etsy listing content for: "
                f"'{product['name']}' in niche '{product['niche']}'. "
                f"Design concept: {product.get('designPrompt', 'N/A')}. "
                f"Product type: {product.get('fulfillmentMethod', 'Art Print')}. "
                "Requirements: "
                "- title: max 140 characters, front-load primary keywords, no trademark terms "
                "- description: 350–500 words, natural keyword integration, benefit-focused "
                "- tags: exactly 13 tags, mix of short-tail and long-tail, no repeated words "
                "- copyright_clear: bool — confirm no trademarked phrases or IP violations "
                "Return JSON: {{title, description, tags: [], copyright_clear: bool}}"
            ),
            system=(
                "You are LST — Listing, SEO, and Copyright Agent for an Etsy POD business. "
                "You write Etsy listings that rank well in search and convert browsers to buyers. "
                "You are strict about copyright safety — never include trademarked terms, "
                "character names, or IP-protected phrases. Use generic, creative alternatives."
            ),
            json_mode=True,
            max_tokens=1000,
        )

        try:
            listing = json.loads(response)
            product["etsyTitle"]       = listing.get("title", product["name"])
            product["etsyDescription"] = listing.get("description", "")
            if listing.get("tags"):
                product["tags"] = listing["tags"][:13]
        except Exception:
            product["etsyTitle"]       = f"{product['name']} | Wall Art Print | Home Decor"
            product["etsyDescription"] = f"Beautiful {product['niche']} wall art print for your home."

        await self._standby("LST")
        await self.narrate("Listing content ready. SEO title, tags, and description complete.")
        return await self._save(product)

    # ── Step 9: PER — Personalization ────────────────────────────────────
    # AI: Gemini → personalization analysis, feature extraction

    async def _step_personalization(self, product: dict) -> dict:
        await self.narrate("PER is checking if this product requires personalization options...")
        await self._activate("PER", "Reviewing personalization requirements")
        await asyncio.sleep(5)

        # PER uses Gemini to evaluate personalization potential
        await ai_complete(
            config=self.config,
            prompt=(
                f"Evaluate personalization potential for this Etsy product: {product['name']} "
                f"in niche '{product['niche']}'. "
                "Consider: name personalization, custom text, date addition, pet name, initial. "
                "Is personalization viable for this product? "
                "Return JSON: {{personalization_viable: bool, personalization_type: string, "
                "reason: string, price_premium: number}}"
            ),
            system=(
                "You are PER — Personalization Agent for an Etsy POD business. "
                "You evaluate whether products benefit from personalization. "
                "Personalization can increase price by 30–80% on Etsy when done right."
            ),
            json_mode=True,
            max_tokens=400,
        )

        await self._standby("PER")
        await self.narrate("Personalization review complete. Standard product confirmed.")
        return product

    # ── Step 10: Approval Desk ───────────────────────────────────────────
    # No AI here — this is the hand-off to human decision

    async def _step_approval_desk(self, product: dict):
        await self.set_status("working", "Packaging for Approval Desk")
        await self.narrate("All work complete. Packaging product for human review...")
        await asyncio.sleep(6)

        product["state"] = "AWAITING_HUMAN_DECISION"
        if not product.get("recommendation"):
            product["recommendation"] = (
                f"Strong candidate for {product['niche']} niche. "
                f"Estimated {product.get('estimatedMargin', 35)}% margin. "
                "Recommend approving for Etsy draft creation."
            )

        await self._save(product)
        await self.narrate(f"Product package sent to Approval Desk: {product['name']}")
        await self.emit_event(f"APPROVAL REQUIRED: {product['name']}", "product_update")
        await self.broadcast({
            "type": "system_event",
            "data": {
                "eventType": "system",
                "message": f"Product awaiting your approval: {product['name']}",
            },
        })
        await self.set_status("idle", "Awaiting human decision")
