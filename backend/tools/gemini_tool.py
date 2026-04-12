"""
Gemini (Google AI) tool — PRIMARY AI engine for all agents.

Supports:
- Text / reasoning / JSON generation  →  gemini_complete()
- Image generation (Imagen 3)         →  gemini_image()

Model priority within Gemini:
  gemini-2.0-flash  (fast, capable, default)
  gemini-1.5-flash  (fallback if 2.0 unavailable)
"""
import asyncio
from typing import Optional


_PRIMARY_MODEL   = "gemini-2.0-flash"
_FALLBACK_MODEL  = "gemini-1.5-flash"
_IMAGE_MODEL     = "imagen-3.0-generate-002"


async def gemini_complete(
    api_key: str,
    prompt: str,
    system: str = "You are an expert e-commerce assistant.",
    json_mode: bool = False,
    max_tokens: int = 800,
) -> Optional[str]:
    """
    Call Gemini for text generation / reasoning.
    Returns the response string, or None if Gemini is unavailable / errors.
    """
    if not api_key:
        return None

    generation_config: dict = {
        "max_output_tokens": max_tokens,
        "temperature": 0.7,
    }
    if json_mode:
        generation_config["response_mime_type"] = "application/json"

    for model_name in (_PRIMARY_MODEL, _FALLBACK_MODEL):
        try:
            import google.generativeai as genai

            genai.configure(api_key=api_key)
            model = genai.GenerativeModel(
                model_name=model_name,
                system_instruction=system,
                generation_config=generation_config,  # type: ignore[arg-type]
            )
            response = await model.generate_content_async(prompt)
            text = response.text
            if text:
                print(f"[Gemini/{model_name}] OK — {len(text)} chars")
                return text

        except Exception as e:
            print(f"[Gemini/{model_name}] Error: {e}")
            continue

    return None


async def gemini_image(
    api_key: str,
    prompt: str,
) -> Optional[str]:
    """
    Generate an image via Gemini Imagen 3.
    Returns a base64 data-URL string, or None if not available.

    Note: Imagen 3 requires Gemini API with image generation access.
    Gracefully returns None so the caller can fall back to DALL-E.
    """
    if not api_key:
        return None

    try:
        import google.generativeai as genai
        import base64

        genai.configure(api_key=api_key)
        # Imagen 3 via the dedicated ImageGenerationModel
        model = genai.ImageGenerationModel(_IMAGE_MODEL)  # type: ignore[attr-defined]
        result = model.generate_images(
            prompt=prompt,
            number_of_images=1,
            safety_filter_level="block_some",
            person_generation="allow_adult",
        )
        if result.images:
            img_bytes = result.images[0]._image_bytes
            b64 = base64.b64encode(img_bytes).decode("utf-8")
            print(f"[Gemini/Imagen3] Image generated — {len(img_bytes)} bytes")
            return f"data:image/png;base64,{b64}"

    except AttributeError:
        # ImageGenerationModel not available in this SDK version — skip silently
        pass
    except Exception as e:
        print(f"[Gemini/Imagen3] Error: {e}")

    return None
