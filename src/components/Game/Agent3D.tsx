import { useRef } from 'react'
import { useFrame } from '@react-three/fiber'
import * as THREE from 'three'
import { useAppStore } from '../../store/useAppStore'

export function Agent3D({ agent }: { agent: ReturnType<typeof useAppStore.getState>['agents'][string] }) {
  const ref = useRef<THREE.Group>(null!)
  const statusColor = agent.status === 'working' ? '#22c55e' :
                      agent.status === 'blocked' ? '#ef4444' :
                      agent.status === 'complete' ? '#3b82f6' :
                      agent.status === 'entering' ? '#f59e0b' : '#6b7280'

  useFrame((state) => {
    if (!ref.current) return
    const t = state.clock.elapsedTime

    if (agent.status === 'idle') {
      const hash = agent.id.split('').reduce((a, c) => a + c.charCodeAt(0), 0)
      ref.current.position.y = Math.sin(t * 1.5 + hash % 10) * 0.1
    } else if (agent.status === 'working') {
      ref.current.position.y = Math.sin(t * 4) * 0.15
    }

    if (agent.status === 'blocked') {
      ref.current.rotation.z = Math.sin(t * 8) * 0.1
    } else {
      ref.current.rotation.z = 0
    }

    ref.current.rotation.y = t * (agent.status === 'working' ? 1.5 : 0.3)
  })

  return (
    <group ref={ref}>
      <mesh>
        <sphereGeometry args={[0.12, 16, 16]} />
        <meshStandardMaterial
          color={statusColor}
          emissive={statusColor}
          emissiveIntensity={agent.status === 'working' ? 4 : 1.5}
          metalness={0.5}
          roughness={0.3}
        />
      </mesh>
      {agent.status === 'working' && (
        <pointLight color={statusColor} intensity={1} distance={2} decay={2} />
      )}
    </group>
  )
}
