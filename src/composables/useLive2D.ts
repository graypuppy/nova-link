import { ref, nextTick } from "vue"
import * as PIXI from "pixi.js"
import { Live2DModel } from "pixi-live2d-display/cubism4"
import { AnimationStateMachine } from "../utils/animationState"
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
      live2dApp = new PIXI.Application({
        view: canvas,
        width: container.clientWidth,
        height: container.clientHeight,
        backgroundAlpha: 0,
        antialias: true,
        resolution: window.devicePixelRatio || 1,
        autoDensity: true,
      } as any)

      canvas.style.pointerEvents = "none"

      window.addEventListener("resize", () => {
        resizeLive2D(containerId)
      })
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
    if (!live2dApp) return

    isLoading.value = true

    try {
      const modelUrl = new URL(modelPath, window.location.origin).href

      live2dModel = await Live2DModel.from(modelUrl)

      if (live2dModel) {
        const container = document.getElementById(containerId)
        if (container) {
          const containerWidth = container.clientWidth
          const containerHeight = container.clientHeight

          const scale =
            Math.min(
              containerWidth / live2dModel.width,
              containerHeight / live2dModel.height,
            ) * 1.2

          live2dModel.scale.set(scale)
          live2dModel.anchor.set(0.5, 0.5)
          live2dModel.x = containerWidth / 2
          live2dModel.y = containerHeight / 2
        }

        live2dApp.stage.addChild(live2dModel)

        initInteractionHandlers(containerId)

        hasModel.value = true
        console.log("[useLive2D] Model loaded successfully")
      }
    } catch (e) {
      console.warn("[useLive2D] No Live2D model found at", modelPath, e)
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
    if (!live2dApp || !live2dModel) return

    const container = document.getElementById(containerId)
    const canvas = document.getElementById("live2d-canvas") as HTMLCanvasElement
    if (!container || !canvas) return

    const containerWidth = container.clientWidth
    const containerHeight = container.clientHeight

    canvas.width = containerWidth
    canvas.height = containerHeight

    live2dApp.renderer.resize(containerWidth, containerHeight)

    const scale =
      Math.min(
        containerWidth / live2dModel.width,
        containerHeight / live2dModel.height,
      ) * 1.2

    live2dModel.scale.set(scale)
    live2dModel.anchor.set(0.5, 0.5)
    live2dModel.x = containerWidth / 2
    live2dModel.y = containerHeight / 2
  }

  async function reloadModel(modelPath: string): Promise<void> {
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

      if (live2dApp.stage) {
        live2dApp.stage.removeAllListeners()
        ;(live2dApp.stage as any).eventMode = "none"
      }

      const renderer = live2dApp.renderer as any
      if (renderer && renderer.events) {
        renderer.events.cursorStyles = {}
        renderer.events.trackedPointers = {}
      }
    }

    hasModel.value = false
    await loadLive2DModel(modelPath)
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
    destroy,
  }
}
