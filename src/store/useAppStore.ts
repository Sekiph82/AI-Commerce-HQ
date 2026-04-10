import { create } from 'zustand'
import type { Agent, Product, Room, SystemEvent, AppConfig, AppScreen, Desk } from '../types'

const API_BASE = 'http://localhost:8765'

interface AppState {
  screen: AppScreen
  config: AppConfig | null
  agents: Record<string, Agent>
  products: Record<string, Product>
  rooms: Record<string, Room>
  events: SystemEvent[]
  selectedRoom: string | null
  approvalPanelOpen: boolean
  approvalPlatform: string | null
  statusPanelOpen: boolean
  settingsPanelOpen: boolean
  backendReady: boolean

  setScreen: (screen: AppScreen) => void
  setConfig: (config: AppConfig) => void
  updateAgent: (agent: Agent) => void
  addAgent: (agent: Agent) => void
  updateProduct: (product: Product) => void
  addProduct: (product: Product) => void
  addEvent: (event: SystemEvent) => void
  setSelectedRoom: (roomId: string | null) => void
  openApproval: (platform: string) => void
  closeApproval: () => void
  toggleStatusPanel: () => void
  toggleSettingsPanel: () => void
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
      { id: 'desk-gmo-approval', nameplate: 'HQ Approval Desk', agentLabel: 'APPROVAL', roomId: 'gmo', isApprovalDesk: true, position: { x: 50, y: 70 } },
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
      { id: 'desk-etmo-main', nameplate: 'Etsy Master Orchestrator', agentLabel: 'ETMO', roomId: 'etsy', position: { x: 20, y: 30 } },
      { id: 'desk-etsy-approval', nameplate: 'Etsy Approval Desk', agentLabel: 'APPROVAL', roomId: 'etsy', isApprovalDesk: true, position: { x: 20, y: 70 } },
    ],
    color: '#4f2d1e',
  },
  amazon: {
    id: 'amazon',
    name: 'Amazon Operations',
    platform: 'amazon',
    isActive: false,
    agents: [],
    desks: [
      { id: 'desk-amazon-main', nameplate: 'Amazon Master Orchestrator', agentLabel: 'AMO', roomId: 'amazon', position: { x: 50, y: 40 } },
      { id: 'desk-amazon-approval', nameplate: 'Amazon Approval Desk', agentLabel: 'APPROVAL', roomId: 'amazon', isApprovalDesk: true, position: { x: 50, y: 70 } },
    ],
    color: '#1a3a2a',
  },
  ebay: {
    id: 'ebay',
    name: 'eBay Operations',
    platform: 'ebay',
    isActive: false,
    agents: [],
    desks: [
      { id: 'desk-ebay-main', nameplate: 'eBay Master Orchestrator', agentLabel: 'EBMO', roomId: 'ebay', position: { x: 50, y: 40 } },
      { id: 'desk-ebay-approval', nameplate: 'eBay Approval Desk', agentLabel: 'APPROVAL', roomId: 'ebay', isApprovalDesk: true, position: { x: 50, y: 70 } },
    ],
    color: '#1a1f3a',
  },
  tiktok: {
    id: 'tiktok',
    name: 'TikTok Shop Operations',
    platform: 'tiktok',
    isActive: false,
    agents: [],
    desks: [
      { id: 'desk-tiktok-main', nameplate: 'TikTok Master Orchestrator', agentLabel: 'TTMO', roomId: 'tiktok', position: { x: 50, y: 40 } },
      { id: 'desk-tiktok-approval', nameplate: 'TikTok Approval Desk', agentLabel: 'APPROVAL', roomId: 'tiktok', isApprovalDesk: true, position: { x: 50, y: 70 } },
    ],
    color: '#2d1a3a',
  },
  instagram: {
    id: 'instagram',
    name: 'Instagram/Facebook Shop',
    platform: 'instagram',
    isActive: false,
    agents: [],
    desks: [
      { id: 'desk-ig-main', nameplate: 'Instagram Master Orchestrator', agentLabel: 'IGMO', roomId: 'instagram', position: { x: 50, y: 40 } },
      { id: 'desk-ig-approval', nameplate: 'IG Approval Desk', agentLabel: 'APPROVAL', roomId: 'instagram', isApprovalDesk: true, position: { x: 50, y: 70 } },
    ],
    color: '#3a1a2d',
  },
  website: {
    id: 'website',
    name: 'Website Store',
    platform: 'website',
    isActive: false,
    agents: [],
    desks: [
      { id: 'desk-web-main', nameplate: 'Website Master Orchestrator', agentLabel: 'WSMO', roomId: 'website', position: { x: 50, y: 40 } },
      { id: 'desk-web-approval', nameplate: 'Web Approval Desk', agentLabel: 'APPROVAL', roomId: 'website', isApprovalDesk: true, position: { x: 50, y: 70 } },
    ],
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
  selectedRoom: null,
  approvalPanelOpen: false,
  approvalPlatform: null,
  statusPanelOpen: false,
  settingsPanelOpen: false,
  backendReady: false,

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
    set((state) => ({
      products: { ...state.products, [product.id]: product },
    })),

  addProduct: (product) =>
    set((state) => ({
      products: { ...state.products, [product.id]: product },
    })),

  addEvent: (event) =>
    set((state) => ({
      events: [event, ...state.events].slice(0, 100),
    })),

  setSelectedRoom: (roomId) => set({ selectedRoom: roomId }),
  openApproval: (platform) => set({ approvalPanelOpen: true, approvalPlatform: platform }),
  closeApproval: () => set({ approvalPanelOpen: false, approvalPlatform: null }),
  toggleStatusPanel: () => set((s) => ({ statusPanelOpen: !s.statusPanelOpen })),
  toggleSettingsPanel: () => set((s) => ({ settingsPanelOpen: !s.settingsPanelOpen })),
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
      const agents: Agent[] = await agentsRes.json()
      const products: Product[] = await productsRes.json()

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
    await fetch(`${API_BASE}/api/products/${productId}/approve`, { method: 'POST' })
    const { products } = get()
    if (products[productId]) {
      set((state) => ({
        products: {
          ...state.products,
          [productId]: { ...state.products[productId], state: 'APPROVED_FOR_PUBLISH' },
        },
      }))
    }
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
