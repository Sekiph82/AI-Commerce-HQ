import { useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { Eye, EyeOff, CheckCircle, AlertCircle, ChevronRight, ChevronLeft } from 'lucide-react'
import { useAppStore } from '../../store/useAppStore'
import type { AppConfig } from '../../types'

interface Props {
  onComplete: () => void
}

type Step = 'welcome' | 'openai' | 'etsy' | 'printify' | 'optional' | 'done'

const STEPS: Step[] = ['welcome', 'openai', 'etsy', 'printify', 'optional', 'done']

interface FormData {
  openaiKey: string
  etsyApiKey: string
  etsyShopId: string
  printifyToken: string
  geminiKey: string
}

export function SetupWizard({ onComplete }: Props) {
  const { setConfig } = useAppStore()
  const [step, setStep] = useState<Step>('welcome')
  const [form, setForm] = useState<FormData>({
    openaiKey: '',
    etsyApiKey: '',
    etsyShopId: '',
    printifyToken: '',
    geminiKey: '',
  })
  const [showKeys, setShowKeys] = useState<Record<string, boolean>>({})
  const [validating, setValidating] = useState(false)
  const [errors, setErrors] = useState<Record<string, string>>({})

  const stepIndex = STEPS.indexOf(step)

  const toggle = (key: string) => setShowKeys((s) => ({ ...s, [key]: !s[key] }))

  const updateField = (field: keyof FormData, value: string) => {
    setForm((f) => ({ ...f, [field]: value }))
    setErrors((e) => ({ ...e, [field]: '' }))
  }

  const validate = async () => {
    setValidating(true)
    const errs: Record<string, string> = {}

    if (step === 'openai' && !form.openaiKey.startsWith('sk-')) {
      errs.openaiKey = 'OpenAI key must start with sk-'
    }
    if (step === 'etsy') {
      if (!form.etsyApiKey) errs.etsyApiKey = 'Etsy API key is required'
      if (!form.etsyShopId) errs.etsyShopId = 'Etsy Shop ID is required'
    }

    setErrors(errs)
    setValidating(false)
    return Object.keys(errs).length === 0
  }

  const next = async () => {
    if (step === 'welcome') {
      setStep('openai')
      return
    }
    const ok = await validate()
    if (!ok) return

    const idx = STEPS.indexOf(step)
    if (idx < STEPS.length - 1) {
      setStep(STEPS[idx + 1])
    }
  }

  const prev = () => {
    const idx = STEPS.indexOf(step)
    if (idx > 0) setStep(STEPS[idx - 1])
  }

  const finish = async () => {
    const config: AppConfig = {
      openaiKey: form.openaiKey,
      etsyApiKey: form.etsyApiKey,
      etsyShopId: form.etsyShopId,
      printifyToken: form.printifyToken,
      geminiKey: form.geminiKey,
      setupComplete: true,
    }

    // Save config to backend
    try {
      await fetch('http://localhost:8765/api/config', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(config),
      })
    } catch (e) {
      console.warn('Could not save config to backend, storing locally')
      localStorage.setItem('hq_config', JSON.stringify(config))
    }

    setConfig(config)
    onComplete()
  }

  return (
    <motion.div
      className="fixed inset-0 bg-office-bg flex items-center justify-center"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
    >
      <div className="w-full max-w-lg">
        {/* Header */}
        <div className="text-center mb-8">
          <div className="w-16 h-16 rounded-2xl bg-gradient-to-br from-blue-600 to-purple-700 flex items-center justify-center mx-auto mb-4">
            <span className="text-2xl">🏢</span>
          </div>
          <h1 className="text-2xl font-bold text-white">AI Commerce HQ Setup</h1>
          <p className="text-gray-400 text-sm mt-1">One-time configuration</p>
        </div>

        {/* Progress bar */}
        <div className="flex gap-1 mb-8">
          {STEPS.filter((s) => s !== 'welcome' && s !== 'done').map((s, i) => (
            <div
              key={s}
              className={`h-1 flex-1 rounded-full transition-colors duration-300 ${
                stepIndex > i + 1 ? 'bg-blue-500' : stepIndex === i + 1 ? 'bg-blue-400' : 'bg-gray-700'
              }`}
            />
          ))}
        </div>

        {/* Steps */}
        <AnimatePresence mode="wait">
          <motion.div
            key={step}
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            exit={{ opacity: 0, x: -20 }}
            className="bg-office-wall rounded-2xl p-8 border border-gray-700"
          >
            {step === 'welcome' && <WelcomeStep />}
            {step === 'openai' && (
              <KeyStep
                title="OpenAI API"
                description="Powers all AI reasoning, design generation, and product research."
                icon="🤖"
                fields={[
                  {
                    label: 'OpenAI API Key',
                    key: 'openaiKey',
                    placeholder: 'sk-...',
                    value: form.openaiKey,
                    error: errors.openaiKey,
                  },
                ]}
                showKeys={showKeys}
                onToggle={toggle}
                onChange={updateField}
              />
            )}
            {step === 'etsy' && (
              <KeyStep
                title="Etsy Integration"
                description="Connect your Etsy store to manage listings and draft products."
                icon="🛍️"
                fields={[
                  {
                    label: 'Etsy API Key',
                    key: 'etsyApiKey',
                    placeholder: 'your-etsy-key',
                    value: form.etsyApiKey,
                    error: errors.etsyApiKey,
                  },
                  {
                    label: 'Etsy Shop ID',
                    key: 'etsyShopId',
                    placeholder: 'YourShopName or numeric ID',
                    value: form.etsyShopId,
                    error: errors.etsyShopId,
                    isText: true,
                  },
                ]}
                showKeys={showKeys}
                onToggle={toggle}
                onChange={updateField}
              />
            )}
            {step === 'printify' && (
              <KeyStep
                title="Printify (POD)"
                description="Print-on-demand fulfillment for your products."
                icon="🖨️"
                fields={[
                  {
                    label: 'Printify API Token',
                    key: 'printifyToken',
                    placeholder: 'printify-token',
                    value: form.printifyToken,
                    error: errors.printifyToken,
                  },
                ]}
                showKeys={showKeys}
                onToggle={toggle}
                onChange={updateField}
                optional
              />
            )}
            {step === 'optional' && (
              <KeyStep
                title="Optional Integrations"
                description="Enhance AI capabilities with additional services."
                icon="✨"
                fields={[
                  {
                    label: 'Google Gemini API Key (optional)',
                    key: 'geminiKey',
                    placeholder: 'AIza...',
                    value: form.geminiKey,
                    error: errors.geminiKey,
                  },
                ]}
                showKeys={showKeys}
                onToggle={toggle}
                onChange={updateField}
                optional
              />
            )}
            {step === 'done' && <DoneStep />}
          </motion.div>
        </AnimatePresence>

        {/* Navigation */}
        <div className="flex gap-3 mt-6">
          {step !== 'welcome' && step !== 'done' && (
            <button
              onClick={prev}
              className="flex items-center gap-2 px-4 py-3 rounded-xl border border-gray-700 text-gray-400 hover:text-white hover:border-gray-500 transition-colors"
            >
              <ChevronLeft size={16} />
              Back
            </button>
          )}
          <button
            onClick={step === 'done' ? finish : next}
            disabled={validating}
            className="flex-1 flex items-center justify-center gap-2 px-6 py-3 rounded-xl bg-blue-600 hover:bg-blue-500 text-white font-medium transition-colors disabled:opacity-50"
          >
            {validating ? 'Validating...' : step === 'done' ? 'Launch AI Commerce HQ' : step === 'welcome' ? 'Get Started' : 'Continue'}
            {step !== 'done' && <ChevronRight size={16} />}
          </button>
        </div>

        {(step === 'printify' || step === 'optional') && (
          <p className="text-center text-gray-500 text-xs mt-3">
            You can skip optional steps and add credentials later in Settings
          </p>
        )}
      </div>
    </motion.div>
  )
}

