import { createApp } from "vue"
import App from "./App.vue"
import { Dialog } from "./components"

const app = createApp(App)

// 全局注册 Dialog 组件
app.component("Dialog", Dialog)

app.mount("#app")
