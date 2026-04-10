import { motion, AnimatePresence } from 'framer-motion'
import { useAppStore } from '../../store/useAppStore'
import { RoomBase } from './RoomBase'
import { AgentDesk } from '../Agents/AgentDesk'

export function EtsyRoom() {
  const { rooms, agents, openApproval } = useAppStore()
  const room = rooms['etsy']
  if (!room) return null

  const etmoAgent = Object.values(agents).find((a) => a.label === 'ETMO')
  const subAgents = Object.values(agents).filter(
    (a) => a.platform === 'etsy' && a.label !== 'ETMO'
  )

  // All desks except approval
  const staticDesks = room.desks.filter((d) => !d.isApprovalDesk)
  const approvalDesk = room.desks.find((d) => d.isApprovalDesk)

  // Dynamic desks from sub-agents
  const dynamicDesks = subAgents.map((agent) => ({
    id: agent.deskId || agent.id,
    nameplate: agent.role,
    agentLabel: agent.label,
    agent,
    isNew: Date.now() - agent.createdAt < 10000,
  }))

  const pendingCount = Object.values(useAppStore.getState().products).filter(
    (p) => p.state === 'AWAITING_HUMAN_DECISION' && p.platform === 'etsy'
  ).length

  return (
    <RoomBase
      title="Etsy Operations"
      subtitle="v1 — Primary active platform"
      color="#ff6b35"
      isActive={true}
      badge="LIVE"
    >
      <div className="flex flex-col gap-3 h-full">
        {/* Static desks row */}
        <div className="flex flex-wrap gap-2">
          {staticDesks.map((desk) => (
            <AgentDesk
              key={desk.id}
              nameplate={desk.nameplate}
              agentLabel={desk.agentLabel}
              agent={desk.agentId === 'etmo' ? etmoAgent : undefined}
            />
          ))}
        </div>

        {/* Dynamic sub-agent desks */}
        {dynamicDesks.length > 0 && (
          <div>
            <div className="text-xs text-gray-500 mb-2 flex items-center gap-2">
              <div className="w-4 h-px bg-gray-600" />
              Sub-agents
              <div className="flex-1 h-px bg-gray-600" />
              <span className="text-blue-400">{dynamicDesks.length} active</span>
            </div>
            <div className="flex flex-wrap gap-2">
              <AnimatePresence>
                {dynamicDesks.map((d) => (
                  <AgentDesk
                    key={d.id}
                    nameplate={d.nameplate}
                    agentLabel={d.agentLabel}
                    agent={d.agent}
                    isNew={d.isNew}
                  />
                ))}
              </AnimatePresence>
            </div>
          </div>
        )}

        {dynamicDesks.length === 0 && (
          <div className="text-xs text-gray-600 italic text-center py-2">
            Sub-agents will appear here as ETMO creates them...
          </div>
        )}

        {/* Approval desk */}
        {approvalDesk && (
          <div className="mt-auto pt-2 border-t border-gray-700/50">
            <div className="flex items-center gap-2 justify-between">
              <AgentDesk
                nameplate="Etsy Approval Desk"
                agentLabel="APPR"
                isApprovalDesk
                onClick={() => openApproval('etsy')}
              />
              {pendingCount > 0 && (
                <motion.div
                  className="flex flex-col items-center"
                  animate={{ scale: [1, 1.05, 1] }}
                  transition={{ duration: 2, repeat: Infinity }}
                >
                  <div className="bg-orange-600 text-white text-sm font-bold rounded-full w-8 h-8 flex items-center justify-center">
                    {pendingCount}
                  </div>
                  <span className="text-xs text-orange-400 mt-1">Awaiting</span>
                </motion.div>
              )}
            </div>
          </div>
        )}
      </div>
    </RoomBase>
  )
}
