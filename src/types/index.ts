/**
 * 按键宽度类型
 */
export type KeyWidth =
  | "normal"
  | "wide-1-25"
  | "wide-1-5"
  | "wide-1-75"
  | "wide-2"
  | "wide-2-25"
  | "wide-2-75"
  | "space"
  | "func"
  | "small"
  | "arrow"

/**
 * 按键配置
 */
export interface KeyConfig {
  /** 虚拟键码 */
  vk: number
  /** 显示文本 */
  label: string
  /** 按键宽度 */
  width?: KeyWidth
}

/**
 * 键盘行类型
 */
export type RowType = "main" | "function" | "edit" | "arrow"

/**
 * 键盘行配置
 */
export interface RowConfig {
  /** 按键列表 */
  keys: (KeyConfig | "spacer" | "spacer-small")[]
  /** 行类型 */
  type?: RowType
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
