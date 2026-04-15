import { Canvas } from '@react-three/fiber'
import { OrbitControls, Stars, Environment, Float } from '@react-three/drei'
import { CentralCore } from './CentralCore'
import { Zone } from './Zone'
import { Agent3D } from './Agent3D'
import { MissionCard3D } from './MissionCard3D'
import { MoneyFlow } from './MoneyFlow'
import { GameHUD } from './GameHUD'
import { useAppStore } from '../../store/useAppStore'
import type { ZoneId } from '../../types'

const ZONE_CONFIG: Record<ZoneId, {
  name: string; icon: string; color: string; glowColor: string;
  position: [number, number, number]; description: string
}> = {
  etsy: {
    name: 'Etsy Planet', icon: '🛒', color: '#ff6b35', glowColor: '#ff8c5a',
    position: [-5, 1.5, 0], description: 'Print-on-Demand\nMarketplace'
  },
  fiverr: {
    name: 'Fiverr Station', icon: '💼', color: '#1dbf73', glowColor: '#2edd8a',
    position: [0, 1.5, -5], description: 'Freelance\nServices'
  },
  trading: {
    name: 'Trading Lab', icon: '📈', color: '#f59e0b', glowColor: '#fbbf24',
    position: [5, 1.5, 0], description: 'Crypto & Stock\nAutomation'
  },
  youtube: {
    name: 'Content Factory', icon: '🎬', color: '#ef4444', glowColor: '#f87171',
    position: [0, 1.5, 5], description: 'Faceless Video\nAutomation'
  },
  tiktok: {
    name: 'Viral Engine', icon: '🎵', color: '#ec4899', glowColor: '#f472b6',
    position: [-3, 0.3, -3], description: 'TikTok Shop\nAutomation'
  },
}

const CORE_AGENTS = ['GMO', 'ETMO']
const ZONE_AGENTS: Record<ZoneId, string[]> = {
  etsy: ['TRD', 'DES-1', 'DES-2', 'PRD-1', 'PRD-2', 'QA', 'POD', 'LST', 'VID', 'PER'],
  fiverr: ['FMO', 'FRD', 'FPR', 'FSP', 'FAQ', 'FDL'],
  trading: ['TMO', 'TSC', 'TAN', 'TEX', 'TPM'],
  youtube: ['YMO', 'YRS', 'YSC', 'YED', 'YTH'],
  tiktok: ['TTO', 'TRS', 'TSC', 'TPU'],
}

export function GameWorld() {
  const { agents, products } = useAppStore()

  const zoneAgents: Record<ZoneId, typeof agents[string][]> = {
    etsy: [],
    fiverr: [],
    trading: [],
    youtube: [],
    tiktok: [],
  }

  Object.values(agents).forEach(agent => {
    if (agent.platform === 'etsy') zoneAgents.etsy.push(agent)
    else if (agent.platform === 'fiverr') zoneAgents.fiverr.push(agent)
    else if (agent.platform === 'trading') zoneAgents.trading.push(agent)
    else if (agent.platform === 'youtube') zoneAgents.youtube.push(agent)
    else if (agent.platform === 'tiktok') zoneAgents.tiktok.push(agent)
  })

  const missions = Object.values(products).map(p => ({
    id: p.id,
    title: p.etsyTitle || p.name,
    description: p.niche || p.nicheReasoning || '',
    zoneId: 'etsy' as ZoneId,
    status: p.state === 'AWAITING_HUMAN_DECISION' ? 'pending' as const :
            p.state === 'ETSY_DRAFT_CREATED' ? 'completed' as const :
            p.state === 'REJECTED' ? 'failed' as const : 'active' as const,
    reward: Math.round((p.estimatedMargin || 35) * (p.price || 25) / 100),
  }))

  return (
    <div className="w-full h-full relative">
      <Canvas camera={{ position: [0, 12, 16], fov: 55 }} gl={{ antialias: true, alpha: false }}>
        <color attach="background" args={['#030308']} />
        <fog attach="fog" args={['#030308', 25, 60]} />

        <ambientLight intensity={0.15} />
        <pointLight position={[0, 8, 0]} intensity={2} color="#3b82f6" distance={20} />
        <pointLight position={[-5, 4, 0]} intensity={1} color="#ff6b35" distance={12} />
        <pointLight position={[5, 4, 0]} intensity={1} color="#f59e0b" distance={12} />
        <pointLight position={[0, 4, -5]} intensity={1} color="#1dbf73" distance={12} />
        <pointLight position={[0, 4, 5]} intensity={1} color="#ef4444" distance={12} />

        <Stars radius={80} depth={50} count={3000} factor={3} saturation={0.5} fade speed={0.5} />

        <CentralCore />

        {(Object.keys(ZONE_CONFIG) as ZoneId[]).map(zoneId => (
          <Zone
            key={zoneId}
            id={zoneId}
            name={ZONE_CONFIG[zoneId].name}
            icon={ZONE_CONFIG[zoneId].icon}
            color={ZONE_CONFIG[zoneId].color}
            glowColor={ZONE_CONFIG[zoneId].glowColor}
            position={ZONE_CONFIG[zoneId].position}
            description={ZONE_CONFIG[zoneId].description}
            agents={zoneAgents[zoneId]}
            isActive={zoneId === 'etsy'}
          />
        ))}

        {Object.values(agents).map(agent => (
          <Agent3D key={agent.id} agent={agent} />
        ))}

        {missions.map(mission => (
          <MissionCard3D key={mission.id} mission={mission} />
        ))}

        <MoneyFlow />

        <OrbitControls
          enablePan={false}
          minDistance={10}
          maxDistance={30}
          maxPolarAngle={Math.PI / 2.2}
          minPolarAngle={Math.PI / 8}
          autoRotate
          autoRotateSpeed={0.3}
        />

        <Environment preset="night" />
      </Canvas>

      <GameHUD />
    </div>
  )
}
