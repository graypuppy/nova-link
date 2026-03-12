<script setup lang="ts">
import { ref, computed } from "vue"

const props = defineProps<{
	visible: boolean
	title?: string
	message: string
	confirmText?: string
	cancelText?: string
	type?: "info" | "warning" | "error" | "success"
	showCancel?: boolean
}>()

const emit = defineEmits<{
	close: []
	confirm: []
	cancel: []
}>()

const dialogType = computed(() => props.type || "info")

// 默认文案
const defaultConfirmText = "确定"
const defaultCancelText = "取消"

// 响应式窗口尺寸
const windowWidth = ref(window.innerWidth)
const windowHeight = ref(window.innerHeight)

const isMobile = computed(() => windowWidth.value < 500)

function handleConfirm() {
	emit("confirm")
	emit("close")
}

function handleCancel() {
	emit("cancel")
	emit("close")
}

function handleOverlayClick(e: MouseEvent) {
	if (e.target === e.currentTarget) {
		handleCancel()
	}
}

// 监听窗口大小变化
if (typeof window !== "undefined") {
	window.addEventListener("resize", () => {
		windowWidth.value = window.innerWidth
		windowHeight.value = window.innerHeight
	})
}
</script>

<template>
	<Teleport to="body">
		<div
			v-if="visible"
			class="dialog-overlay"
			@click="handleOverlayClick"
		>
			<div
				class="dialog-content"
				:class="[dialogType, { mobile: isMobile }]"
				role="dialog"
				:aria-labelledby="title ? 'dialog-title' : undefined"
			>
				<!-- 图标 -->
				<div class="dialog-icon" :class="dialogType">
					<template v-if="dialogType === 'success'">✓</template>
					<template v-else-if="dialogType === 'error'">✕</template>
					<template v-else-if="dialogType === 'warning'">⚠</template>
					<template v-else>ℹ</template>
				</div>

				<!-- 标题 -->
				<h3
					v-if="title"
					id="dialog-title"
					class="dialog-title"
				>
					{{ title }}
				</h3>

				<!-- 内容 -->
				<p class="dialog-message">
					{{ message }}
				</p>

				<!-- 按钮 -->
				<div class="dialog-buttons" :class="{ 'single': !showCancel }">
					<button
						v-if="showCancel"
						class="btn btn-cancel"
						@click="handleCancel"
					>
						{{ cancelText || defaultCancelText }}
					</button>
					<button
						class="btn"
						:class="dialogType === 'warning' || dialogType === 'error' ? 'btn-danger' : 'btn-primary'"
						@click="handleConfirm"
					>
						{{ confirmText || defaultConfirmText }}
					</button>
				</div>
			</div>
		</div>
	</Teleport>
</template>

<style scoped>
.dialog-overlay {
	position: fixed;
	inset: 0;
	background: rgba(0, 0, 0, 0.6);
	display: flex;
	align-items: center;
	justify-content: center;
	z-index: 3000;
	backdrop-filter: blur(4px);
	padding: 20px;
	animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
	from {
		opacity: 0;
	}
	to {
		opacity: 1;
	}
}

.dialog-content {
	background: linear-gradient(
		145deg,
		rgba(30, 41, 59, 0.98),
		rgba(15, 23, 42, 0.99)
	);
	border-radius: 16px;
	padding: 24px;
	width: 100%;
	max-width: 360px;
	box-shadow:
		0 25px 50px -12px rgba(0, 0, 0, 0.5),
		0 0 0 1px rgba(255, 255, 255, 0.1);
	text-align: center;
	animation: scaleIn 0.2s ease;
}

@keyframes scaleIn {
	from {
		transform: scale(0.95);
		opacity: 0;
	}
	to {
		transform: scale(1);
		opacity: 1;
	}
}

/* 响应式调整 */
.dialog-content.mobile {
	max-width: 90vw;
	padding: 20px;
	border-radius: 12px;
}

/* 图标 */
.dialog-icon {
	width: 48px;
	height: 48px;
	border-radius: 50%;
	display: flex;
	align-items: center;
	justify-content: center;
	font-size: 24px;
	margin: 0 auto 16px;
}

.dialog-icon.info {
	background: rgba(56, 189, 248, 0.2);
	color: #22d3ee;
}

.dialog-icon.success {
	background: rgba(34, 197, 94, 0.2);
	color: #22c55e;
}

.dialog-icon.warning {
	background: rgba(234, 179, 8, 0.2);
	color: #eab308;
}

.dialog-icon.error {
	background: rgba(239, 68, 68, 0.2);
	color: #ef4444;
}

/* 标题 */
.dialog-title {
	margin: 0 0 12px;
	font-size: 18px;
	font-weight: 600;
	color: #f1f5f9;
}

.mobile .dialog-title {
	font-size: 16px;
}

/* 内容 */
.dialog-message {
	margin: 0 0 24px;
	font-size: 14px;
	color: #94a3b8;
	line-height: 1.6;
	white-space: pre-wrap;
	word-break: break-word;
}

.mobile .dialog-message {
	font-size: 13px;
	margin-bottom: 20px;
}

/* 按钮 */
.dialog-buttons {
	display: flex;
	gap: 12px;
	justify-content: center;
}

.dialog-buttons.single {
	justify-content: flex-end;
}

.btn {
	padding: 10px 24px;
	border-radius: 10px;
	font-size: 14px;
	font-weight: 500;
	cursor: pointer;
	transition: all 0.2s;
	border: none;
}

.mobile .btn {
	padding: 8px 20px;
	font-size: 13px;
}

.btn-cancel {
	background: rgba(255, 255, 255, 0.1);
	color: #94a3b8;
}

.btn-cancel:hover {
	background: rgba(255, 255, 255, 0.15);
}

.btn-primary {
	background: linear-gradient(135deg, #22d3ee, #3b82f6);
	color: white;
}

.btn-primary:hover {
	background: linear-gradient(135deg, #67e8f9, #60a5fa);
	transform: translateY(-1px);
}

.btn-danger {
	background: linear-gradient(135deg, #ef4444, #dc2626);
	color: white;
}

.btn-danger:hover {
	background: linear-gradient(135deg, #f87171, #ef4444);
	transform: translateY(-1px);
}
</style>
