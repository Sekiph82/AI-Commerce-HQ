import { motion, AnimatePresence } from 'framer-motion'
import { X, TrendingUp, TrendingDown, Target, AlertTriangle, Activity, BarChart3, Clock, DollarSign, Zap } from 'lucide-react'
import { useAppStore } from '../store/useAppStore'
import { useEffect, useState } from 'react'

interface TradingSignal {
  id: string
  symbol: string
  action: string
  entryPrice: number
  stopLoss: number
  takeProfit: number
  riskReward: number
  confidence: number
  reasoning: string
  timeHorizon: string
  timestamp: number
  performance?: {
    totalTrades: number
    winningTrades: number
    winRate: number
    totalPnl: number
    currentEquity: number
    drawdown: number
  }
}

const MOCK_SIGNALS: TradingSignal[] = [
  {
    id: '1',
    symbol: 'BTC/USD',
    action: 'BUY',
    entryPrice: 67240,
    stopLoss: 65850,
    takeProfit: 70100,
    riskReward: 2.5,
    confidence: 78,
    reasoning: 'Bullish momentum on daily chart. RSI showing oversold conditions. Volume confirmation above average.',
    timeHorizon: 'short',
    timestamp: Date.now() - 3600000,
    performance: {
      totalTrades: 47,
      winningTrades: 31,
      winRate: 66,
      totalPnl: 2340.50,
      currentEquity: 12340.50,
      drawdown: 4.2,
    },
  },
  {
    id: '2',
    symbol: 'ETH/USD',
    action: 'BUY',
    entryPrice: 3456,
    stopLoss: 3380,
    takeProfit: 3620,
    riskReward: 2.0,
    confidence: 72,
    reasoning: 'Strong support at $3400. MACD crossing bullish. Institutional interest increasing.',
    timeHorizon: 'medium',
    timestamp: Date.now() - 7200000,
    performance: {
      totalTrades: 47,
      winningTrades: 31,
      winRate: 66,
      totalPnl: 2340.50,
      currentEquity: 12340.50,
      drawdown: 4.2,
    },
  },
]

