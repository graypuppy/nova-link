<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue"

const props = defineProps<{
  visible: boolean
  x: number
  y: number
}>()

const emit = defineEmits<{
  close: []
  settings: []
  reloadModel: []
  toggleAlwaysOnTop: []
  minimize: []
  closeWindow: []
  resetWindowSize: []
  previewState: [state: string]
  previewMotion: [motion: string]
  resetToIdle: []
  runGateway: []
  reconnectWs: []
}>()

// 开发环境标识
const isDev = ref(import.meta.env.DEV)

// 动画状态列表
const animationStates = [
  { label: "IDLE (待机)", value: "IDLE" },
  { label: "GREETING (问候)", value: "GREETING" },
  { label: "TALKING (说话)", value: "TALKING" },
  { label: "LISTENING (倾听)", value: "LISTENING" },
  { label: "THINKING (思考)", value: "THINKING" },
  { label: "HAPPY (开心)", value: "HAPPY" },
  { label: "SAD (难过)", value: "SAD" },
  { label: "SURPRISED (惊讶)", value: "SURPRISED" },
  { label: "ANGRY (生气)", value: "ANGRY" },
  { label: "SLEEPING (睡觉)", value: "SLEEPING" },
]

// 动画动作列表
const motionGroups = [
  { label: "Tap (点击)", value: "Tap" },
  { label: "Tap@Body (点击身体)", value: "Tap@Body" },
  { label: "Flick (轻拂)", value: "Flick" },
  { label: "FlickUp (向上)", value: "FlickUp" },
  { label: "FlickDown (向下)", value: "FlickDown" },
  { label: "Idle (待机)", value: "Idle" },
]

const showDebugSubmenu = ref(false)
const showStateSubmenu = ref(false)
const showMotionSubmenu = ref(false)

function handleClick(item: string) {
  switch (item) {
    case "settings":
      emit("settings")
      break
    case "reloadModel":
      emit("reloadModel")
      break
    case "toggleAlwaysOnTop":
      emit("toggleAlwaysOnTop")
      break
    case "minimize":
      emit("minimize")
      break
    case "close":
      emit("closeWindow")
      break
    case "resetWindowSize":
      emit("resetWindowSize")
      break
    case "resetToIdle":
      emit("resetToIdle")
      break
    case "reconnectWs":
      emit("reconnectWs")
      break
  }
  emit("close")
}

function handleStateClick(state: string) {
  emit("previewState", state)
  showStateSubmenu.value = false
  showDebugSubmenu.value = false
  emit("close")
}

function handleMotionClick(motion: string) {
  emit("previewMotion", motion)
  showMotionSubmenu.value = false
  showDebugSubmenu.value = false
  emit("close")
}

function toggleDebugMenu() {
  showDebugSubmenu.value = !showDebugSubmenu.value
  showStateSubmenu.value = false
  showMotionSubmenu.value = false
}

function toggleStateMenu() {
  showStateSubmenu.value = !showStateSubmenu.value
  showMotionSubmenu.value = false
}

function toggleMotionMenu() {
  showMotionSubmenu.value = !showMotionSubmenu.value
  showStateSubmenu.value = false
}

function handleDocumentClick() {
  emit("close")
}

onMounted(() => {
  setTimeout(() => {
    document.addEventListener("click", handleDocumentClick, { once: true })
  }, 0)
})

onUnmounted(() => {
  document.removeEventListener("click", handleDocumentClick)
})
</script>

<template>
  <Teleport to="body">
    <div
      v-if="visible"
      id="context-menu"
      class="context-menu"
      :style="{ left: `${x}px`, top: `${y}px` }"
    >
      <div
        class="menu-item"
        @click="handleClick('settings')"
      >
        设置
      </div>
      <div
        class="menu-item"
        @click="handleClick('reloadModel')"
      >
        重载模型
      </div>
      <div
        class="menu-item"
        @click="handleClick('toggleAlwaysOnTop')"
      >
        置顶
      </div>
      <div
        class="menu-item"
        @click="handleClick('minimize')"
      >
        最小化
      </div>
      <div
        class="menu-item"
        @click="handleClick('close')"
      >
        关闭
      </div>

      <!-- 启动 Gateway 和 Debug (仅开发环境显示) -->
      <template v-if="isDev">
        <div class="menu-divider"></div>
        <div
          class="menu-item"
          @click="handleClick('reconnectWs')"
        >
          重连 WebSocket
        </div>
        <div
          class="menu-item"
          @click="handleClick('runGateway')"
        >
          启动 Gateway
        </div>
        <div class="menu-divider"></div>
        <div class="menu-item has-submenu" @click.stop="toggleDebugMenu">
          Debug ▼
        </div>
        <div v-if="showDebugSubmenu">
          <div class="menu-item submenu-item" @click.stop="handleClick('resetWindowSize')">
            重置窗体大小
          </div>
          <div class="menu-item submenu-item" @click.stop="handleClick('resetToIdle')">
            重置为待机
          </div>
          <div class="menu-item has-submenu submenu-item" @click.stop="toggleStateMenu">
            动画状态 ►
          </div>
          <div v-if="showStateSubmenu">
            <div
              v-for="state in animationStates"
              :key="state.value"
              class="menu-item submenu-item submenu-item-end"
              @click="handleStateClick(state.value)"
            >
              {{ state.label }}
            </div>
          </div>
          <div class="menu-item has-submenu submenu-item" @click.stop="toggleMotionMenu">
            播放动作 ►
          </div>
          <div v-if="showMotionSubmenu">
            <div
              v-for="motion in motionGroups"
              :key="motion.value"
              class="menu-item submenu-item submenu-item-end"
              @click="handleMotionClick(motion.value)"
            >
              {{ motion.label }}
            </div>
          </div>
        </div>
      </template>
    </div>
  </Teleport>
</template>

<style scoped>
.context-menu {
  position: fixed;
  background: rgba(15, 23, 42, 0.95);
  border-radius: 8px;
  padding: 4px 0;
  min-width: 150px;
  z-index: 1000;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
}

.menu-item {
  padding: 8px 16px;
  cursor: pointer;
  color: #e2e8f0;
  font-size: 13px;
  transition: background 0.15s;
}

.menu-item:hover {
  background: rgba(56, 189, 248, 0.3);
}

.menu-divider {
  height: 1px;
  background: rgba(255, 255, 255, 0.1);
  margin: 4px 0;
}

.submenu-item {
  padding-left: 24px;
}

.submenu-item-end {
  padding-left: 32px;
}

.has-submenu {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
</style>
