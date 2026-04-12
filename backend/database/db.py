import os
import json
from pathlib import Path
from sqlalchemy.ext.asyncio import create_async_engine, AsyncSession, async_sessionmaker
from .models import Base

# Data directory: use env var or fallback to local
DATA_DIR = Path(os.environ.get("HQ_DATA_DIR", Path.home() / ".ai-commerce-hq"))
DATA_DIR.mkdir(parents=True, exist_ok=True)

DB_PATH = DATA_DIR / "hq.db"
ENGINE = create_async_engine(f"sqlite+aiosqlite:///{DB_PATH}", echo=False)
SessionFactory = async_sessionmaker(ENGINE, class_=AsyncSession, expire_on_commit=False)


async def init_db():
    async with ENGINE.begin() as conn:
        await conn.run_sync(Base.metadata.create_all)
    # Apply any missing column migrations (SQLite doesn't support ALTER TABLE IF NOT EXISTS)
    await _run_migrations()


async def _run_migrations():
    """Add new columns to existing tables without dropping data."""
    migrations = [
        "ALTER TABLE products ADD COLUMN etsy_title TEXT",
        "ALTER TABLE products ADD COLUMN etsy_description TEXT",
    ]
    async with ENGINE.begin() as conn:
        for sql in migrations:
            try:
                await conn.execute(__import__("sqlalchemy").text(sql))
            except Exception:
                pass  # Column already exists — that's fine


async def get_session() -> AsyncSession:
    async with SessionFactory() as session:
        yield session


async def get_config() -> dict:
    """Load config from DB."""
    from .models import ConfigRecord
    async with SessionFactory() as session:
        from sqlalchemy import select
        result = await session.execute(select(ConfigRecord))
        rows = result.scalars().all()
        config = {}
        for row in rows:
            try:
                config[row.key] = json.loads(row.value)
            except Exception:
                config[row.key] = row.value
        return config


async def save_config(data: dict):
    """Save config to DB."""
    from .models import ConfigRecord
    from sqlalchemy.dialects.sqlite import insert as sqlite_insert
    async with SessionFactory() as session:
        for key, value in data.items():
            serialized = json.dumps(value) if not isinstance(value, str) else value
            stmt = sqlite_insert(ConfigRecord).values(key=key, value=serialized)
            stmt = stmt.on_conflict_do_update(index_elements=["key"], set_={"value": serialized})
            await session.execute(stmt)
        await session.commit()
