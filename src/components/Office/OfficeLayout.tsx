import { useEffect } from 'react'
import { motion } from 'framer-motion'
import { Activity, Settings, Bell, Wifi, WifiOff } from 'lucide-react'
import { useAppStore } from '../../store/useAppStore'
import { useWebSocket } from '../../hooks/useWebSocket'
import { GMORoom } from '../Rooms/GMORoom'
import { EtsyRoom } from '../Rooms/EtsyRoom'
import { DormantRoom } from '../Rooms/DormantRoom'
import { ApprovalPanel } from '../ApprovalDesk/ApprovalPanel'
import { SystemStatus } from '../StatusPanel/SystemStatus'
import { SettingsPanel } from '../Settings/SettingsPanel'

export function OfficeLayout() {
  useWebSocket()

  const { backendReady, toggleStatusPanel, toggleSettingsPanel, config, products, fetchInitialState } = useAppStore()

  useEffect(() => {
    if (backendReady) {
      fetchInitialState()
    }
  }, [backendReady, fetchInitialState])

  // Auto-open settings once on first launch if no API keys are saved yet
  useEffect(() => {
    const hasKeys = config?.openaiKey || config?.etsyApiKey
    if (!hasKeys) {
      const timer = setTimeout(() => toggleSettingsPanel(), 1200)
      return () => clearTimeout(timer)
    }
  }, []) // eslint-disable-line react-hooks/exhaustive-deps

  const pendingApprovals = Object.values(products).filter(
    (p) => p.state === 'AWAITING_HUMAN_DECISION'
  ).length

  return (
    <div className="fixed inset-0 flex flex-col bg-office-bg">
      {/* Top bar */}
      <div className="flex items-center justify-between px-4 py-2 bg-office-floor border-b border-gray-800 flex-shrink-0">
        <div className="flex items-center gap-3">
          <div className="flex items-center gap-2">
            <div className="w-6 h-6 rounded bg-gradient-to-br from-blue-600 to-purple-700 flex items-center justify-center text-xs">
              🏢
            </div>
            <span className="font-bold text-white text-sm">AI Commerce HQ</span>
          </div>
          <div className={`flex items-center gap-1 text-xs ${backendReady ? 'text-green-400' : 'text-red-400'}`}>
            {backendReady ? <Wifi size={12} /> : <WifiOff size={12} />}
            <span>{backendReady ? 'Runtime Active' : 'Reconnecting...'}</span>
          </div>
        </div>

        <div className="flex items-center gap-2">
          {pendingApprovals > 0 && (
            <motion.div
              animate={{ scale: [1, 1.1, 1] }}
              transition={{ duration: 1.5, repeat: Infinity }}
              className="flex items-center gap-1.5 bg-orange-700/30 border border-orange-600/50 text-orange-300 text-xs px-3 py-1 rounded-full cursor-pointer"
              onClick={() => useAppStore.getState().openApproval('etsy')}
            >
              <Bell size={12} />
              {pendingApprovals} awaiting approval
            </motion.div>
          )}

          <button
            onClick={toggleStatusPanel}
            className="p-2 rounded-lg hover:bg-gray-700 text-gray-400 hover:text-white transition-colors"
            title="System Status"
          >
            <Activity size={16} />
          </button>
          <button
            onClick={toggleSettingsPanel}
            className="p-2 rounded-lg hover:bg-gray-700 text-gray-400 hover:text-white transition-colors"
            title="Settings"
          >
            <Settings size={16} />
          </button>
        </div>
      </div>

      {/* Office floor */}
      <div className="flex-1 overflow-auto p-3">
        <div className="h-full min-h-0">
          {/* Floor plan label */}
          <div className="text-xs text-gray-600 font-mono mb-3 flex items-center gap-2">
            <div className="h-px flex-1 bg-gray-800" />
            AI COMMERCE HQ — FLOOR PLAN
            <div className="h-px flex-1 bg-gray-800" />
          </div>

          {/* Top row: GMO (350px) + Etsy (1fr) — same height via CSS grid stretch */}
          <div className="grid gap-3" style={{ gridTemplateColumns: '350px 1fr' }}>
            <GMORoom />
            <EtsyRoom />
          </div>

          {/* Bottom row: all 5 dormant rooms at equal width (~226px each) */}
          <div className="grid grid-cols-5 gap-3 mt-3">
            <DormantRoom id="amazon"    name="Amazon Operations"     color="#1a3a2a" icon="📦" />
            <DormantRoom id="ebay"      name="eBay Operations"       color="#1a1f3a" icon="🔨" />
            <DormantRoom id="tiktok"    name="TikTok Shop"           color="#2d1a3a" icon="🎵" />
            <DormantRoom id="instagram" name="Instagram / Facebook"  color="#3a1a2d" icon="📸" />
            <DormantRoom id="website"   name="Website Store"         color="#1a2a3a" icon="🌐" />
          </div>
        </div>
      </div>

      {/* Panels */}
      <ApprovalPanel />
      <SystemStatus />
      <SettingsPanel />
    </div>
  )
}
