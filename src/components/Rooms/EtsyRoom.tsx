import { motion, AnimatePresence } from 'framer-motion'
import { useAppStore } from '../../store/useAppStore'
import { RoomBase } from './RoomBase'
import { AgentDesk } from '../Agents/AgentDesk'
import { AgentAvatar } from '../Agents/AgentAvatar'
import type { Agent } from '../../types'

// ── Desk grid definition ────────────────────────────────────────────────────
// Top row:    DES-1  PRD-1  TRD  PER  QA
// Middle row: ETMO (center) + Approval Desk
// Bottom row: DES-2  PRD-2  VID  POD  LST

const DESK_META: Record<string, { nameplate: string; deskId: string }> = {
  'DES-1': { nameplate: 'Primary Design',            deskId: 'desk-etsy-des1' },
  'PRD-1': { nameplate: 'Primary Product',           deskId: 'desk-etsy-prd1' },
  'TRD':   { nameplate: 'Trend Research',            deskId: 'desk-etsy-trd'  },
  'PER':   { nameplate: 'Personalization',           deskId: 'desk-etsy-per'  },
  'QA':    { nameplate: 'Quality Assurance',         deskId: 'desk-etsy-qa'   },
  'DES-2': { nameplate: 'Variant Design',            deskId: 'desk-etsy-des2' },
  'PRD-2': { nameplate: 'Alt. Product',              deskId: 'desk-etsy-prd2' },
  'VID':   { nameplate: 'Video',                     deskId: 'desk-etsy-vid'  },
  'POD':   { nameplate: 'POD / Mockup',              deskId: 'desk-etsy-pod'  },
  'LST':   { nameplate: 'Listing / SEO',             deskId: 'desk-etsy-lst'  },
}

const TOP_ROW    = ['DES-1', 'PRD-1', 'TRD', 'PER', 'QA']    as const
const BOTTOM_ROW = ['DES-2', 'PRD-2', 'VID', 'POD', 'LST'] as const

// Walking overlay positions — percentage of the container dimensions
const DESK_WALK_POSITIONS: Record<string, { x: number; y: number }> = {
  'desk-etmo-main':  { x: 50,  y: 50  },
  'desk-etsy-des1':  { x: 10,  y: 14  },
  'desk-etsy-prd1':  { x: 27,  y: 14  },
  'desk-etsy-trd':   { x: 45,  y: 14  },
  'desk-etsy-per':   { x: 63,  y: 14  },
  'desk-etsy-qa':    { x: 80,  y: 14  },
  'desk-etsy-des2':  { x: 10,  y: 86  },
  'desk-etsy-prd2':  { x: 27,  y: 86  },
  'desk-etsy-vid':   { x: 45,  y: 86  },
  'desk-etsy-pod':   { x: 63,  y: 86  },
  'desk-etsy-lst':   { x: 80,  y: 86  },
}

// Per-agent walking/active colour
const WALK_COLOR: Record<string, string> = {
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
}

function getAgentByLabel(agents: Record<string, Agent>, label: string): Agent | undefined {
  return Object.values(agents).find((a) => a.label === label)
}

// ── Walking character marker ────────────────────────────────────────────────
function WalkingMarker({
  agentLabel, fromDesk, toDesk, durationMs,
}: {
  agentLabel: string
  fromDesk: string
  toDesk: string
  durationMs: number
}) {
  const from = DESK_WALK_POSITIONS[fromDesk]
  const to   = DESK_WALK_POSITIONS[toDesk]
  if (!from || !to) return null

  const color = WALK_COLOR[agentLabel] || '#94a3b8'

  return (
    <motion.div
      className="absolute pointer-events-none"
      style={{ transform: 'translate(-50%, -50%)', zIndex: 30 }}
      initial={{ left: `${from.x}%`, top: `${from.y}%`, opacity: 0, scale: 0.8 }}
      animate={{ left: `${to.x}%`, top: `${to.y}%`, opacity: 1, scale: 1 }}
      exit={{ opacity: 0, scale: 0.5 }}
      transition={{ duration: durationMs / 1000, ease: 'linear' }}
    >
      {/* Vertical bobbing */}
      <motion.div
        animate={{ y: [0, -4, 0] }}
        transition={{ duration: 0.42, repeat: Infinity, ease: 'easeInOut' }}
      >
        <svg
          width="38"
          height="52"
          viewBox="0 0 28 38"
          style={{ filter: `drop-shadow(0 0 8px ${color}dd)` }}
        >
          {/* Glow ring */}
          <circle cx="14" cy="8" r="10" fill="none" stroke={color} strokeWidth="1" opacity="0.35" />

          {/* Head */}
          <circle cx="14" cy="8" r="6.5" fill={color} opacity={0.95} />

          {/* Eyes */}
          <circle cx="11.8" cy="7.5" r="1.3" fill="#000" opacity={0.65} />
          <circle cx="16.2" cy="7.5" r="1.3" fill="#000" opacity={0.65} />

          {/* Body */}
          <rect x="9" y="15.5" width="10" height="11" rx="2.5" fill={color} opacity={0.85} />

          {/* Left leg — scaleY from bottom to simulate stepping */}
          <motion.rect
            x="9.5" y="26" width="4" height="10" rx="1.8"
            fill={color} opacity={0.78}
            animate={{ scaleY: [1, 0.45, 1] }}
            transition={{ duration: 0.4, repeat: Infinity, ease: 'easeInOut' }}
            style={{ transformBox: 'fill-box', transformOrigin: 'bottom center' }}
          />
          {/* Right leg — opposite phase */}
          <motion.rect
            x="14.5" y="26" width="4" height="10" rx="1.8"
            fill={color} opacity={0.78}
            animate={{ scaleY: [0.45, 1, 0.45] }}
            transition={{ duration: 0.4, repeat: Infinity, ease: 'easeInOut' }}
            style={{ transformBox: 'fill-box', transformOrigin: 'bottom center' }}
          />
        </svg>
      </motion.div>

      {/* Label badge below character */}
      <div
        style={{
          position: 'absolute',
          bottom: -18,
          left: '50%',
          transform: 'translateX(-50%)',
          background: color,
          color: '#000',
          fontSize: '9px',
          fontWeight: 800,
          padding: '2px 6px',
          borderRadius: '4px',
          whiteSpace: 'nowrap',
          opacity: 0.95,
          letterSpacing: '0.04em',
          boxShadow: `0 2px 8px ${color}88`,
        }}
      >
        {agentLabel}
      </div>
    </motion.div>
  )
}

