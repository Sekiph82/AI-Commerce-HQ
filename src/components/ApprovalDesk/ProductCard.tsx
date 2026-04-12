import { motion } from 'framer-motion'
import { CheckCircle, XCircle, Archive, TrendingUp, AlertTriangle, Clock, Tag } from 'lucide-react'
import type { Product } from '../../types'

// ── Design placeholder rendered when no mockup/design image is available ──
function DesignPlaceholder({ product }: { product: Product }) {
  // Deterministic gradient derived from the niche string
  const hash = product.niche.split('').reduce((a, c) => a + c.charCodeAt(0), 0)
  const hue1 = hash % 360
  const hue2 = (hue1 + 45) % 360
  const accent = `hsl(${hue1}, 75%, 62%)`

  const getEmoji = (niche: string) => {
    const n = niche.toLowerCase()
    if (n.includes('floral') || n.includes('flower') || n.includes('botanical')) return '🌸'
    if (n.includes('nature') || n.includes('forest') || n.includes('tree') || n.includes('leaf')) return '🌿'
    if (n.includes('galaxy') || n.includes('space') || n.includes('star') || n.includes('cosmos')) return '✨'
    if (n.includes('cat') || n.includes('dog') || n.includes('pet') || n.includes('animal')) return '🐾'
    if (n.includes('coffee') || n.includes('kitchen') || n.includes('food')) return '☕'
    if (n.includes('vintage') || n.includes('retro') || n.includes('classic')) return '🎞️'
    if (n.includes('motivat') || n.includes('inspir') || n.includes('quote') || n.includes('typography')) return '💬'
    if (n.includes('holiday') || n.includes('christmas') || n.includes('seasonal')) return '🎄'
    if (n.includes('abstract') || n.includes('geometric') || n.includes('modern')) return '◻️'
    if (n.includes('ocean') || n.includes('beach') || n.includes('sea') || n.includes('wave')) return '🌊'
    if (n.includes('mountain') || n.includes('hiking') || n.includes('outdoor')) return '⛰️'
    if (n.includes('music') || n.includes('guitar') || n.includes('piano')) return '🎵'
    return '🎨'
  }

  return (
    <div
      className="w-full h-44 relative flex flex-col items-center justify-center overflow-hidden border-b border-gray-700"
      style={{
        background: `linear-gradient(145deg, hsl(${hue1},55%,14%), hsl(${hue2},65%,10%))`,
      }}
    >
      {/* Decorative corner accents */}
      <div style={{ position: 'absolute', top: 10, left: 10, width: 36, height: 36, borderTop: `2px solid ${accent}44`, borderLeft: `2px solid ${accent}44`, borderRadius: '4px 0 0 0' }} />
      <div style={{ position: 'absolute', bottom: 10, right: 10, width: 36, height: 36, borderBottom: `2px solid ${accent}44`, borderRight: `2px solid ${accent}44`, borderRadius: '0 0 4px 0' }} />

      {/* Subtle grid pattern */}
      <div style={{
        position: 'absolute', inset: 0, opacity: 0.06,
        backgroundImage: `linear-gradient(${accent} 1px, transparent 1px), linear-gradient(90deg, ${accent} 1px, transparent 1px)`,
        backgroundSize: '24px 24px',
      }} />

      {/* Content */}
      <div className="relative text-center px-6">
        <div style={{ fontSize: '44px', marginBottom: '8px', lineHeight: 1, filter: 'drop-shadow(0 2px 8px rgba(0,0,0,0.5))' }}>
          {getEmoji(product.niche)}
        </div>
        <div style={{ color: accent, fontWeight: 700, fontSize: '12px', letterSpacing: '0.05em', marginBottom: '5px' }}>
          {product.niche}
        </div>
        {product.designPrompt && (
          <div style={{ color: 'rgba(255,255,255,0.5)', fontSize: '9px', lineHeight: '1.45', maxWidth: '230px', margin: '0 auto' }}>
            {product.designPrompt.length > 100
              ? product.designPrompt.slice(0, 100) + '…'
              : product.designPrompt}
          </div>
        )}
      </div>

      {/* Top-right tag chips */}
      {product.tags && product.tags.length > 0 && (
        <div style={{ position: 'absolute', top: 8, right: 8, display: 'flex', gap: 3 }}>
          {product.tags.slice(0, 2).map((tag) => (
            <span key={tag} style={{
              fontSize: '7px', padding: '1px 4px', borderRadius: 3,
              background: `${accent}22`, color: accent, border: `1px solid ${accent}44`,
              whiteSpace: 'nowrap',
            }}>
              {tag}
            </span>
          ))}
        </div>
      )}

      {/* Watermark */}
      <div style={{
        position: 'absolute', bottom: 7, right: 9,
        fontSize: '7.5px', color: 'rgba(255,255,255,0.22)',
        fontStyle: 'italic', letterSpacing: '0.04em',
      }}>
        AI Concept
      </div>
    </div>
  )
}

