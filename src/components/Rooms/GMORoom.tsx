import { useRef, useEffect } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { useAppStore } from '../../store/useAppStore'
import { RoomBase } from './RoomBase'
import { AgentDesk } from '../Agents/AgentDesk'

export function GMORoom() {
  const { agents, talkingTable } = useAppStore()
  const gmoAgent = Object.values(agents).find((a) => a.label === 'GMO')
  const tableRef = useRef<HTMLDivElement>(null)

  // Auto-scroll talking table to newest message
  useEffect(() => {
    if (tableRef.current) {
      tableRef.current.scrollTop = 0
    }
  }, [talkingTable.length])

  return (
    <RoomBase
      title="Global Master Orchestrator"
      subtitle="HQ Command Center"
      color="#1f3a6e"
      isActive={true}
      badge="HQ"
    >
      <div className="flex flex-col gap-3 h-full">
        {/* GMO Desk — compact */}
        <div className="flex justify-center">
          <AgentDesk
            nameplate="Global Master Orchestrator"
            agentLabel="GMO"
            agent={gmoAgent}
            compact
          />
        </div>

        {/* Platform status — single row */}
        <div className="flex gap-1 flex-wrap justify-center">
          {[
            { name: 'Etsy', active: true },
            { name: 'Amazon', active: false },
            { name: 'eBay', active: false },
          ].map((p) => (
            <div
              key={p.name}
              className={`text-center py-0.5 px-1.5 rounded border ${
                p.active
                  ? 'border-green-600/50 bg-green-900/20 text-green-300'
                  : 'border-gray-700/40 bg-gray-800/20 text-gray-600'
              }`}
              style={{ fontSize: '8px' }}
            >
              {p.name}
            </div>
          ))}
        </div>

        {/* ── Talking Table / Operation Screen ─────────────────────── */}
        <div className="flex-1 flex flex-col min-h-0">
          <div
            className="flex items-center gap-1.5 mb-1.5 px-1"
          >
            <motion.div
              className="w-1.5 h-1.5 rounded-full bg-blue-400"
              animate={{ opacity: [1, 0.3, 1] }}
              transition={{ duration: 1.5, repeat: Infinity }}
            />
            <span style={{ fontSize: '10px', color: '#60a5fa', letterSpacing: '0.12em', fontFamily: 'monospace', fontWeight: 700 }}>
              OPERATION LOG
            </span>
          </div>

          <div
            ref={tableRef}
            className="flex-1 overflow-y-auto space-y-2 rounded-lg p-2"
            style={{
              background: 'rgba(0,0,0,0.4)',
              border: '1px solid rgba(37,99,235,0.2)',
              scrollbarWidth: 'none',
            }}
          >
            {talkingTable.length === 0 ? (
              <div className="flex flex-col items-center justify-center h-full py-6 gap-2">
                <motion.div
                  className="flex gap-1"
                  animate={{ opacity: [0.4, 1, 0.4] }}
                  transition={{ duration: 1.5, repeat: Infinity }}
                >
                  {[0,1,2].map(i => (
                    <div key={i} className="w-1.5 h-1.5 rounded-full bg-blue-500" />
                  ))}
                </motion.div>
                <span style={{ fontSize: '11px', color: '#6b7280', fontFamily: 'monospace' }}>
                  Waiting for agents...
                </span>
              </div>
            ) : (
              <AnimatePresence initial={false}>
                {talkingTable.map((entry, i) => (
                  <motion.div
                    key={`${entry.timestamp}-${i}`}
                    initial={{ opacity: 0, x: -8 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ duration: 0.3 }}
                    className="flex gap-2 items-start"
                    style={{
                      borderLeft: i === 0 ? '2px solid rgba(96,165,250,0.6)' : '2px solid transparent',
                      paddingLeft: '6px',
                    }}
                  >
                    <span
                      style={{
                        fontSize: '9px',
                        color: '#4b5563',
                        fontFamily: 'monospace',
                        flexShrink: 0,
                        marginTop: '2px',
                      }}
                    >
                      {new Date(entry.timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                    </span>
                    <span
                      style={{
                        fontSize: '11px',
                        color: i === 0 ? '#bfdbfe' : '#9ca3af',
                        fontFamily: 'monospace',
                        lineHeight: '1.5',
                        transition: 'color 0.3s',
                      }}
                    >
                      {entry.message}
                    </span>
                  </motion.div>
                ))}
              </AnimatePresence>
            )}
          </div>
        </div>
      </div>
    </RoomBase>
  )
}