// ── ETMO center desk ────────────────────────────────────────────────────────
function ETMODesk({ agent, onClick }: { agent?: Agent; onClick?: () => void }) {
  const isActive  = agent?.status === 'working' || agent?.status === 'entering'
  const isStandby = !agent || agent.status === 'idle' || agent.status === 'offline'

  return (
    <motion.div
      className="relative rounded-2xl border-2 cursor-pointer select-none"
      style={{
        background: isActive ? 'rgba(79,45,30,0.9)' : 'rgba(40,28,20,0.85)',
        borderColor: isActive ? 'rgba(255,107,53,0.7)' : 'rgba(120,80,40,0.3)',
        padding: '10px 16px',
        minWidth: '160px',
        filter: isStandby ? 'saturate(0.3) brightness(0.7)' : 'none',
      }}
      animate={isActive ? {
        boxShadow: [
          '0 0 12px rgba(255,107,53,0.2)',
          '0 0 32px rgba(255,107,53,0.45)',
          '0 0 12px rgba(255,107,53,0.2)',
        ],
      } : {}}
      transition={isActive ? { duration: 2, repeat: Infinity } : {}}
      whileHover={{ scale: 1.03, filter: 'saturate(1) brightness(1)' }}
      onClick={onClick}
    >
      {/* Monitor screen */}
      <div
        className="w-full rounded mb-2 flex items-center justify-center relative overflow-hidden"
        style={{ height: '22px', background: 'rgba(0,0,0,0.6)' }}
      >
        {isActive ? (
          <>
            <motion.div
              className="absolute inset-0 rounded"
              style={{ background: 'rgba(255,107,53,0.08)' }}
              animate={{ opacity: [0, 0.5, 0] }}
              transition={{ duration: 1.5, repeat: Infinity }}
            />
            <span className="text-xs text-orange-300 font-mono truncate px-1" style={{ fontSize: '9px' }}>
              {agent?.currentTask || 'Operating'}
            </span>
          </>
        ) : (
          <span className="text-xs text-gray-600 font-mono" style={{ fontSize: '9px' }}>standby</span>
        )}
      </div>

      {/* Character + info side by side */}
      <div className="flex items-center gap-3">
        <AgentAvatar label="ETMO" status={agent?.status ?? 'idle'} size="md" />
        <div>
          <div className="font-bold text-sm" style={{ color: isActive ? '#fff' : '#6b7280' }}>ETMO</div>
          <div className="text-xs" style={{ color: isActive ? '#9ca3af' : '#4b5563', fontSize: '9px' }}>
            Etsy Master Orchestrator
          </div>
          {isActive && agent?.currentTask && (
            <div style={{ fontSize: '8px', color: '#f97316', marginTop: '2px', maxWidth: '90px' }}
              className="truncate">
              {agent.currentTask}
            </div>
          )}
        </div>
      </div>

      {/* Working dots */}
      {isActive && (
        <div className="flex gap-1 justify-center mt-2">
          {[0, 1, 2].map((i) => (
            <motion.div
              key={i}
              className="w-1 h-1 rounded-full bg-orange-400"
              animate={{ opacity: [0, 1, 0] }}
              transition={{ duration: 0.9, repeat: Infinity, delay: i * 0.25 }}
            />
          ))}
        </div>
      )}
    </motion.div>
  )
}

