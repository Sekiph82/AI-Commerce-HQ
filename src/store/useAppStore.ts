import { create } from 'zustand'
import type {
  Agent, Product, Room, SystemEvent, AppConfig, AppScreen,
  Desk, TalkingTableEntry, WalkEvent, RevenueData,
} from '../types'

const API_BASE = 'http://localhost:8765'

interface AppState {
  screen: AppScreen
  config: AppConfig | null
  agents: Record<string, Agent>
  products: Record<string, Product>
  rooms: Record<string, Room>
  events: SystemEvent[]
  talkingTable: TalkingTableEntry[]
  walkingAgents: Record<string, WalkEvent>
  selectedRoom: string | null
  approvalPanelOpen: boolean
  approvalPlatform: string | null
  statusPanelOpen: boolean
  settingsPanelOpen: boolean
  tradingPanelOpen: boolean
  achievementsPanelOpen: boolean
  backendReady: boolean
  revenue: RevenueData

  setScreen: (screen: AppScreen) => void
  setConfig: (config: AppConfig) => void
  updateAgent: (agent: Agent) => void
  addAgent: (agent: Agent) => void
  updateProduct: (product: Product) => void
  addProduct: (product: Product) => void
  addEvent: (event: SystemEvent) => void
  addTalkingMessage: (entry: TalkingTableEntry) => void
  setWalk: (walk: WalkEvent) => void
  setSelectedRoom: (roomId: string | null) => void
  openApproval: (platform: string) => void
  closeApproval: () => void
  toggleStatusPanel: () => void
  toggleSettingsPanel: () => void
  openTradingPanel: () => void
  closeTradingPanel: () => void
  toggleAchievementsPanel: () => void
  setBackendReady: (ready: boolean) => void
  addDesk: (desk: Desk, roomId: string) => void
  initRooms: () => void
  fetchInitialState: () => Promise<void>
  approveProduct: (productId: string) => Promise<void>
  rejectProduct: (productId: string) => Promise<void>
  archiveProduct: (productId: string) => Promise<void>
}

