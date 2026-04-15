import { motion } from 'framer-motion'
import { CheckCircle, XCircle, Archive, TrendingUp, AlertTriangle, Clock, Tag, DollarSign, Play, BarChart3, Hash } from 'lucide-react'
import type { Product } from '../../types'

const PLATFORM_ICONS: Record<string, string> = {
  etsy: '🛒',
  fiverr: '💼',
  trading: '📈',
  youtube: '🎬',
  tiktok: '🎵',
}

const PLATFORM_COLORS: Record<string, string> = {
  etsy: '#ff6b35',
  fiverr: '#1dbf73',
  trading: '#f59e0b',
  youtube: '#ef4444',
  tiktok: '#ec4899',
}

function getPlatformEmoji(platform: string) {
  return PLATFORM_ICONS[platform] || '📦'
}

function getPlatformColor(platform: string) {
  return PLATFORM_COLORS[platform] || '#3b82f6'
}

function DesignPlaceholder({ product }: { product: Product }) {
  const hash = product.niche.split('').reduce((a, c) => a + c.charCodeAt(0), 0)
  const hue1 = hash % 360
  const hue2 = (hue1 + 45) % 360
  const accent = `hsl(${hue1}, 75%, 62%)`
  const platformColor = getPlatformColor(product.platform)

  const getEmoji = (niche: string) => {
    const n = niche.toLowerCase()
    if (n.includes('floral') || n.includes('flower')) return '🌸'
    if (n.includes('nature') || n.includes('forest')) return '🌿'
    if (n.includes('ai') || n.includes('art')) return '🎨'
    if (n.includes('crypto') || n.includes('bitcoin')) return '₿'
    if (n.includes('video') || n.includes('content')) return '🎥'
    if (n.includes('viral') || n.includes('trending')) return '🔥'
    if (n.includes('script') || n.includes('hook')) return '📝'
    if (n.includes('script') || n.includes('trade')) return '📊'
    return '✨'
  }

  return (
    <div
      className="w-full h-40 relative flex flex-col items-center justify-center overflow-hidden border-b border-gray-700"
      style={{
        background: `linear-gradient(145deg, hsl(${hue1},55%,14%), hsl(${hue2},65%,10%))`,
      }}
    >
      <div style={{ position: 'absolute', top: 10, left: 10, width: 36, height: 36, borderTop: `2px solid ${accent}44`, borderLeft: `2px solid ${accent}44`, borderRadius: '4px 0 0 0' }} />
      <div style={{ position: 'absolute', bottom: 10, right: 10, width: 36, height: 36, borderBottom: `2px solid ${accent}44`, borderRight: `2px solid ${accent}44`, borderRadius: '0 0 4px 0' }} />

      <div className="relative text-center px-6">
        <div className="text-4xl mb-2">{getEmoji(product.niche)}</div>
        <div className="flex items-center justify-center gap-2 mb-2">
          <span style={{ fontSize: '20px' }}>{getPlatformEmoji(product.platform)}</span>
          <span style={{ color: accent, fontWeight: 700, fontSize: '12px' }}>
            {product.niche}
          </span>
        </div>
        {product.designPrompt && (
          <div style={{ color: 'rgba(255,255,255,0.5)', fontSize: '9px', maxWidth: '230px', margin: '0 auto' }}>
            {product.designPrompt.length > 80 ? product.designPrompt.slice(0, 80) + '…' : product.designPrompt}
          </div>
        )}
      </div>

      <div style={{ position: 'absolute', bottom: 7, right: 9, fontSize: '7.5px', color: 'rgba(255,255,255,0.22)', fontStyle: 'italic' }}>
        AI Generated
      </div>
    </div>
  )
}

interface Props {
  product: Product
  onApprove: () => void
  onReject: () => void
  onArchive: () => void
}

