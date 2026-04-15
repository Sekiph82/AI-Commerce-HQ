import { useRef } from 'react'
import { useFrame } from '@react-three/fiber'
import { Float, Text } from '@react-three/drei'
import * as THREE from 'three'
import { useAppStore } from '../../store/useAppStore'

export function CentralCore() {
  const meshRef = useRef<THREE.Mesh>(null!)
  const glowRef = useRef<THREE.Mesh>(null!)
  const ringRef = useRef<THREE.Mesh>(null!)
  const innerRef = useRef<THREE.Group>(null!)

  const { talkingTable } = useAppStore()
  const latestMessage = talkingTable[0]

  useFrame((state) => {
    const t = state.clock.elapsedTime

    if (meshRef.current) {
      meshRef.current.rotation.y = t * 0.5
      meshRef.current.rotation.x = Math.sin(t * 0.3) * 0.1
    }

    if (glowRef.current) {
      glowRef.current.rotation.y = -t * 0.3
      const scale = 1 + Math.sin(t * 2) * 0.05
      glowRef.current.scale.setScalar(scale)
    }

    if (ringRef.current) {
      ringRef.current.rotation.z = t * 0.8
      ringRef.current.rotation.x = Math.sin(t * 0.5) * 0.2
    }

    if (innerRef.current) {
      innerRef.current.rotation.y = -t * 0.4
    }
  })

  return (
    <group position={[0, 3, 0]}>
      <Float speed={2} rotationIntensity={0.2} floatIntensity={0.5}>
        <group>
          <mesh ref={glowRef}>
            <icosahedronGeometry args={[3.2, 1]} />
            <meshStandardMaterial
              color="#1e40af"
              emissive="#1d4ed8"
              emissiveIntensity={0.4}
              transparent
              opacity={0.15}
              wireframe
            />
          </mesh>

          <mesh ref={meshRef}>
            <icosahedronGeometry args={[2.5, 0]} />
            <meshStandardMaterial
              color="#1e3a8a"
              emissive="#3b82f6"
              emissiveIntensity={0.6}
              metalness={0.8}
              roughness={0.2}
            />
          </mesh>

          <group ref={innerRef}>
            <mesh>
              <octahedronGeometry args={[1.2, 0]} />
              <meshStandardMaterial
                color="#60a5fa"
                emissive="#3b82f6"
                emissiveIntensity={1.2}
                metalness={0.9}
                roughness={0.1}
                transparent
                opacity={0.9}
              />
            </mesh>
          </group>

          <mesh ref={ringRef}>
            <torusGeometry args={[3.5, 0.08, 16, 64]} />
            <meshStandardMaterial
              color="#60a5fa"
              emissive="#3b82f6"
              emissiveIntensity={2}
              transparent
              opacity={0.6}
            />
          </mesh>

          <mesh position={[0, 0, 0]}>
            <sphereGeometry args={[0.3, 16, 16]} />
            <meshStandardMaterial
              color="#ffffff"
              emissive="#ffffff"
              emissiveIntensity={4}
            />
          </mesh>

          <pointLight color="#3b82f6" intensity={5} distance={15} decay={2} />
        </group>
      </Float>

      <Text
        position={[0, -3.5, 0]}
        fontSize={0.5}
        color="#60a5fa"
        anchorX="center"
        anchorY="middle"
        font="https://fonts.gstatic.com/s/orbitron/v29/yMJRMIlzdpvBhQQL_Qq7dy0.woff2"
      >
        AI COMMERCE HQ
      </Text>

      <Text
        position={[0, -4.1, 0]}
        fontSize={0.18}
        color="#94a3b8"
        anchorX="center"
        anchorY="middle"
        maxWidth={4}
      >
        {latestMessage?.message?.slice(0, 50) || 'System Online — Awaiting Agents'}
      </Text>
    </group>
  )
}
