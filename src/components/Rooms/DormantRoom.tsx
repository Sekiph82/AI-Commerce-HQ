import { RoomBase } from './RoomBase'

interface Props {
  id: string
  name: string
  color: string
  icon: string
}

export function DormantRoom({ id, name, color, icon }: Props) {
  const rooms: Record<string, { label: string; agent: string }> = {
    amazon: { label: 'Amazon Master Orchestrator', agent: 'AMO' },
    ebay: { label: 'eBay Master Orchestrator', agent: 'EBMO' },
    tiktok: { label: 'TikTok Master Orchestrator', agent: 'TTMO' },
    instagram: { label: 'IG/FB Master Orchestrator', agent: 'IGMO' },
    website: { label: 'Website Master Orchestrator', agent: 'WSMO' },
  }
  const info = rooms[id] || { label: 'Master Orchestrator', agent: 'MO' }

  return (
    <RoomBase title={name} color={color} isActive={false}>
      <div className="flex flex-col items-center justify-center h-full gap-3 opacity-40">
        <div className="text-3xl">{icon}</div>
        <div className="text-center">
          <div className="text-sm font-bold text-gray-400">{info.agent}</div>
          <div className="text-xs text-gray-600">{info.label}</div>
        </div>
        <div className="flex gap-1">
          {[...Array(3)].map((_, i) => (
            <div key={i} className="w-12 h-8 rounded bg-gray-800/50 border border-gray-700/50" />
          ))}
        </div>
      </div>
    </RoomBase>
  )
}