// ── Individual desk cell ────────────────────────────────────────────────────
function DeskCell({ label, nameplate, agents }: { label: string; nameplate: string; agents: Record<string, Agent> }) {
  const agent = getAgentByLabel(agents, label)
  return <AgentDesk nameplate={nameplate} agentLabel={label} agent={agent} compact />
}

// ── Main EtsyRoom ───────────────────────────────────────────────────────────
export function EtsyRoom() {
  const { agents, walkingAgents, openApproval } = useAppStore()

  const etmoAgent = getAgentByLabel(agents, 'ETMO')

  const pendingCount = useAppStore((s) =>
    Object.values(s.products).filter(
      (p) => p.state === 'AWAITING_HUMAN_DECISION' && p.platform === 'etsy'
    ).length
  )

  return (
    <RoomBase
      title="Etsy Operations"
      subtitle="v1 — Primary active platform — 10-agent fixed team"
      color="#ff6b35"
      isActive={true}
      badge="LIVE"
    >
      {/* Relative container — walking overlay is positioned here */}
      <div className="relative flex flex-col h-full">

        {/* ── Walking characters overlay ─────────────────────────────── */}
        <div className="absolute inset-0 pointer-events-none" style={{ zIndex: 20 }}>
          <AnimatePresence mode="popLayout">
            {Object.values(walkingAgents).map((walk) => (
              <WalkingMarker
                key={`walk-${walk.agentLabel}`}
                agentLabel={walk.agentLabel}
                fromDesk={walk.fromDesk}
                toDesk={walk.toDesk}
                durationMs={walk.durationMs}
              />
            ))}
          </AnimatePresence>
        </div>

        {/* ── Top row ────────────────────────────────────────────────── */}
        <div className="flex-none py-2">
          <div className="text-center mb-1.5" style={{ fontSize: '8px', color: '#4b5563', letterSpacing: '0.1em' }}>
            DESIGN · PRODUCT · RESEARCH · PERSONALIZATION · QA
          </div>
          <div className="flex justify-around gap-1">
            {TOP_ROW.map((label) => (
              <DeskCell key={label} label={label} nameplate={DESK_META[label].nameplate} agents={agents} />
            ))}
          </div>
        </div>

        {/* ── Corridor ─────────────────────────────────────────────── */}
        <div className="flex-none flex items-center gap-2 py-1 px-4">
          <div className="flex-1 h-px" style={{ background: 'rgba(255,107,53,0.12)' }} />
          <div style={{ fontSize: '8px', color: '#374151' }}>CORRIDOR</div>
          <div className="flex-1 h-px" style={{ background: 'rgba(255,107,53,0.12)' }} />
        </div>

        {/* ── Middle row — ETMO + Approval ─────────────────────────── */}
        <div className="flex-none flex justify-center items-center gap-6 py-2">
          <ETMODesk agent={etmoAgent} />

          {/* Approval Desk */}
          <div className="flex flex-col items-center gap-1.5">
            <AgentDesk
              nameplate="Etsy Approval Desk"
              agentLabel="APPR"
              isApprovalDesk
              onClick={() => openApproval('etsy')}
            />
            {pendingCount > 0 && (
              <motion.div
                className="flex items-center gap-1.5 rounded-full px-2 py-0.5"
                style={{ background: 'rgba(234,88,12,0.3)', border: '1px solid rgba(234,88,12,0.5)' }}
                animate={{ scale: [1, 1.05, 1] }}
                transition={{ duration: 2, repeat: Infinity }}
              >
                <motion.div
                  className="w-2 h-2 rounded-full bg-orange-400"
                  animate={{ opacity: [1, 0.4, 1] }}
                  transition={{ duration: 1, repeat: Infinity }}
                />
                <span style={{ fontSize: '9px', color: '#fb923c', fontWeight: 700 }}>
                  {pendingCount} awaiting
                </span>
              </motion.div>
            )}
          </div>
        </div>

        {/* ── Corridor ─────────────────────────────────────────────── */}
        <div className="flex-none flex items-center gap-2 py-1 px-4">
          <div className="flex-1 h-px" style={{ background: 'rgba(255,107,53,0.12)' }} />
          <div style={{ fontSize: '8px', color: '#374151' }}>CORRIDOR</div>
          <div className="flex-1 h-px" style={{ background: 'rgba(255,107,53,0.12)' }} />
        </div>

        {/* ── Bottom row ────────────────────────────────────────────── */}
        <div className="flex-none py-2">
          <div className="flex justify-around gap-1">
            {BOTTOM_ROW.map((label) => (
              <DeskCell key={label} label={label} nameplate={DESK_META[label].nameplate} agents={agents} />
            ))}
          </div>
          <div className="text-center mt-1.5" style={{ fontSize: '8px', color: '#4b5563', letterSpacing: '0.1em' }}>
            VARIANT · ALT-PRODUCT · VIDEO · MOCKUP · LISTING
          </div>
        </div>

      </div>
    </RoomBase>
  )
}
