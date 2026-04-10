import { useAppStore } from '../../store/useAppStore'
import { RoomBase } from './RoomBase'
import { AgentDesk } from '../Agents/AgentDesk'

export function GMORoom() {
  const { rooms, agents, openApproval } = useAppStore()
  const room = rooms['gmo']
  if (!room) return null

  const gmoAgent = Object.values(agents).find((a) => a.label === 'GMO')

  return (
    <RoomBase
      title="Global Master Orchestrator"
      subtitle="System command center"
      color="#1f3a6e"
      isActive={true}
      badge="HQ"
    >
      <div className="flex flex-wrap gap-3 justify-center">
        {/* Main GMO desk */}
        <AgentDesk
          nameplate="Global Master Orchestrator"
          agentLabel="GMO"
          agent={gmoAgent}
          isNew={false}
        />

        {/* Approval desk */}
        <AgentDesk
          nameplate="HQ Approval Desk"
          agentLabel="APPR"
          isApprovalDesk
          onClick={() => openApproval('global')}
        />
      </div>

      {/* Platform status grid */}
      <div className="mt-3 grid grid-cols-3 gap-1.5">
        {[
          { name: 'Etsy', active: true },
          { name: 'Amazon', active: false },
          { name: 'eBay', active: false },
          { name: 'TikTok', active: false },
          { name: 'Instagram', active: false },
          { name: 'Website', active: false },
        ].map((p) => (
          <div
            key={p.name}
            className={`text-xs text-center py-1 px-2 rounded-md border ${
              p.active
                ? 'border-green-600/50 bg-green-900/20 text-green-300'
                : 'border-gray-700/50 bg-gray-800/20 text-gray-600'
            }`}
          >
            {p.name}
          </div>
        ))}
      </div>
    </RoomBase>
  )
}