interface Props {
  product: Product
  onApprove: () => void
  onReject: () => void
  onArchive: () => void
  onKeepPending?: () => void
}

const STATE_LABELS: Record<string, string> = {
  IDEA_DISCOVERED: 'Idea Found',
  BRIEF_CREATED: 'Brief Ready',
  DESIGN_GENERATED: 'Design Generated',
  DESIGN_QA_PASSED: 'QA Passed',
  POD_PRODUCT_PREPARED: 'POD Ready',
  ETSY_DRAFT_CREATED: 'Draft on Etsy',
  APPROVAL_PACKET_READY: 'Packet Ready',
  AWAITING_HUMAN_DECISION: 'Awaiting Decision',
  APPROVED_FOR_PUBLISH: 'Approved',
  REJECTED: 'Rejected',
  ARCHIVED: 'Archived',
}

const STATE_COLORS: Record<string, string> = {
  AWAITING_HUMAN_DECISION: 'text-orange-400 bg-orange-900/30 border-orange-700/50',
  APPROVED_FOR_PUBLISH: 'text-green-400 bg-green-900/30 border-green-700/50',
  ETSY_DRAFT_CREATED: 'text-blue-400 bg-blue-900/30 border-blue-700/50',
  REJECTED: 'text-red-400 bg-red-900/30 border-red-700/50',
  ARCHIVED: 'text-gray-400 bg-gray-900/30 border-gray-700/50',
}

