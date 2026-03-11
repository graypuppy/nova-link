import { ref } from "vue"
import { GatewayClient } from "../sdk/index.js"

export type WsStatus = "connected" | "connecting" | "disconnected" | "error"

export interface UseWebSocketOptions {
  onMessage?: (message: any) => void
  onStatusChange?: (status: WsStatus) => void
  onStreamUpdate?: (text: string) => void
  onMessageStart?: (payload: any) => void
  onMessageStop?: (payload: any) => void
  onConnected?: (hello: any) => void
  onError?: (error: string) => void
}

export function useWebSocket(options: UseWebSocketOptions = {}) {
  const wsStatus = ref<WsStatus>("disconnected")
  let gwClient: GatewayClient | null = null

  // 自动重连相关
  let reconnectTimer: ReturnType<typeof setTimeout> | null = null
  let reconnectUrl: string = ""
  let reconnectToken: string = ""
  const RECONNECT_INTERVAL = 5000 // 5秒重连一次

  function connectWebSocket(url: string, token?: string): void {
    console.log("[useWebSocket] Connecting to Gateway with params:", {
      url,
      token: token ? "***" : "",
    })

    // 保存连接参数用于重连
    reconnectUrl = url
    reconnectToken = token || ""

    if (gwClient) {
      console.log("[useWebSocket] Disconnecting existing Gateway client")
      gwClient.disconnect()
    }

    // 清除重连定时器
    if (reconnectTimer) {
      clearTimeout(reconnectTimer)
      reconnectTimer = null
    }

    console.log("[useWebSocket] Creating new GatewayClient...")
    gwClient = new GatewayClient({
      url,
      token,
      onStatusChange: (status) => {
        console.log("[useWebSocket] Gateway status changed:", status)
        wsStatus.value = status as WsStatus
        options.onStatusChange?.(status as WsStatus)
      },
      onMessage: (message) => {
        console.log("[useWebSocket] Gateway message received:", message)
        options.onMessage?.(message)
      },
      onStreamUpdate: (text) => {
        console.log("[useWebSocket] Gateway stream update:", text)
        options.onStreamUpdate?.(text)
      },
      onMessageStart: (payload) => {
        console.log("[useWebSocket] Message start:", payload)
        options.onMessageStart?.(payload)
      },
      onContentDelta: (payload) => {
        console.log("[useWebSocket] Content delta:", payload.delta)
      },
      onMessageDelta: (payload) => {
        console.log("[useWebSocket] Message delta:", payload)
      },
      onMessageStop: (payload) => {
        console.log("[useWebSocket] Message stop:", payload)
        options.onMessageStop?.(payload)
      },
      onToolUse: (payload) => {
        console.log("[useWebSocket] Tool use:", payload)
      },
      onToolResult: (payload) => {
        console.log("[useWebSocket] Tool result:", payload)
      },
      onConnected: (hello) => {
        console.log("[useWebSocket] Gateway connected, version:", hello.server.version)
        // 连接成功后清除重连定时器
        if (reconnectTimer) {
          clearTimeout(reconnectTimer)
          reconnectTimer = null
        }
        options.onConnected?.(hello)
      },
      onError: (error) => {
        console.error("[useWebSocket] Gateway error:", error)
        options.onError?.(error)
      },
      onDisconnected: () => {
        console.log("[useWebSocket] Gateway disconnected, scheduling reconnect...")
        scheduleReconnect()
      },
    })

    gwClient.connect().catch((err) => {
      console.error("Failed to connect to Gateway:", err)
      scheduleReconnect()
    })
  }

  function scheduleReconnect(): void {
    if (reconnectTimer) {
      return // 已经在等待重连
    }
    console.log(`[useWebSocket] Scheduling reconnect in ${RECONNECT_INTERVAL}ms...`)
    reconnectTimer = setTimeout(() => {
      reconnectTimer = null
      if (reconnectUrl) {
        console.log("[useWebSocket] Attempting to reconnect...")
        connectWebSocket(reconnectUrl, reconnectToken)
      }
    }, RECONNECT_INTERVAL)
  }

  function disconnectWebSocket(): void {
    // 清除重连定时器
    if (reconnectTimer) {
      clearTimeout(reconnectTimer)
      reconnectTimer = null
    }
    reconnectUrl = ""
    reconnectToken = ""

    if (gwClient) {
      gwClient.disconnect()
      gwClient = null
    }
    wsStatus.value = "disconnected"
  }

  async function sendMessage(content: string): Promise<void> {
    if (gwClient && gwClient.isConnected) {
      console.log("[useWebSocket] Sending message via Gateway:", content)
      await gwClient.sendMessage({ message: content })
      console.log("[useWebSocket] Message sent successfully")
    } else {
      throw new Error("Gateway not connected")
    }
  }

  function isConnected(): boolean {
    return gwClient?.isConnected ?? false
  }

  return {
    wsStatus,
    connectWebSocket,
    disconnectWebSocket,
    sendMessage,
    isConnected,
  }
}
