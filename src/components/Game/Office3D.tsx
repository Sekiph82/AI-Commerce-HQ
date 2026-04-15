import { useRef, useMemo, useState, useEffect } from 'react'
import { useFrame } from '@react-three/fiber'
import { Float, RoundedBox, MeshTransmissionMaterial, Text, Html } from '@react-three/drei'
import * as THREE from 'three'
import { useAppStore } from '../../store/useAppStore'
import type { Agent } from '../../types'

const WALL_COLOR = '#1a1a2e'
const FLOOR_COLOR = '#16213e'
const DESK_COLOR = '#0f3460'
const MONITOR_COLOR = '#1a1a2e'

const AGENT_COLORS: Record<string, string> = {
  'GMO': '#3b82f6',
  'ETMO': '#ff6b35',
  'TRD': '#f59e0b',
  'DES-1': '#6366f1',
  'DES-2': '#8b5cf6',
  'PRD-1': '#06b6d4',
  'PRD-2': '#0ea5e9',
  'QA': '#ef4444',
  'POD': '#10b981',
  'LST': '#f97316',
  'VID': '#ec4899',
  'PER': '#a78bfa',
}

function Floor() {
  const floorRef = useRef<THREE.Mesh>(null)
  
  return (
    <group>
      <mesh rotation={[-Math.PI / 2, 0, 0]} position={[0, -0.01, 0]} receiveShadow>
        <planeGeometry args={[40, 40]} />
        <meshStandardMaterial color={FLOOR_COLOR} />
      </mesh>
      
      <mesh rotation={[-Math.PI / 2, 0, 0]} position={[0, 0, 0]}>
        <planeGeometry args={[40, 40]} />
        <meshStandardMaterial color={FLOOR_COLOR} roughness={0.8} />
      </mesh>

      {[...Array(20)].map((_, i) => (
        <mesh key={`grid-${i}`} rotation={[-Math.PI / 2, 0, 0]} position={[i * 2 - 20, 0.001, 0]}>
          <planeGeometry args={[0.02, 40]} />
          <meshBasicMaterial color="#60a5fa" transparent opacity={0.1} />
        </mesh>
      ))}
      {[...Array(20)].map((_, i) => (
        <mesh key={`grid2-${i}`} rotation={[-Math.PI / 2, 0, 0]} position={[0, 0.001, i * 2 - 20]}>
          <planeGeometry args={[40, 0.02]} />
          <meshBasicMaterial color="#60a5fa" transparent opacity={0.1} />
        </mesh>
      ))}
    </group>
  )
}

function Walls() {
  return (
    <group>
      <mesh position={[0, 4, -20]} receiveShadow>
        <boxGeometry args={[40, 8, 0.5]} />
        <meshStandardMaterial color={WALL_COLOR} />
      </mesh>
      
      <mesh position={[-20, 4, 0]} receiveShadow>
        <boxGeometry args={[0.5, 8, 40]} />
        <meshStandardMaterial color={WALL_COLOR} />
      </mesh>
      
      <mesh position={[20, 4, 0]} receiveShadow>
        <boxGeometry args={[0.5, 8, 40]} />
        <meshStandardMaterial color={WALL_COLOR} />
      </mesh>
    </group>
  )
}

