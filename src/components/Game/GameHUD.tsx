import { motion, AnimatePresence } from 'framer-motion'
import { useState } from 'react'
import { Wifi, WifiOff, Settings, Bell, Activity, Cpu, TrendingUp, DollarSign, Zap, ChevronDown, ChevronUp, Users, Target, Award, BarChart3, Trophy } from 'lucide-react'
import { useAppStore } from '../../store/useAppStore'
import { ApprovalPanel } from '../ApprovalDesk/ApprovalPanel'
import { SettingsPanel } from '../Settings/SettingsPanel'
import { TradingSignalsPanel } from '../TradingSignalsPanel'
import { AchievementsPanel } from '../AchievementsPanel'

export function GameHUD() {
  const { backendReady, config, agents, products, talkingTable, openApproval, toggleSettingsPanel, toggleStatusPanel, openTradingPanel, toggleAchievementsPanel } = useAppStore()
  const [minimized, setMinimized] = useState(false)
  const [revenueExpanded, setRevenueExpanded] = useState(false)
  const [agentsExpanded, setAgentsExpanded] = useState(false)

  const pendingApprovals = Object.values(products).filter(p => p.state === 'AWAITING_HUMAN_DECISION').length
  const workingAgents = Object.values(agents).filter(a => a.status === 'working').length
  const totalAgents = Object.values(agents).length

  const revenueData = {
    total: 0,
    etsy: 0,
    fiverr: 0,
    trading: 0,
    youtube: 0,
    tiktok: 0,
  }

  Object.values(products).forEach(p => {
    if (p.state === 'ETSY_DRAFT_CREATED') {
      revenueData.etsy += (p.price || 25) * (p.estimatedMargin || 35) / 100
      revenueData.total += (p.price || 25) * (p.estimatedMargin || 35) / 100
    }
  })

  const level = Math.floor(revenueData.total / 100) + 1
  const xp = revenueData.total % 100
  const xpToNext = 100

  const latestNarrations = talkingTable.slice(0, 3)

  return (
    <>
      <div className="absolute inset-0 pointer-events-none">
        <div className="absolute top-0 left-0 right-0 flex justify-between items-start p-4 gap-4">

          <div className="flex flex-col gap-3 pointer-events-auto">
            <StatusBar backendReady={backendReady} />
            <MiniRevenue revenue={revenueData.total} onExpand={() => setRevenueExpanded(true)} />
          </div>

          <div className="flex flex-col gap-3 pointer-events-auto">
            {pendingApprovals > 0 && (
              <motion.div
                animate={{ scale: [1, 1.05, 1] }}
                transition={{ duration: 1.5, repeat: Infinity }}
                className="flex items-center gap-2 bg-orange-900/80 backdrop-blur border border-orange-600/50 text-orange-300 px-4 py-2 rounded-xl cursor-pointer"
                onClick={() => openApproval('etsy')}
              >
                <Bell size={16} className="animate-pulse" />
                <span className="font-bold text-sm">{pendingApprovals} Awaiting Approval</span>
              </motion.div>
            )}

            <div className="flex items-center gap-2">
              <button
                onClick={toggleAchievementsPanel}
                className="p-2.5 rounded-xl bg-gray-900/80 backdrop-blur border border-purple-600/50 text-purple-400 hover:text-purple-300 transition-colors"
                title="Achievements"
              >
                <Trophy size={18} />
              </button>
              <button
                onClick={openTradingPanel}
                className="p-2.5 rounded-xl bg-gray-900/80 backdrop-blur border border-yellow-600/50 text-yellow-400 hover:text-yellow-300 transition-colors"
                title="Trading Signals"
              >
                <BarChart3 size={18} />
              </button>
              <button
                onClick={toggleStatusPanel}
                className="p-2.5 rounded-xl bg-gray-900/80 backdrop-blur border border-gray-700 text-gray-400 hover:text-white transition-colors"
                title="System Status"
              >
                <Activity size={18} />
              </button>
              <button
                onClick={toggleSettingsPanel}
                className="p-2.5 rounded-xl bg-gray-900/80 backdrop-blur border border-gray-700 text-gray-400 hover:text-white transition-colors"
                title="Settings"
              >
                <Settings size={18} />
              </button>
            </div>
          </div>
        </div>

        <div className="absolute bottom-0 left-0 right-0 p-4">
          <div className="flex gap-4 pointer-events-auto">
            <AgentPanel
              total={totalAgents}
              working={workingAgents}
              agents={agents}
              expanded={agentsExpanded}
              onToggle={() => setAgentsExpanded(!agentsExpanded)}
            />

            <NarrationPanel narrations={latestNarrations} />

            <LevelPanel level={level} xp={xp} xpToNext={xpToNext} />
          </div>
        </div>
      </div>

      <ApprovalPanel />
      <SettingsPanel />
      <TradingSignalsPanel />
      <AchievementsPanel />
    </>
  )
}

