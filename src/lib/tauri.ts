import { invoke } from "@tauri-apps/api/core"

/**
 * Tauri 后端命令封装
 */
export const tauriCommands = {
  /** 获取已启用的按键 */
  getEnabledKeys: (): Promise<number[]> => invoke("get_enabled_keys"),

  /** 切换按键状态 */
  toggleKey: (vk: number): Promise<boolean> => invoke("toggle_key", { vk }),

  /** 设置所有按键 */
  setKeys: (keys: number[]): Promise<void> => invoke("set_keys", { keys }),

  /** 清空所有按键 */
  clearKeys: (): Promise<void> => invoke("clear_keys"),

  /** 启动连发 */
  startAutofire: (): Promise<boolean> => invoke("start_autofire"),

  /** 停止连发 */
  stopAutofire: (): Promise<boolean> => invoke("stop_autofire"),

  /** 获取运行状态 */
  isRunning: (): Promise<boolean> => invoke("is_running"),

  /** 检查是否以管理员权限运行 */
  isElevated: (): Promise<boolean> => invoke("is_elevated"),

  /** 以管理员权限重启 */
  restartAsAdmin: (): Promise<boolean> => invoke("restart_as_admin"),
}
