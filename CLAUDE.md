# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Nova Link is a Tauri v2 desktop overlay application featuring a floating glassmorphism UI with Live2D character display. It connects to OpenClaw Gateway via WebSocket for chat functionality, with optional LLM integration.

## Technology Stack

- **Frontend**: Vue 3 + TypeScript + Vite 6
- **Desktop**: Tauri v2 (Rust)
- **Live2D**: pixi.js + pixi-live2d-display (Cubism 4.x)
- **Storage**: SQLite (rusqlite) for settings persistence
- **WebSocket**: GatewayClient SDK connecting to OpenClaw Gateway

## Common Commands

```bash
# Development
npm run dev              # Start Vite dev server (port 18080)
npm run tauri dev        # Run full Tauri app in dev mode

# Build
npm run build            # TypeScript compile + Vite build
npm run tauri build      # Build production Tauri app
```

## Architecture

```
┌─────────────────────────┐
│   OpenClaw Gateway     │  (ws://127.0.0.1:18789)
└──────────┬────────────┘
           │ WebSocket (GatewayClient SDK)
           ▼
┌─────────────────────────┐
│   Nova Link (Tauri)    │
│   + Vue 3 Frontend     │  - Live2D Display
│   + WebSocket Client   │  - Chat Panel
└─────────────────────────┘
```

### Key Files

| File | Purpose |
|------|---------|
| `src/App.vue` | Main Vue component - initialization, event handling |
| `src/composables/useWebSocket.ts` | WebSocket connection management with auto-reconnect |
| `src/composables/useChat.ts` | Chat state management |
| `src/composables/useLive2D.ts` | Live2D model loading and control |
| `src/composables/useSettings.ts` | Settings persistence |
| `src/sdk/client.ts` | GatewayClient SDK - WebSocket protocol handling |
| `src/components/ChatPanel.vue` | Chat UI with glassmorphism styling |
| `src/components/ContextMenu.vue` | Right-click context menu |
| `src-tauri/src/lib.rs` | Rust backend - Gateway startup, settings storage, system tray |

### Chat Message Flow

1. **User sends message** → ChatPanel emits send event → App.vue calls `sendWsMessage()`
2. **Gateway responds** → SDK receives `agent` event with lifecycle (start/end)
3. **Show "thinking"** → `onMessageStart` callback displays thinking indicator
4. **Receive response** → `onMessageStop` callback + fetch history or stream content
5. **Display in chat** → Add bot message to chat panel

### Frontend ↔ Rust Commands

- `invoke("run_gateway")` - Start OpenClaw Gateway via PowerShell
- `invoke("save_setting", { key, value })` - Save to SQLite
- `invoke("get_setting", { key })` - Load from SQLite

## Window Behavior

- **Close button**: Hides to system tray instead of exiting
- **System tray**: Left-click shows window, right-click shows menu (显示/退出)
- **Draggable**: Drag the title bar region to move window
- **Always on top**: Enabled by default, configurable via settings

## Settings (Right-click Context Menu)

- Model path (Live2D model .model3.json)
- WebSocket URL (default: ws://127.0.0.1:18789/)
- Chat provider (openclaw/llm)
- LLM configuration (when chat provider is llm)

## Important Implementation Details

- Gateway events: `agent` (lifecycle stream), `chat` (message state), `tick` (heartbeat)
- Auto-reconnect: 5-second interval when disconnected
- Chat history: Loaded when chat panel is opened (20 messages)
- Message handling: Filters user messages (role=user), only displays assistant messages
