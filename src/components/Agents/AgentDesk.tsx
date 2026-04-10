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
}

const DESK_BG = 'rgba(33, 38, 45, 0.9)'
const APPROVAL_BG = 'rgba(63, 78, 50, 0.9)'

export function AgentDesk({ nameplate, agentLabel, agent, isApprovalDesk, isNew, onClick }: Props) {
  const status: AgentStatus = agent?.status || 'idle'

  const statusColors: Record<AgentStatus, string> = {
    idle: 'border-blue-500/30',
    working: 'border-green-500/50',
    blocked: 'border-red-500/50',
    complete: 'border-blue-300/50',
    entering: 'border-yellow-500/50',
    offline: 'border-gray-600/30',
  }

  return (
    <motion.div
      initial={isNew ? { scale: 0, rotate: -10, opacity: 0 } : false}
      animate={{ scale: 1, rotate: 0, opacity: 1 }}
      transition={isNew ? { type: 'spring', stiffness: 200, damping: 15 } : undefined}
      onClick={onClick}
      className={`relative rounded-xl border-2 p-3 cursor-pointer transition-all select-none
        ${isApprovalDesk ? 'border-green-600/50 hover:border-green-400' : statusColors[status]}
        hover:brightness-110`}
      style={{
        background: isApprovalDesk ? APPROVAL_BG : DESK_BG,
        minWidth: '110px',
        minHeight: '100px',
      }}
      whileHover={{ scale: 1.03 }}
      whileTap={{ scale: 0.97 }}
    >
      {/* Screen/monitor on desk */}
      <div className="w-full h-8 bg-gray-900 rounded-md mb-2 flex items-center justify-center relative overflow-hidden">
        {agent?.status === 'working' && (
          <motion.div
            className="absolute inset-0 bg-green-500/10"
            animate={{ opacity: [0, 0.3, 0] }}
            transition={{ duration: 1, repeat: Infinity }}
          />
        )}
        {isApprovalDesk ? (
          <span className="text-xs text-green-400 font-mono">APPROVAL</span>
        ) : agent?.currentTask ? (
          <span className="text-xs text-green-300 font-mono truncate px-1">{agent.currentTask}</span>
        ) : (
          <span className="text-xs text-gray-600 font-mono">standby</span>
        )}
      </div>

      {/* Avatar */}
      <div className="flex justify-center mb-2">
        <AgentAvatar
          label={agentLabel}
          status={isApprovalDesk ? 'idle' : status}
          size="sm"
        />
      </div>

      {/* Nameplate */}
      <div className="text-center">
        <div className="text-xs font-bold text-white/90 leading-tight truncate">{agentLabel}</div>
        <div className="text-xs text-gray-500 leading-tight truncate mt-0.5">{nameplate}</div>
      </div>

      {/* Approval indicator */}
      {isApprovalDesk && (
        <div className="absolute top-2 right-2">
          <motion.div
            className="w-2 h-2 rounded-full bg-green-400"
            animate={{ opacity: [1, 0.3, 1] }}
            transition={{ duration: 2, repeat: Infinity }}
          />
        </div>
      )}

      {/* Task count */}
      {agent && agent.taskCount > 0 && (
        <div className="absolute top-2 right-2 bg-blue-600 text-white text-xs rounded-full w-5 h-5 flex items-center justify-center font-bold">
          {agent.taskCount > 9 ? '9+' : agent.taskCount}
        </div>
      )}
    </motion.div>
  )
}