export function ProductCard({ product, onApprove, onReject, onArchive }: Props) {
  const isActionable = product.state === 'AWAITING_HUMAN_DECISION'
  const stateColor = STATE_COLORS[product.state] || 'text-blue-400 bg-blue-900/30 border-blue-700/50'

  return (
    <motion.div
      className="bg-gray-900 border border-gray-700 rounded-xl overflow-hidden"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      layout
    >
      {/* Header */}
      <div className="p-4 border-b border-gray-700">
        <div className="flex items-start justify-between gap-3">
          <div className="flex-1">
            <h3 className="font-bold text-white text-base leading-tight">{product.etsyTitle || product.name}</h3>
            <p className="text-sm text-gray-400 mt-0.5">Niche: {product.niche}</p>
          </div>
          <span className={`text-xs px-2 py-1 rounded-full border font-medium flex-shrink-0 ${stateColor}`}>
            {STATE_LABELS[product.state] || product.state}
          </span>
        </div>
      </div>

      {/* Design / Mockup preview */}
      {product.mockupUrl || product.designUrl ? (
        <div className="border-b border-gray-700">
          <img
            src={product.mockupUrl || product.designUrl}
            alt="Product design"
            className="w-full h-44 object-cover"
            onError={(e) => {
              // On image load failure, swap to placeholder
              const parent = e.currentTarget.parentElement
              if (parent) parent.innerHTML = ''
            }}
          />
        </div>
      ) : (
        <DesignPlaceholder product={product} />
      )}

      {/* Details */}
      <div className="p-4 space-y-3">

        {/* Niche reasoning */}
        {product.nicheReasoning && (
          <div>
            <div className="text-xs font-semibold text-gray-400 uppercase tracking-wide mb-1">Niche Analysis</div>
            <p className="text-sm text-gray-300 leading-relaxed">{product.nicheReasoning}</p>
          </div>
        )}

        {/* Stats */}
        <div className="grid grid-cols-2 gap-2">
          {product.estimatedMargin !== undefined && (
            <div className="bg-gray-800 rounded-lg p-3">
              <div className="flex items-center gap-1 text-xs text-gray-400 mb-1">
                <TrendingUp size={11} />
                Est. Margin
              </div>
              <div className={`text-xl font-bold ${
                product.estimatedMargin > 30 ? 'text-green-400'
                : product.estimatedMargin > 15 ? 'text-yellow-400'
                : 'text-red-400'
              }`}>
                {product.estimatedMargin}%
              </div>
            </div>
          )}
          {product.price && (
            <div className="bg-gray-800 rounded-lg p-3">
              <div className="text-xs text-gray-400 mb-1">List Price</div>
              <div className="text-xl font-bold text-white">${product.price.toFixed(2)}</div>
            </div>
          )}
          {product.fulfillmentMethod && (
            <div className="bg-gray-800 rounded-lg p-3 col-span-2">
              <div className="text-xs text-gray-400 mb-1">Fulfillment</div>
              <div className="text-sm font-medium text-white">{product.fulfillmentMethod}</div>
            </div>
          )}
        </div>

        {/* Listing content preview */}
        {product.etsyTitle && (
          <div>
            <div className="text-xs font-semibold text-gray-400 uppercase tracking-wide mb-1">Etsy Title</div>
            <p className="text-sm text-gray-200 font-medium">{product.etsyTitle}</p>
          </div>
        )}

        {product.etsyDescription && (
          <div>
            <div className="text-xs font-semibold text-gray-400 uppercase tracking-wide mb-1">Description Preview</div>
            <p className="text-xs text-gray-400 leading-relaxed line-clamp-4">{product.etsyDescription}</p>
          </div>
        )}

        {/* Tags */}
        {product.tags && product.tags.length > 0 && (
          <div>
            <div className="flex items-center gap-1 text-xs font-semibold text-gray-400 mb-1.5">
              <Tag size={11} />
              Tags
            </div>
            <div className="flex flex-wrap gap-1">
              {product.tags.slice(0, 13).map((tag) => (
                <span key={tag} className="text-xs px-2 py-0.5 rounded-full bg-gray-700 text-gray-300">
                  {tag}
                </span>
              ))}
            </div>
          </div>
        )}

        {/* Risks */}
        {product.risks && product.risks.length > 0 && (
          <div>
            <div className="flex items-center gap-1 text-xs font-semibold text-yellow-400 mb-1">
              <AlertTriangle size={11} />
              Risk Notes
            </div>
            <ul className="space-y-1">
              {product.risks.map((r, i) => (
                <li key={i} className="text-xs text-gray-400 flex items-start gap-1.5">
                  <span className="text-yellow-600 mt-0.5 flex-shrink-0">•</span>
                  {r}
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* AI Recommendation */}
        {product.recommendation && (
          <div className="rounded-lg p-3" style={{ background: 'rgba(37,99,235,0.1)', border: '1px solid rgba(37,99,235,0.25)' }}>
            <div className="text-xs font-semibold text-blue-400 mb-1">AI Recommendation</div>
            <p className="text-xs text-blue-200 leading-relaxed">{product.recommendation}</p>
          </div>
        )}

        {/* Design concept */}
        {product.designPrompt && (
          <div>
            <div className="text-xs font-semibold text-gray-400 uppercase tracking-wide mb-1">Design Concept</div>
            <p className="text-xs text-gray-500 italic">{product.designPrompt}</p>
          </div>
        )}
      </div>

      {/* Action buttons */}
      {isActionable && (
        <div className="p-4 pt-0 space-y-2">
          {/* Approve — triggers Etsy draft creation */}
          <motion.button
            onClick={onApprove}
            className="w-full flex items-center justify-center gap-2 py-3 rounded-xl font-bold text-sm transition-colors"
            style={{ background: 'rgba(22,163,74,0.85)', color: '#fff' }}
            whileHover={{ scale: 1.01, background: 'rgba(22,163,74,1)' }}
            whileTap={{ scale: 0.98 }}
          >
            <CheckCircle size={16} />
            Approve — Create Etsy Draft
          </motion.button>

          <div className="grid grid-cols-2 gap-2">
            {/* Keep Pending */}
            <motion.button
              onClick={onArchive}
              className="flex items-center justify-center gap-1.5 py-2.5 rounded-xl font-medium text-sm transition-colors"
              style={{ background: 'rgba(55,65,81,0.8)', color: '#d1d5db' }}
              whileHover={{ scale: 1.01 }}
              whileTap={{ scale: 0.98 }}
            >
              <Clock size={14} />
              Keep Pending
            </motion.button>

            {/* Reject */}
            <motion.button
              onClick={onReject}
              className="flex items-center justify-center gap-1.5 py-2.5 rounded-xl font-medium text-sm transition-colors"
              style={{ background: 'rgba(127,29,29,0.5)', color: '#fca5a5' }}
              whileHover={{ scale: 1.01 }}
              whileTap={{ scale: 0.98 }}
            >
              <XCircle size={14} />
              Reject
            </motion.button>
          </div>

          <p className="text-center text-xs text-gray-600 pt-1">
            Approving will create the Etsy draft listing. This is the first action taken on Etsy.
          </p>
        </div>
      )}

      {/* Etsy draft created state */}
      {product.state === 'ETSY_DRAFT_CREATED' && (
        <div className="p-4 pt-0">
          <div className="rounded-xl py-2.5 px-3 text-center" style={{ background: 'rgba(37,99,235,0.15)', border: '1px solid rgba(37,99,235,0.3)' }}>
            <span className="text-xs text-blue-300 font-medium">Draft listing created on Etsy</span>
          </div>
          <button
            onClick={onArchive}
            className="w-full flex items-center justify-center gap-2 py-2 rounded-xl mt-2 font-medium text-sm transition-colors"
            style={{ background: 'rgba(31,41,55,0.8)', color: '#9ca3af' }}
          >
            <Archive size={13} />
            Archive record
          </button>
        </div>
      )}

      {/* Rejected state */}
      {product.state === 'REJECTED' && (
        <div className="p-4 pt-0">
          <button
            onClick={onArchive}
            className="w-full flex items-center justify-center gap-2 py-2.5 rounded-xl font-medium text-sm"
            style={{ background: 'rgba(31,41,55,0.8)', color: '#6b7280' }}
          >
            <Archive size={13} />
            Archive
          </button>
        </div>
      )}
    </motion.div>
  )
}
