import { motion } from 'framer-motion'
import type { AgentStatus } from '../../types'

interface Props {
  label: string
  status: AgentStatus
  size?: 'sm' | 'md' | 'lg'
  showStatus?: boolean
}

// Each agent has a distinct shirt / body colour so you can tell them apart at a glance
const AGENT_COLORS: Record<string, string> = {
  'TRD':   '#f59e0b',
  'DES-1': '#6366f1',
  'DES-2': '#8b5cf6',
  'PRD-1': '#06b6d4',
  'PRD-2': '#0ea5e9',
  'QA':    '#ef4444',
  'POD':   '#10b981',
  'LST':   '#f97316',
  'VID':   '#ec4899',
  'PER':   '#a78bfa',
  'ETMO':  '#ff6b35',
  'GMO':   '#3b82f6',
  'APPR':  '#22c55e',
}

const STATUS_LABELS: Record<AgentStatus, string> = {
  idle:     'Standby',
  working:  'Working',
  blocked:  'Blocked',
  complete: 'Done',
  entering: 'Arriving',
  offline:  'Offline',
}

// [svgWidth, svgHeight]
const SIZE_DIM: Record<string, [number, number]> = {
  sm: [38, 52],
  md: [48, 64],
  lg: [60, 80],
}

export function AgentAvatar({ label, status, size = 'md', showStatus = false }: Props) {
  const [W, H] = SIZE_DIM[size]

  const isWorking  = status === 'working'
  const isEntering = status === 'entering'
  const isBlocked  = status === 'blocked'
  const isStandby  = status === 'idle' || status === 'offline'
  const isActive   = !isStandby

  const bodyColor = isStandby ? '#4b5563' : (AGENT_COLORS[label] || '#3fb950')
  const skinColor = isStandby ? '#6b7280' : '#fde68a'

  // Proportional geometry derived from SVG width
  const HR  = W * 0.27              // head radius
  const HCX = W / 2                 // head center X
  const HCY = HR + 2                // head center Y (top pad)
  const BY  = HCY + HR + 2          // body top
  const BH  = H - BY - 2            // body height
  const BW  = W * 0.50              // body width
  const BX  = (W - BW) / 2          // body left
  const AW  = W * 0.16              // arm width
  const AH  = BH * 0.62             // arm height
  const EYE = Math.max(1.3, HR * 0.19) // eye radius

  return (
    <div className="relative flex flex-col items-center">
      <motion.svg
        width={W}
        height={H}
        viewBox={`0 0 ${W} ${H}`}
        style={{
          overflow: 'visible',
          filter: isStandby
            ? 'grayscale(0.4) brightness(0.75)'
            : isWorking
            ? `drop-shadow(0 0 ${W * 0.14}px ${bodyColor}bb)`
            : 'none',
        }}
        animate={
          isWorking  ? { scale: [1, 1.05, 1] } :
          isEntering ? { scale: [0.6, 1], opacity: [0, 1] } :
          isStandby  ? { y: [0, -2, 0] } :
          { y: [0, -3, 0] }
        }
        transition={
          isWorking  ? { duration: 2, repeat: Infinity } :
          isEntering ? { duration: 0.6 } :
          isStandby  ? { duration: 4, repeat: Infinity, ease: 'easeInOut' } :
          { duration: 2.5, repeat: Infinity, ease: 'easeInOut' }
        }
      >
        {/* Active glow halo around head */}
        {isActive && (
          <motion.circle
            cx={HCX} cy={HCY}
            r={HR + W * 0.2}
            fill="none"
            stroke={bodyColor}
            strokeWidth={1.2}
            animate={{ opacity: [0, 0.55, 0] }}
            transition={{ duration: 2.2, repeat: Infinity }}
          />
        )}

        {/* Head */}
        <circle cx={HCX} cy={HCY} r={HR} fill={skinColor} />

        {/* Hair arc */}
        <path
          d={`M ${HCX - HR * 0.88} ${HCY - HR * 0.52}
              Q ${HCX} ${HCY - HR * 1.18}
              ${HCX + HR * 0.88} ${HCY - HR * 0.52}`}
          fill={bodyColor}
          opacity={isStandby ? 0.35 : 0.9}
        />

        {/* Left eye */}
        <circle
          cx={HCX - HR * 0.36} cy={HCY - HR * 0.1}
          r={EYE}
          fill={isStandby ? '#374151' : '#1f2937'}
        />
        {/* Right eye */}
        <circle
          cx={HCX + HR * 0.36} cy={HCY - HR * 0.1}
          r={EYE}
          fill={isStandby ? '#374151' : '#1f2937'}
        />

        {/* Eye highlights (only when active) */}
        {isActive && (
          <>
            <circle cx={HCX - HR * 0.3}  cy={HCY - HR * 0.17} r={EYE * 0.45} fill="white" opacity={0.65} />
            <circle cx={HCX + HR * 0.42} cy={HCY - HR * 0.17} r={EYE * 0.45} fill="white" opacity={0.65} />
          </>
        )}

        {/* Mouth — smile when working, frown when blocked, neutral otherwise */}
        {isWorking ? (
          <path
            d={`M ${HCX - HR * 0.3} ${HCY + HR * 0.32}
                Q ${HCX} ${HCY + HR * 0.58}
                ${HCX + HR * 0.3} ${HCY + HR * 0.32}`}
            stroke="#92400e"
            strokeWidth={Math.max(1, HR * 0.16)}
            fill="none"
            strokeLinecap="round"
          />
        ) : isBlocked ? (
          <path
            d={`M ${HCX - HR * 0.28} ${HCY + HR * 0.44}
                Q ${HCX} ${HCY + HR * 0.28}
                ${HCX + HR * 0.28} ${HCY + HR * 0.44}`}
            stroke="#6b7280"
            strokeWidth={Math.max(1, HR * 0.15)}
            fill="none"
            strokeLinecap="round"
          />
        ) : (
          <line
            x1={HCX - HR * 0.24} y1={HCY + HR * 0.36}
            x2={HCX + HR * 0.24} y2={HCY + HR * 0.36}
            stroke="#6b7280"
            strokeWidth={Math.max(1, HR * 0.14)}
            strokeLinecap="round"
          />
        )}

        {/* Torso / shirt */}
        <rect
          x={BX} y={BY}
          width={BW} height={BH}
          rx={BW * 0.2}
          fill={bodyColor}
          opacity={isStandby ? 0.22 : 0.88}
        />

        {/* Left arm */}
        <rect
          x={BX - AW - 1} y={BY + 1}
          width={AW} height={AH}
          rx={AW * 0.4}
          fill={bodyColor}
          opacity={isStandby ? 0.18 : 0.65}
        />

        {/* Right arm */}
        <rect
          x={BX + BW + 1} y={BY + 1}
          width={AW} height={AH}
          rx={AW * 0.4}
          fill={bodyColor}
          opacity={isStandby ? 0.18 : 0.65}
        />

        {/* Status dot badge on shoulder */}
        <circle
          cx={HCX + HR * 0.82} cy={HCY + HR * 0.82}
          r={Math.max(2, W * 0.1)}
          fill={isStandby ? '#374151' : bodyColor}
          opacity={isStandby ? 0.35 : 1}
        />
      </motion.svg>

      {/* Typing / activity dots when working */}
      {isWorking && (
        <div className="flex gap-0.5 mt-0.5">
          {[0, 1, 2].map((i) => (
            <motion.div
              key={i}
              style={{ width: 4, height: 4, borderRadius: '50%', backgroundColor: bodyColor }}
              animate={{ opacity: [0, 1, 0] }}
              transition={{ duration: 0.8, repeat: Infinity, delay: i * 0.2 }}
            />
          ))}
        </div>
      )}

      {showStatus && (
        <span className="text-xs mt-1 font-medium" style={{ color: isStandby ? '#6b7280' : bodyColor }}>
          {STATUS_LABELS[status]}
        </span>
      )}
    </div>
  )
}
