import { useEffect, useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'

interface Props {
  onComplete: () => void
}

const STAGES = [
  'Initializing AI Commerce HQ...',
  'Loading orchestration runtime...',
  'Connecting agent systems...',
  'Verifying configuration...',
  'Starting office environment...',
]

export function SplashScreen({ onComplete }: Props) {
  const [stage, setStage] = useState(0)
  const [progress, setProgress] = useState(0)

  useEffect(() => {
    const interval = setInterval(() => {
      setProgress((p) => {
        if (p >= 100) {
          clearInterval(interval)
          setTimeout(onComplete, 400)
          return 100
        }
        return p + 2
      })
    }, 40)

    const stageInterval = setInterval(() => {
      setStage((s) => Math.min(s + 1, STAGES.length - 1))
    }, 400)

    return () => {
      clearInterval(interval)
      clearInterval(stageInterval)
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
            transition={{ duration: 0.1 }}
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

      {/* Floating dots */}
      {[...Array(6)].map((_, i) => (
        <motion.div
          key={i}
          className="absolute w-1 h-1 rounded-full bg-blue-500"
          style={{
            left: `${10 + Math.random() * 80}%`,
            top: `${10 + Math.random() * 80}%`,
          }}
          animate={{
            opacity: [0, 0.5, 0],
            scale: [0, 1.5, 0],
          }}
          transition={{
            duration: 2 + Math.random() * 2,
            repeat: Infinity,
            delay: Math.random() * 2,
          }}
        />
      ))}
    </motion.div>
  )
}
