import { useEffect, useState } from 'react'
import { GameWorld } from './components/Game/GameWorld'
import { useAppStore } from './store/useAppStore'
import type { AppConfig } from './types'

export default function App() {
  const { setConfig, setBackendReady } = useAppStore()
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    const initApp = async () => {
      const stored = localStorage.getItem('hq_config')
      if (stored) {
        try {
          const config: AppConfig = JSON.parse(stored)
          setConfig(config)
        } catch {}
      }

      try {
        const res = await fetch('http://localhost:8765/api/config', {
          signal: AbortSignal.timeout(5000),
        })
        if (res.ok) {
          const config: AppConfig = await res.json()
          setConfig(config)
          localStorage.setItem('hq_config', JSON.stringify(config))
        }
      } catch {}

      setBackendReady(true)
      setLoading(false)
    }

    initApp()
  }, [setConfig, setBackendReady])

  if (loading) {
    return (
      <div className="w-screen h-screen overflow-hidden bg-[#0a0a14] text-white flex items-center justify-center">
        <div className="text-center">
          <div className="text-4xl mb-4">🏢</div>
          <div className="text-xl font-bold">Loading AI Commerce HQ...</div>
        </div>
      </div>
    )
  }

  return (
    <div className="w-screen h-screen overflow-hidden bg-[#0a0a14] text-white">
      <GameWorld />
    </div>
  )
}