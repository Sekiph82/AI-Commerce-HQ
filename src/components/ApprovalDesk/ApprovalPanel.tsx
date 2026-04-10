import { motion, AnimatePresence } from 'framer-motion'
import { X, Inbox } from 'lucide-react'
import { useAppStore } from '../../store/useAppStore'
import { ProductCard } from './ProductCard'

export function ApprovalPanel() {
  const { approvalPanelOpen, approvalPlatform, products, closeApproval, approveProduct, rejectProduct, archiveProduct } = useAppStore()

  const relevantProducts = Object.values(products)
    .filter((p) => {
      if (!approvalPlatform || approvalPlatform === 'global') return true
      return p.platform === approvalPlatform
    })
    .sort((a, b) => {
      // Sort: awaiting first, then by date
      if (a.state === 'AWAITING_HUMAN_DECISION' && b.state !== 'AWAITING_HUMAN_DECISION') return -1
      if (b.state === 'AWAITING_HUMAN_DECISION' && a.state !== 'AWAITING_HUMAN_DECISION') return 1
      return b.updatedAt - a.updatedAt
    })
    .filter((p) => !['ARCHIVED'].includes(p.state))

  const awaitingCount = relevantProducts.filter((p) => p.state === 'AWAITING_HUMAN_DECISION').length

  return (
    <AnimatePresence>
      {approvalPanelOpen && (
        <>
          {/* Backdrop */}
          <motion.div
            className="fixed inset-0 bg-black/60 z-40"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            onClick={closeApproval}
          />

          {/* Panel */}
          <motion.div
            className="fixed right-0 top-0 bottom-0 w-96 bg-office-floor border-l border-gray-700 z-50 flex flex-col"
            initial={{ x: '100%' }}
            animate={{ x: 0 }}
            exit={{ x: '100%' }}
            transition={{ type: 'spring', stiffness: 300, damping: 30 }}
          >
            {/* Header */}
            <div className="p-4 border-b border-gray-700 flex items-center justify-between">
              <div>
                <h2 className="text-lg font-bold text-white capitalize">
                  {approvalPlatform === 'global' ? 'All' : approvalPlatform} Approval Desk
                </h2>
                <p className="text-xs text-gray-400 mt-0.5">
                  {awaitingCount > 0
                    ? `${awaitingCount} product${awaitingCount > 1 ? 's' : ''} awaiting your decision`
                    : 'No products awaiting approval'}
                </p>
              </div>
              <button
                onClick={closeApproval}
                className="p-2 rounded-lg hover:bg-gray-700 text-gray-400 hover:text-white transition-colors"
              >
                <X size={18} />
              </button>
            </div>

            {/* Products list */}
            <div className="flex-1 overflow-y-auto p-4 space-y-4">
              {relevantProducts.length === 0 && (
                <div className="flex flex-col items-center justify-center h-full text-center">
                  <Inbox className="w-12 h-12 text-gray-600 mb-3" />
                  <p className="text-gray-500 text-sm">No products yet</p>
                  <p className="text-gray-600 text-xs mt-1">
                    Agents are working on it. Check back soon.
                  </p>
                </div>
              )}

              {relevantProducts.map((product) => (
                <ProductCard
                  key={product.id}
                  product={product}
                  onApprove={() => approveProduct(product.id)}
                  onReject={() => rejectProduct(product.id)}
                  onArchive={() => archiveProduct(product.id)}
                />
              ))}
            </div>

            {/* Footer */}
            <div className="p-4 border-t border-gray-700 bg-gray-900/50">
              <p className="text-xs text-gray-500 text-center">
                Publishing requires your explicit approval. No automatic publishing.
              </p>
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  )
}
