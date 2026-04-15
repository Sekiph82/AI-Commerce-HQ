import { useRef, useMemo } from 'react'
import { useFrame } from '@react-three/fiber'
import { Line } from '@react-three/drei'
import * as THREE from 'three'
import { useAppStore } from '../../store/useAppStore'

const ZONE_POSITIONS: Record<string, [number, number, number]> = {
  etsy: [-5, 1.5, 0],
  fiverr: [0, 1.5, -5],
  trading: [5, 1.5, 0],
  youtube: [0, 1.5, 5],
  tiktok: [-3, 0.3, -3],
}

const ZONE_COLORS: Record<string, string> = {
  etsy: '#ff6b35',
  fiverr: '#1dbf73',
  trading: '#f59e0b',
  youtube: '#ef4444',
  tiktok: '#ec4899',
}

export function MoneyFlow() {
  const { products } = useAppStore()

  const incomeByZone = useMemo(() => {
    const zones: Record<string, number> = { etsy: 0, fiverr: 0, trading: 0, youtube: 0, tiktok: 0 }
    Object.values(products).forEach(p => {
      if (p.platform === 'etsy' && p.state === 'ETSY_DRAFT_CREATED') {
        zones.etsy += (p.price || 25) * (p.estimatedMargin || 35) / 100
      }
    })
    return zones
  }, [products])

  const flows = useMemo(() => {
    const flowsArr: Array<{
      start: [number, number, number]
      end: [number, number, number]
      color: string
      speed: number
      offset: number
    }> = []

    Object.entries(incomeByZone).forEach(([zone, income]) => {
      if (income > 0) {
        flowsArr.push({
          start: ZONE_POSITIONS[zone] || [0, 2, 0],
          end: [0, 3, 0],
          color: ZONE_COLORS[zone] || '#ffffff',
          speed: 0.5 + income * 0.1,
          offset: Math.random() * Math.PI * 2,
        })
      }
    })

    if (flowsArr.length === 0) {
      flowsArr.push({
        start: [-5, 1.5, 0],
        end: [0, 3, 0],
        color: '#3b82f6',
        speed: 0.5,
        offset: 0,
      })
    }

    return flowsArr
  }, [incomeByZone])

  return (
    <group>
      {flows.map((flow, i) => (
        <EnergyStream key={i} {...flow} />
      ))}

      <mesh position={[0, 0.05, 0]} rotation={[-Math.PI / 2, 0, 0]}>
        <ringGeometry args={[0.3, 12, 64]} />
        <meshStandardMaterial
          color="#1e3a5f"
          emissive="#1e40af"
          emissiveIntensity={0.3}
          transparent
          opacity={0.4}
        />
      </mesh>

      <mesh position={[0, 0.08, 0]} rotation={[-Math.PI / 2, 0, 0]}>
        <circleGeometry args={[12, 64]} />
        <meshStandardMaterial
          color="#0a1628"
          emissive="#1e3a8a"
          emissiveIntensity={0.1}
          transparent
          opacity={0.3}
        />
      </mesh>

      {Object.entries(ZONE_POSITIONS).map(([zone, pos]) => (
        <EnergyRing key={zone} position={pos} color={ZONE_COLORS[zone]} income={incomeByZone[zone] || 0} />
      ))}
    </group>
  )
}

function EnergyStream({ start, end, color, speed, offset }: {
  start: [number, number, number]; end: [number, number, number]
  color: string; speed: number; offset: number
}) {
  const points = useMemo(() => {
    const curve = new THREE.CatmullRomCurve3([
      new THREE.Vector3(...start),
      new THREE.Vector3((start[0] + end[0]) / 2, start[1] + 3, (start[2] + end[2]) / 2),
      new THREE.Vector3(...end),
    ])
    return curve.getPoints(50)
  }, [start, end])

  const geometry = useMemo(() => {
    const geo = new THREE.BufferGeometry().setFromPoints(points)
    return geo
  }, [points])

  const linePoints = useMemo(() => points.map(p => [p.x, p.y, p.z] as [number, number, number]), [points])

  const progressRef = useRef(0)

  useFrame((state) => {
    progressRef.current = (progressRef.current + speed * 0.005) % 1
  })

  const progress = useRef(0)
  const particleRef = useRef<THREE.Mesh>(null!)

  useFrame((state) => {
    progress.current = (progress.current + speed * 0.008) % 1

    if (particleRef.current) {
      const curve = new THREE.CatmullRomCurve3([
        new THREE.Vector3(...start),
        new THREE.Vector3((start[0] + end[0]) / 2, start[1] + 3, (start[2] + end[2]) / 2),
        new THREE.Vector3(...end),
      ])
      const point = curve.getPoint(progress.current)
      particleRef.current.position.copy(point)
    }
  })

  return (
    <group>
      <Line
        points={linePoints}
        color={color}
        lineWidth={1}
        transparent
        opacity={0.15}
      />
      <mesh ref={particleRef}>
        <sphereGeometry args={[0.08, 8, 8]} />
        <meshStandardMaterial color={color} emissive={color} emissiveIntensity={4} />
      </mesh>
    </group>
  )
}

function EnergyRing({ position, color, income }: { position: [number, number, number]; color: string; income: number }) {
  const ref = useRef<THREE.Mesh>(null!)

  useFrame((state) => {
    if (ref.current) {
      ref.current.rotation.z = state.clock.elapsedTime * 0.5
      const scale = 1 + income * 0.02 + Math.sin(state.clock.elapsedTime * 2) * 0.1
      ref.current.scale.setScalar(scale)
    }
  })

  return (
    <mesh ref={ref} position={[position[0], 0.1, position[2]]} rotation={[-Math.PI / 2, 0, 0]}>
      <ringGeometry args={[1.9, 2.1, 6]} />
      <meshStandardMaterial
        color={color}
        emissive={color}
        emissiveIntensity={income > 0 ? 1 : 0.3}
        transparent
        opacity={0.4}
      />
    </mesh>
  )
}
