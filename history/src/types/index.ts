/**
 * 按键配置
 */
export interface KeyConfig {
  /** 虚拟键码 */
  vk: number
  /** 显示文本 */
  label: string
  /** 按键宽度倍数 (默认 1) */
  width?: number
  /** 按键高度倍数 (默认 1) */
  height?: number
  /** 是否为空占位 */
  empty?: boolean
  /** 是否为小键盘回车 */
  isNumpadEnter?: boolean
}

/**
 * 配置项
 */
export interface Profile {
  /** 配置 ID */
  id: string
  /** 配置名称 */
  name: string
  /** 启用的按键 VK 码列表 */
  enabledKeys: number[]
}

/**
 * 连发状态
 */
export interface AutofireState {
  /** 是否正在运行 */
  isRunning: boolean
  /** 已启用的按键 VK 码集合 */
  enabledKeys: Set<number>
}

/**
 * 管理员状态
 */
export interface AdminState {
  /** 是否以管理员权限运行 */
  isElevated: boolean
  /** 是否正在检查 */
  isChecking: boolean
}