const initialRooms: Record<string, Room> = {
  gmo: {
    id: 'gmo',
    name: 'Global Master Orchestrator',
    platform: 'global',
    isActive: true,
    agents: [],
    desks: [
      { id: 'desk-gmo-main', nameplate: 'Global Master Orchestrator', agentLabel: 'GMO', roomId: 'gmo', position: { x: 50, y: 40 } },
    ],
    color: '#1f3a6e',
  },
  etsy: {
    id: 'etsy',
    name: 'Etsy Operations',
    platform: 'etsy',
    isActive: true,
    agents: [],
    desks: [
      { id: 'desk-etmo-main', nameplate: 'Etsy Master Orchestrator', agentLabel: 'ETMO', roomId: 'etsy', position: { x: 50, y: 50 } },
      { id: 'desk-etsy-approval', nameplate: 'Etsy Approval Desk', agentLabel: 'APPROVAL', roomId: 'etsy', isApprovalDesk: true, position: { x: 85, y: 50 } },
      { id: 'desk-etsy-trd',  nameplate: 'Trend Research Agent',            agentLabel: 'TRD',   roomId: 'etsy', position: { x: 45, y: 12 } },
      { id: 'desk-etsy-des1', nameplate: 'Primary Design Agent',            agentLabel: 'DES-1', roomId: 'etsy', position: { x: 10, y: 12 } },
      { id: 'desk-etsy-des2', nameplate: 'Variant Design Agent',            agentLabel: 'DES-2', roomId: 'etsy', position: { x: 10, y: 88 } },
      { id: 'desk-etsy-prd1', nameplate: 'Primary Product Agent',           agentLabel: 'PRD-1', roomId: 'etsy', position: { x: 27, y: 12 } },
      { id: 'desk-etsy-prd2', nameplate: 'Alternative Product Agent',       agentLabel: 'PRD-2', roomId: 'etsy', position: { x: 27, y: 88 } },
      { id: 'desk-etsy-qa',   nameplate: 'Quality Assurance Agent',         agentLabel: 'QA',    roomId: 'etsy', position: { x: 80, y: 12 } },
      { id: 'desk-etsy-pod',  nameplate: 'POD Fulfillment / Mockup Agent',  agentLabel: 'POD',   roomId: 'etsy', position: { x: 62, y: 88 } },
      { id: 'desk-etsy-lst',  nameplate: 'Listing / SEO / Copyright Agent', agentLabel: 'LST',   roomId: 'etsy', position: { x: 80, y: 88 } },
      { id: 'desk-etsy-vid',  nameplate: 'Video Agent',                     agentLabel: 'VID',   roomId: 'etsy', position: { x: 45, y: 88 } },
      { id: 'desk-etsy-per',  nameplate: 'Personalization Agent',           agentLabel: 'PER',   roomId: 'etsy', position: { x: 62, y: 12 } },
    ],
    color: '#4f2d1e',
  },
  amazon: {
    id: 'amazon', name: 'Amazon Operations', platform: 'amazon', isActive: false, agents: [],
    desks: [{ id: 'desk-amazon-main', nameplate: 'Amazon MO', agentLabel: 'AMO', roomId: 'amazon', position: { x: 50, y: 40 } }],
    color: '#1a3a2a',
  },
  ebay: {
    id: 'ebay', name: 'eBay Operations', platform: 'ebay', isActive: false, agents: [],
    desks: [{ id: 'desk-ebay-main', nameplate: 'eBay MO', agentLabel: 'EBMO', roomId: 'ebay', position: { x: 50, y: 40 } }],
    color: '#1a1f3a',
  },
  tiktok: {
    id: 'tiktok', name: 'TikTok Shop Operations', platform: 'tiktok', isActive: false, agents: [],
    desks: [{ id: 'desk-tiktok-main', nameplate: 'TikTok MO', agentLabel: 'TTMO', roomId: 'tiktok', position: { x: 50, y: 40 } }],
    color: '#2d1a3a',
  },
  instagram: {
    id: 'instagram', name: 'Instagram/Facebook Shop', platform: 'instagram', isActive: false, agents: [],
    desks: [{ id: 'desk-ig-main', nameplate: 'Instagram MO', agentLabel: 'IGMO', roomId: 'instagram', position: { x: 50, y: 40 } }],
    color: '#3a1a2d',
  },
  website: {
    id: 'website', name: 'Website Store', platform: 'website', isActive: false, agents: [],
    desks: [{ id: 'desk-web-main', nameplate: 'Website MO', agentLabel: 'WSMO', roomId: 'website', position: { x: 50, y: 40 } }],
    color: '#1a2a3a',
  },
}