export function TradingSignalsPanel() {
  const { tradingPanelOpen, closeTradingPanel } = useAppStore()
  const [signals, setSignals] = useState<TradingSignal[]>(MOCK_SIGNALS)
  const [selectedSignal, setSelectedSignal] = useState<TradingSignal | null>(null)

  const latestSignal = signals[0]

  return (
    <AnimatePresence>
      {tradingPanelOpen && (
        <>
          <motion.div
            className="fixed inset-0 bg-black/60 z-40"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            onClick={closeTradingPanel}
          />

          <motion.div
            className="fixed right-0 top-0 bottom-0 w-[520px] bg-[#0a0a14] border-l border-gray-700 z-50 flex flex-col"
            initial={{ x: '100%' }}
            animate={{ x: 0 }}
            exit={{ x: '100%' }}
            transition={{ type: 'spring', stiffness: 300, damping: 30 }}
          >
            <div className="p-4 border-b border-gray-700 flex items-center justify-between" style={{ background: '#f59e0b15' }}>
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 rounded-xl flex items-center justify-center" style={{ background: '#f59e0b25' }}>
                  <BarChart3 size={20} style={{ color: '#f59e0b' }} />
                </div>
                <div>
                  <h2 className="text-lg font-bold text-white">Trading Lab</h2>
                  <p className="text-xs text-gray-400 mt-0.5">
                    AI-generated trading signals — {signals.length} active
                  </p>
                </div>
              </div>
              <button
                onClick={closeTradingPanel}
                className="p-2 rounded-lg hover:bg-gray-700 text-gray-400 hover:text-white transition-colors"
              >
                <X size={18} />
              </button>
            </div>

            {latestSignal && (
              <div className="p-4 border-b border-gray-700" style={{ background: '#f59e0b08' }}>
                <div className="flex items-center justify-between mb-3">
                  <div className="flex items-center gap-2">
                    <span className={`text-lg font-bold ${latestSignal.action === 'BUY' ? 'text-green-400' : 'text-red-400'}`}>
                      {latestSignal.action === 'BUY' ? <TrendingUp size={20} /> : <TrendingDown size={20} />}
                    </span>
                    <span className="text-2xl font-black text-white">{latestSignal.symbol}</span>
                  </div>
                  <div className="flex items-center gap-2 px-3 py-1 rounded-full" style={{ background: latestSignal.confidence > 70 ? '#22c55e20' : '#f59e0b20' }}>
                    <Zap size={12} style={{ color: latestSignal.confidence > 70 ? '#22c55e' : '#f59e0b' }} />
                    <span className="text-sm font-bold" style={{ color: latestSignal.confidence > 70 ? '#22c55e' : '#f59e0b' }}>
                      {latestSignal.confidence}%
                    </span>
                  </div>
                </div>

                <div className="grid grid-cols-3 gap-2 text-xs">
                  <div className="bg-gray-800/50 rounded-lg p-2">
                    <div className="text-gray-500 mb-1">Entry</div>
                    <div className="font-bold text-white">${latestSignal.entryPrice.toLocaleString()}</div>
                  </div>
                  <div className="bg-gray-800/50 rounded-lg p-2">
                    <div className="text-gray-500 mb-1">Stop Loss</div>
                    <div className="font-bold text-red-400">${latestSignal.stopLoss.toLocaleString()}</div>
                  </div>
                  <div className="bg-gray-800/50 rounded-lg p-2">
                    <div className="text-gray-500 mb-1">Take Profit</div>
                    <div className="font-bold text-green-400">${latestSignal.takeProfit.toLocaleString()}</div>
                  </div>
                </div>

                <div className="mt-3 flex items-center gap-2">
                  <Clock size={12} className="text-gray-500" />
                  <span className="text-xs text-gray-400">Signal generated: {new Date(latestSignal.timestamp).toLocaleTimeString()}</span>
                </div>
              </div>
            )}

            {latestSignal?.performance && (
              <div className="p-4 border-b border-gray-700">
                <div className="text-xs font-semibold text-gray-400 uppercase mb-3">Performance Stats</div>
                <div className="grid grid-cols-4 gap-2">
                  <div className="text-center">
                    <div className="text-xl font-bold text-white">{latestSignal.performance.totalTrades}</div>
                    <div className="text-xs text-gray-500">Trades</div>
                  </div>
                  <div className="text-center">
                    <div className="text-xl font-bold text-green-400">{latestSignal.performance.winRate}%</div>
                    <div className="text-xs text-gray-500">Win Rate</div>
                  </div>
                  <div className="text-center">
                    <div className="text-xl font-bold text-green-400">${latestSignal.performance.totalPnl.toFixed(0)}</div>
                    <div className="text-xs text-gray-500">Total P&L</div>
                  </div>
                  <div className="text-center">
                    <div className="text-xl font-bold text-white">${latestSignal.performance.currentEquity.toFixed(0)}</div>
                    <div className="text-xs text-gray-500">Equity</div>
                  </div>
                </div>
              </div>
            )}

            <div className="flex-1 overflow-y-auto p-4">
              <div className="text-xs font-semibold text-gray-400 uppercase mb-3">Signal History</div>
              <div className="space-y-2">
                {signals.map((signal) => (
                  <motion.div
                    key={signal.id}
                    className="p-3 rounded-lg border border-gray-700 bg-gray-800/30 cursor-pointer hover:bg-gray-800/50 transition-colors"
                    onClick={() => setSelectedSignal(signal)}
                    whileHover={{ scale: 1.01 }}
                  >
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-2">
                        <span className={`font-bold ${signal.action === 'BUY' ? 'text-green-400' : 'text-red-400'}`}>
                          {signal.action}
                        </span>
                        <span className="font-bold text-white">{signal.symbol}</span>
                      </div>
                      <div className="flex items-center gap-2">
                        <span className="text-xs text-gray-500">{new Date(signal.timestamp).toLocaleTimeString()}</span>
                        <span className="text-xs px-2 py-0.5 rounded" style={{ 
                          background: signal.confidence > 70 ? '#22c55e20' : '#f59e0b20',
                          color: signal.confidence > 70 ? '#22c55e' : '#f59e0b'
                        }}>
                          {signal.confidence}%
                        </span>
                      </div>
                    </div>
                    <div className="text-xs text-gray-500 mt-1">
                      Entry: ${signal.entryPrice.toLocaleString()} → TP: ${signal.takeProfit.toLocaleString()}
                    </div>
                  </motion.div>
                ))}
              </div>
            </div>

            <div className="p-4 border-t border-gray-700 bg-gray-900/50">
              <p className="text-xs text-gray-500 text-center">
                ⚠️ Trading signals are for analysis purposes only. Always do your own research before trading.
              </p>
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  )
}