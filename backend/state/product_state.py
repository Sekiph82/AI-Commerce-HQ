"""Product state machine definitions."""

from enum import Enum


class ProductState(str, Enum):
    IDEA_DISCOVERED = "IDEA_DISCOVERED"
    BRIEF_CREATED = "BRIEF_CREATED"
    DESIGN_GENERATED = "DESIGN_GENERATED"
    DESIGN_QA_PASSED = "DESIGN_QA_PASSED"
    POD_PRODUCT_PREPARED = "POD_PRODUCT_PREPARED"
    ETSY_DRAFT_CREATED = "ETSY_DRAFT_CREATED"
    APPROVAL_PACKET_READY = "APPROVAL_PACKET_READY"
    AWAITING_HUMAN_DECISION = "AWAITING_HUMAN_DECISION"
    APPROVED_FOR_PUBLISH = "APPROVED_FOR_PUBLISH"
    REJECTED = "REJECTED"
    ARCHIVED = "ARCHIVED"


# Valid state transitions
TRANSITIONS: dict[str, list[str]] = {
    ProductState.IDEA_DISCOVERED: [ProductState.BRIEF_CREATED, ProductState.ARCHIVED],
    ProductState.BRIEF_CREATED: [ProductState.DESIGN_GENERATED, ProductState.ARCHIVED],
    ProductState.DESIGN_GENERATED: [ProductState.DESIGN_QA_PASSED, ProductState.BRIEF_CREATED, ProductState.ARCHIVED],
    ProductState.DESIGN_QA_PASSED: [ProductState.POD_PRODUCT_PREPARED, ProductState.DESIGN_GENERATED],
    ProductState.POD_PRODUCT_PREPARED: [ProductState.ETSY_DRAFT_CREATED, ProductState.ARCHIVED],
    ProductState.ETSY_DRAFT_CREATED: [ProductState.APPROVAL_PACKET_READY],
    ProductState.APPROVAL_PACKET_READY: [ProductState.AWAITING_HUMAN_DECISION],
    ProductState.AWAITING_HUMAN_DECISION: [
        ProductState.APPROVED_FOR_PUBLISH,
        ProductState.REJECTED,
    ],
    ProductState.APPROVED_FOR_PUBLISH: [ProductState.ARCHIVED],
    ProductState.REJECTED: [ProductState.ARCHIVED],
    ProductState.ARCHIVED: [],
}


def can_transition(from_state: str, to_state: str) -> bool:
    return to_state in TRANSITIONS.get(from_state, [])


STATE_DESCRIPTIONS = {
    ProductState.IDEA_DISCOVERED: "A trending niche has been identified",
    ProductState.BRIEF_CREATED: "Product brief and specifications ready",
    ProductState.DESIGN_GENERATED: "AI design created",
    ProductState.DESIGN_QA_PASSED: "Design passed quality checks",
    ProductState.POD_PRODUCT_PREPARED: "Print-on-demand product configured",
    ProductState.ETSY_DRAFT_CREATED: "Etsy listing draft saved",
    ProductState.APPROVAL_PACKET_READY: "Full approval packet assembled",
    ProductState.AWAITING_HUMAN_DECISION: "Waiting for your approval",
    ProductState.APPROVED_FOR_PUBLISH: "Approved — ready to go live",
    ProductState.REJECTED: "Rejected — returned to agents",
    ProductState.ARCHIVED: "Archived",
}
