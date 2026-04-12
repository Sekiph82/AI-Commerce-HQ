"""
AI Router — Gemini-first, OpenAI-fallback.

This is the SINGLE entry point every agent uses for AI tasks.
No agent should import openai_tool or gemini_tool directly.

Priority order:
  1. Gemini 2.0 Flash   (primary — reasoning, text, analysis)
  2. OpenAI GPT-4o-mini (fallback — when Gemini key missing or call fails)
  3. Demo mode          (static fallback when no keys — keeps office alive)

For images:
  1. Gemini Imagen 3    (primary)
  2. DALL-E 3           (fallback)

Usage:
    from tools.ai_router import ai_complete, ai_image

    text = await ai_complete(config, prompt, system="...", json_mode=True)
    url  = await ai_image(config, prompt)
"""
from typing import Optional

from tools.gemini_tool import gemini_complete, gemini_image
from tools.openai_tool import (
    chat_completion as _openai_complete,
    generate_image  as _openai_image,
    _fallback_response,
)


async def ai_complete(
    config: dict,
    prompt: str,
    system: str = "You are an expert e-commerce assistant.",
    json_mode: bool = False,
    max_tokens: int = 800,
) -> str:
    """
    Generate text / reasoning output for any agent.

    Tries Gemini first. Falls back to OpenAI. Falls back to demo data.
    Always returns a non-empty string — never raises.
    """
    gemini_key = config.get("geminiKey", "")
    openai_key = config.get("openaiKey", "")

    # ── 1. Gemini (primary) ───────────────────────────────────────────────
    if gemini_key:
        result = await gemini_complete(
            api_key=gemini_key,
            prompt=prompt,
            system=system,
            json_mode=json_mode,
            max_tokens=max_tokens,
        )
        if result:
            return result

    # ── 2. OpenAI (fallback) ──────────────────────────────────────────────
    if openai_key:
        result = await _openai_complete(
            api_key=openai_key,
            prompt=prompt,
            system=system,
            json_mode=json_mode,
            max_tokens=max_tokens,
        )
        if result:
            return result

    # ── 3. Demo mode (no keys / all calls failed) ────────────────────────
    return _fallback_response(prompt, json_mode)


async def ai_image(
    config: dict,
    prompt: str,
) -> Optional[str]:
    """
    Generate an image for design / mockup use.

    Tries Gemini Imagen 3 first. Falls back to DALL-E 3.
    Returns a URL or base64 data-URL, or None if no image API is available.
    """
    gemini_key = config.get("geminiKey", "")
    openai_key  = config.get("openaiKey", "")

    # ── 1. Gemini Imagen 3 (primary) ────────────────────────────────────
    if gemini_key:
        url = await gemini_image(api_key=gemini_key, prompt=prompt)
        if url:
            return url

    # ── 2. DALL-E 3 (fallback) ───────────────────────────────────────────
    if openai_key:
        url = await _openai_image(api_key=openai_key, prompt=prompt)
        if url:
            return url

    return None
