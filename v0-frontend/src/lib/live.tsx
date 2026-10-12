import { createContext, useContext, useEffect, useState, type ReactNode } from 'react'
import { useSWRConfig } from 'swr'
import { config } from '@/config'
import { applyEvent, type ArgoWorkflow, type WorkflowEvent } from './argo'
import { subscribeDemo } from './demo'
import { RUNS_KEY } from './api'
import { useAuth } from './auth'

export type LiveStatus = 'connecting' | 'live' | 'reconnecting' | 'offline'

interface LiveContextValue {
  status: LiveStatus
  lastEventAt: number | null
}

const LiveContext = createContext<LiveContextValue>({ status: 'offline', lastEventAt: null })

export function LiveUpdatesProvider({ children }: { children: ReactNode }) {
  const { session } = useAuth()
  const { mutate } = useSWRConfig()
  const [status, setStatus] = useState<LiveStatus>('connecting')
  const [lastEventAt, setLastEventAt] = useState<number | null>(null)
  const token = session?.accessToken

  useEffect(() => {
    if (!token) return

    const handle = (event: WorkflowEvent) => {
      setLastEventAt(Date.now())
      mutate<ArgoWorkflow[]>(
        (key) => Array.isArray(key) && key[0] === RUNS_KEY,
        (current) => (current ? applyEvent(current, event) : current),
        { revalidate: false },
      )
    }

    if (config.demoMode) {
      setStatus('live')
      return subscribeDemo(handle)
    }

    if (!config.wsUrl) {
      setStatus('offline')
      return
    }

    let socket: WebSocket | null = null
    let retry = 0
    let retryTimer: ReturnType<typeof setTimeout> | undefined
    let closedByUs = false

    const connect = () => {
      setStatus(retry === 0 ? 'connecting' : 'reconnecting')
      const url = new URL(config.wsUrl)
      url.searchParams.set('namespace', config.argoNamespace)
      url.searchParams.set('access_token', token)
      socket = new WebSocket(url.toString())

      socket.onopen = () => {
        retry = 0
        setStatus('live')
        // Fill any gap that occurred while disconnected.
        mutate((key) => Array.isArray(key) && key[0] === RUNS_KEY)
      }
      socket.onmessage = (msg) => {
        try {
          const data = JSON.parse(msg.data as string) as WorkflowEvent | { result: WorkflowEvent }
          const event = 'result' in data ? data.result : data
          if (event?.workflow?.metadata) handle(event)
        } catch {
          // ignore non-JSON heartbeat frames
        }
      }
      socket.onclose = () => {
        if (closedByUs) return
        retry += 1
        setStatus('reconnecting')
        const delay = Math.min(30_000, 1000 * 2 ** Math.min(retry, 5))
        retryTimer = setTimeout(connect, delay)
      }
      socket.onerror = () => socket?.close()
    }

    connect()
    return () => {
      closedByUs = true
      if (retryTimer) clearTimeout(retryTimer)
      socket?.close()
    }
  }, [token, mutate])

  return <LiveContext.Provider value={{ status, lastEventAt }}>{children}</LiveContext.Provider>
}

export function useLiveStatus() {
  return useContext(LiveContext)
}
