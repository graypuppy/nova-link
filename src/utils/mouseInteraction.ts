export interface HitArea {
  name: string
  id: string
}

export type ClickCallback = (hitArea: HitArea | null, x: number, y: number) => void
export type DoubleClickCallback = (hitArea: HitArea | null, x: number, y: number) => void

export class MouseInteractionHandler {
  private model: any = null
  private container: HTMLElement | null = null
  private hitAreas: HitArea[] = []
  private trackingEnabled: boolean = false
  private mouseX: number = 0
  private mouseY: number = 0
  private lastClickTime: number = 0
  private readonly DOUBLE_CLICK_DELAY = 300
  private readonly TRACKING_SMOOTHING = 0.1

  private clickCallback: ClickCallback | null = null
  private doubleClickCallback: DoubleClickCallback | null = null

  // 绑定的事件处理器引用，用于移除
  private boundPointerMove: ((e: PointerEvent) => void) | null = null
  private boundPointerDown: ((e: PointerEvent) => void) | null = null

  constructor(model: any, container: HTMLElement) {
    this.model = model
    this.container = container
    this.initHitAreas()
  }

  private initHitAreas(): void {
    try {
      const model3 = this.model?.internalModel?.model?.model3
      if (model3?.HitAreas) {
        this.hitAreas = model3.HitAreas.map((area: any) => ({
          name: area.Name || area.name || 'Unknown',
          id: area.Id || area.id || 'Unknown',
        }))
        console.log('[MouseInteraction] HitAreas loaded:', this.hitAreas)
      }
    } catch (e) {
      console.warn('[MouseInteraction] Failed to load HitAreas:', e)
    }

    if (this.hitAreas.length === 0) {
      this.hitAreas = [
        { name: 'Body', id: 'HitArea' },
      ]
    }
  }

  init(): void {
    if (!this.container) return

    // 使用 window 监听全局事件，实现全屏鼠标跟踪
    this.boundPointerMove = this.handlePointerMove.bind(this)
    this.boundPointerDown = this.handlePointerDown.bind(this)

    window.addEventListener('pointermove', this.boundPointerMove, { passive: true })
    window.addEventListener('pointerdown', this.boundPointerDown, { passive: true })

    // 容器不阻挡鼠标事件
    this.container.style.pointerEvents = 'none'
  }

  onClick(callback: ClickCallback): void {
    this.clickCallback = callback
  }

  onDoubleClick(callback: DoubleClickCallback): void {
    this.doubleClickCallback = callback
  }

  // onHover 暂时禁用
  // onHover(callback: HoverCallback): void { }

  getHitArea(x: number, y: number): HitArea | null {
    if (!this.container) return { name: 'Default', id: 'Default' }

    const rect = this.container.getBoundingClientRect()

    if (rect.width === 0 || rect.height === 0) {
      return { name: 'Default', id: 'Default' }
    }

    const relX = (x - rect.left) / rect.width
    const relY = (y - rect.top) / rect.height

    // 全屏模式下，不再限制在容器范围内
    // 但点击检测仍基于容器位置

    const centerX = 0.5
    const centerY = 0.5
    const distFromCenter = Math.sqrt(
      Math.pow(relX - centerX, 2) + Math.pow(relY - centerY, 2)
    )

    // 头部区域（中心区域）
    if (distFromCenter < 0.25) {
      return { name: 'Head', id: 'Head' }
    }

    // 身体区域
    if (relY > 0.4 && relY < 0.9 && Math.abs(relX - 0.5) < 0.3) {
      return this.hitAreas.find(h => h.name === 'Body') || { name: 'Body', id: 'Body' }
    }

    // 底部 30% 区域 - 用于弹出聊天面板
    if (relY > 0.7) {
      return { name: 'Bottom', id: 'Bottom' }
    }

    return { name: 'Default', id: 'Default' }
  }

  enableTracking(enabled: boolean): void {
    this.trackingEnabled = enabled
    if (!enabled) {
      this.resetTracking()
    }
  }

