<script setup lang="ts">
import { onMounted, onUnmounted } from "vue"

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
}>()

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
  }
  emit("close")
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
</style>
