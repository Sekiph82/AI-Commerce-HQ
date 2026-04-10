import { motion } from 'framer-motion'
import { CheckCircle, XCircle, Archive, TrendingUp, AlertTriangle } from 'lucide-react'
import type { Product } from '../../types'

interface Props {
  product: Product
  onApprove: () => void
  onReject: () => void
  onArchive: () => void
}

const STATE_LABELS: Record<string, string> = {
  IDEA_DISCOVERED: 'Idea Found',
  BRIEF_CREATED: 'Brief Ready',
  DESIGN_GENERATED: 'Design Generated',
  DESIGN_QA_PASSED: 'QA Passed',
  POD_PRODUCT_PREPARED: 'POD Ready',
  ETSY_DRAFT_CREATED: 'Draft Created',
  APPROVAL_PACKET_READY: 'Packet Ready',
  AWAITING_HUMAN_DECISION: 'Awaiting You',
  APPROVED_FOR_PUBLISH: 'Approved',
  REJECTED: 'Rejected',
  ARCHIVED: 'Archived',
}

const STATE_COLORS: Record<string, string> = {
  AWAITING_HUMAN_DECISION: 'text-orange-400 bg-orange-900/30 border-orange-700/50',
  APPROVED_FOR_PUBLISH: 'text-green-400 bg-green-900/30 border-green-700/50',
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
      {/* Product header */}
      <div className="p-4 border-b border-gray-700">
        <div className="flex items-start justify-between gap-3">
          <div className="flex-1">
            <h3 className="font-bold text-white text-base">{product.name}</h3>
            <p className="text-sm text-gray-400 mt-0.5">Niche: {product.niche}</p>
          </div>
          <span className={`text-xs px-2 py-1 rounded-full border font-medium flex-shrink-0 ${stateColor}`}>
            {STATE_LABELS[product.state] || product.state}
          </span>
        </div>
      </div>

      {/* Mockup preview */}
      {product.mockupUrl && (
        <div className="border-b border-gray-700">
          <img
            src={product.mockupUrl}
            alt="Product mockup"
            className="w-full h-40 object-cover"
            onError={(e) => {
              e.currentTarget.style.display = 'none'
            }}
          />
        </div>
      )}

      {!product.mockupUrl && (
        <div className="h-32 bg-gray-800 flex items-center justify-center border-b border-gray-700">
          <div className="text-center">
            <span className="text-4xl block mb-1">🎨</span>
            <span className="text-xs text-gray-500">Design preview</span>
          </div>
        </div>
      )}

      {/* Details */}
      <div className="p-4 space-y-3">
        {/* Niche reasoning */}
        <div>
          <div className="text-xs font-semibold text-gray-400 uppercase tracking-wide mb-1">Niche Analysis</div>
          <p className="text-sm text-gray-300">{product.nicheReasoning || 'Analyzing market trends...'}</p>
        </div>

        {/* Stats row */}
        <div className="grid grid-cols-2 gap-3">
          {product.estimatedMargin !== undefined && (
            <div className="bg-gray-800 rounded-lg p-3">
              <div className="flex items-center gap-1 text-xs text-gray-400 mb-1">
                <TrendingUp size={12} />
                Est. Margin
              </div>
              <div className={`text-lg font-bold ${product.estimatedMargin > 30 ? 'text-green-400' : product.estimatedMargin > 15 ? 'text-yellow-400' : 'text-red-400'}`}>
                {product.estimatedMargin}%
              </div>
            </div>
          )}
          {product.fulfillmentMethod && (
            <div className="bg-gray-800 rounded-lg p-3">
              <div className="text-xs text-gray-400 mb-1">Fulfillment</div>
              <div className="text-sm font-medium text-white">{product.fulfillmentMethod}</div>
            </div>
          )}
          {product.price && (
            <div className="bg-gray-800 rounded-lg p-3">
              <div className="text-xs text-gray-400 mb-1">Price</div>
              <div className="text-sm font-medium text-white">${product.price.toFixed(2)}</div>
            </div>
          )}
        </div>

        {/* Risks */}
        {product.risks && product.risks.length > 0 && (
          <div>
            <div className="flex items-center gap-1 text-xs font-semibold text-yellow-400 mb-1">
              <AlertTriangle size={12} />
              Risks
            </div>
            <ul className="space-y-0.5">
              {product.risks.map((r, i) => (
                <li key={i} className="text-xs text-gray-400 flex items-start gap-1">
                  <span className="text-yellow-600 mt-0.5">•</span>
                  {r}
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* Recommendation */}
        {product.recommendation && (
          <div className="bg-blue-900/20 border border-blue-700/30 rounded-lg p-3">
            <div className="text-xs font-semibold text-blue-400 mb-1">AI Recommendation</div>
            <p className="text-xs text-blue-200">{product.recommendation}</p>
          </div>
        )}

        {/* Tags */}
        {product.tags && product.tags.length > 0 && (
          <div className="flex flex-wrap gap-1">
            {product.tags.slice(0, 8).map((tag) => (
              <span key={tag} className="text-xs px-2 py-0.5 rounded-full bg-gray-700 text-gray-300">
                {tag}
              </span>
            ))}
          </div>
        )}
      </div>

      {/* Action buttons */}
      {isActionable && (
        <div className="p-4 pt-0 flex gap-2">
          <motion.button
            onClick={onApprove}
            className="flex-1 flex items-center justify-center gap-2 py-2.5 rounded-lg bg-green-700 hover:bg-green-600 text-white font-medium text-sm transition-colors"
            whileHover={{ scale: 1.02 }}
            whileTap={{ scale: 0.98 }}
          >
            <CheckCircle size={16} />
            Publish
          </motion.button>
          <motion.button
            onClick={onReject}
            className="flex-1 flex items-center justify-center gap-2 py-2.5 rounded-lg bg-gray-700 hover:bg-gray-600 text-white font-medium text-sm transition-colors"
            whileHover={{ scale: 1.02 }}
            whileTap={{ scale: 0.98 }}
          >
            Keep Draft
          </motion.button>
          <motion.button
            onClick={onArchive}
            className="flex items-center justify-center gap-2 py-2.5 px-3 rounded-lg bg-red-900/50 hover:bg-red-900/70 text-red-400 font-medium text-sm transition-colors"
            whileHover={{ scale: 1.02 }}
            whileTap={{ scale: 0.98 }}
          >
            <XCircle size={16} />
          </motion.button>
        </div>
      )}

      {/* Already decided */}
      {!isActionable && (product.state === 'APPROVED_FOR_PUBLISH' || product.state === 'REJECTED') && (
        <div className="p-4 pt-0">
          <button
            onClick={onArchive}
            className="w-full flex items-center justify-center gap-2 py-2.5 rounded-lg bg-gray-800 hover:bg-gray-700 text-gray-400 font-medium text-sm transition-colors"
          >
            <Archive size={14} />
            Archive
          </button>
        </div>
      )}
    </motion.div>
  )
}