function WelcomeStep() {
  return (
    <div>
      <h2 className="text-xl font-bold text-white mb-3">Welcome to your AI office</h2>
      <p className="text-gray-400 text-sm mb-6">
        AI Commerce HQ runs a fully autonomous e-commerce operation. Your AI agents will research niches,
        design products, prepare listings, and bring them to you for final approval.
      </p>
      <div className="space-y-3">
        {[
          { icon: '🤖', text: 'AI agents work 24/7 discovering profitable niches' },
          { icon: '🎨', text: 'Automatic product design and mockup generation' },
          { icon: '✅', text: 'You only approve — never publish without your consent' },
          { icon: '📈', text: 'Starts with Etsy, expands to 6 platforms' },
        ].map((item) => (
          <div key={item.text} className="flex items-start gap-3 p-3 rounded-lg bg-gray-800/50">
            <span className="text-lg flex-shrink-0">{item.icon}</span>
            <span className="text-sm text-gray-300">{item.text}</span>
          </div>
        ))}
      </div>
    </div>
  )
}

function DoneStep() {
  return (
    <div className="text-center py-4">
      <motion.div
        initial={{ scale: 0 }}
        animate={{ scale: 1 }}
        transition={{ type: 'spring', stiffness: 200 }}
      >
        <CheckCircle className="w-16 h-16 text-green-400 mx-auto mb-4" />
      </motion.div>
      <h2 className="text-xl font-bold text-white mb-2">Setup Complete!</h2>
      <p className="text-gray-400 text-sm">
        Your AI agents are ready to start working. The Etsy operations room will activate immediately.
        Other platforms will come online in future updates.
      </p>
    </div>
  )
}

