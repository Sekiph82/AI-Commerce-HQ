import { motion } from 'framer-motion'
import type { ReactNode } from 'react'

interface Props {
  title: string
  subtitle?: string
  color: string
  isActive: boolean
  children: ReactNode
  onApprovalClick?: () => void
  badge?: string
  style?: React.CSSProperties
}

export function RoomBase({ title, subtitle, color, isActive, children, badge, style }: Props) {
  return (
    <motion.div
      className={`relative rounded-2xl border overflow-hidden flex flex-col ${
        isActive ? 'room-active border-gray-600' : 'room-dormant border-gray-800'
      }`}
      style={{
        background: `linear-gradient(135deg, ${color}22, ${color}11)`,
        borderColor: isActive ? color + '44' : undefined,
        ...style,
      }}
      initial={{ opacity: 0, scale: 0.95 }}
      animate={{ opacity: 1, scale: 1 }}
      transition={{ duration: 0.4 }}
    >
      {/* Room header */}
      <div
        className="px-4 py-2 flex items-center justify-between border-b"
        style={{
          background: `${color}22`,
          borderColor: color + '33',
        }}
      >
        <div>
          <h3 className="font-bold text-white text-sm">{title}</h3>
          {subtitle && <p className="text-xs text-gray-400">{subtitle}</p>}
        </div>
        <div className="flex items-center gap-2">
          {badge && (
            <span className="text-xs px-2 py-0.5 rounded-full bg-blue-600/30 text-blue-300 border border-blue-600/30">
              {badge}
            </span>
          )}
          <div
            className={`w-2 h-2 rounded-full ${isActive ? 'bg-green-400' : 'bg-gray-600'}`}
          />
        </div>
      </div>

      {/* Dormant overlay */}
      {!isActive && (
        <div className="absolute inset-0 z-10 flex flex-col items-center justify-center bg-black/50 pointer-events-none">
          <span className="text-gray-500 font-bold text-sm tracking-widest uppercase">Coming Soon</span>
          <span className="text-gray-600 text-xs mt-1">Activation in future release</span>
        </div>
      )}

      {/* Content */}
      <div className="flex-1 p-3 overflow-hidden">
        {children}
      </div>

      {/* Active glow line at top */}
      {isActive && (
        <motion.div
          className="absolute top-0 left-0 right-0 h-0.5"
          style={{ background: `linear-gradient(90deg, transparent, ${color}, transparent)` }}
          animate={{ opacity: [0.5, 1, 0.5] }}
          transition={{ duration: 2, repeat: Infinity }}
        />
      )}
    </motion.div>
  )
}
