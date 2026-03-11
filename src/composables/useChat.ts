import { ref, nextTick } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { useSettings } from "./useSettings"

export interface ChatMessage {
  type: "user" | "bot"
  content: string
}

export function useChat() {
  const { settings } = useSettings()

  const messages = ref<ChatMessage[]>([])
  const inputMessage = ref("")
  const isChatVisible = ref(false)
  const isSending = ref(false)
  let lastBotMessageEl: HTMLElement | null = null

  function addMessage(type: "user" | "bot", content: string): void {
    messages.value.push({ type, content })

    nextTick(() => {
      const container = document.getElementById("messages")
      if (container) {
        container.scrollTop = container.scrollHeight
      }

      const chatPanel = document.getElementById("chat-panel")
      if (chatPanel?.classList.contains("hidden")) {
        toggleChat(true)
      }
    })
  }

  function toggleChat(show: boolean): void {
    isChatVisible.value = show
    if (show) {
      nextTick(() => {
        const inputEl = document.getElementById("message-input") as HTMLInputElement
        inputEl?.focus()
      })
    }
  }

  function clearMessages(): void {
    messages.value = []
    lastBotMessageEl = null
  }

  async function sendViaLlm(content: string): Promise<void> {
    if (
      settings.value.llmProvider !== "none" &&
      settings.value.llmApiKey &&
      settings.value.llmApiUrl &&
      settings.value.llmModel
    ) {
      addMessage("bot", "正在思考...")

      const response = await invoke<string>("chat_with_llm", {
        provider: settings.value.llmProvider,
        apiKey: settings.value.llmApiKey,
        apiUrl: settings.value.llmApiUrl,
        model: settings.value.llmModel,
        message: content,
      })

      const msgEls = document.querySelectorAll(".message.bot")
      const lastBotMsg = msgEls[msgEls.length - 1]
      if (lastBotMsg && lastBotMsg.textContent === "正在思考...") {
        lastBotMsg.textContent = response
      } else {
        addMessage("bot", response)
      }
    } else {
      addMessage("bot", "未配置 LLM。请在设置中配置 LLM API。")
    }
  }

  async function sendMessage(sendViaWs: () => Promise<void>): Promise<void> {
    const content = inputMessage.value.trim()
    if (!content || isSending.value) return

    isSending.value = true

    try {
      addMessage("user", content)
      inputMessage.value = ""

      if (settings.value.chatProvider === "llm") {
        await sendViaLlm(content)
      } else {
        await sendViaWs()
      }
    } catch (e) {
      console.error("Failed to send message:", e)
      addMessage("bot", `发送失败: ${e}`)
    } finally {
      isSending.value = false
    }
  }

  function updateLastBotMessage(text: string): void {
    if (lastBotMessageEl) {
      lastBotMessageEl.textContent = text
    } else {
      addMessage("bot", text)
      const msgEls = document.querySelectorAll(".message.bot")
      lastBotMessageEl = msgEls[msgEls.length - 1] as HTMLElement
    }
  }

  function startThinking(): void {
    addMessage("bot", "正在思考...")
    const msgEls = document.querySelectorAll(".message.bot")
    lastBotMessageEl = msgEls[msgEls.length - 1] as HTMLElement
  }

  function stopStreaming(): void {
    lastBotMessageEl = null
  }

  function handleKeyPress(e: KeyboardEvent, sendFn: () => void): void {
    if (e.key === "Enter") {
      sendFn()
    }
  }

  return {
    messages,
    inputMessage,
    isChatVisible,
    isSending,
    addMessage,
    toggleChat,
    clearMessages,
    sendMessage,
    updateLastBotMessage,
    startThinking,
    stopStreaming,
    handleKeyPress,
  }
}
