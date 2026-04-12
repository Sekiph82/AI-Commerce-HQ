import { useEffect, useState } from 'react'
import { AnimatePresence } from 'framer-motion'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'
import { SplashScreen } from './components/Splash/SplashScreen'
import { SetupWizard } from './components/Wizard/SetupWizard'
import { OfficeLayout } from './components/Office/OfficeLayout'
import { useAppStore } from './store/useAppStore'
import type { AppConfig } from './types'

export default function App() {
  const { screen, setScreen, setConfig, setBackendReady } = useAppStore()
  const [splashDone, setSplashDone] = useState(false)
  const [backendCrashed, setBackendCrashed] = useState(false)
  const [recovering, setRecovering] = useState(false)

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
    setScreen('office')
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
    <div className="w-screen h-screen overflow-hidden bg-office-bg text-white">
      <AnimatePresence mode="wait">
        {!splashDone && (
          <SplashScreen key="splash" onComplete={handleSplashComplete} />
        )}
        {splashDone && screen === 'office' && (
          <OfficeLayout key="office" />
        )}
      </AnimatePresence>
    </div>
  )
}
