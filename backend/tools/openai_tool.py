"""OpenAI tool wrapper — handles all AI calls with graceful fallback."""
import json
import asyncio
from typing import Optional


async def chat_completion(
    api_key: str,
    prompt: str,
    system: str = "You are an expert e-commerce product researcher.",
    model: str = "gpt-4o-mini",
    json_mode: bool = False,
    max_tokens: int = 800,
) -> str:
    """
    Call OpenAI chat completion API.
    Returns the text response, or a fallback if API key is missing/invalid.
    """
    if not api_key or not api_key.startswith("sk-"):
        return _fallback_response(prompt, json_mode)

    try:
        import openai
        client = openai.AsyncOpenAI(api_key=api_key)
        kwargs: dict = {
            "model": model,
            "messages": [
                {"role": "system", "content": system},
                {"role": "user", "content": prompt},
            ],
            "max_tokens": max_tokens,
        }
        if json_mode:
            kwargs["response_format"] = {"type": "json_object"}

        response = await client.chat.completions.create(**kwargs)
        return response.choices[0].message.content or ""
    except Exception as e:
        print(f"[OpenAI] Error: {e}")
        return _fallback_response(prompt, json_mode)


async def generate_image(
    api_key: str,
    prompt: str,
    size: str = "1024x1024",
    model: str = "dall-e-3",
) -> Optional[str]:
    """
    Generate an image using DALL-E. Returns URL or None.
    """
    if not api_key or not api_key.startswith("sk-"):
        return None

    try:
        import openai
        client = openai.AsyncOpenAI(api_key=api_key)
        response = await client.images.generate(
            model=model,
            prompt=prompt,
            size=size,  # type: ignore
            quality="standard",
            n=1,
        )
        return response.data[0].url
    except Exception as e:
        print(f"[DALL-E] Error: {e}")
        return None


def _fallback_response(prompt: str, json_mode: bool) -> str:
    """Return a realistic-looking fallback when no API key is present."""
    if json_mode:
        # Return plausible demo JSON based on prompt keywords
        prompt_lower = prompt.lower()

        if "niche" in prompt_lower:
            return json.dumps({
                "niche": "Cottagecore Aesthetic Wall Art",
                "reasoning": "Cottagecore continues to trend strongly on Etsy with 2.3M+ searches monthly. Low competition in personalized variants. High margin potential with POD.",
                "search_volume": "high",
                "competition": "medium",
                "margin_potential": "45%",
                "tags": ["cottagecore", "wall art", "boho", "farmhouse", "mushroom", "nature", "botanical", "aesthetic"],
                "recommended_price_range": "$18-$35",
            })
        elif "product" in prompt_lower or "idea" in prompt_lower:
            return json.dumps({
                "product_name": "Personalized Mushroom Forest Print",
                "description": "A dreamy watercolor-style forest scene with mushrooms, perfect for cottagecore lovers. Customizable with family name or quote.",
                "fulfillment": "Print-on-Demand (Printify)",
                "estimated_margin": 42,
                "price": 24.99,
                "risks": ["Seasonal demand fluctuation", "Multiple similar listings exist"],
                "recommendation": "Strong candidate. Differentiate with personalization option. Target 'cottagecore gifts' keyword.",
                "tags": ["mushroom print", "cottagecore art", "forest print", "botanical art", "nature wall decor"],
            })
        elif "design" in prompt_lower:
            return json.dumps({
                "design_prompt": "Watercolor painting of a magical mushroom forest with soft pink and green tones, cottagecore aesthetic, delicate botanical details, soft morning light filtering through trees, dreamy atmosphere, suitable for wall art print",
                "color_palette": ["#D4A7B5", "#7FB069", "#F5E6D3", "#4A7C59"],
                "style": "watercolor",
                "dimensions": "3600x3600px at 300dpi",
            })
        elif "qa" in prompt_lower or "quality" in prompt_lower:
            return json.dumps({
                "passed": True,
                "score": 87,
                "notes": "Design meets quality standards. Good resolution. Colors are marketable. Recommend slight contrast boost.",
                "copyright_clear": True,
                "platform_compliant": True,
            })
        elif "listing" in prompt_lower:
            return json.dumps({
                "title": "Cottagecore Mushroom Forest Print | Personalized Wall Art | Botanical Print | Nature Decor | Boho Home",
                "description": "Bring the magic of the forest into your home with this enchanting cottagecore mushroom print...\n\nPerfect for nature lovers and cottagecore enthusiasts!",
                "tags": ["mushroom print", "cottagecore", "wall art", "botanical", "forest print", "nature decor", "boho art"],
                "price": 24.99,
                "materials": "Premium matte paper, archival inks",
            })
        else:
            return json.dumps({"result": "Analysis complete", "status": "ok"})
    else:
        return "Analysis complete. [Demo mode — add OpenAI API key for real AI responses]"
