import { motion, AnimatePresence } from 'framer-motion'
import { X, Trophy, Star, Zap, Target, ShoppingBag, Briefcase, Video, Music, TrendingUp } from 'lucide-react'
import { useAppStore } from '../store/useAppStore'
import { useState } from 'react'

const ACHIEVEMENTS = [
  { id: 'first_product', title: 'First Steps', description: 'Create your first product', icon: '🎁', requirement: { type: 'products' as const, value: 1 } },
  { id: 'ten_products', title: 'Productive', description: 'Create 10 products', icon: '📦', requirement: { type: 'products' as const, value: 10 } },
  { id: 'fifty_products', title: 'Factory', description: 'Create 50 products', icon: '🏭', requirement: { type: 'products' as const, value: 50 } },
  { id: 'first_revenue', title: 'Making Money', description: 'Earn your first $100', icon: '💰', requirement: { type: 'revenue' as const, value: 100 } },
  { id: 'thousand_revenue', title: 'Rising Star', description: 'Earn $1,000 total', icon: '⭐', requirement: { type: 'revenue' as const, value: 1000 } },
  { id: 'ten_k_revenue', title: 'Empire Builder', description: 'Earn $10,000 total', icon: '👑', requirement: { type: 'revenue' as const, value: 10000 } },
  { id: 'level_5', title: 'Rising Force', description: 'Reach Level 5', icon: '⚡', requirement: { type: 'level' as const, value: 5 } },
  { id: 'level_10', title: 'Elite', description: 'Reach Level 10', icon: '🔥', requirement: { type: 'level' as const, value: 10 } },
  { id: 'level_25', title: 'Master', description: 'Reach Level 25', icon: '🏆', requirement: { type: 'level' as const, value: 25 } },
  { id: 'all_platforms', title: 'Full Stack', description: 'Have products on all 5 platforms', icon: '🌐', requirement: { type: 'platforms' as const, value: 5 } },
  { id: 'team_10', title: 'Team Lead', description: 'Have 10 active agents', icon: '👥', requirement: { type: 'agents' as const, value: 10 } },
]

function checkAchievement(achievement: typeof ACHIEVEMENTS[0], products: Record<string, unknown>, revenue: number, level: number, platforms: Set<string>, agentCount: number): boolean {
  switch (achievement.requirement.type) {
    case 'products':
      return Object.keys(products).length >= achievement.requirement.value
    case 'revenue':
      return revenue >= achievement.requirement.value
    case 'level':
      return level >= achievement.requirement.value
    case 'platforms':
      return platforms.size >= achievement.requirement.value
    case 'agents':
      return agentCount >= achievement.requirement.value
    default:
      return false
  }
}

export function AchievementsPanel() {
  const { achievementsPanelOpen, toggleAchievementsPanel, products, revenue, agents } = useAppStore()
  
  const productCount = Object.keys(products).length
  const level = Math.floor(revenue.total / 100) + 1
  const platformSet = new Set(Object.values(products).map(p => (p as any).platform))
  const agentCount = Object.keys(agents).length

  const unlockedIds = ACHIEVEMENTS
    .filter(a => checkAchievement(a, products, revenue.total, level, platformSet, agentCount))
    .map(a => a.id)

  return (
    <AnimatePresence>
      {achievementsPanelOpen && (
        <>
          <motion.div
            className="fixed inset-0 bg-black/60 z-40"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            onClick={toggleAchievementsPanel}
          />

          <motion.div
            className="fixed left-0 top-0 bottom-0 w-[400px] bg-[#0a0a14] border-r border-gray-700 z-50 flex flex-col"
            initial={{ x: '-100%' }}
            animate={{ x: 0 }}
            exit={{ x: '-100%' }}
            transition={{ type: 'spring', stiffness: 300, damping: 30 }}
          >
            <div className="p-4 border-b border-gray-700 flex items-center justify-between" style={{ background: '#f59e0b15' }}>
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 rounded-xl flex items-center justify-center" style={{ background: '#f59e0b25' }}>
                  <Trophy size={20} style={{ color: '#f59e0b' }} />
                </div>
                <div>
                  <h2 className="text-lg font-bold text-white">Achievements</h2>
                  <p className="text-xs text-gray-400 mt-0.5">
                    {unlockedIds.length} / {ACHIEVEMENTS.length} unlocked
                  </p>
                </div>
              </div>
              <button
                onClick={toggleAchievementsPanel}
                className="p-2 rounded-lg hover:bg-gray-700 text-gray-400 hover:text-white transition-colors"
              >
                <X size={18} />
              </button>
            </div>

            <div className="flex-1 overflow-y-auto p-4">
              <div className="space-y-3">
                {ACHIEVEMENTS.map((achievement) => {
                  const unlocked = unlockedIds.includes(achievement.id)
                  
                  return (
                    <motion.div
                      key={achievement.id}
                      className={`p-4 rounded-xl border ${
                        unlocked 
                          ? 'bg-yellow-900/20 border-yellow-600/50' 
                          : 'bg-gray-800/30 border-gray-700'
                      }`}
                      whileHover={{ scale: 1.01 }}
                    >
                      <div className="flex items-center gap-3">
                        <div className={`text-3xl ${unlocked ? '' : 'opacity-30 grayscale'}`}>
                          {achievement.icon}
                        </div>
                        <div className="flex-1">
                          <div className="flex items-center gap-2">
                            <span className={`font-bold ${unlocked ? 'text-yellow-400' : 'text-gray-400'}`}>
                              {achievement.title}
                            </span>
                            {unlocked && <Star size={14} className="text-yellow-400 fill-yellow-400" />}
                          </div>
                          <p className={`text-xs ${unlocked ? 'text-gray-300' : 'text-gray-500'}`}>
                            {achievement.description}
                          </p>
                        </div>
                      </div>

                      {!unlocked && (
                        <div className="mt-2 text-xs text-gray-600">
                          Progress: {
                            achievement.requirement.type === 'products' ? `${productCount}/${achievement.requirement.value}` :
                            achievement.requirement.type === 'revenue' ? `$${revenue.total.toFixed(0)}/$${achievement.requirement.value}` :
                            achievement.requirement.type === 'level' ? `Level ${level}/${achievement.requirement.value}` :
                            achievement.requirement.type === 'platforms' ? `${platformSet.size}/${achievement.requirement.value}` :
                            `${agentCount}/${achievement.requirement.value}`
                          }
                        </div>
                      )}
                    </motion.div>
                  )
                })}
              </div>
            </div>

            <div className="p-4 border-t border-gray-700 bg-gray-900/50">
              <p className="text-xs text-gray-500 text-center">
                Keep generating revenue and products to unlock more achievements!
              </p>
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  )
}