interface FieldDef {
  label: string
  key: string
  placeholder: string
  value: string
  error?: string
  isText?: boolean
}

interface KeyStepProps {
  title: string
  description: string
  icon: string
  fields: FieldDef[]
  showKeys: Record<string, boolean>
  onToggle: (key: string) => void
  onChange: (key: any, value: string) => void
  optional?: boolean
}

function KeyStep({ title, description, icon, fields, showKeys, onToggle, onChange, optional }: KeyStepProps) {
  return (
    <div>
      <div className="flex items-center gap-3 mb-4">
        <span className="text-2xl">{icon}</span>
        <div>
          <h2 className="text-lg font-bold text-white">{title}</h2>
          {optional && <span className="text-xs text-blue-400 bg-blue-900/30 px-2 py-0.5 rounded-full">Optional</span>}
        </div>
      </div>
      <p className="text-gray-400 text-sm mb-6">{description}</p>
      <div className="space-y-4">
        {fields.map((field) => (
          <div key={field.key}>
            <label className="block text-sm text-gray-300 mb-1">{field.label}</label>
            <div className="relative">
              <input
                type={field.isText || showKeys[field.key] ? 'text' : 'password'}
                value={field.value}
                onChange={(e) => onChange(field.key, e.target.value)}
                placeholder={field.placeholder}
                className={`w-full bg-gray-900 border rounded-lg px-4 py-3 text-white placeholder-gray-600 focus:outline-none focus:border-blue-500 pr-12 font-mono text-sm ${
                  field.error ? 'border-red-500' : 'border-gray-700'
                }`}
              />
              {!field.isText && (
                <button
                  type="button"
                  onClick={() => onToggle(field.key)}
                  className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 hover:text-gray-300"
                >
                  {showKeys[field.key] ? <EyeOff size={16} /> : <Eye size={16} />}
                </button>
              )}
            </div>
            {field.error && (
              <div className="flex items-center gap-1 mt-1 text-red-400 text-xs">
                <AlertCircle size={12} />
                {field.error}
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  )
}
