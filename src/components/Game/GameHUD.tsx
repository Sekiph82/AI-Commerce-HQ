import { useState } from 'react'
import { Wifi, WifiOff, Activity, Cpu, TrendingUp, DollarSign, Users, Award, BarChart3, Trophy } from 'lucide-react'
import { useAppStore } from '../../store/useAppStore'

export function GameHUD() {
  const { backendReady, agents, products, talkingTable } = useAppStore()
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
    <div className="absolute inset-0 pointer-events-none">
      <div className="absolute top-0 left-0 right-0 flex justify-between items-start p-4 gap-4">
        <div className="flex flex-col gap-3 pointer-events-auto">
          <div className="flex items-center gap-3 bg-gray-900/80 backdrop-blur border border-gray-700 px-4 py-2 rounded-xl">
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
          </div>

          <div className="bg-gray-900/80 backdrop-blur border border-gray-700 px-4 py-3 rounded-xl">
            <div className="flex items-center gap-3">
              <div className="w-8 h-8 rounded-lg bg-yellow-900/50 border border-yellow-600/50 flex items-center justify-center">
                <DollarSign size={16} className="text-yellow-400" />
              </div>
              <div>
                <div className="text-xs text-gray-400">Total Revenue</div>
                <div className="text-xl font-bold text-yellow-400">
                  ${revenueData.total.toFixed(2)}
                </div>
              </div>
              <TrendingUp size={16} className="text-green-400" />
            </div>
          </div>
        </div>

        <div className="flex flex-col gap-3 pointer-events-auto">
          {pendingApprovals > 0 && (
            <div className="flex items-center gap-2 bg-orange-900/80 backdrop-blur border border-orange-600/50 text-orange-300 px-4 py-2 rounded-xl">
              <Activity size={16} className="animate-pulse" />
              <span className="font-bold text-sm">{pendingApprovals} Awaiting Approval</span>
            </div>
          )}
        </div>
      </div>

      <div className="absolute bottom-0 left-0 right-0 p-4">
        <div className="flex gap-4 pointer-events-auto">
          <div className="bg-gray-900/80 backdrop-blur border border-gray-700 rounded-xl overflow-hidden min-w-[200px]">
            <button
              className="w-full flex items-center justify-between px-4 py-3 hover:bg-gray-800/50 transition-colors"
              onClick={() => setAgentsExpanded(!agentsExpanded)}
            >
              <div className="flex items-center gap-2">
                <Users size={14} className="text-blue-400" />
                <span className="font-bold text-white text-sm">Agents</span>
                <span className="text-xs text-gray-500">({workingAgents}/{totalAgents})</span>
              </div>
              <span className="text-gray-400">{agentsExpanded ? '▼' : '▲'}</span>
            </button>

            {agentsExpanded && (
              <div className="border-t border-gray-800">
                <div className="p-2 space-y-1 max-h-[200px] overflow-y-auto">
                  {Object.values(agents).map((agent) => (
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
                  {totalAgents === 0 && (
                    <div className="text-xs text-gray-500 p-2">No agents yet...</div>
                  )}
                </div>
              </div>
            )}
          </div>

          <div className="bg-gray-900/80 backdrop-blur border border-gray-700 rounded-xl px-4 py-3 min-w-[300px] max-w-[400px]">
            <div className="flex items-center gap-2 mb-2">
              <Cpu size={14} className="text-purple-400" />
              <span className="font-bold text-white text-sm">AI Narration</span>
            </div>
            <div className="space-y-1.5 max-h-[80px] overflow-y-auto">
              {latestNarrations.length === 0 ? (
                <div className="text-xs text-gray-500 italic">Waiting for agent narrations...</div>
              ) : (
                latestNarrations.map((n, i) => (
                  <div key={n.timestamp} className={`text-xs ${i === 0 ? 'text-gray-200' : 'text-gray-500'}`}>
                    <span className="text-gray-600 mr-2">{new Date(n.timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}</span>
                    {n.message?.slice(0, 80)}
                  </div>
                ))
              )}
            </div>
          </div>

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
              <div
                className="h-full rounded-full bg-gradient-to-r from-purple-600 to-blue-500"
                style={{ width: `${(xp / xpToNext) * 100}%` }}
              />
            </div>
            <div className="text-xs text-gray-500 mt-1 text-right">
              {xp} / {xpToNext} XP
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}