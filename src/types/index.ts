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

export type Platform = 'etsy' | 'amazon' | 'ebay' | 'tiktok' | 'instagram' | 'website'

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

export type AppScreen = 'splash' | 'wizard' | 'office'

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
  data: Record<string, unknown>
}
