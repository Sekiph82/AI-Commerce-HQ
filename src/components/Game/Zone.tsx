import { useRef, useState } from 'react'
import { useFrame, ThreeEvent } from '@react-three/fiber'
import { Float, Text, Html } from '@react-three/drei'
import * as THREE from 'three'
import type { Agent, ZoneId } from '../../types'

interface ZoneProps {
  id: ZoneId
  name: string
  icon: string
  color: string
  glowColor: string
  position: [number, number, number]
  description: string
  agents: Agent[]
  isActive: boolean
}

const PLATFORM_ICONS: Record<ZoneId, string> = {
  etsy: '🛒',
  fiverr: '💼',
  trading: '📈',
  youtube: '🎬',
  tiktok: '🎵',
}

export function Zone({ id, name, icon, color, glowColor, position, description, agents, isActive }: ZoneProps) {
  const groupRef = useRef<THREE.Group>(null)
  const ringRef = useRef<THREE.Mesh>(null)
  const platformRef = useRef<THREE.Mesh>(null)
  const [hovered, setHovered] = useState(false)
  const [selected, setSelected] = useState(false)

  const workingAgents = agents.filter(a => a.status === 'working').length
  const totalAgents = agents.length

  useFrame((state) => {
    const t = state.clock.elapsedTime

    if (ringRef.current) {
      ringRef.current.rotation.y = t * 0.5
      ringRef.current.rotation.z = t * 0.2
      const scale = hovered ? 1.15 : (selected ? 1.1 : 1)
      ringRef.current.scale.lerp(new THREE.Vector3(scale, scale, scale), 0.05)
    }

    if (platformRef.current) {
      platformRef.current.rotation.y = t * 0.15
    }

    if (groupRef.current) {
      groupRef.current.position.y = position[1] + Math.sin(t * 0.8 + position[0]) * 0.15
    }
  })

  const handleClick = (e: ThreeEvent<MouseEvent>) => {
    e.stopPropagation()
    setSelected(!selected)
  }

  return (
    <group
      ref={groupRef}
      position={position}
      onClick={handleClick}
      onPointerOver={() => setHovered(true)}
      onPointerOut={() => setHovered(false)}
    >
      <Float speed={1.5} rotationIntensity={0.1} floatIntensity={0.2}>
        <group>
          <mesh ref={ringRef}>
            <torusGeometry args={[2.2, 0.06, 16, 64]} />
            <meshStandardMaterial
              color={color}
              emissive={color}
              emissiveIntensity={hovered ? 3 : 1.5}
              transparent
              opacity={0.8}
            />
          </mesh>

          <mesh ref={platformRef} position={[0, -0.8, 0]}>
            <cylinderGeometry args={[1.8, 2.2, 0.4, 6]} />
            <meshStandardMaterial
              color={color}
              emissive={color}
              emissiveIntensity={0.3}
              metalness={0.6}
              roughness={0.3}
              transparent
              opacity={0.85}
            />
          </mesh>

          <mesh position={[0, -1.15, 0]}>
            <cylinderGeometry args={[1.6, 1.8, 0.15, 6]} />
            <meshStandardMaterial
              color={color}
              emissive={color}
              emissiveIntensity={0.5}
              metalness={0.9}
              roughness={0.1}
            />
          </mesh>

          <mesh position={[0, 0, 0]}>
            <sphereGeometry args={[0.5, 32, 32]} />
            <meshStandardMaterial
              color="#ffffff"
              emissive={color}
              emissiveIntensity={hovered ? 5 : 3}
              transparent
              opacity={0.9}
            />
          </mesh>

          <Text
            position={[0, 1.3, 0]}
            fontSize={0.35}
            color={color}
            anchorX="center"
            anchorY="middle"
          >
            {name}
          </Text>

          <Text
            position={[0, 0.9, 0]}
            fontSize={0.7}
            anchorX="center"
            anchorY="middle"
          >
            {icon}
          </Text>
        </group>
      </Float>

      <pointLight color={glowColor} intensity={hovered ? 4 : 2} distance={8} decay={2} />

      {(hovered || selected) && (
        <Html position={[0, 2.5, 0]} center>
          <div className="bg-gray-900/95 border border-gray-700 rounded-xl px-4 py-3 min-w-[200px] pointer-events-none">
            <div className="flex items-center gap-2 mb-1">
              <span className="text-2xl">{icon}</span>
              <span className="font-bold text-white">{name}</span>
              {isActive && (
                <span className="text-xs px-2 py-0.5 rounded-full bg-green-900/50 text-green-400 border border-green-700/50">
                  LIVE
                </span>
              )}
            </div>
            <div className="text-xs text-gray-400 mb-2 whitespace-pre-line">
              {description}
            </div>
            <div className="flex items-center gap-3 text-xs">
              <div className="flex items-center gap-1">
                <div className="w-2 h-2 rounded-full bg-blue-400" />
                <span className="text-gray-400">Agents: <span className="text-white font-bold">{totalAgents}</span></span>
              </div>
              {workingAgents > 0 && (
                <div className="flex items-center gap-1">
                  <div className="w-2 h-2 rounded-full bg-green-400 animate-pulse" />
                  <span className="text-green-400 font-bold">{workingAgents} working</span>
                </div>
              )}
            </div>
          </div>
        </Html>
      )}

      {agents.map((agent, i) => (
        <AgentOrbit
          key={agent.id}
          agent={agent}
          color={color}
          index={i}
          total={Math.max(agents.length, 1)}
        />
      ))}
    </group>
  )
}

function AgentOrbit({ agent, color, index, total }: {
  agent: Agent; color: string; index: number; total: number
}) {
  const ref = useRef<THREE.Mesh>(null)
  const angle = (index / total) * Math.PI * 2

  useFrame((state) => {
    if (ref.current) {
      const t = state.clock.elapsedTime
      ref.current.position.x = Math.cos(angle + t * 0.3) * 2.8
      ref.current.position.z = Math.sin(angle + t * 0.3) * 2.8
      ref.current.position.y = Math.sin(t * 2 + index) * 0.15

      ref.current.rotation.y = t * 2
    }
  })

  const statusColor = agent.status === 'working' ? '#22c55e' :
                      agent.status === 'blocked' ? '#ef4444' :
                      agent.status === 'complete' ? '#3b82f6' : '#6b7280'

  return (
    <mesh ref={ref}>
      <sphereGeometry args={[0.15, 16, 16]} />
      <meshStandardMaterial
        color={statusColor}
        emissive={statusColor}
        emissiveIntensity={agent.status === 'working' ? 3 : 1}
      />
    </mesh>
  )
}