export const useAppStore = create<AppState>((set, get) => ({
  screen: 'splash',
  config: null,
  agents: {},
  products: {},
  rooms: initialRooms,
  events: [],
  talkingTable: [],
  walkingAgents: {},
  selectedRoom: null,
  approvalPanelOpen: false,
  approvalPlatform: null,
  statusPanelOpen: false,
  settingsPanelOpen: false,
  tradingPanelOpen: false,
  achievementsPanelOpen: false,
  backendReady: false,
  revenue: { total: 0, etsy: 0, fiverr: 0, trading: 0, youtube: 0, tiktok: 0, level: 1, xp: 0, xpToNext: 100 },

  setScreen: (screen) => set({ screen }),
  setConfig: (config) => set({ config }),

  updateAgent: (agent) =>
    set((state) => ({
      agents: { ...state.agents, [agent.id]: agent },
    })),

  addAgent: (agent) =>
    set((state) => ({
      agents: { ...state.agents, [agent.id]: agent },
      rooms: {
        ...state.rooms,
        [agent.platform === 'global' ? 'gmo' : agent.platform]: {
          ...state.rooms[agent.platform === 'global' ? 'gmo' : agent.platform],
          agents: [
            ...(state.rooms[agent.platform === 'global' ? 'gmo' : agent.platform]?.agents || []),
            agent.id,
          ],
        },
      },
    })),

  updateProduct: (product) =>
    set((state) => {
      const newProducts = { ...state.products, [product.id]: product }
      let etsyTotal = 0
      Object.values(newProducts).forEach(p => {
        if (p.platform === 'etsy' && p.state === 'ETSY_DRAFT_CREATED') {
          etsyTotal += (p.price || 25) * (p.estimatedMargin || 35) / 100
        }
      })
      const newTotal = etsyTotal + state.revenue.fiverr + state.revenue.trading + state.revenue.youtube + state.revenue.tiktok
      const newLevel = Math.floor(newTotal / 100) + 1
      const newXp = newTotal % 100
      return {
        products: newProducts,
        revenue: {
          ...state.revenue,
          total: newTotal,
          etsy: etsyTotal,
          level: newLevel,
          xp: newXp,
        },
      }
    }),

  addProduct: (product) =>
    set((state) => ({
      products: { ...state.products, [product.id]: product },
    })),

  addEvent: (event) =>
    set((state) => ({
      events: [event, ...state.events].slice(0, 100),
    })),

  addTalkingMessage: (entry) =>
    set((state) => ({
      talkingTable: [entry, ...state.talkingTable].slice(0, 30),
    })),

  setWalk: (walk) => {
    set((state) => ({
      walkingAgents: { ...state.walkingAgents, [walk.agentLabel]: walk },
    }))
    // Auto-clear after the walk duration + buffer
    setTimeout(() => {
      set((state) => {
        const { [walk.agentLabel]: _removed, ...rest } = state.walkingAgents
        return { walkingAgents: rest }
      })
    }, walk.durationMs + 600)
  },

  setSelectedRoom: (roomId) => set({ selectedRoom: roomId }),
  openApproval: (platform) => set({ approvalPanelOpen: true, approvalPlatform: platform }),
  closeApproval: () => set({ approvalPanelOpen: false, approvalPlatform: null }),
  toggleStatusPanel: () => set((s) => ({ statusPanelOpen: !s.statusPanelOpen })),
  toggleSettingsPanel: () => set((s) => ({ settingsPanelOpen: !s.settingsPanelOpen })),
  openTradingPanel: () => set({ tradingPanelOpen: true }),
  closeTradingPanel: () => set({ tradingPanelOpen: false }),
  toggleAchievementsPanel: () => set((s) => ({ achievementsPanelOpen: !s.achievementsPanelOpen })),
  setBackendReady: (ready) => set({ backendReady: ready }),

  addDesk: (desk, roomId) =>
    set((state) => ({
      rooms: {
        ...state.rooms,
        [roomId]: {
          ...state.rooms[roomId],
          desks: [...(state.rooms[roomId]?.desks || []), desk],
        },
      },
    })),

  initRooms: () => set({ rooms: initialRooms }),

  fetchInitialState: async () => {
    try {
      const [agentsRes, productsRes] = await Promise.all([
        fetch(`${API_BASE}/api/agents`),
        fetch(`${API_BASE}/api/products`),
      ])

      const agentsData = agentsRes.ok ? await agentsRes.json() : []
      const productsData = productsRes.ok ? await productsRes.json() : []

      const agents: Agent[] = Array.isArray(agentsData) ? agentsData : []
      const products: Product[] = Array.isArray(productsData) ? productsData : []

      const agentMap: Record<string, Agent> = {}
      agents.forEach((a) => (agentMap[a.id] = a))

      const productMap: Record<string, Product> = {}
      products.forEach((p) => (productMap[p.id] = p))

      set({ agents: agentMap, products: productMap })
    } catch (e) {
      console.error('Failed to fetch initial state', e)
    }
  },

  approveProduct: async (productId) => {
    // Step 1: Mark as approved
    await fetch(`${API_BASE}/api/products/${productId}/approve`, { method: 'POST' })
    // Step 2: Create Etsy draft (the ONLY path to Etsy)
    await fetch(`${API_BASE}/api/products/${productId}/publish`, { method: 'POST' })
    // State updates come via WebSocket broadcasts from the server
  },

  rejectProduct: async (productId) => {
    await fetch(`${API_BASE}/api/products/${productId}/reject`, { method: 'POST' })
    set((state) => ({
      products: {
        ...state.products,
        [productId]: { ...state.products[productId], state: 'REJECTED' },
      },
    }))
  },

  archiveProduct: async (productId) => {
    await fetch(`${API_BASE}/api/products/${productId}/archive`, { method: 'POST' })
    set((state) => ({
      products: {
        ...state.products,
        [productId]: { ...state.products[productId], state: 'ARCHIVED' },
      },
    }))
  },
}))