function Desk({ position, name, agentLabel }: { position: [number, number, number]; name: string; agentLabel: string }) {
  return (
    <group position={position}>
      <mesh position={[0, 0.4, 0]} castShadow receiveShadow>
        <boxGeometry args={[2, 0.1, 1]} />
        <meshStandardMaterial color={DESK_COLOR} />
      </mesh>
      
      <mesh position={[-0.8, 0.2, -0.3]} castShadow>
        <boxGeometry args={[0.1, 0.4, 0.1]} />
        <meshStandardMaterial color="#0a0a14" />
      </mesh>
      <mesh position={[0.8, 0.2, -0.3]} castShadow>
        <boxGeometry args={[0.1, 0.4, 0.1]} />
        <meshStandardMaterial color="#0a0a14" />
      </mesh>
      <mesh position={[0, 0.2, 0.4]} castShadow>
        <boxGeometry args={[1.6, 0.4, 0.1]} />
        <meshStandardMaterial color="#0a0a14" />
      </mesh>

      <mesh position={[0, 0.85, -0.2]} castShadow>
        <boxGeometry args={[1.2, 0.7, 0.05]} />
        <meshStandardMaterial color={MONITOR_COLOR} emissive="#3b82f6" emissiveIntensity={0.3} />
      </mesh>
      
      <Text
        position={[0, 0.5, 0.6]}
        fontSize={0.15}
        color="#60a5fa"
        anchorX="center"
        anchorY="middle"
      >
        {agentLabel}
      </Text>
      <Text
        position={[0, 0.35, 0.6]}
        fontSize={0.1}
        color="#9ca3af"
        anchorX="center"
        anchorY="middle"
      >
        {name}
      </Text>
    </group>
  )
}

function RobotCharacter({ agent, targetPosition, onArrived }: {
  agent: Agent
  targetPosition: THREE.Vector3 | null
  onArrived?: () => void
}) {
  const groupRef = useRef<THREE.Group>(null)
  const color = AGENT_COLORS[agent.label] || '#22c55e'
  const isWorking = agent.status === 'working'
  const isWalking = targetPosition !== null

  const [walkCycle, setWalkCycle] = useState(0)

  useFrame((state, delta) => {
    if (!groupRef.current) return
    const t = state.clock.elapsedTime

    if (isWorking) {
      groupRef.current.position.y = 0.8 + Math.sin(t * 4) * 0.05
    } else {
      groupRef.current.position.y = 0.8 + Math.sin(t * 1.5) * 0.03
    }

    if (targetPosition) {
      const currentPos = groupRef.current.position
      const dx = targetPosition.x - currentPos.x
      const dz = targetPosition.z - currentPos.z
      const dist = Math.sqrt(dx * dx + dz * dz)

      if (dist > 0.1) {
        const speed = 0.05
        groupRef.current.position.x += (dx / dist) * speed
        groupRef.current.position.z += (dz / dist) * speed

        groupRef.current.rotation.y = Math.atan2(dx, dz)
        
        setWalkCycle(prev => prev + delta * 10)
      } else {
        onArrived?.()
      }
    }
  })

  return (
    <group ref={groupRef} position={[0, 0, 0]}>
      <mesh castShadow>
        <sphereGeometry args={[0.3, 16, 16]} />
        <meshStandardMaterial color={color} metalness={0.8} roughness={0.2} />
      </mesh>

      <mesh position={[0, -0.35, 0]} castShadow>
        <cylinderGeometry args={[0.2, 0.25, 0.3, 8]} />
        <meshStandardMaterial color={color} metalness={0.8} roughness={0.2} />
      </mesh>

      <mesh position={[-0.35, -0.1, 0]} rotation={[0, 0, Math.PI / 4]}>
        <capsuleGeometry args={[0.08, 0.3, 4, 8]} />
        <meshStandardMaterial color={color} metalness={0.8} roughness={0.2} />
      </mesh>
      <mesh position={[0.35, -0.1, 0]} rotation={[0, 0, -Math.PI / 4]}>
        <capsuleGeometry args={[0.08, 0.3, 4, 8]} />
        <meshStandardMaterial color={color} metalness={0.8} roughness={0.2} />
      </mesh>

      <mesh position={[0, -0.7, 0]} castShadow>
        <cylinderGeometry args={[0.1, 0.12, 0.4, 8]} />
        <meshStandardMaterial color="#0a0a14" metalness={0.9} roughness={0.1} />
      </mesh>

      <mesh position={[-0.15, 0.05, 0.25]}>
        <sphereGeometry args={[0.06, 8, 8]} />
        <meshStandardMaterial color="#000" />
      </mesh>
      <mesh position={[0.15, 0.05, 0.25]}>
        <sphereGeometry args={[0.06, 8, 8]} />
        <meshStandardMaterial color="#000" />
      </mesh>

      {isWorking && (
        <pointLight position={[0, 0.3, 0]} intensity={1} color={color} distance={3} decay={2} />
      )}

      <Html position={[0, 1, 0]} center>
        <div className="bg-black/70 px-2 py-1 rounded text-xs text-white whitespace-nowrap">
          {agent.label} - {agent.status}
        </div>
      </Html>
    </group>
  )
}

