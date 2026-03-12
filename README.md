# Nova Link

<p align="center">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-blue" alt="Platform">
  <img src="https://img.shields.io/github/v/release/nova-link/nova-link" alt="Release">
  <img src="https://img.shields.io/github/license/nova-link/nova-link" alt="License">
</p>

<p align="center">
  ✨ 精美的桌面伴侣应用，集成 Live2D 角色与 AI 聊天 ✨
</p>

<p align="center">
  <img src="https://via.placeholder.com/400x500?text=Nova+Link+Screenshot" alt="Nova Link 截图" width="400">
</p>

---

## 目录

- [功能特点](#功能特点)
- [快速开始](#快速开始)
- [配置说明](#配置说明)
- [技术架构](#技术架构)
- [技术栈](#技术栈)
- [贡献者](#贡献者)
- [许可证](#许可证)

---

## 功能特点

- 🎨 **Live2D 角色展示** - 使用 WebGL 渲染精美的动画角色
- 🤖 **AI 聊天集成** - 连接 LLM API（OpenAI、MiniMax 或任何 OpenAI 兼容端点）
- 🌐 **WebSocket 服务器** - 内置 WebSocket 服务器，支持外部集成
- 🔲 **无框悬浮窗** - 透明、可置顶的悬浮窗口
- 💬 **右键菜单设置** - 右键配置模型、窗口和 LLM
- 📦 **系统托盘** - 后台静默运行
- 👤 **身份与灵魂设置** - 自定义角色的身份信息和人格设定

---

## 快速开始

### 预构建版本

下载对应平台的最新版本：

- **Windows**: `Nova Link_x.x.x_x64-setup.exe`
- **macOS**: `Nova Link_x.x.x_x64.dmg` / `Nova Link_x.x.x_aarch64.dmg`
- **Linux**: `Nova Link_x.x.x_amd64.AppImage`

[查看所有版本 →](https://github.com/nova-link/nova-link/releases)

### 从源码构建

#### 环境要求

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) 1.70+
- [npm](https://npmjs.com/)

#### 开发模式

```bash
# 安装依赖
npm install

# 开发模式运行
npm run tauri dev
```

#### 生产构建

```bash
# 构建当前平台
npm run tauri build

# 构建所有平台（需要交叉编译工具链）
npm run tauri build -- --target x86_64-pc-windows-msvc
npm run tauri build -- --target x86_64-apple-darwin
npm run tauri build -- --target aarch64-apple-darwin
npm run tauri build -- --target x86_64-unknown-linux-gnu
```

---

## 配置说明

在窗口任意位置右键访问上下文菜单：

| 设置项 | 说明 |
|--------|------|
| 模型路径 | Live2D 模型路径 (.model3.json) |
| 窗口 | 宽度和高度尺寸 |
| WebSocket 地址 | 外部客户端连接的服务器端点 |
| 聊天服务 | OpenClaw Gateway / LLM |
| LLM 提供商 | 选择：无 / MiniMax / OpenAI 兼容 |
| API Key | 你的 LLM API 密钥 |
| API 地址 | LLM API 端点 |
| 模型名称 | 使用的模型名称 |

### 身份与灵魂设置

通过右键菜单打开"角色设置"可以配置：

- **身份设置**: 名称、生物类型、气质、表情符号、头像
- **灵魂设置**: 角色性格、说话风格、情绪表达时机
- **应用设置**: 模型路径、窗口、聊天服务、LLM 配置

身份和灵魂设置会自动同步到 `~/.openclaw/workspace/` 目录。

### WebSocket API

Nova Link 在 `ws://localhost:18789`（可配置）运行 WebSocket 服务器。外部客户端可以连接并发送消息：

```json
{
  "type": "message",
  "content": "你好！",
  "sender_id": "client_1",
  "chat_id": "default"
}
```

---

## 技术架构

```
┌─────────────────────────┐
│      Nova Link 应用     │
├─────────────────────────┤
│  Tauri (Rust)          │
│  ├─ WebSocket 服务器   │
│  ├─ LLM 集成          │
│  ├─ SQLite 存储       │
│  └─ 系统托盘           │
├─────────────────────────┤
│  WebView (TypeScript)  │
│  ├─ PIXI.js           │
│  ├─ Live2D Display    │
│  └─ 聊天 UI            │
└─────────────────────────┘
```

### 透明窗口实现

采用多层透明机制，跨平台支持（macOS/Windows/Linux）：
1. Tauri 配置：`"transparent": true`
2. Rust 初始化：`window.eval()` 注入 JS 设置透明样式
3. Vue 渲染：`setTimeout(..., 100)` 延迟设置确保 DOM 渲染完成
4. 多层设置：body、documentElement、#app、#live2d-canvas 全部设为透明

---

## 技术栈

- **前端**: TypeScript, Vite, PIXI.js, pixi-live2d-display, Vue 3
- **后端**: Rust, Tauri v2, tokio-tungstenite, rusqlite
- **构建**: GitHub Actions, tauri-action

---

## 贡献者

感谢以下贡献者的贡献：

<!-- 在此处添加贡献者，按字母顺序排列 -->

| 贡献者 | 描述 |
|--------|------|
| [tangtianshuo](mailto:1034524076@qq.com) | 创始人、主要开发者 |

---

## 许可证

MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

---

<p align="center">
  用 ❤️ 制作 by Nova Link Team
</p>

---

# English Version

See [README_EN.md](README_EN.md)