function StatusBar({ backendReady }: { backendReady: boolean }) {
  return (
    <motion.div
      initial={{ x: -50, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      className="flex items-center gap-3 bg-gray-900/80 backdrop-blur border border-gray-700 px-4 py-2 rounded-xl"
    >
      <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-blue-600 to-purple-700 flex items-center justify-center text-sm font-bold">
        HQ
      </div>
      <div>
        <div className="font-bold text-white text-sm">AI Commerce HQ</div>
        <div className={`flex items-center gap-1 text-xs ${backendReady ? 'text-green-400' : 'text-red-400'}`}>
          {backendReady ? <Wifi size={10} /> : <WifiOff size={10} />}
          <span>{backendReady ? 'Runtime Active' : 'Reconnecting...'}</span>
        </div>
      </div>
    </motion.div>
  )
}

function MiniRevenue({ revenue, onExpand }: { revenue: number; onExpand: () => void }) {
  return (
    <motion.div
      initial={{ x: -50, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      transition={{ delay: 0.1 }}
      className="bg-gray-900/80 backdrop-blur border border-gray-700 px-4 py-3 rounded-xl"
    >
      <div className="flex items-center gap-3 cursor-pointer" onClick={onExpand}>
        <div className="w-8 h-8 rounded-lg bg-yellow-900/50 border border-yellow-600/50 flex items-center justify-center">
          <DollarSign size={16} className="text-yellow-400" />
        </div>
        <div>
          <div className="text-xs text-gray-400">Total Revenue</div>
          <div className="text-xl font-bold text-yellow-400">
            ${revenue.toFixed(2)}
          </div>
        </div>
        <TrendingUp size={16} className="text-green-400" />
      </div>
    </motion.div>
  )
}

function AgentPanel({ total, working, agents, expanded, onToggle }: {
  total: number; working: number; agents: Record<string, unknown>; expanded: boolean; onToggle: () => void
}) {
  const agentList = Object.values(agents).slice(0, expanded ? 20 : 5)

  return (
    <div className="bg-gray-900/80 backdrop-blur border border-gray-700 rounded-xl overflow-hidden min-w-[200px]">
      <button
        className="w-full flex items-center justify-between px-4 py-3 hover:bg-gray-800/50 transition-colors"
        onClick={onToggle}
      >
        <div className="flex items-center gap-2">
          <Users size={14} className="text-blue-400" />
          <span className="font-bold text-white text-sm">Agents</span>
          <span className="text-xs text-gray-500">({working}/{total})</span>
        </div>
        {expanded ? <ChevronDown size={14} className="text-gray-400" /> : <ChevronUp size={14} className="text-gray-400" />}
      </button>

      <AnimatePresence>
        {expanded && (
          <motion.div
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: 'auto', opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            className="border-t border-gray-800"
          >
            <div className="p-2 space-y-1 max-h-[200px] overflow-y-auto">
              {agentList.map((agent: any) => (
                <div key={agent.id} className="flex items-center gap-2 px-2 py-1 rounded hover:bg-gray-800/50">
                  <div className={`w-2 h-2 rounded-full ${
                    agent.status === 'working' ? 'bg-green-400 animate-pulse' :
                    agent.status === 'blocked' ? 'bg-red-400' :
                    agent.status === 'complete' ? 'bg-blue-400' : 'bg-gray-600'
                  }`} />
                  <span className="text-xs font-bold text-gray-300">{agent.label}</span>
                  <span className="text-xs text-gray-500 flex-1 truncate">{agent.currentTask || '—'}</span>
                </div>
              ))}
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  )
}

function NarrationPanel({ narrations }: { narrations: Array<{ message: string; timestamp: number }> }) {
  return (
    <div className="bg-gray-900/80 backdrop-blur border border-gray-700 rounded-xl px-4 py-3 min-w-[300px] max-w-[400px]">
      <div className="flex items-center gap-2 mb-2">
        <Cpu size={14} className="text-purple-400" />
        <span className="font-bold text-white text-sm">AI Narration</span>
      </div>
      <div className="space-y-1.5 max-h-[80px] overflow-y-auto">
        {narrations.length === 0 ? (
          <div className="text-xs text-gray-500 italic">Waiting for agent narrations...</div>
        ) : (
          narrations.map((n, i) => (
            <div key={n.timestamp} className={`text-xs ${i === 0 ? 'text-gray-200' : 'text-gray-500'}`}>
              <span className="text-gray-600 mr-2">{new Date(n.timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}</span>
              {n.message?.slice(0, 80)}
            </div>
          ))
        )}
      </div>
    </div>
  )
}

function LevelPanel({ level, xp, xpToNext }: { level: number; xp: number; xpToNext: number }) {
  const progress = (xp / xpToNext) * 100

  return (
    <div className="bg-gray-900/80 backdrop-blur border border-gray-700 rounded-xl px-4 py-3 min-w-[180px]">
      <div className="flex items-center gap-3 mb-2">
        <div className="w-8 h-8 rounded-lg bg-purple-900/50 border border-purple-600/50 flex items-center justify-center">
          <Award size={16} className="text-purple-400" />
        </div>
        <div>
          <div className="flex items-center gap-1.5">
            <span className="font-bold text-white text-sm">Level</span>
            <span className="text-xl font-black text-purple-400">{level}</span>
          </div>
        </div>
      </div>

      <div className="w-full bg-gray-700 rounded-full h-2 overflow-hidden">
        <motion.div
          className="h-full rounded-full bg-gradient-to-r from-purple-600 to-blue-500"
          initial={{ width: 0 }}
          animate={{ width: `${progress}%` }}
          transition={{ duration: 1, ease: 'easeOut' }}
        />
      </div>
      <div className="text-xs text-gray-500 mt-1 text-right">
        {xp} / {xpToNext} XP
      </div>

      <div className="flex items-center gap-1 mt-2 text-xs text-gray-500">
        <Zap size={12} className="text-yellow-400" />
        <span>Generating revenue earns XP</span>
      </div>
    </div>
  )
}
