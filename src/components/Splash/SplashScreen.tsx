import { useEffect, useRef, useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { listen } from '@tauri-apps/api/event'

interface Props {
  onComplete: () => void
}

const STAGES = [
  'Launching AI Commerce HQ...',
  'Starting backend services...',
  'Initializing agent runtime...',
  'Connecting to agent systems...',
  'Loading office environment...',
  'All systems online.',
]

export function SplashScreen({ onComplete }: Props) {
  const [stage, setStage] = useState(0)
  const [progress, setProgress] = useState(0)
  const completedRef = useRef(false)

  useEffect(() => {
    let unlisten: (() => void) | null = null

    const complete = () => {
      if (completedRef.current) return
      completedRef.current = true
      setProgress(100)
      setStage(STAGES.length - 1)
      setTimeout(onComplete, 700)
    }

    // Hard timeout — if backend never responds, unblock the user after 50 s
    const hardTimeout = setTimeout(complete, 50_000)

    // Listen for the Tauri backend-ready event emitted by main.rs
    listen<boolean>('backend-ready', (event) => {
      clearTimeout(hardTimeout)
      // Payload is true = healthy, false = timed out but we proceed either way
      complete()
    }).then((fn) => {
      unlisten = fn
    })

    // Animate progress to ~80 % while waiting for the backend.
    // Each tick = 100 ms, so we reach 80 in ~8 s — typical backend cold-start time.
    const progressTick = setInterval(() => {
      setProgress((p) => {
        if (p >= 80) {
          clearInterval(progressTick)
          return p
        }
        return p + 1
      })
    }, 100)

    // Cycle through stage labels every ~1.5 s (stops at second-to-last)
    const stageTick = setInterval(() => {
      setStage((s) => Math.min(s + 1, STAGES.length - 2))
    }, 1_500)

    return () => {
      clearTimeout(hardTimeout)
      clearInterval(progressTick)
      clearInterval(stageTick)
      unlisten?.()
    }
  }, [onComplete])

  return (
    <motion.div
      className="fixed inset-0 flex flex-col items-center justify-center bg-office-bg"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
      {/* Logo */}
      <motion.div
        className="mb-12 text-center"
        initial={{ y: -30, opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        transition={{ delay: 0.2 }}
      >
        <div className="relative inline-block">
          <div className="w-24 h-24 rounded-2xl bg-gradient-to-br from-blue-600 to-purple-700 flex items-center justify-center mb-6 mx-auto shadow-2xl">
            <span className="text-4xl">🏢</span>
          </div>
          <motion.div
            className="absolute inset-0 rounded-2xl"
            animate={{
              boxShadow: [
                '0 0 20px rgba(88, 166, 255, 0.3)',
                '0 0 60px rgba(88, 166, 255, 0.6)',
                '0 0 20px rgba(88, 166, 255, 0.3)',
              ],
            }}
            transition={{ duration: 2, repeat: Infinity }}
          />
        </div>
        <h1 className="text-4xl font-bold text-white tracking-tight">AI Commerce HQ</h1>
        <p className="text-blue-400 mt-2 text-sm tracking-widest uppercase">Autonomous E-Commerce Operations</p>
      </motion.div>

      {/* Progress */}
      <motion.div
        className="w-80"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ delay: 0.5 }}
      >
        <div className="flex justify-between text-xs text-gray-500 mb-2">
          <span>Loading</span>
          <span>{progress}%</span>
        </div>
        <div className="h-1 bg-gray-800 rounded-full overflow-hidden">
          <motion.div
            className="h-full bg-gradient-to-r from-blue-500 to-purple-500 rounded-full"
            style={{ width: `${progress}%` }}
            transition={{ duration: 0.15 }}
          />
        </div>

        <AnimatePresence mode="wait">
          <motion.p
            key={stage}
            className="text-xs text-gray-500 mt-3 text-center"
            initial={{ opacity: 0, y: 5 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -5 }}
          >
            {STAGES[stage]}
          </motion.p>
        </AnimatePresence>
      </motion.div>

      {/* Ambient floating dots */}
      {[...Array(6)].map((_, i) => (
        <motion.div
          key={i}
          className="absolute w-1 h-1 rounded-full bg-blue-500"
          style={{
            left: `${15 + i * 14}%`,
            top: `${20 + (i % 3) * 25}%`,
          }}
          animate={{ opacity: [0, 0.5, 0], scale: [0, 1.5, 0] }}
          transition={{ duration: 2 + i * 0.4, repeat: Infinity, delay: i * 0.3 }}
        />
      ))}
    </motion.div>
  )
}
