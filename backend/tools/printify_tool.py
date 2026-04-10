"""Printify POD tool."""
import httpx
from typing import Optional

PRINTIFY_API = "https://api.printify.com/v1"


async def create_product(
    token: str,
    shop_id: str,
    title: str,
    description: str,
    design_url: Optional[str],
    blueprint_id: int = 5,  # Unisex Staple T-Shirt for default
    print_provider_id: int = 29,
) -> Optional[dict]:
    """Create a product on Printify. Returns product data or simulated data."""
    if not token or not shop_id:
        return _simulate_pod_product(title)

    if not design_url:
        return _simulate_pod_product(title)

    try:
        async with httpx.AsyncClient(timeout=30.0) as client:
            # Upload image first
            upload_r = await client.post(
                f"{PRINTIFY_API}/uploads/images.json",
                headers={"Authorization": f"Bearer {token}"},
                json={"file_name": "design.png", "url": design_url},
            )
            if upload_r.status_code not in (200, 201):
                return _simulate_pod_product(title)

            image_id = upload_r.json().get("id")

            # Create product
            r = await client.post(
                f"{PRINTIFY_API}/shops/{shop_id}/products.json",
                headers={"Authorization": f"Bearer {token}", "Content-Type": "application/json"},
                json={
                    "title": title,
                    "description": description,
                    "blueprint_id": blueprint_id,
                    "print_provider_id": print_provider_id,
                    "variants": [
                        {"id": 17887, "price": 2100, "is_enabled": True}
                    ],
                    "print_areas": [
                        {
                            "variant_ids": [17887],
                            "placeholders": [
                                {"position": "front", "images": [{"id": image_id, "x": 0.5, "y": 0.5, "scale": 1, "angle": 0}]}
                            ],
                        }
                    ],
                },
            )
            if r.status_code in (200, 201):
                data = r.json()
                return {"product_id": data.get("id"), "title": title, "simulated": False}

    except Exception as e:
        print(f"[Printify] Error: {e}")

    return _simulate_pod_product(title)


def _simulate_pod_product(title: str) -> dict:
    import random
    return {
        "product_id": f"sim-{random.randint(100000, 999999)}",
        "title": title,
        "simulated": True,
        "blueprint": "Canvas Print 8x10",
        "provider": "Printify (simulated)",
        "base_cost": 8.50,
        "shipping": 3.99,
    }
