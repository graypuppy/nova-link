import { ref, nextTick } from "vue"
import * as PIXI from "pixi.js"
import { Live2DModel } from "pixi-live2d-display/cubism4"
import { AnimationStateMachine, AnimationState } from "../utils/animationState"
import { MouseInteractionHandler } from "../utils/mouseInteraction"

// Make PIXI available globally for Live2D
;(window as any).PIXI = PIXI

export function useLive2D() {
  const hasModel = ref(false)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  let live2dApp: PIXI.Application | null = null
  let live2dModel: any = null
  let stateMachine: AnimationStateMachine | null = null
  let mouseHandler: MouseInteractionHandler | null = null

  const onModelClickCallbacks: Array<() => void> = []
  const onModelDoubleClickCallbacks: Array<(hitArea: any) => void> = []
  const onModelHoverCallbacks: Array<(hitArea: any) => void> = []

  async function initLive2D(containerId: string = "live2d-container"): Promise<void> {
    // 如果已经初始化过，直接返回
    if (live2dApp) {
      console.log("[useLive2D] Already initialized")
      return
    }

    await nextTick()
    const canvas = document.getElementById("live2d-canvas") as HTMLCanvasElement
    const container = document.getElementById(containerId)

    if (!canvas || !container) {
      console.warn("[useLive2D] Canvas or container not found")
      return
    }

    isLoading.value = true
    error.value = null

    try {
      console.log("[useLive2D] Creating PIXI Application...")
      live2dApp = new PIXI.Application({
        view: canvas,
        width: container.clientWidth,
        height: container.clientHeight,
        backgroundAlpha: 0,
        antialias: true,
        resolution: 1,
        autoDensity: true,
      } as any)

      canvas.style.pointerEvents = "none"
      canvas.style.width = "100%"
      canvas.style.height = "100%"

      window.addEventListener("resize", () => {
        resizeLive2D(containerId)
      })

      console.log("[useLive2D] PIXI Application created")
    } catch (e) {
      console.warn("Live2D initialization failed:", e)
      error.value = String(e)
    } finally {
      isLoading.value = false
    }
  }

  async function loadLive2DModel(
    modelPath: string,
    containerId: string = "live2d-container",
  ): Promise<void> {
    console.log("[useLive2D] loadLive2DModel called, live2dApp exists:", !!live2dApp)
    if (!live2dApp) {
      console.warn("[useLive2D] live2dApp is null, cannot load model")
      return
    }

    isLoading.value = true

    try {
      const modelUrl = new URL(modelPath, window.location.origin).href
      console.log("[useLive2D] Loading model from:", modelUrl)

      live2dModel = await Live2DModel.from(modelUrl)

      if (live2dModel) {
        const container = document.getElementById(containerId)
        if (container) {
          const containerWidth = container.clientWidth
          const containerHeight = container.clientHeight
          console.log(`[useLive2D] Container size: ${containerWidth}x${containerHeight}, Model native size: ${live2dModel.width}x${live2dModel.height}`)

          // 计算缩放 - 使用容器高度的 90%
          const scale = (containerHeight * 0.9) / live2dModel.height

          live2dModel.scale.set(scale)
          // 居中显示
          live2dModel.anchor.set(0.5, 0.5)
          live2dModel.x = containerWidth / 2
          live2dModel.y = containerHeight / 2

          console.log(`[useLive2D] Model scale: ${scale}, position: ${live2dModel.x}, ${live2dModel.y}`)
        }

        // 确保模型不在 stage 中（防止重复添加）
        const stageChildren = live2dApp.stage.children as any[]
        if (stageChildren.includes(live2dModel)) {
          live2dApp.stage.removeChild(live2dModel)
        }
        live2dApp.stage.addChild(live2dModel)

        // 调试：检查 canvas 和 stage
        const canvas = document.getElementById("live2d-canvas") as HTMLCanvasElement
        console.log(`[useLive2D] Canvas size: ${canvas.width}x${canvas.height}, style: ${canvas.style.width}x${canvas.style.height}`)
        console.log(`[useLive2D] Stage children: ${live2dApp.stage.children.length}`)
        console.log(`[useLive2D] Model visible: ${live2dModel.visible}`)
        console.log(`[useLive2D] Model position: x=${live2dModel.x}, y=${live2dModel.y}`)
        console.log(`[useLive2D] Model scale: ${live2dModel.scale.x}, ${live2dModel.scale.y}`)
        console.log(`[useLive2D] Model anchor: x=${live2dModel.anchor.x}, y=${live2dModel.anchor.y}`)
        console.log(`[useLive2D] Model actual size: ${live2dModel.width * live2dModel.scale.x}x${live2dModel.height * live2dModel.scale.y}`)

        initInteractionHandlers(containerId)

        hasModel.value = true
        console.log("[useLive2D] Model loaded successfully")
      } else {
        console.warn("[useLive2D] Model is null after loading")
      }
    } catch (e) {
      console.warn("[useLive2D] Failed to load model:", e)
      hasModel.value = false
    } finally {
      isLoading.value = false
    }
  }

  function initInteractionHandlers(containerId: string = "live2d-container"): void {
    if (!live2dModel) return

    const container = document.getElementById(containerId)
    if (!container) return

    stateMachine = new AnimationStateMachine(live2dModel)
    stateMachine.onStateChange((event) => {
      console.log(
        "[useLive2D] Animation state changed:",
        event.oldState,
        "->",
        event.newState,
      )
    })

    mouseHandler = new MouseInteractionHandler(live2dModel, container)

    mouseHandler.onClick((_hitArea) => {
      if (stateMachine) {
        stateMachine.handleUserInteraction()
      }
      onModelClickCallbacks.forEach((cb) => cb())
    })

    mouseHandler.onDoubleClick(async (hitArea) => {
      console.log("[useLive2D] Double click on:", hitArea?.name)
      if (stateMachine) {
        await stateMachine.playMotion("Tap")
      }
      onModelDoubleClickCallbacks.forEach((cb) => cb(hitArea))
    })

    mouseHandler.onHover((hitArea) => {
      console.log("[useLive2D] Hover on:", hitArea?.name)
      onModelHoverCallbacks.forEach((cb) => cb(hitArea))
    })

    mouseHandler.init()
    mouseHandler.enableTracking(true)

    console.log("[useLive2D] Interaction handlers initialized")
  }

  function resizeLive2D(containerId: string = "live2d-container"): void {
    if (!live2dApp || !live2dModel) {
      console.log("[useLive2D] resizeLive2D: no app or model")
      return
    }

    const container = document.getElementById(containerId)
    const canvas = document.getElementById("live2d-canvas") as HTMLCanvasElement
    if (!container || !canvas) return

    const containerWidth = container.clientWidth
    const containerHeight = container.clientHeight
    console.log(`[useLive2D] resizeLive2D: container ${containerWidth}x${containerHeight}`)

    canvas.width = containerWidth
    canvas.height = containerHeight

    live2dApp.renderer.resize(containerWidth, containerHeight)

    const scale = (containerHeight * 0.9) / live2dModel.height

    live2dModel.scale.set(scale)
    live2dModel.anchor.set(0.5, 0.5)
    live2dModel.x = containerWidth / 2
    live2dModel.y = containerHeight / 2
    console.log(`[useLive2D] resizeLive2D: scale=${scale}`)
  }

  async function reloadModel(modelPath: string): Promise<void> {
    console.log("[useLive2D] Reloading model:", modelPath)

    // 销毁旧的交互处理器
    if (mouseHandler) {
      mouseHandler.destroy()
      mouseHandler = null
    }
    if (stateMachine) {
      stateMachine.destroy()
      stateMachine = null
    }

    // 销毁旧模型
    if (live2dApp && live2dModel) {
      console.log("[useLive2D] Removing old model from stage")

      // 从 stage 移除
      const stageChildren = live2dApp.stage.children as any[]
      if (stageChildren.includes(live2dModel)) {
        live2dApp.stage.removeChild(live2dModel)
      }

      live2dModel.removeAllListeners()

      // 销毁模型
      try {
        live2dModel.destroy({
          children: true,
          texture: true,
          baseTexture: true,
        })
      } catch (e) {
        console.warn("[useLive2D] Error destroying model:", e)
      }
      live2dModel = null
      console.log("[useLive2D] Old model destroyed")

      // 重置 stage 事件
      if (live2dApp.stage) {
        live2dApp.stage.removeAllListeners()
        ;(live2dApp.stage as any).eventMode = "none"
      }

      // 重置渲染器事件
      const renderer = live2dApp.renderer as any
      if (renderer && renderer.events) {
        renderer.events.cursorStyles = {}
        renderer.events.trackedPointers = {}
      }
    }

    hasModel.value = false
    console.log("[useLive2D] Loading new model...")
    await loadLive2DModel(modelPath)
    console.log("[useLive2D] Model reloaded")
  }

  function handleUserInteraction(): void {
    if (stateMachine) {
      stateMachine.handleUserInteraction()
    }
  }

  function handleUserMessage(): void {
    if (stateMachine) {
      stateMachine.handleUserMessage()
    }
  }

  function handleBotThinking(): void {
    if (stateMachine) {
      stateMachine.handleBotThinking()
    }
  }

  function handleBotMessage(text: string): void {
    if (stateMachine) {
      stateMachine.handleBotMessage(text)
    }
  }

  function handleMessageComplete(): void {
    if (stateMachine) {
      stateMachine.handleMessageComplete()
    }
  }

  function onModelClick(callback: () => void): void {
    onModelClickCallbacks.push(callback)
  }

  function onModelDoubleClick(callback: (hitArea: any) => void): void {
    onModelDoubleClickCallbacks.push(callback)
  }

  function onModelHover(callback: (hitArea: any) => void): void {
    onModelHoverCallbacks.push(callback)
  }

  // Debug functions
  function getCurrentState(): string {
    return stateMachine?.getState() || "unknown"
  }

  function previewState(stateName: string): void {
    if (!stateMachine) return
    const state = stateName as AnimationState
    if (Object.values(AnimationState).includes(state)) {
      stateMachine.transition(state)
    }
  }

  function previewMotion(motionName: string): void {
    if (stateMachine) {
      stateMachine.playMotion(motionName)
    }
  }

  function resetToIdle(): void {
    if (stateMachine) {
      stateMachine.transition(AnimationState.IDLE)
    }
  }

  async function destroy(): Promise<void> {
    if (mouseHandler) {
      mouseHandler.destroy()
      mouseHandler = null
    }
    if (stateMachine) {
      stateMachine.destroy()
      stateMachine = null
    }

    if (live2dApp && live2dModel) {
      live2dModel.removeAllListeners()
      live2dApp.stage.removeChild(live2dModel)
      live2dModel.destroy({
        children: true,
        texture: true,
        baseTexture: true,
      })
      live2dModel = null
    }

    if (live2dApp) {
      await live2dApp.destroy(true)
      live2dApp = null
    }

    hasModel.value = false
  }

  return {
    hasModel,
    isLoading,
    error,
    stateMachine,
    initLive2D,
    loadLive2DModel,
    reloadModel,
    resizeLive2D,
    handleUserInteraction,
    handleUserMessage,
    handleBotThinking,
    handleBotMessage,
    handleMessageComplete,
    onModelClick,
    onModelDoubleClick,
    onModelHover,
    // Debug functions
    getCurrentState,
    previewState,
    previewMotion,
    resetToIdle,
    destroy,
  }
}