function MainDesk() {
  return (
    <group position={[0, 0, 0]}>
      <mesh position={[0, 0.5, 0]} castShadow receiveShadow>
        <boxGeometry args={[4, 0.15, 2]} />
        <meshStandardMaterial color="#1e3a5f" />
      </mesh>

      <mesh position={[-1.5, 0.25, -0.8]} castShadow>
        <boxGeometry args={[0.8, 0.5, 0.8]} />
        <meshStandardMaterial color="#0a0a14" />
      </mesh>
      <mesh position={[1.5, 0.25, -0.8]} castShadow>
        <boxGeometry args={[0.8, 0.5, 0.8]} />
        <meshStandardMaterial color="#0a0a14" />
      </mesh>

      <mesh position={[0, 1.2, -0.5]} castShadow>
        <boxGeometry args={[3, 1, 0.1]} />
        <meshStandardMaterial color="#0a0a14" emissive="#3b82f6" emissiveIntensity={0.5} />
      </mesh>

      <Text
        position={[0, 1.8, -0.5]}
        fontSize={0.3}
        color="#3b82f6"
        anchorX="center"
        anchorY="middle"
      >
        AI COMMERCE HQ
      </Text>
      <Text
        position={[0, 1.5, -0.5]}
        fontSize={0.15}
        color="#60a5fa"
        anchorX="center"
        anchorY="middle"
      >
        Global Master Orchestrator
      </Text>
    </group>
  )
}

export function Office3D() {
  const { agents, products } = useAppStore()
  const [agentPositions, setAgentPositions] = useState<Record<string, THREE.Vector3>>({})

  const desks = useMemo(() => [
    { label: 'TRD', name: 'Trend Research', pos: [-6, 0, -4] as [number, number, number] },
    { label: 'DES-1', name: 'Design Agent', pos: [-3, 0, -4] as [number, number, number] },
    { label: 'PRD-1', name: 'Product Agent', pos: [0, 0, -4] as [number, number, number] },
    { label: 'QA', name: 'QA Agent', pos: [3, 0, -4] as [number, number, number] },
    { label: 'POD', name: 'POD Agent', pos: [6, 0, -4] as [number, number, number] },
    { label: 'DES-2', name: 'Design 2', pos: [-6, 0, 4] as [number, number, number] },
    { label: 'PRD-2', name: 'Product 2', pos: [-3, 0, 4] as [number, number, number] },
    { label: 'VID', name: 'Video Agent', pos: [0, 0, 4] as [number, number, number] },
    { label: 'LST', name: 'Listing Agent', pos: [3, 0, 4] as [number, number, number] },
    { label: 'PER', name: 'Personalization', pos: [6, 0, 4] as [number, number, number] },
  ], [])

  const agentList = useMemo(() => Object.values(agents), [agents])

  useEffect(() => {
    const newPositions: Record<string, THREE.Vector3> = {}
    agentList.forEach((agent, index) => {
      const desk = desks[index % desks.length]
      newPositions[agent.id] = new THREE.Vector3(desk.pos[0], 0, desk.pos[2])
    })
    setAgentPositions(newPositions)
  }, [agentList, desks])

  return (
    <group>
      <Floor />
      <Walls />
      <MainDesk />

      {desks.map((desk) => (
        <Desk
          key={desk.label}
          position={desk.pos}
          name={desk.name}
          agentLabel={desk.label}
        />
      ))}

      {agentList.map((agent) => (
        <RobotCharacter
          key={agent.id}
          agent={agent}
          targetPosition={agentPositions[agent.id] || null}
        />
      ))}
    </group>
  )
}