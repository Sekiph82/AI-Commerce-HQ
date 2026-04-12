import { useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { X, Save, Eye, EyeOff } from 'lucide-react'
import { useAppStore } from '../../store/useAppStore'
import type { AppConfig } from '../../types'

export function SettingsPanel() {
  const { settingsPanelOpen, toggleSettingsPanel, config, setConfig } = useAppStore()
  const [form, setForm] = useState<AppConfig>(
    config || { openaiKey: '', etsyApiKey: '', etsyShopId: '', printifyToken: '', geminiKey: '', setupComplete: true }
  )
  const [showKeys, setShowKeys] = useState<Record<string, boolean>>({})
  const [saved, setSaved] = useState(false)

  const isFirstTime = !config?.openaiKey && !config?.etsyApiKey

  const toggle = (k: string) => setShowKeys((s) => ({ ...s, [k]: !s[k] }))

  const save = async () => {
    const toSave = { ...form, setupComplete: true }
    try {
      await fetch('http://localhost:8765/api/config', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(toSave),
      })
    } catch {}
    localStorage.setItem('hq_config', JSON.stringify(toSave))
    setConfig(toSave)
    setSaved(true)
    // Close automatically after saving on first-time setup
    if (isFirstTime) {
      setTimeout(() => toggleSettingsPanel(), 1000)
    } else {
      setTimeout(() => setSaved(false), 2000)
    }
  }

  const fields = [
    { label: 'OpenAI API Key', key: 'openaiKey' as keyof AppConfig, placeholder: 'sk-...' },
    { label: 'Etsy API Key', key: 'etsyApiKey' as keyof AppConfig, placeholder: 'etsy-api-key' },
    { label: 'Etsy Shop ID', key: 'etsyShopId' as keyof AppConfig, placeholder: 'ShopName', isText: true },
    { label: 'Printify Token', key: 'printifyToken' as keyof AppConfig, placeholder: 'printify-token' },
    { label: 'Gemini API Key (optional)', key: 'geminiKey' as keyof AppConfig, placeholder: 'AIza...' },
  ]

  return (
    <AnimatePresence>
      {settingsPanelOpen && (
        <>
          <motion.div
            className="fixed inset-0 bg-black/60 z-40"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            onClick={toggleSettingsPanel}
          />
          <motion.div
            className="fixed right-0 top-0 bottom-0 w-96 bg-office-floor border-l border-gray-700 z-50 flex flex-col"
            initial={{ x: '100%' }}
            animate={{ x: 0 }}
            exit={{ x: '100%' }}
            transition={{ type: 'spring', stiffness: 300, damping: 30 }}
          >
            <div className="p-4 border-b border-gray-700 flex items-center justify-between">
              <div>
                <h2 className="font-bold text-white">
                  {isFirstTime ? 'Welcome — Add Your API Keys' : 'Settings'}
                </h2>
                {isFirstTime && (
                  <p className="text-xs text-blue-400 mt-0.5">
                    The app works without keys in demo mode. Add them to go live.
                  </p>
                )}
              </div>
              <button onClick={toggleSettingsPanel} className="p-1 rounded hover:bg-gray-700 text-gray-400">
                <X size={16} />
              </button>
            </div>

            <div className="flex-1 overflow-y-auto p-4 space-y-4">
              <p className="text-xs text-gray-400">Credentials are stored locally on your device only.</p>
              {fields.map((f) => (
                <div key={f.key}>
                  <label className="block text-sm text-gray-300 mb-1">{f.label}</label>
                  <div className="relative">
                    <input
                      type={f.isText || showKeys[f.key] ? 'text' : 'password'}
                      value={String(form[f.key] || '')}
                      onChange={(e) => setForm((prev) => ({ ...prev, [f.key]: e.target.value }))}
                      placeholder={f.placeholder}
                      className="w-full bg-gray-900 border border-gray-700 rounded-lg px-4 py-2.5 text-white placeholder-gray-600 focus:outline-none focus:border-blue-500 pr-10 font-mono text-sm"
                    />
                    {!f.isText && (
                      <button
                        type="button"
                        onClick={() => toggle(f.key)}
                        className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 hover:text-gray-300"
                      >
                        {showKeys[f.key] ? <EyeOff size={14} /> : <Eye size={14} />}
                      </button>
                    )}
                  </div>
                </div>
              ))}
            </div>

            <div className="p-4 border-t border-gray-700">
              <button
                onClick={save}
                className={`w-full flex items-center justify-center gap-2 py-2.5 rounded-lg font-medium text-sm transition-colors ${
                  saved ? 'bg-green-700 text-white' : 'bg-blue-600 hover:bg-blue-500 text-white'
                }`}
              >
                <Save size={16} />
                {saved ? 'Saved!' : 'Save Settings'}
              </button>
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  )
}
