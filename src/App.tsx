import { useEffect, useState } from 'react'
import { AnimatePresence } from 'framer-motion'
import { SplashScreen } from './components/Splash/SplashScreen'
import { SetupWizard } from './components/Wizard/SetupWizard'
import { OfficeLayout } from './components/Office/OfficeLayout'
import { useAppStore } from './store/useAppStore'
import type { AppConfig } from './types'

export default function App() {
  const { screen, setScreen, setConfig } = useAppStore()
  const [splashDone, setSplashDone] = useState(false)

  useEffect(() => {
    // Check if backend is reachable and config exists
    const checkSetup = async () => {
      // Check local storage first
      const stored = localStorage.getItem('hq_config')
      if (stored) {
        try {
          const config: AppConfig = JSON.parse(stored)
          if (config.setupComplete) {
            setConfig(config)
            return
          }
        } catch {}
      }

      // Try backend
      try {
        const res = await fetch('http://localhost:8765/api/config', { signal: AbortSignal.timeout(3000) })
        if (res.ok) {
          const config: AppConfig = await res.json()
          if (config.setupComplete) {
            setConfig(config)
            localStorage.setItem('hq_config', JSON.stringify(config))
          }
        }
      } catch {}
    }

    checkSetup()
  }, [setConfig])

  const handleSplashComplete = () => {
    setSplashDone(true)
    const { config } = useAppStore.getState()
    setScreen(config?.setupComplete ? 'office' : 'wizard')
  }

  const handleWizardComplete = () => {
    setScreen('office')
  }

  return (
    <div className="w-screen h-screen overflow-hidden bg-office-bg text-white">
      <AnimatePresence mode="wait">
        {!splashDone && (
          <SplashScreen key="splash" onComplete={handleSplashComplete} />
        )}
        {splashDone && screen === 'wizard' && (
          <SetupWizard key="wizard" onComplete={handleWizardComplete} />
        )}
        {splashDone && screen === 'office' && (
          <OfficeLayout key="office" />
        )}
      </AnimatePresence>
    </div>
  )
}
