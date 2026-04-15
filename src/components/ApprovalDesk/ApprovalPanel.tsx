import { motion, AnimatePresence } from 'framer-motion'
import { X, Inbox, ShoppingBag, Briefcase, TrendingUp, Video, Music } from 'lucide-react'
import { useAppStore } from '../../store/useAppStore'
import { ProductCard } from './ProductCard'
import type { Platform } from '../../types'

const PLATFORM_ICONS: Record<string, typeof ShoppingBag> = {
  etsy: ShoppingBag,
  fiverr: Briefcase,
  trading: TrendingUp,
  youtube: Video,
  tiktok: Music,
}

const PLATFORM_COLORS: Record<string, string> = {
  etsy: '#ff6b35',
  fiverr: '#1dbf73',
  trading: '#f59e0b',
  youtube: '#ef4444',
  tiktok: '#ec4899',
}

export function ApprovalPanel() {
  const { approvalPanelOpen, approvalPlatform, products, closeApproval, approveProduct, rejectProduct, archiveProduct } = useAppStore()

  const relevantProducts = Object.values(products)
    .filter((p) => {
      if (!approvalPlatform || approvalPlatform === 'global') return true
      return p.platform === approvalPlatform
    })
    .sort((a, b) => {
      if (a.state === 'AWAITING_HUMAN_DECISION' && b.state !== 'AWAITING_HUMAN_DECISION') return -1
      if (b.state === 'AWAITING_HUMAN_DECISION' && a.state !== 'AWAITING_HUMAN_DECISION') return 1
      return b.updatedAt - a.updatedAt
    })
    .filter((p) => !['ARCHIVED'].includes(p.state))

  const awaitingCount = relevantProducts.filter((p) => p.state === 'AWAITING_HUMAN_DECISION').length

  const platformLabel = approvalPlatform === 'global' ? 'All' :
                        approvalPlatform === 'etsy' ? 'Etsy' :
                        approvalPlatform === 'fiverr' ? 'Fiverr' :
                        approvalPlatform === 'trading' ? 'Trading' :
                        approvalPlatform === 'youtube' ? 'YouTube' :
                        approvalPlatform === 'tiktok' ? 'TikTok' : approvalPlatform

  const platformDesc = approvalPlatform === 'etsy' ? 'approve to create Etsy draft' :
                       approvalPlatform === 'fiverr' ? 'approve to prepare Fiverr service' :
                       approvalPlatform === 'trading' ? 'approve to execute signal' :
                       approvalPlatform === 'youtube' ? 'approve to prepare video package' :
                       approvalPlatform === 'tiktok' ? 'approve to prepare TikTok content' :
                       'approve to proceed'

  const IconComponent = approvalPlatform ? PLATFORM_ICONS[approvalPlatform] : Inbox
  const color = approvalPlatform ? PLATFORM_COLORS[approvalPlatform] || '#3b82f6' : '#3b82f6'

  return (
    <AnimatePresence>
      {approvalPanelOpen && (
        <>
          <motion.div
            className="fixed inset-0 bg-black/60 z-40"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            onClick={closeApproval}
          />

          <motion.div
            className="fixed right-0 top-0 bottom-0 w-[480px] bg-[#0a0a14] border-l border-gray-700 z-50 flex flex-col"
            initial={{ x: '100%' }}
            animate={{ x: 0 }}
            exit={{ x: '100%' }}
            transition={{ type: 'spring', stiffness: 300, damping: 30 }}
          >
            <div className="p-4 border-b border-gray-700 flex items-center justify-between" style={{ background: `${color}15` }}>
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 rounded-xl flex items-center justify-center" style={{ background: `${color}25` }}>
                  <IconComponent size={20} style={{ color }} />
                </div>
                <div>
                  <h2 className="text-lg font-bold text-white">
                    {platformLabel} Approval Desk
                  </h2>
                  <p className="text-xs text-gray-400 mt-0.5">
                    {awaitingCount > 0
                      ? `${awaitingCount} item${awaitingCount > 1 ? 's' : ''} awaiting decision — ${platformDesc}`
                      : 'No items awaiting approval'}
                  </p>
                </div>
              </div>
              <button
                onClick={closeApproval}
                className="p-2 rounded-lg hover:bg-gray-700 text-gray-400 hover:text-white transition-colors"
              >
                <X size={18} />
              </button>
            </div>

            <div className="flex-1 overflow-y-auto p-4 space-y-4">
              {relevantProducts.length === 0 ? (
                <div className="flex flex-col items-center justify-center h-full text-center">
                  <Inbox className="w-12 h-12 text-gray-600 mb-3" />
                  <p className="text-gray-500 text-sm">No items yet</p>
                  <p className="text-gray-600 text-xs mt-1">
                    AI agents are working on it. Check back soon.
                  </p>
                </div>
              ) : (
                relevantProducts.map((product) => (
                  <ProductCard
                    key={product.id}
                    product={product}
                    onApprove={() => approveProduct(product.id)}
                    onReject={() => rejectProduct(product.id)}
                    onArchive={() => archiveProduct(product.id)}
                  />
                ))
              )}
            </div>

            <div className="p-4 border-t border-gray-700 bg-gray-900/50">
              <p className="text-xs text-gray-500 text-center">
                {approvalPlatform === 'etsy' 
                  ? 'Etsy draft is created only after your explicit approval. Nothing goes to Etsy automatically.'
                  : approvalPlatform === 'fiverr'
                  ? 'Fiverr service package is finalized only after your explicit approval.'
                  : approvalPlatform === 'trading'
                  ? 'Trading signals are logged only after your explicit approval.'
                  : approvalPlatform === 'youtube'
                  ? 'YouTube video package is finalized only after your explicit approval.'
                  : approvalPlatform === 'tiktok'
                  ? 'TikTok content is finalized only after your explicit approval.'
                  : 'Content is created only after your explicit approval.'}
              </p>
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  )
}