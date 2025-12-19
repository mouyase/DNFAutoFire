import { isTauriEnvironment, mockCommands } from "./tauri-mock"

/**
 * Tauri 后端命令封装
 *
 * 自动检测运行环境：
 * - Tauri 环境：调用真实后端
 * - 浏览器环境：使用 Mock 实现
 */

// 动态导入 Tauri API，避免在浏览器环境报错
async function getTauriInvoke() {
  if (!isTauriEnvironment()) {
    return null
  }
  const { invoke } = await import("@tauri-apps/api/core")
  return invoke
}

// 创建通用调用函数
function createCommand<T, A extends unknown[]>(
  name: string,
  mockFn: (...args: A) => Promise<T>,
  argMapper?: (...args: A) => Record<string, unknown>,
): (...args: A) => Promise<T> {
  return async (...args: A): Promise<T> => {
    const invoke = await getTauriInvoke()
    if (invoke) {
      const params = argMapper ? argMapper(...args) : undefined
      return invoke(name, params) as Promise<T>
    }
    return mockFn(...args)
  }
}

/**
 * Tauri 命令接口
 */
export const tauriCommands = {
  /** 获取已启用的按键 */
  getEnabledKeys: createCommand("get_enabled_keys", mockCommands.getEnabledKeys),

  /** 切换按键状态 */
  toggleKey: createCommand("toggle_key", mockCommands.toggleKey, (vk: number) => ({ vk })),

  /** 设置所有按键 */
  setKeys: createCommand("set_keys", mockCommands.setKeys, (keys: number[]) => ({ keys })),

  /** 清空所有按键 */
  clearKeys: createCommand("clear_keys", mockCommands.clearKeys),

  /** 启动连发 */
  startAutofire: createCommand("start_autofire", mockCommands.startAutofire),

  /** 停止连发 */
  stopAutofire: createCommand("stop_autofire", mockCommands.stopAutofire),

  /** 获取运行状态 */
  isRunning: createCommand("is_running", mockCommands.isRunning),

  /** 检查是否以管理员权限运行 */
  isElevated: createCommand("is_elevated", mockCommands.isElevated),

  /** 以管理员权限重启 */
  restartAsAdmin: createCommand("restart_as_admin", mockCommands.restartAsAdmin),

  /** 更新托盘菜单的连发状态 */
  updateTrayStatus: createCommand<void, [boolean]>(
    "update_tray_status",
    async () => {},
    (isRunning: boolean) => ({ isRunning })
  ),

  /** 隐藏主窗口到托盘 */
  hideToTray: createCommand("hide_to_tray", async () => {}),

  /** 显示主窗口 */
  showMainWindow: createCommand("show_main_window", async () => {}),

  /** 切换迷你窗口显示状态 */
  toggleMiniWindow: createCommand("toggle_mini_window", async () => {}),

  /** 更新快捷键配置 */
  updateShortcuts: createCommand<void, [string, string]>(
    "update_shortcuts",
    async () => {},
    (popupShortcut: string, toggleShortcut: string) => ({
      popupShortcut,
      toggleShortcut,
    })
  ),

  /** 播放系统音效 */
  playSound: createCommand<void, [string]>(
    "play_sound",
    async () => {},
    (soundType: string) => ({ soundType })
  ),
}

/**
 * 检测当前是否为浏览器 Mock 模式
 */
export function isMockMode(): boolean {
  return !isTauriEnvironment()
}
