export type AgentStatus = 'idle' | 'working' | 'blocked' | 'complete' | 'entering' | 'offline'

export type ProductState =
  | 'IDEA_DISCOVERED'
  | 'BRIEF_CREATED'
  | 'DESIGN_GENERATED'
  | 'DESIGN_QA_PASSED'
  | 'POD_PRODUCT_PREPARED'
  | 'ETSY_DRAFT_CREATED'
  | 'APPROVAL_PACKET_READY'
  | 'AWAITING_HUMAN_DECISION'
  | 'APPROVED_FOR_PUBLISH'
  | 'REJECTED'
  | 'ARCHIVED'

export type Platform = 'etsy' | 'amazon' | 'ebay' | 'tiktok' | 'instagram' | 'website' | 'fiverr' | 'youtube' | 'trading'

export type ZoneId = 'etsy' | 'fiverr' | 'trading' | 'youtube' | 'tiktok'

export interface Agent {
  id: string
  label: string
  role: string
  status: AgentStatus
  platform: Platform | 'global'
  currentTask?: string
  deskId?: string
  createdAt: number
  taskCount: number
}

export interface Desk {
  id: string
  nameplate: string
  agentLabel: string
  agentId?: string
  roomId: string
  isApprovalDesk?: boolean
  position: { x: number; y: number }
}

export interface Room {
  id: string
  name: string
  platform: Platform | 'global'
  isActive: boolean
  agents: string[]
  desks: Desk[]
  color: string
}

export interface Product {
  id: string
  name: string
  platform: Platform
  state: ProductState
  niche: string
  nicheReasoning: string
  designPrompt?: string
  designUrl?: string
  mockupUrl?: string
  estimatedMargin?: number
  fulfillmentMethod?: string
  risks?: string[]
  recommendation?: string
  etsyTitle?: string
  etsyDescription?: string
  createdAt: number
  updatedAt: number
  tags?: string[]
  price?: number
}

export interface SystemEvent {
  id: string
  type: 'agent_created' | 'agent_update' | 'product_update' | 'system' | 'error'
  message: string
  data?: Record<string, unknown>
  timestamp: number
}

export interface TalkingTableEntry {
  message: string
  timestamp: number
}

export interface WalkEvent {
  agentLabel: string
  fromDesk: string
  toDesk: string
  durationMs: number
}

export interface AppConfig {
  openaiKey: string
  etsyApiKey: string
  etsyShopId: string
  printifyToken: string
  geminiKey?: string
  setupComplete: boolean
}

export type AppScreen = 'splash' | 'wizard' | 'office' | 'game'

export interface WSMessage {
  type:
    | 'agent_update'
    | 'product_update'
    | 'system_event'
    | 'desk_created'
    | 'heartbeat'
    | 'talking_table'
    | 'agent_walk'
    | 'agent_arrival'
    | 'trading_signal'
  data: Record<string, unknown>
}

export interface TradingSignal {
  id: string
  symbol: string
  currentPrice: number
  change24h: number
  trend: string
  action: 'BUY' | 'SELL' | 'HOLD'
  entryPrice: number
  stopLoss: number
  takeProfit: number
  riskPercent: number
  riskReward: number
  confidence: number
  reasoning: string
  timeHorizon: string
  performance: {
    totalTrades: number
    winningTrades: number
    winRate: number
    totalPnl: number
    currentEquity: number
    drawdown: number
  }
  timestamp: number
}

export interface FiverrService {
  id: string
  name: string
  category: string
  description: string
  platform: 'fiverr'
  state: string
  startingPrice: number
  earningsPerOrder: number
  profileTitle: string
  profileOverview: string
  pricingTiers: Record<string, {
    price: number
    delivery_days: number
    revisions: number
    description: string
    includes: string[]
  }>
  faqs: Array<{ question: string; answer: string }>
  workflow: Record<string, unknown>
  createdAt: number
  updatedAt: number
}

export interface YouTubeVideo {
  id: string
  title: string
  niche: string
  hook: string
  hookAngle: string
  targetKeywords: string[]
  estimatedViews: string
  competition: string
  videoLength: string
  platform: 'youtube'
  state: string
  script: string
  editorPlan: Record<string, unknown>
  thumbnailPrompt: string
  thumbnailText: string
  thumbnailUrl: string
  estimatedEarnings: number
  createdAt: number
  updatedAt: number
}

export interface Achievement {
  id: string
  title: string
  description: string
  icon: string
  unlockedAt?: number
  requirement: {
    type: 'products' | 'revenue' | 'level' | 'agents' | 'platforms'
    value: number
  }
}

export interface AgentUpgrade {
  agentId: string
  type: 'speed' | 'efficiency' | 'quality'
  level: number
  cost: number
}

export interface AchievementsState {
  achievements: Achievement[]
  unlockedIds: string[]
  addAchievement: (achievement: Achievement) => void
  unlockAchievement: (id: string) => void
}

export interface UpgradesState {
  upgrades: Record<string, AgentUpgrade[]>
  purchaseUpgrade: (agentId: string, type: 'speed' | 'efficiency' | 'quality') => void
  getUpgradeMultiplier: (agentId: string, type: 'speed' | 'efficiency' | 'quality') => number
}

export interface ZoneData {
  id: ZoneId
  name: string
  icon: string
  color: string
  glowColor: string
  position: [number, number, number]
  isActive: boolean
  income: number
  agents: Agent[]
}

export interface GameMission {
  id: string
  title: string
  description: string
  zoneId: ZoneId
  status: 'pending' | 'active' | 'completed' | 'failed'
  reward: number
  assignedAgent?: string
}

export interface RevenueData {
  total: number
  etsy: number
  fiverr: number
  trading: number
  youtube: number
  tiktok: number
  level: number
  xp: number
  xpToNext: number
}
