/**
 * 日志工具 - 将前端日志输出到 Rust 控制台
 */
import { info, error, warn, debug, trace } from "@tauri-apps/plugin-log"

/**
 * 信息日志
 */
export function logInfo(message: string, ...args: unknown[]) {
  const formatted = args.length > 0 ? `${message} ${JSON.stringify(args)}` : message
  info(`[Nova Link] ${formatted}`)
}

/**
 * 错误日志
 */
export function logError(message: string, err?: unknown) {
  const errorMsg = err instanceof Error ? err.message : String(err || "")
  error(`[Nova Link] ${message}${errorMsg ? `: ${errorMsg}` : ""}`)
}

/**
 * 警告日志
 */
export function logWarn(message: string, ...args: unknown[]) {
  const formatted = args.length > 0 ? `${message} ${JSON.stringify(args)}` : message
  warn(`[Nova Link] ${formatted}`)
}

/**
 * 调试日志
 */
export function logDebug(message: string, ...args: unknown[]) {
  const formatted = args.length > 0 ? `${message} ${JSON.stringify(args)}` : message
  debug(`[Nova Link] ${formatted}`)
}

/**
 * 跟踪日志
 */
export function logTrace(message: string, ...args: unknown[]) {
  const formatted = args.length > 0 ? `${message} ${JSON.stringify(args)}` : message
  trace(`[Nova Link] ${formatted}`)
}

/**
 * 通用日志方法 - 支持日志级别
 */
export function log(level: "info" | "warn" | "error" | "debug" | "trace", message: string, ...args: unknown[]) {
  const formatted = args.length > 0 ? `${message} ${JSON.stringify(args)}` : message
  switch (level) {
    case "info":
      info(`[Nova Link] ${formatted}`)
      break
    case "warn":
      warn(`[Nova Link] ${formatted}`)
      break
    case "error":
      error(`[Nova Link] ${formatted}`)
      break
    case "debug":
      debug(`[Nova Link] ${formatted}`)
      break
    case "trace":
      trace(`[Nova Link] ${formatted}`)
      break
  }
}

export { info, error, warn, debug, trace }