  private handlePointerMove(e: PointerEvent): void {
    if (!this.container) return

    const rect = this.container.getBoundingClientRect()
    if (rect.width === 0 || rect.height === 0) return

    // 计算鼠标相对于容器的位置
    let relX = (e.clientX - rect.left) / rect.width
    let relY = (e.clientY - rect.top) / rect.height

    // 限制在 0-1 范围内
    relX = Math.max(0, Math.min(1, relX))
    relY = Math.max(0, Math.min(1, relY))

    // 转换为 -1 到 1 的范围
    const normalizedX = (relX - 0.5) * 2
    const normalizedY = (relY - 0.5) * 2

    this.mouseX += (normalizedX - this.mouseX) * this.TRACKING_SMOOTHING
    this.mouseY += (normalizedY - this.mouseY) * this.TRACKING_SMOOTHING

    if (this.trackingEnabled) {
      this.updateTracking()
    }
  }

  private handlePointerDown(e: PointerEvent): void {
    if (e.button !== 0) return

    const now = Date.now()
    const hitArea = this.getHitArea(e.clientX, e.clientY)

    if (now - this.lastClickTime < this.DOUBLE_CLICK_DELAY) {
      if (this.doubleClickCallback && hitArea) {
        this.doubleClickCallback(hitArea, e.clientX, e.clientY)
      }
      this.lastClickTime = 0
    } else {
      if (this.clickCallback && hitArea) {
        this.clickCallback(hitArea, e.clientX, e.clientY)
      }
      this.lastClickTime = now
    }
  }

  private updateTracking(): void {
    if (!this.model) return

    try {
      const internalModel = this.model.internalModel
      if (!internalModel?.model) return

      const model = internalModel.model
      const paramAngleX = 'ParamAngleX'
      const paramAngleY = 'ParamAngleY'
      const paramEyeX = 'ParamEyeX'
      const paramEyeY = 'ParamEyeY'

      const maxAngle = 15
      const targetAngleX = this.mouseX * maxAngle
      const targetAngleY = this.mouseY * maxAngle

      if (model.getParameterById) {
        const angleXParam = model.getParameterById(paramAngleX)
        const angleYParam = model.getParameterById(paramAngleY)
        const eyeXParam = model.getParameterById(paramEyeX)
        const eyeYParam = model.getParameterById(paramEyeY)

        if (angleXParam) angleXParam.value = targetAngleX
        if (angleYParam) angleYParam.value = targetAngleY
        if (eyeXParam) eyeXParam.value = this.mouseX * 0.5
        if (eyeYParam) eyeYParam.value = this.mouseY * 0.5
      }
    } catch (e) {
      console.warn('[MouseInteraction] Failed to update tracking:', e)
    }
  }

  private resetTracking(): void {
    if (!this.model) return

    try {
      const internalModel = this.model.internalModel
      if (!internalModel?.model) return

      const model = internalModel.model

      if (model.getParameterById) {
        const angleXParam = model.getParameterById('ParamAngleX')
        const angleYParam = model.getParameterById('ParamAngleY')
        const eyeXParam = model.getParameterById('ParamEyeX')
        const eyeYParam = model.getParameterById('ParamEyeY')

        if (angleXParam) angleXParam.value = 0
        if (angleYParam) angleYParam.value = 0
        if (eyeXParam) eyeXParam.value = 0
        if (eyeYParam) eyeYParam.value = 0
      }
    } catch (e) {
      console.warn('[MouseInteraction] Failed to reset tracking:', e)
    }
  }

  // 保留接口但返回 null
  getCurrentHoverArea(): HitArea | null {
    return null
  }

  destroy(): void {
    // 移除 window 上的全局事件监听
    if (this.boundPointerMove) {
      window.removeEventListener('pointermove', this.boundPointerMove)
    }
    if (this.boundPointerDown) {
      window.removeEventListener('pointerdown', this.boundPointerDown)
    }

    if (this.container) {
      this.container.style.pointerEvents = 'auto'
    }

    this.model = null
    this.container = null
    this.clickCallback = null
    this.doubleClickCallback = null
  }
}
