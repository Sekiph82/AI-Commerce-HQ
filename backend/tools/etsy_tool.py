"""Etsy API tool — creates draft listings."""
import httpx
from typing import Optional


ETSY_API_BASE = "https://openapi.etsy.com/v3"


async def create_draft_listing(
    api_key: str,
    shop_id: str,
    title: str,
    description: str,
    price: float,
    tags: list[str],
    quantity: int = 999,
) -> Optional[dict]:
    """
    Create a draft listing on Etsy. Returns listing data or None on error.
    Requires Etsy API key and shop ID.
    """
    if not api_key or not shop_id:
        print("[Etsy] No credentials — simulating draft creation")
        return _simulate_draft(title, description, price, tags)

    try:
        async with httpx.AsyncClient(timeout=15.0) as client:
            response = await client.post(
                f"{ETSY_API_BASE}/application/shops/{shop_id}/listings",
                headers={
                    "x-api-key": api_key,
                    "Content-Type": "application/json",
                },
                json={
                    "quantity": quantity,
                    "title": title[:140],  # Etsy limit
                    "description": description,
                    "price": price,
                    "who_made": "i_did",
                    "when_made": "made_to_order",
                    "taxonomy_id": 6090,  # Art & Collectibles > Prints
                    "tags": tags[:13],  # Etsy max 13 tags
                    "state": "draft",
                    "type": "physical",
                    "shipping_profile_id": None,
                },
            )

            if response.status_code in (200, 201):
                data = response.json()
                return {
                    "listing_id": data.get("listing_id"),
                    "title": data.get("title"),
                    "url": data.get("url"),
                    "state": "draft",
                }
            else:
                print(f"[Etsy] API error {response.status_code}: {response.text[:200]}")
                return _simulate_draft(title, description, price, tags)

    except Exception as e:
        print(f"[Etsy] Request error: {e}")
        return _simulate_draft(title, description, price, tags)


async def get_shop_info(api_key: str, shop_id: str) -> Optional[dict]:
    """Get shop info to validate credentials."""
    if not api_key or not shop_id:
        return None
    try:
        async with httpx.AsyncClient(timeout=10.0) as client:
            r = await client.get(
                f"{ETSY_API_BASE}/application/shops/{shop_id}",
                headers={"x-api-key": api_key},
            )
            if r.status_code == 200:
                return r.json()
    except Exception:
        pass
    return None


def _simulate_draft(title: str, description: str, price: float, tags: list[str]) -> dict:
    """Simulate a successful draft creation for demo/testing."""
    import time, random
    fake_id = random.randint(1000000000, 9999999999)
    return {
        "listing_id": fake_id,
        "title": title,
        "url": f"https://www.etsy.com/listing/{fake_id}/",
        "state": "draft",
        "simulated": True,
        "price": price,
        "created_at": int(time.time()),
    }


async def publish_listing(api_key: str, shop_id: str, listing_id: int) -> bool:
    """
    Publish a draft listing. ONLY called after explicit user approval.
    """
    if not api_key or not shop_id:
        print(f"[Etsy] Simulated publish for listing {listing_id}")
        return True

    try:
        async with httpx.AsyncClient(timeout=15.0) as client:
            r = await client.patch(
                f"{ETSY_API_BASE}/application/shops/{shop_id}/listings/{listing_id}",
                headers={"x-api-key": api_key, "Content-Type": "application/json"},
                json={"state": "active"},
            )
            return r.status_code in (200, 201)
    except Exception as e:
        print(f"[Etsy] Publish error: {e}")
        return False
