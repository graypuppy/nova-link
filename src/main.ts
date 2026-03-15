import { createApp } from "vue"
import App from "./App.vue"
import { Dialog } from "./components"
import { attachConsole, error } from "@tauri-apps/plugin-log"

// 初始化日志插件
attachConsole().then(() => {
  console.log("[Frontend] Nova Link frontend initialized")
})

const app = createApp(App)

// 全局注册 Dialog 组件
app.component("Dialog", Dialog)

// 全局错误处理 - 将错误日志发送到 Rust 控制台
app.config.errorHandler = (err, _instance, info) => {
  error(`[Vue Error] ${err}: ${info}`)
}

app.mount("#app")
