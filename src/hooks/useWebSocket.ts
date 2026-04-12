import { useEffect, useRef, useCallback } from 'react'
import { useAppStore } from '../store/useAppStore'
import type { WSMessage, Agent, Product, Desk, WalkEvent, TalkingTableEntry } from '../types'

const WS_URL = 'ws://localhost:8765/ws'

export function useWebSocket() {
  const wsRef = useRef<WebSocket | null>(null)
  const reconnectTimer = useRef<ReturnType<typeof setTimeout> | null>(null)
  const {
    updateAgent, addAgent, updateProduct, addEvent, addDesk,
    setBackendReady, addTalkingMessage, setWalk,
  } = useAppStore()

  const handleMessage = useCallback(
    (msg: WSMessage) => {
      switch (msg.type) {
        case 'agent_update': {
          const agent = msg.data as unknown as Agent
          updateAgent(agent)
          break
        }
        case 'agent_arrival': {
          // Treat as agent_update — the visual arrival is handled by status 'entering'
          // The agent record is already in DB and broadcast via agent_update separately
          break
        }
        case 'agent_walk': {
          const walk = msg.data as unknown as WalkEvent
          setWalk(walk)
          break
        }
        case 'talking_table': {
          const entry: TalkingTableEntry = {
            message: String(msg.data.message || ''),
            timestamp: Number(msg.data.timestamp || Date.now()),
          }
          addTalkingMessage(entry)
          break
        }
        case 'desk_created': {
          const { desk, roomId } = msg.data as { desk: Desk; roomId: string }
          addDesk(desk, roomId)
          const agent = msg.data.agent as Agent | undefined
          if (agent) addAgent(agent)
          break
        }
        case 'product_update': {
          const product = msg.data as unknown as Product
          updateProduct(product)
          break
        }
        case 'system_event': {
          addEvent({
            id: String(Date.now()),
            type: (msg.data.eventType as 'agent_created' | 'agent_update' | 'product_update' | 'system' | 'error') || 'system',
            message: String(msg.data.message || ''),
            data: msg.data,
            timestamp: Date.now(),
          })
          break
        }
        case 'heartbeat':
          break
      }
    },
    [updateAgent, addAgent, updateProduct, addEvent, addDesk, setBackendReady, addTalkingMessage, setWalk]
  )

  const connect = useCallback(() => {
    if (wsRef.current?.readyState === WebSocket.OPEN) return

    const ws = new WebSocket(WS_URL)
    wsRef.current = ws

    ws.onopen = () => {
      setBackendReady(true)
      addEvent({
        id: String(Date.now()),
        type: 'system',
        message: 'Connected to AI Commerce HQ runtime',
        timestamp: Date.now(),
      })
    }

    ws.onmessage = (e) => {
      try {
        const msg: WSMessage = JSON.parse(e.data)
        handleMessage(msg)
      } catch (err) {
        console.error('WS parse error', err)
      }
    }

    ws.onclose = () => {
      setBackendReady(false)
      reconnectTimer.current = setTimeout(connect, 3000)
    }

    ws.onerror = () => {
      ws.close()
    }
  }, [handleMessage, setBackendReady, addEvent])

  useEffect(() => {
    connect()
    return () => {
      if (reconnectTimer.current) clearTimeout(reconnectTimer.current)
      wsRef.current?.close()
    }
  }, [connect])
}