const STATE_LABELS: Record<string, string> = {
  IDEA_DISCOVERED: 'Idea Found',
  BRIEF_CREATED: 'Brief Ready',
  DESIGN_GENERATED: 'Generated',
  DESIGN_QA_PASSED: 'QA Passed',
  POD_PRODUCT_PREPARED: 'POD Ready',
  ETSY_DRAFT_CREATED: 'Draft Created',
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

const APPROVE_LABELS: Record<string, string> = {
  etsy: 'Approve — Create Etsy Draft',
  fiverr: 'Approve — Prepare Fiverr Service',
  trading: 'Approve — Log Trading Signal',
  youtube: 'Approve — Prepare Video Package',
  tiktok: 'Approve — Prepare TikTok Content',
}

export function ProductCard({ product, onApprove, onReject, onArchive }: Props) {
  const isActionable = product.state === 'AWAITING_HUMAN_DECISION'
  const stateColor = STATE_COLORS[product.state] || 'text-blue-400 bg-blue-900/30 border-blue-700/50'
  const platformColor = getPlatformColor(product.platform)
  const platformEmoji = getPlatformEmoji(product.platform)

  return (
    <motion.div
      className="bg-gray-900 border border-gray-700 rounded-xl overflow-hidden"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      layout
    >
      <div className="p-4 border-b border-gray-700" style={{ borderLeft: `3px solid ${platformColor}` }}>
        <div className="flex items-start justify-between gap-3">
          <div className="flex-1">
            <div className="flex items-center gap-2 mb-1">
              <span className="text-lg">{platformEmoji}</span>
              <h3 className="font-bold text-white text-base leading-tight">
                {product.etsyTitle || product.name || 'Untitled'}
              </h3>
            </div>
            <p className="text-sm text-gray-400">Niche: {product.niche || 'General'}</p>
          </div>
          <span className={`text-xs px-2 py-1 rounded-full border font-medium flex-shrink-0 ${stateColor}`}>
            {STATE_LABELS[product.state] || product.state}
          </span>
        </div>
      </div>

      {product.mockupUrl || product.designUrl ? (
        <div className="border-b border-gray-700">
          <img
            src={product.mockupUrl || product.designUrl}
            alt="Design"
            className="w-full h-36 object-cover"
            onError={(e) => {
              const parent = e.currentTarget.parentElement
              if (parent) parent.innerHTML = ''
            }}
          />
        </div>
      ) : (
        <DesignPlaceholder product={product} />
      )}

      <div className="p-4 space-y-3">
        {product.nicheReasoning && (
          <div>
            <div className="text-xs font-semibold text-gray-400 uppercase tracking-wide mb-1">Analysis</div>
            <p className="text-sm text-gray-300 leading-relaxed">{product.nicheReasoning}</p>
          </div>
        )}

        <div className="grid grid-cols-2 gap-2">
          {product.estimatedMargin !== undefined && (
            <div className="bg-gray-800 rounded-lg p-3">
              <div className="flex items-center gap-1 text-xs text-gray-400 mb-1">
                <TrendingUp size={11} />
                Est. Margin
              </div>
              <div className={`text-xl font-bold ${
                product.estimatedMargin > 30 ? 'text-green-400' :
                product.estimatedMargin > 15 ? 'text-yellow-400' : 'text-red-400'
              }`}>
                {product.estimatedMargin}%
              </div>
            </div>
          )}
          {product.price && (
            <div className="bg-gray-800 rounded-lg p-3">
              <div className="text-xs text-gray-400 mb-1">Price</div>
              <div className="text-xl font-bold text-white">${product.price.toFixed(2)}</div>
            </div>
          )}
        </div>

        {product.tags && product.tags.length > 0 && (
          <div>
            <div className="flex items-center gap-1 text-xs font-semibold text-gray-400 mb-1.5">
              <Hash size={11} />
              Tags
            </div>
            <div className="flex flex-wrap gap-1">
              {product.tags.slice(0, 10).map((tag) => (
                <span key={tag} className="text-xs px-2 py-0.5 rounded-full bg-gray-700 text-gray-300">
                  {tag}
                </span>
              ))}
            </div>
          </div>
        )}

        {product.recommendation && (
          <div className="rounded-lg p-3" style={{ background: `${platformColor}15`, border: `1px solid ${platformColor}40` }}>
            <div className="text-xs font-semibold" style={{ color: platformColor }}>AI Recommendation</div>
            <p className="text-xs" style={{ color: `${platformColor}cc` }}>{product.recommendation}</p>
          </div>
        )}

        {product.designPrompt && (
          <div>
            <div className="text-xs font-semibold text-gray-400 uppercase tracking-wide mb-1">Concept</div>
            <p className="text-xs text-gray-500 italic line-clamp-2">{product.designPrompt}</p>
          </div>
        )}
      </div>

      {isActionable && (
        <div className="p-4 pt-0 space-y-2">
          <motion.button
            onClick={onApprove}
            className="w-full flex items-center justify-center gap-2 py-3 rounded-xl font-bold text-sm transition-colors"
            style={{ background: `${platformColor}dd`, color: '#fff' }}
            whileHover={{ scale: 1.01 }}
            whileTap={{ scale: 0.98 }}
          >
            <CheckCircle size={16} />
            {APPROVE_LABELS[product.platform] || 'Approve'}
          </motion.button>

          <div className="grid grid-cols-2 gap-2">
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
        </div>
      )}

      {product.state === 'ETSY_DRAFT_CREATED' && (
        <div className="p-4 pt-0">
          <div className="rounded-xl py-2.5 px-3 text-center" style={{ background: `${platformColor}20`, border: `1px solid ${platformColor}50` }}>
            <span className="text-xs" style={{ color: platformColor }}>✓ Completed on {product.platform}</span>
          </div>
          <button
            onClick={onArchive}
            className="w-full flex items-center justify-center gap-2 py-2 rounded-xl mt-2 font-medium text-sm transition-colors"
            style={{ background: 'rgba(31,41,55,0.8)', color: '#9ca3af' }}
          >
            <Archive size={13} />
            Archive
          </button>
        </div>
      )}

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