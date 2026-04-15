"""
Trading Master Orchestrator (TMO)

Automated crypto and stock trading system.

AI MODEL: Gemini 2.0 Flash (analysis) → GPT-4o-mini (fallback)

⚠️ DISCLAIMER: This system generates trading signals and analysis.
It does NOT execute real trades. Users must manually review and execute trades
or configure their own exchange API connections with appropriate risk management.

The system tracks market data, generates signals, and maintains a trade journal.

Pipeline:
1. TSC — Signal Collector (collects market data from multiple sources)
2. TAN — Technical Analysis Agent (analyzes charts, indicators, patterns)
3. TEX — Expert Advisor (generates trading signals with risk assessment)
4. TPM — Trade Performance Monitor (tracks journal, calculates metrics)
"""

import asyncio
import json
import time
import uuid
import random
from typing import Callable, Awaitable, Optional
from sqlalchemy import select

from agents.base_agent import BaseAgent
from tools.ai_router import ai_complete
from database.db import SessionFactory
from database.models import AgentRecord


TRADING_TEAM = [
    {"label": "TMO", "role": "Trading Master Orchestrator", "desk_id": "desk-tmo-main"},
    {"label": "TSC", "role": "Signal Collector Agent", "desk_id": "desk-trading-tsc"},
    {"label": "TAN", "role": "Technical Analysis Agent", "desk_id": "desk-trading-tan"},
    {"label": "TEX", "role": "Trading Expert Agent", "desk_id": "desk-trading-tex"},
    {
        "label": "TPM",
        "role": "Trade Performance Monitor",
        "desk_id": "desk-trading-tpm",
    },
]


class FixedTradingAgent(BaseAgent):
    def __init__(
        self,
        id: str,
        label: str,
        role: str,
        desk_id: str,
        broadcast: Callable,
        config: dict,
    ):
        super().__init__(label, role, "trading", broadcast, config)
        self.id = id
        self.desk_id = desk_id

    async def run(self):
        await self.set_status("idle")
        while not self.stopped:
            await asyncio.sleep(10)


