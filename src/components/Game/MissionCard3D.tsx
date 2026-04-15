import { useRef, useState } from 'react'
import { useFrame, ThreeEvent } from '@react-three/fiber'
import { Float, Text, Html } from '@react-three/drei'
import * as THREE from 'three'
import type { GameMission } from '../../types'

const STATUS_COLORS = {
  pending: '#f59e0b',
  active: '#3b82f6',
  completed: '#22c55e',
  failed: '#ef4444',
}

const STATUS_ICONS = {
  pending: '⏳',
  active: '⚡',
  completed: '✅',
  failed: '❌',
}

export function MissionCard3D({ mission }: { mission: GameMission }) {
  const ref = useRef<THREE.Group>(null!)
  const [hovered, setHovered] = useState(false)
  const color = STATUS_COLORS[mission.status]

  const positions: Record<string, [number, number, number]> = {
    etsy: [-5, 3.5, 0],
    fiverr: [0, 3.5, -5],
    trading: [5, 3.5, 0],
    youtube: [0, 3.5, 5],
    tiktok: [-3, 3, -3],
  }

  const pos = positions[mission.zoneId] || [0, 3, 0]

  useFrame((state) => {
    if (!ref.current) return
    const t = state.clock.elapsedTime
    ref.current.rotation.y = t * 0.2
    ref.current.position.y = pos[1] + Math.sin(t * 0.8) * 0.2

    if (hovered) {
      ref.current.scale.lerp(new THREE.Vector3(1.3, 1.3, 1.3), 0.1)
    } else {
      ref.current.scale.lerp(new THREE.Vector3(1, 1, 1), 0.1)
    }
  })

  return (
    <Float speed={2} rotationIntensity={0} floatIntensity={0.3}>
      <group
        ref={ref}
        position={pos}
        onPointerOver={() => setHovered(true)}
        onPointerOut={() => setHovered(false)}
      >
        <mesh>
          <boxGeometry args={[1.8, 1.1, 0.08]} />
          <meshStandardMaterial
            color="#0a0a14"
            emissive={color}
            emissiveIntensity={hovered ? 0.5 : 0.2}
            metalness={0.3}
            roughness={0.7}
          />
        </mesh>

        <mesh position={[0, 0, 0.05]}>
          <boxGeometry args={[1.7, 1.0, 0.01]} />
          <meshStandardMaterial
            color={color}
            emissive={color}
            emissiveIntensity={0.8}
            transparent
            opacity={0.15}
          />
        </mesh>

        <Text
          position={[0, 0.25, 0.1]}
          fontSize={0.13}
          color={color}
          anchorX="center"
          anchorY="middle"
          maxWidth={1.5}
        >
          {STATUS_ICONS[mission.status]} {mission.title?.slice(0, 30)}
        </Text>

        <Text
          position={[0, 0, 0.1]}
          fontSize={0.09}
          color="#9ca3af"
          anchorX="center"
          anchorY="middle"
          maxWidth={1.5}
        >
          {mission.zoneId?.toUpperCase()}
        </Text>

        <Text
          position={[0, -0.25, 0.1]}
          fontSize={0.11}
          color="#fbbf24"
          anchorX="center"
          anchorY="middle"
        >
          💰 ${mission.reward}
        </Text>

        <pointLight color={color} intensity={hovered ? 2 : 0.5} distance={3} decay={2} />

        {hovered && (
          <Html position={[0, 1.2, 0]} center>
            <div className="bg-gray-900/95 border border-gray-700 rounded-xl px-4 py-3 min-w-[220px] pointer-events-none">
              <div className="flex items-center gap-2 mb-1">
                <span className="text-lg">{STATUS_ICONS[mission.status]}</span>
                <span className="font-bold text-white text-sm">{mission.title?.slice(0, 40)}</span>
              </div>
              <div className="text-xs text-gray-400 mb-2">
                {mission.description?.slice(0, 100)}
              </div>
              <div className="flex items-center justify-between text-xs">
                <span className="text-gray-500">Reward:</span>
                <span className="text-yellow-400 font-bold">${mission.reward}</span>
              </div>
              <div className="flex items-center justify-between text-xs mt-1">
                <span className="text-gray-500">Status:</span>
                <span className={`font-bold capitalize ${
                  mission.status === 'completed' ? 'text-green-400' :
                  mission.status === 'failed' ? 'text-red-400' :
                  mission.status === 'active' ? 'text-blue-400' : 'text-yellow-400'
                }`}>
                  {mission.status}
                </span>
              </div>
            </div>
          </Html>
        )}
      </group>
    </Float>
  )
}
