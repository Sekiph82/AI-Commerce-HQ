import { motion } from 'framer-motion'
import type { AgentStatus } from '../../types'

interface Props {
  label: string
  status: AgentStatus
  size?: 'sm' | 'md' | 'lg'
  showStatus?: boolean
}

const STATUS_COLORS: Record<AgentStatus, string> = {
  idle: '#58a6ff',
  working: '#3fb950',
  blocked: '#f85149',
  complete: '#a5d6ff',
  entering: '#e3b341',
  offline: '#484f58',
}

const STATUS_LABELS: Record<AgentStatus, string> = {
  idle: 'Idle',
  working: 'Working',
  blocked: 'Blocked',
  complete: 'Done',
  entering: 'Entering',
  offline: 'Offline',
}

const SIZE_MAP = {
  sm: 28,
  md: 36,
  lg: 48,
}

export function AgentAvatar({ label, status, size = 'md', showStatus = false }: Props) {
  const px = SIZE_MAP[size]
  const color = STATUS_COLORS[status]

  return (
    <div className="relative flex flex-col items-center">
      <motion.div
        className="rounded-full flex items-center justify-center font-bold text-white relative"
        style={{
          width: px,
          height: px,
          backgroundColor: color + '22',
          border: `2px solid ${color}`,
          fontSize: px * 0.35,
        }}
        animate={
          status === 'working'
            ? { scale: [1, 1.05, 1], boxShadow: [`0 0 8px ${color}55`, `0 0 20px ${color}88`, `0 0 8px ${color}55`] }
            : status === 'blocked'
            ? { boxShadow: [`0 0 8px ${color}55`, `0 0 20px ${color}88`, `0 0 8px ${color}55`] }
            : status === 'idle'
            ? { y: [0, -2, 0] }
            : {}
        }
        transition={
          status === 'working' || status === 'blocked'
            ? { duration: 1.5, repeat: Infinity }
            : status === 'idle'
            ? { duration: 3, repeat: Infinity }
            : {}
        }
      >
        {label.slice(0, 2)}

        {/* Status dot */}
        <div
          className="absolute -bottom-0.5 -right-0.5 rounded-full border border-gray-900"
          style={{
            width: px * 0.28,
            height: px * 0.28,
            backgroundColor: color,
          }}
        />
      </motion.div>

      {showStatus && (
        <span className="text-xs mt-1 font-medium" style={{ color }}>
          {STATUS_LABELS[status]}
        </span>
      )}

      {/* Typing indicator when working */}
      {status === 'working' && (
        <div className="flex gap-0.5 mt-1">
          {[0, 1, 2].map((i) => (
            <motion.div
              key={i}
              className="w-1 h-1 rounded-full bg-green-400"
              animate={{ opacity: [0, 1, 0] }}
              transition={{ duration: 0.8, repeat: Infinity, delay: i * 0.2 }}
            />
          ))}
        </div>
      )}
    </div>
  )
}