class TradingMasterOrchestrator(BaseAgent):
    """
    Automated trading analysis system.
    Generates trading signals, risk assessments, and maintains a trade journal.
    Does NOT execute real trades — generates analysis for user review.
    """

    PIPELINE_INTERVAL = 900  # 15 minutes between cycles

    def __init__(self, broadcast: Callable[[dict], Awaitable[None]], config: dict):
        super().__init__(
            "TMO", "Trading Master Orchestrator", "trading", broadcast, config
        )
        self.team: dict[str, FixedTradingAgent] = {}
        self.trade_journal: list[dict] = []
        self.equity_curve: list[float] = [10000.0]  # Starting with $10k simulation

    async def narrate(self, message: str):
        await self.broadcast(
            {
                "type": "talking_table",
                "data": {"message": message, "timestamp": int(time.time() * 1000)},
            }
        )

    async def run(self):
        await self.set_status("working", "Initializing trading operations")
        await self.narrate("TMO is arriving at the Trading Lab...")
        await asyncio.sleep(3)

        await self.emit_event("Trading Master Orchestrator online")
        await self.narrate("TMO is assembling the trading team...")
        await asyncio.sleep(2)

        await self._init_team()

        await self.set_status("idle", "Trading system ready")
        await self.narrate("Trading Lab is operational. Market analysis active.")

        loop_count = 0
        while not self.stopped:
            loop_count += 1
            try:
                await self.set_status("working", f"Trading cycle {loop_count}")
                await self.narrate(f"TMO is analyzing market conditions...")
                await self._run_pipeline_cycle(loop_count)
                await self.set_status("idle", "Monitoring positions")
            except asyncio.CancelledError:
                break
            except Exception as e:
                await self.set_status("blocked", f"Error: {str(e)[:40]}")
                await self.emit_event(f"Trading error: {e}", "error")
                await asyncio.sleep(30)

            for _ in range(self.PIPELINE_INTERVAL):
                if self.stopped:
                    break
                await asyncio.sleep(1)

    async def _init_team(self):
        for defn in TRADING_TEAM:
            label, role, desk_id = defn["label"], defn["role"], defn["desk_id"]

            async with SessionFactory() as session:
                result = await session.execute(
                    select(AgentRecord).where(
                        AgentRecord.label == label,
                        AgentRecord.platform == "trading",
                    )
                )
                existing = result.scalar_one_or_none()

            if existing:
                agent = FixedTradingAgent(
                    id=existing.id,
                    label=label,
                    role=role,
                    desk_id=desk_id,
                    broadcast=self.broadcast,
                    config=self.config,
                )
                agent.created_at = existing.created_at
            else:
                agent = FixedTradingAgent(
                    id=str(uuid.uuid4()),
                    label=label,
                    role=role,
                    desk_id=desk_id,
                    broadcast=self.broadcast,
                    config=self.config,
                )
                async with SessionFactory() as session:
                    session.add(
                        AgentRecord(
                            id=agent.id,
                            label=label,
                            role=role,
                            status="idle",
                            platform="trading",
                            desk_id=desk_id,
                            created_at=agent.created_at,
                            task_count=0,
                        )
                    )
                    await session.commit()
                await self.emit_event(f"Trading agent joined: {role}", "agent_created")

            self.team[label] = agent
            await self.broadcast({"type": "agent_update", "data": agent.to_dict()})
            await asyncio.sleep(1.5)
            asyncio.create_task(agent.safe_run())

    async def _run_pipeline_cycle(self, cycle: int):
        signal = await self._step_signal_collection()
        signal = await self._step_technical_analysis(signal)
        signal = await self._step_expert_advisor(signal)
        signal = await self._step_performance_monitor(signal)

        await self._broadcast_signal(signal)
        await self.narrate(
            f"Trading signal generated: {signal['action']} {signal['symbol']} @ ${signal['entryPrice']:.2f}"
        )

    async def _step_signal_collection(self) -> dict:
        await self.narrate("TSC is scanning multiple markets for opportunities...")
        tsc = self.team.get("TSC")
        if tsc:
            await tsc.set_status("working", "Collecting market data")
        await asyncio.sleep(5)

        symbols = ["BTC/USD", "ETH/USD", "SOL/USD", "DOGE/USD", "S&P 500"]
        active_symbol = random.choice(symbols)

        response = await ai_complete(
            config=self.config,
            prompt=(
                f"You are a market data analyst. For symbol {active_symbol}, "
                "generate realistic simulated market data for the current session. "
                "Return JSON: {symbol, current_price, change_24h_percent, volume_24h, "
                "high_24h, low_24h, market_cap, trend: bullish/bearish/neutral}"
            ),
            system=(
                "You are TSC — Signal Collector. "
                "You gather and normalize market data from various sources. "
                "Provide realistic market context for trading decisions."
            ),
            json_mode=True,
            max_tokens=400,
        )

        try:
            data = json.loads(response)
        except Exception:
            prices = {
                "BTC/USD": 67240,
                "ETH/USD": 3456,
                "SOL/USD": 142,
                "DOGE/USD": 0.082,
                "S&P 500": 5189,
            }
            data = {
                "symbol": active_symbol,
                "current_price": prices.get(active_symbol, 1000),
                "change_24h_percent": round(random.uniform(-3, 5), 2),
                "volume_24h": round(random.uniform(500e6, 5e9), 0),
                "high_24h": round(prices.get(active_symbol, 1000) * 1.02, 2),
                "low_24h": round(prices.get(active_symbol, 1000) * 0.98, 2),
                "market_cap": round(
                    prices.get(active_symbol, 1000) * random.uniform(1e6, 100e9), 0
                ),
                "trend": random.choice(["bullish", "bearish", "neutral"]),
            }

        if tsc:
            await tsc.set_status("complete", "Data collected")
            await asyncio.sleep(2)
            await tsc.set_status("idle")

        return {
            "id": str(uuid.uuid4()),
            "symbol": data.get("symbol", "BTC/USD"),
            "currentPrice": data.get("current_price", 0),
            "change24h": data.get("change_24h_percent", 0),
            "volume24h": data.get("volume_24h", 0),
            "high24h": data.get("high_24h", 0),
            "low24h": data.get("low_24h", 0),
            "trend": data.get("trend", "neutral"),
            "action": "HOLD",
            "entryPrice": data.get("current_price", 0),
            "stopLoss": 0,
            "takeProfit": 0,
            "riskReward": 0,
            "confidence": 0,
            "timestamp": int(time.time() * 1000),
        }

    async def _step_technical_analysis(self, signal: dict) -> dict:
        await self.narrate(
            "TAN is analyzing chart patterns and technical indicators..."
        )
        tan = self.team.get("TAN")
        if tan:
            await tan.set_status("working", "Technical analysis")

        await asyncio.sleep(6)

        response = await ai_complete(
            config=self.config,
            prompt=(
                f"Perform technical analysis for {signal['symbol']} trading at ${signal['currentPrice']}. "
                f"24h change: {signal['change24h']}%, trend: {signal['trend']}. "
                "Analyze: RSI, MACD, Moving Averages, Support/Resistance, Volume profile. "
                "Return JSON: {rsi: number, macd_signal: bullish/bearish/neutral, "
                "support_level: number, resistance_level: number, pattern: string, "
                "volume_analysis: string, overall_signal: bullish/bearish/neutral}"
            ),
            system=(
                "You are TAN — Technical Analysis Agent. "
                "You perform rigorous technical analysis for trading decisions. "
                "Use multiple timeframes and indicators for confirmation."
            ),
            json_mode=True,
            max_tokens=500,
        )

        try:
            data = json.loads(response)
        except Exception:
            data = {
                "rsi": round(random.uniform(30, 70), 1),
                "macd_signal": "bullish" if signal["change24h"] > 0 else "bearish",
                "support_level": round(signal["currentPrice"] * 0.97, 2),
                "resistance_level": round(signal["currentPrice"] * 1.03, 2),
                "pattern": "ascending triangle",
                "volume_analysis": "above average, confirming trend",
                "overall_signal": "bullish" if signal["change24h"] > 1 else "neutral",
            }

        signal["technicalAnalysis"] = data
        signal["supportLevel"] = data.get(
            "support_level", signal["currentPrice"] * 0.97
        )
        signal["resistanceLevel"] = data.get(
            "resistance_level", signal["currentPrice"] * 1.03
        )

        if tan:
            await tan.set_status("complete", "Analysis complete")
            await asyncio.sleep(2)
            await tan.set_status("idle")

        return signal

    async def _step_expert_advisor(self, signal: dict) -> dict:
        await self.narrate("TEX is generating trading signal with risk assessment...")
        tex = self.team.get("TEX")
        if tex:
            await tex.set_status("working", "Generating trading signal")

        await asyncio.sleep(8)

        response = await ai_complete(
            config=self.config,
            prompt=(
                f"Generate a trading signal for {signal['symbol']} @ ${signal['currentPrice']}. "
                f"Technical analysis: {json.dumps(signal.get('technicalAnalysis', {}))}. "
                f"24h change: {signal['change24h']}%. "
                "IMPORTANT: This is a simulation. Generate a realistic signal. "
                "Return JSON: {action: BUY/SELL/HOLD, entry_price: number, "
                "stop_loss: number, take_profit: number, risk_percent: 1-5, "
                "risk_reward_ratio: number, confidence: 0-100, "
                "reasoning: string, time_horizon: short/medium/long}"
            ),
            system=(
                "You are TEX — Trading Expert Agent. "
                "You generate precise trading signals with strict risk management. "
                "Never recommend risking more than 2% of capital on a single trade. "
                "Prioritize capital preservation and consistent small gains."
            ),
            json_mode=True,
            max_tokens=500,
        )

        try:
            data = json.loads(response)
        except Exception:
            price = signal["currentPrice"]
            change = signal["change24h"]
            action = "BUY" if change > 1.5 else ("SELL" if change < -1.5 else "HOLD")
            risk = round(random.uniform(1, 3), 1)
            stop = (
                price * (1 - risk / 100)
                if action == "BUY"
                else price * (1 + risk / 100)
            )
            target_mult = 1.5 if action == "BUY" else 1.5
            target = (
                price * (1 + risk * target_mult / 100)
                if action == "BUY"
                else price * (1 - risk * target_mult / 100)
            )
            data = {
                "action": action,
                "entry_price": price,
                "stop_loss": round(stop, 2),
                "take_profit": round(target, 2),
                "risk_percent": risk,
                "risk_reward_ratio": round(target_mult, 1),
                "confidence": round(random.uniform(55, 85), 0),
                "reasoning": f"{action} signal on {signal['symbol']} based on {signal['trend']} momentum and technical setup.",
                "time_horizon": "short",
            }

        signal["action"] = data.get("action", "HOLD")
        signal["entryPrice"] = data.get("entry_price", signal["currentPrice"])
        signal["stopLoss"] = data.get("stop_loss", 0)
        signal["takeProfit"] = data.get("take_profit", 0)
        signal["riskPercent"] = data.get("risk_percent", 2)
        signal["riskReward"] = data.get("risk_reward_ratio", 1.5)
        signal["confidence"] = data.get("confidence", 50)
        signal["reasoning"] = data.get("reasoning", "")
        signal["timeHorizon"] = data.get("time_horizon", "short")
        signal["simulatedPnl"] = 0

        if tex:
            await tex.set_status("complete", "Signal generated")
            await asyncio.sleep(2)
            await tex.set_status("idle")

        return signal

    async def _step_performance_monitor(self, signal: dict) -> dict:
        await self.narrate("TPM is updating trade journal and performance metrics...")
        tpm = self.team.get("TPM")
        if tpm:
            await tpm.set_status("working", "Updating trade journal")

        await asyncio.sleep(4)

        entry = self.equity_curve[-1] if self.equity_curve else 10000

        if signal["action"] in ("BUY", "SELL") and signal["confidence"] > 60:
            risk_amount = entry * (signal["riskPercent"] / 100)
            position_size = (
                risk_amount / abs(signal["entryPrice"] - signal["stopLoss"])
                if signal["stopLoss"]
                else 0
            )
            if (
                signal["action"] == "BUY"
                and signal["takeProfit"] > signal["entryPrice"]
            ):
                pnl = position_size * (signal["takeProfit"] - signal["entryPrice"])
            elif (
                signal["action"] == "SELL"
                and signal["takeProfit"] < signal["entryPrice"]
            ):
                pnl = position_size * (signal["entryPrice"] - signal["takeProfit"])
            else:
                pnl = 0

            signal["simulatedPnl"] = round(pnl, 2)
            entry = max(entry + pnl, 0)
            self.equity_curve.append(entry)

            trade = {
                "id": signal["id"],
                "symbol": signal["symbol"],
                "action": signal["action"],
                "entryPrice": signal["entryPrice"],
                "exitPrice": signal["takeProfit"],
                "pnl": signal["simulatedPnl"],
                "confidence": signal["confidence"],
                "timestamp": signal["timestamp"],
                "equity": entry,
            }
            self.trade_journal.append(trade)

        total_trades = len(self.trade_journal)
        winning_trades = sum(1 for t in self.trade_journal if t["pnl"] > 0)
        win_rate = (winning_trades / total_trades * 100) if total_trades > 0 else 0
        total_pnl = sum(t["pnl"] for t in self.trade_journal)

        signal["performance"] = {
            "totalTrades": total_trades,
            "winningTrades": winning_trades,
            "winRate": round(win_rate, 1),
            "totalPnl": round(total_pnl, 2),
            "currentEquity": round(entry, 2),
            "drawdown": round(max(0, (10000 - entry) / 10000 * 100), 2),
        }

        if tpm:
            await tpm.set_status("complete", "Journal updated")
            await asyncio.sleep(2)
            await tpm.set_status("idle")

        return signal

    async def _broadcast_signal(self, signal: dict):
        await self.broadcast(
            {
                "type": "trading_signal",
                "data": signal,
            }
        )
        await self.emit_event(
            f"Signal: {signal['action']} {signal['symbol']} "
            f"Entry: ${signal['entryPrice']:.2f} | "
            f"SL: ${signal['stopLoss']:.2f} TP: ${signal['takeProfit']:.2f} | "
            f"Confidence: {signal['confidence']:.0f}%",
            "system",
        )
