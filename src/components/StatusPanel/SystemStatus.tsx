import { motion, AnimatePresence } from 'framer-motion'
import { X, Activity, Wifi, WifiOff } from 'lucide-react'
import { useAppStore } from '../../store/useAppStore'

export function SystemStatus() {
  const { statusPanelOpen, toggleStatusPanel, events, agents, backendReady } = useAppStore()

  const agentList = Object.values(agents)
  const workingCount = agentList.filter((a) => a.status === 'working').length
  const idleCount = agentList.filter((a) => a.status === 'idle').length

  return (
    <AnimatePresence>
      {statusPanelOpen && (
        <>
          <motion.div
            className="fixed inset-0 bg-black/40 z-30"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            onClick={toggleStatusPanel}
          />
          <motion.div
            className="fixed left-0 top-0 bottom-0 w-80 bg-office-floor border-r border-gray-700 z-40 flex flex-col"
            initial={{ x: '-100%' }}
            animate={{ x: 0 }}
            exit={{ x: '-100%' }}
            transition={{ type: 'spring', stiffness: 300, damping: 30 }}
          >
            <div className="p-4 border-b border-gray-700 flex items-center justify-between">
              <h2 className="font-bold text-white flex items-center gap-2">
                <Activity size={16} className="text-blue-400" />
                System Status
              </h2>
              <button onClick={toggleStatusPanel} className="p-1 rounded hover:bg-gray-700 text-gray-400">
                <X size={16} />
              </button>
            </div>

            {/* Connection status */}
            <div className="p-3 m-3 rounded-lg bg-gray-800 flex items-center gap-2">
              {backendReady ? (
                <Wifi size={14} className="text-green-400" />
              ) : (
                <WifiOff size={14} className="text-red-400" />
              )}
              <span className={`text-sm font-medium ${backendReady ? 'text-green-400' : 'text-red-400'}`}>
                Runtime {backendReady ? 'Connected' : 'Disconnected'}
              </span>
            </div>

            {/* Agent stats */}
            <div className="px-3 grid grid-cols-2 gap-2 mb-3">
              <div className="bg-gray-800 rounded-lg p-3 text-center">
                <div className="text-2xl font-bold text-green-400">{workingCount}</div>
                <div className="text-xs text-gray-400">Working</div>
              </div>
              <div className="bg-gray-800 rounded-lg p-3 text-center">
                <div className="text-2xl font-bold text-blue-400">{idleCount}</div>
                <div className="text-xs text-gray-400">Idle</div>
              </div>
            </div>

            {/* Agent list */}
            <div className="px-3 mb-3">
              <h3 className="text-xs font-semibold text-gray-400 uppercase mb-2">Active Agents</h3>
              <div className="space-y-1 max-h-32 overflow-y-auto">
                {agentList.map((agent) => (
                  <div key={agent.id} className="flex items-center justify-between text-xs p-2 rounded bg-gray-800">
                    <span className="font-mono text-gray-300">{agent.label}</span>
                    <div className="flex items-center gap-1">
                      <div
                        className={`w-1.5 h-1.5 rounded-full ${
                          agent.status === 'working' ? 'bg-green-400' :
                          agent.status === 'blocked' ? 'bg-red-400' : 'bg-blue-400'
                        }`}
                      />
                      <span className="text-gray-500">{agent.status}</span>
                    </div>
                  </div>
                ))}
                {agentList.length === 0 && (
                  <div className="text-gray-600 text-xs italic">No agents yet</div>
                )}
              </div>
            </div>

            {/* Event log */}
            <div className="flex-1 overflow-hidden flex flex-col px-3 pb-3">
              <h3 className="text-xs font-semibold text-gray-400 uppercase mb-2">Event Log</h3>
              <div className="flex-1 overflow-y-auto space-y-1">
                {events.map((event) => (
                  <motion.div
                    key={event.id}
                    initial={{ opacity: 0, x: -10 }}
                    animate={{ opacity: 1, x: 0 }}
                    className="text-xs p-2 rounded bg-gray-800"
                  >
                    <div className="flex items-center justify-between gap-2 mb-0.5">
                      <span
                        className={`font-mono font-semibold ${
                          event.type === 'error' ? 'text-red-400' :
                          event.type === 'agent_created' ? 'text-blue-400' : 'text-gray-400'
                        }`}
                      >
                        [{event.type.toUpperCase().slice(0, 6)}]
                      </span>
                      <span className="text-gray-600 flex-shrink-0">
                        {new Date(event.timestamp).toLocaleTimeString()}
                      </span>
                    </div>
                    <p className="text-gray-300 leading-tight">{event.message}</p>
                  </motion.div>
                ))}
                {events.length === 0 && (
                  <div className="text-gray-600 text-xs italic">No events yet</div>
                )}
              </div>
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  )
}
