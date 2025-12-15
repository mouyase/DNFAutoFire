/**
 * 浏览器环境 Mock API
 *
 * 在纯浏览器环境下模拟 Tauri 后端行为，用于 UI 开发调试
 */

// Mock 状态存储
const mockState = {
  enabledKeys: new Set<number>(),
  isRunning: false,
  isElevated: true,
}

// 模拟延迟，使行为更接近真实环境
const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))

/**
 * 浏览器环境 Mock 命令
 */
export const mockCommands = {
  getEnabledKeys: async (): Promise<number[]> => {
    await delay(10)
    return Array.from(mockState.enabledKeys)
  },

  toggleKey: async (vk: number): Promise<boolean> => {
    await delay(10)
    if (mockState.enabledKeys.has(vk)) {
      mockState.enabledKeys.delete(vk)
      console.log(`[Mock] 按键 ${vk.toString(16).toUpperCase()} 已禁用`)
      return false
    } else {
      mockState.enabledKeys.add(vk)
      console.log(`[Mock] 按键 ${vk.toString(16).toUpperCase()} 已启用`)
      return true
    }
  },

  setKeys: async (keys: number[]): Promise<void> => {
    await delay(10)
    mockState.enabledKeys.clear()
    keys.forEach((k) => mockState.enabledKeys.add(k))
    console.log(`[Mock] 设置按键:`, keys.map((k) => k.toString(16).toUpperCase()))
  },

  clearKeys: async (): Promise<void> => {
    await delay(10)
    mockState.enabledKeys.clear()
    console.log("[Mock] 已清空所有按键")
  },

  startAutofire: async (): Promise<boolean> => {
    await delay(50)
    if (!mockState.isRunning) {
      mockState.isRunning = true
      console.log("[Mock] 连发已启动")
      return true
    }
    return false
  },

  stopAutofire: async (): Promise<boolean> => {
    await delay(50)
    if (mockState.isRunning) {
      mockState.isRunning = false
      console.log("[Mock] 连发已停止")
      return true
    }
    return false
  },

  isRunning: async (): Promise<boolean> => {
    await delay(5)
    return mockState.isRunning
  },

  isElevated: async (): Promise<boolean> => {
    await delay(5)
    return mockState.isElevated
  },

  restartAsAdmin: async (): Promise<boolean> => {
    console.log("[Mock] 请求管理员权限重启（浏览器环境不支持）")
    return false
  },
}

/**
 * 检测是否在 Tauri 环境中运行
 */
export function isTauriEnvironment(): boolean {
  return typeof window !== "undefined" && "__TAURI__" in window
}

/**
 * 获取 Mock 状态（用于调试）
 */
export function getMockState() {
  return {
    enabledKeys: Array.from(mockState.enabledKeys),
    isRunning: mockState.isRunning,
    isElevated: mockState.isElevated,
  }
}

/**
 * 重置 Mock 状态（用于调试）
 */
export function resetMockState() {
  mockState.enabledKeys.clear()
  mockState.isRunning = false
  console.log("[Mock] 状态已重置")
}

// 暴露到全局，方便调试
if (typeof window !== "undefined") {
  ;(window as unknown as Record<string, unknown>).__MOCK_STATE__ = {
    get: getMockState,
    reset: resetMockState,
    state: mockState,
  }
}
