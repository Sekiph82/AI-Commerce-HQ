import { Canvas } from '@react-three/fiber'
import { OrbitControls, Stars, PerspectiveCamera, Text, Html } from '@react-three/drei'
import { Suspense } from 'react'
import { Office3D } from './Office3D'
import { GameHUD } from './GameHUD'
import { useAppStore } from '../../store/useAppStore'

export function GameWorld() {
  const { backendReady } = useAppStore()

  return (
    <div className="w-full h-full relative bg-[#0a0a14]">
      <Canvas shadows gl={{ antialias: true }}>
        <PerspectiveCamera makeDefault position={[0, 8, 15]} fov={50} />
        
        <color attach="background" args={['#0a0a14']} />
        <fog attach="fog" args={['#0a0a14', 20, 50]} />

        <ambientLight intensity={0.4} />
        <directionalLight
          position={[10, 15, 10]}
          intensity={1}
          castShadow
          shadow-mapSize={[2048, 2048]}
        />
        <pointLight position={[0, 10, 0]} intensity={0.5} color="#60a5fa" />

        <Suspense fallback={null}>
          <Office3D />
        </Suspense>

        <Stars radius={60} depth={40} count={2000} factor={2} saturation={0.3} fade speed={0.3} />

        <OrbitControls
          enablePan={true}
          enableZoom={true}
          enableRotate={true}
          minDistance={5}
          maxDistance={30}
          maxPolarAngle={Math.PI / 2.1}
          minPolarAngle={Math.PI / 8}
          target={[0, 1, 0]}
        />
      </Canvas>

      <GameHUD />
    </div>
  )
}