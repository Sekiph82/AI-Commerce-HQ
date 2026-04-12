import { motion } from 'framer-motion'
import { AgentAvatar } from './AgentAvatar'
import type { Agent, AgentStatus } from '../../types'

interface Props {
  nameplate: string
  agentLabel: string
  agent?: Agent
  isApprovalDesk?: boolean
  isNew?: boolean
  onClick?: () => void
  compact?: boolean
}

export function AgentDesk({ nameplate, agentLabel, agent, isApprovalDesk, isNew, onClick, compact }: Props) {
  const status: AgentStatus = agent?.status || 'idle'
  const isStandby = status === 'idle' || status === 'offline' || !agent
  const isWorking = status === 'working'

  // Standby = muted gray. Active = colored border.
  const borderColor = isApprovalDesk
    ? 'rgba(34,197,94,0.5)'
    : isWorking
    ? 'rgba(63,185,80,0.6)'
    : status === 'blocked'
    ? 'rgba(248,81,73,0.5)'
    : status === 'complete'
    ? 'rgba(165,214,255,0.4)'
    : status === 'entering'
    ? 'rgba(227,179,65,0.5)'
    : 'rgba(75,85,99,0.3)'

  const bgColor = isApprovalDesk
    ? 'rgba(39,68,38,0.85)'
    : isStandby
    ? 'rgba(26,32,40,0.7)'
    : 'rgba(26,38,28,0.85)'

  const minW = compact ? '100px' : '120px'
  const minH = compact ? '130px' : '148px'

  return (
    <motion.div
      initial={isNew ? { scale: 0, opacity: 0 } : false}
      animate={{ scale: 1, opacity: 1 }}
      transition={isNew ? { type: 'spring', stiffness: 250, damping: 18 } : undefined}
      onClick={onClick}
      className="relative rounded-xl border-2 p-2 cursor-pointer select-none transition-colors"
      style={{
        background: bgColor,
        borderColor,
        minWidth: minW,
        minHeight: minH,
        filter: isStandby && !isApprovalDesk ? 'saturate(0.3) brightness(0.75)' : 'none',
      }}
      whileHover={{ scale: 1.04, filter: 'saturate(1) brightness(1)' }}
      whileTap={{ scale: 0.97 }}
    >
      {/* Monitor screen */}
      <div
        className="w-full rounded-md mb-2 flex items-center justify-center relative overflow-hidden"
        style={{ height: compact ? '22px' : '28px', background: 'rgba(0,0,0,0.6)' }}
      >
        {isWorking && (
          <motion.div
            className="absolute inset-0 bg-green-500/10"
            animate={{ opacity: [0, 0.4, 0] }}
            transition={{ duration: 1.2, repeat: Infinity }}
          />
        )}
        {isApprovalDesk ? (
          <span className="text-xs text-green-400 font-mono tracking-wider">APPROVAL</span>
        ) : agent?.currentTask ? (
          <span className="text-xs text-green-300 font-mono truncate px-1 leading-tight" style={{ fontSize: '9px' }}>
            {agent.currentTask}
          </span>
        ) : (
          <span className="text-xs text-gray-600 font-mono" style={{ fontSize: '9px' }}>standby</span>
        )}
      </div>

      {/* Avatar */}
      <div className="flex justify-center mb-1.5">
        <AgentAvatar
          label={agentLabel}
          status={isApprovalDesk ? 'idle' : status}
          size="sm"
        />
      </div>

      {/* Label */}
      <div className="text-center">
        <div
          className="font-bold leading-tight truncate"
          style={{
            fontSize: '11px',
            color: isStandby && !isApprovalDesk ? '#9ca3af' : '#f3f4f6',
          }}
        >
          {agentLabel}
        </div>
        <div
          className="leading-tight truncate mt-0.5"
          style={{ fontSize: '8px', color: isStandby ? '#6b7280' : '#9ca3af' }}
        >
          {nameplate}
        </div>
      </div>

      {/* Approval pulse */}
      {isApprovalDesk && (
        <div className="absolute top-1.5 right-1.5">
          <motion.div
            className="w-2 h-2 rounded-full bg-green-400"
            animate={{ opacity: [1, 0.3, 1] }}
            transition={{ duration: 2, repeat: Infinity }}
          />
        </div>
      )}

      {/* Task count badge */}
      {agent && agent.taskCount > 0 && !isStandby && (
        <div className="absolute top-1.5 right-1.5 bg-blue-600 text-white rounded-full w-4 h-4 flex items-center justify-center font-bold"
          style={{ fontSize: '8px' }}>
          {agent.taskCount > 9 ? '9+' : agent.taskCount}
        </div>
      )}
    </motion.div>
  )
}
