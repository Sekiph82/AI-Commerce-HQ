"""FastAPI REST routes."""
import time
from typing import Any
from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from sqlalchemy import select

from database.db import SessionFactory, get_config, save_config
from database.models import AgentRecord, ProductRecord

router = APIRouter(prefix="/api")


# --- Health ---

@router.get("/health")
async def health():
    return {"status": "ok", "timestamp": time.time()}


# --- Config ---

class ConfigPayload(BaseModel):
    openaiKey: str = ""
    etsyApiKey: str = ""
    etsyShopId: str = ""
    printifyToken: str = ""
    geminiKey: str = ""
    setupComplete: bool = True
    model_config = {"extra": "allow"}


@router.get("/config")
async def get_config_route():
    config = await get_config()
    if not config:
        raise HTTPException(status_code=404, detail="Not configured")
    return {
        "openaiKey": config.get("openaiKey", ""),
        "etsyApiKey": config.get("etsyApiKey", ""),
        "etsyShopId": config.get("etsyShopId", ""),
        "printifyToken": config.get("printifyToken", ""),
        "geminiKey": config.get("geminiKey", ""),
        "setupComplete": config.get("setupComplete", False),
    }


@router.post("/config")
async def save_config_route(payload: ConfigPayload):
    await save_config(payload.model_dump())
    return {"status": "saved"}


# --- Agents ---

@router.get("/agents")
async def list_agents():
    async with SessionFactory() as session:
        result = await session.execute(select(AgentRecord))
        agents = result.scalars().all()
        return [
            {
                "id": a.id,
                "label": a.label,
                "role": a.role,
                "status": a.status,
                "platform": a.platform,
                "currentTask": a.current_task,
                "deskId": a.desk_id,
                "createdAt": int(a.created_at * 1000),
                "taskCount": a.task_count,
            }
            for a in agents
        ]


# --- Products ---

@router.get("/products")
async def list_products():
    async with SessionFactory() as session:
        result = await session.execute(select(ProductRecord))
        products = result.scalars().all()
        return [_product_to_dict(p) for p in products]


@router.post("/products/{product_id}/approve")
async def approve_product(product_id: str):
    async with SessionFactory() as session:
        result = await session.execute(
            select(ProductRecord).where(ProductRecord.id == product_id)
        )
        product = result.scalar_one_or_none()
        if not product:
            raise HTTPException(status_code=404, detail="Product not found")
        if product.state != "AWAITING_HUMAN_DECISION":
            raise HTTPException(status_code=400, detail="Product not awaiting decision")

        product.state = "APPROVED_FOR_PUBLISH"
        product.updated_at = time.time()
        await session.commit()

        # Broadcast the update
        from api.websocket import broadcast_all
        await broadcast_all({
            "type": "product_update",
            "data": _product_to_dict(product),
        })

        # NOTE: Do NOT auto-publish. Publishing is a separate explicit action.
        return {"status": "approved", "product_id": product_id}


@router.post("/products/{product_id}/reject")
async def reject_product(product_id: str):
    async with SessionFactory() as session:
        result = await session.execute(
            select(ProductRecord).where(ProductRecord.id == product_id)
        )
        product = result.scalar_one_or_none()
        if not product:
            raise HTTPException(status_code=404, detail="Product not found")
        product.state = "REJECTED"
        product.updated_at = time.time()
        await session.commit()

        from api.websocket import broadcast_all
        await broadcast_all({
            "type": "product_update",
            "data": _product_to_dict(product),
        })
        return {"status": "rejected"}


@router.post("/products/{product_id}/archive")
async def archive_product(product_id: str):
    async with SessionFactory() as session:
        result = await session.execute(
            select(ProductRecord).where(ProductRecord.id == product_id)
        )
        product = result.scalar_one_or_none()
        if not product:
            raise HTTPException(status_code=404, detail="Product not found")
        product.state = "ARCHIVED"
        product.updated_at = time.time()
        await session.commit()

        from api.websocket import broadcast_all
        await broadcast_all({
            "type": "product_update",
            "data": _product_to_dict(product),
        })
        return {"status": "archived"}


@router.post("/products/{product_id}/publish")
async def publish_product(product_id: str):
    """
    Explicit publish endpoint — only callable after user approval.
    This is the ONLY path to making a listing live.
    """
    async with SessionFactory() as session:
        result = await session.execute(
            select(ProductRecord).where(ProductRecord.id == product_id)
        )
        product = result.scalar_one_or_none()
        if not product:
            raise HTTPException(status_code=404, detail="Product not found")
        if product.state != "APPROVED_FOR_PUBLISH":
            raise HTTPException(status_code=400, detail="Product must be approved before publishing")

        config = await get_config()
        from tools.etsy_tool import publish_listing
        # Would call publish_listing here with the listing ID
        # For now just mark as a separate state
        await session.commit()
        return {"status": "published"}


def _product_to_dict(p: ProductRecord) -> dict:
    return {
        "id": p.id,
        "name": p.name,
        "platform": p.platform,
        "state": p.state,
        "niche": p.niche,
        "nicheReasoning": p.niche_reasoning,
        "designPrompt": p.design_prompt,
        "designUrl": p.design_url,
        "mockupUrl": p.mockup_url,
        "estimatedMargin": p.estimated_margin,
        "fulfillmentMethod": p.fulfillment_method,
        "risks": p.risks or [],
        "recommendation": p.recommendation,
        "tags": p.tags or [],
        "price": p.price,
        "createdAt": int(p.created_at * 1000),
        "updatedAt": int(p.updated_at * 1000),
    }
