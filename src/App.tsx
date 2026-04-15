import { useEffect, useState } from 'react'
import { AnimatePresence } from 'framer-motion'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'
import { SplashScreen } from './components/Splash/SplashScreen'
import { SetupWizard } from './components/Wizard/SetupWizard'
import { OfficeLayout } from './components/Office/OfficeLayout'
import { GameWorld } from './components/Game/GameWorld'
import { useAppStore } from './store/useAppStore'
import type { AppConfig } from './types'

export default function App() {
  const { screen, setScreen, setConfig, setBackendReady } = useAppStore()
  const [splashDone, setSplashDone] = useState(false)
  const [backendCrashed, setBackendCrashed] = useState(false)
  const [recovering, setRecovering] = useState(false)
  const [viewMode, setViewMode] = useState<'office' | 'game'>('game')

  // Pre-load config from localStorage so the wizard/office decision is instant
  useEffect(() => {
    const checkSetup = async () => {
      const stored = localStorage.getItem('hq_config')
      if (stored) {
        try {
          const config: AppConfig = JSON.parse(stored)
          if (config.setupComplete) {
            setConfig(config)
            return
          }
        } catch { /* ignore */ }
      }

      // Try backend (backend may not be ready yet — that's fine)
      try {
        const res = await fetch('http://localhost:8765/api/config', {
          signal: AbortSignal.timeout(5000),
        })
        if (res.ok) {
          const config: AppConfig = await res.json()
          if (config.setupComplete) {
            setConfig(config)
            localStorage.setItem('hq_config', JSON.stringify(config))
          }
        }
      } catch { /* backend not yet ready */ }
    }

    checkSetup()
  }, [setConfig])

  // Listen for backend crash events from main.rs
  useEffect(() => {
    let unlisten: (() => void) | null = null
    listen<number>('backend-crashed', () => {
      setBackendReady(false)
      setBackendCrashed(true)
    }).then((fn) => { unlisten = fn })
    return () => unlisten?.()
  }, [setBackendReady])

  const handleSplashComplete = () => {
    setSplashDone(true)
    setBackendReady(true)
    setScreen('game')
  }

  const handleRecover = async () => {
    setRecovering(true)
    try {
      await invoke('restart_backend')
      setBackendCrashed(false)
      setBackendReady(true)
    } catch (e) {
      console.error('Recovery failed:', e)
    } finally {
      setRecovering(false)
    }
  }

  // Full-screen crash overlay — shown over whatever is active if backend dies
  if (backendCrashed && splashDone) {
    return (
      <div className="w-screen h-screen overflow-hidden bg-office-bg text-white flex items-center justify-center">
        <div className="text-center max-w-sm">
          <div className="text-5xl mb-6">⚠️</div>
          <h2 className="text-2xl font-bold mb-2">Backend Stopped</h2>
          <p className="text-gray-400 text-sm mb-8">
            The AI agent runtime stopped unexpectedly. Click below to restart it automatically.
          </p>
          <button
            onClick={handleRecover}
            disabled={recovering}
            className="px-6 py-3 rounded-xl bg-blue-600 hover:bg-blue-500 text-white font-medium transition-colors disabled:opacity-50"
          >
            {recovering ? 'Restarting...' : 'Restart Backend'}
          </button>
        </div>
      </div>
    )
  }

  return (
    <div className="w-screen h-screen overflow-hidden bg-[#030308] text-white">
      <AnimatePresence mode="wait">
        {!splashDone && (
          <SplashScreen key="splash" onComplete={handleSplashComplete} />
        )}
        {splashDone && screen === 'game' && (
          <div key="game" className="w-full h-full flex flex-col">
            <div className="flex items-center gap-3 px-4 py-2 bg-[#0a0a14] border-b border-gray-800 flex-shrink-0">
              <div className="flex items-center gap-2">
                <div className="w-6 h-6 rounded bg-gradient-to-br from-blue-600 to-purple-700 flex items-center justify-center text-xs">
                  HQ
                </div>
                <span className="font-bold text-white text-sm">AI Commerce HQ</span>
              </div>
              <div className="flex items-center gap-1 ml-4">
                <button
                  onClick={() => setViewMode('game')}
                  className={`px-3 py-1 rounded-lg text-xs font-medium transition-colors ${
                    viewMode === 'game' ? 'bg-blue-600/30 text-blue-300' : 'text-gray-500 hover:text-gray-300'
                  }`}
                >
                  3D Game View
                </button>
                <button
                  onClick={() => setViewMode('office')}
                  className={`px-3 py-1 rounded-lg text-xs font-medium transition-colors ${
                    viewMode === 'office' ? 'bg-blue-600/30 text-blue-300' : 'text-gray-500 hover:text-gray-300'
                  }`}
                >
                  Office View
                </button>
              </div>
            </div>
            <div className="flex-1 min-h-0">
              {viewMode === 'game' ? <GameWorld /> : <OfficeLayout />}
            </div>
          </div>
        )}
        {splashDone && screen === 'office' && (
          <OfficeLayout key="office" />
        )}
      </AnimatePresence>
    </div>
  )
}
