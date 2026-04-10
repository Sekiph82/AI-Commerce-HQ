from sqlalchemy import Column, String, Float, Integer, JSON, Boolean, Text
from sqlalchemy.orm import DeclarativeBase


class Base(DeclarativeBase):
    pass


class AgentRecord(Base):
    __tablename__ = "agents"

    id = Column(String, primary_key=True)
    label = Column(String, nullable=False)
    role = Column(String, nullable=False)
    status = Column(String, default="idle")
    platform = Column(String, nullable=False)
    current_task = Column(String, nullable=True)
    desk_id = Column(String, nullable=True)
    created_at = Column(Float, nullable=False)
    task_count = Column(Integer, default=0)


class ProductRecord(Base):
    __tablename__ = "products"

    id = Column(String, primary_key=True)
    name = Column(String, nullable=False)
    platform = Column(String, nullable=False)
    state = Column(String, nullable=False)
    niche = Column(String, nullable=False)
    niche_reasoning = Column(Text, nullable=True)
    design_prompt = Column(Text, nullable=True)
    design_url = Column(String, nullable=True)
    mockup_url = Column(String, nullable=True)
    estimated_margin = Column(Float, nullable=True)
    fulfillment_method = Column(String, nullable=True)
    risks = Column(JSON, nullable=True)
    recommendation = Column(Text, nullable=True)
    tags = Column(JSON, nullable=True)
    price = Column(Float, nullable=True)
    created_at = Column(Float, nullable=False)
    updated_at = Column(Float, nullable=False)


class ConfigRecord(Base):
    __tablename__ = "config"

    key = Column(String, primary_key=True)
    value = Column(Text, nullable=False)


class EventRecord(Base):
    __tablename__ = "events"

    id = Column(String, primary_key=True)
    event_type = Column(String, nullable=False)
    message = Column(Text, nullable=False)
    data = Column(JSON, nullable=True)
    timestamp = Column(Float, nullable=False)
