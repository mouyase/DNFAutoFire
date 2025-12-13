import { cn } from "@/lib/utils"
import type { KeyWidth } from "@/types"

interface KeyProps {
  /** 虚拟键码 */
  vk: number
  /** 显示文本 */
  label: string
  /** 是否激活 */
  active?: boolean
  /** 按键宽度 */
  width?: KeyWidth
  /** 点击回调 */
  onClick?: (vk: number) => void
}

/** 宽度类名映射 */
const widthClasses: Record<KeyWidth, string> = {
  normal: "min-w-10",
  "wide-1-25": "min-w-[50px]",
  "wide-1-5": "min-w-[60px]",
  "wide-1-75": "min-w-[70px]",
  "wide-2": "min-w-20",
  "wide-2-25": "min-w-[90px]",
  "wide-2-75": "min-w-[110px]",
  space: "min-w-60",
  func: "min-w-[42px] h-8 text-[11px]",
  small: "min-w-[42px] h-8 text-[10px]",
  arrow: "min-w-[42px] h-8 text-sm",
}

/**
 * 单个键盘按键组件
 */
export function Key({ vk, label, active = false, width = "normal", onClick }: KeyProps) {
  return (
    <button
      type="button"
      onClick={() => onClick?.(vk)}
      className={cn(
        // 基础样式
        "h-10 px-2",
        "bg-key border border-white/10 rounded-md",
        "text-text text-xs font-medium",
        "flex items-center justify-center",
        "cursor-pointer transition-all duration-150",
        // 悬停效果
        "hover:bg-key-hover hover:-translate-y-0.5 hover:shadow-lg hover:shadow-black/30",
        // 激活状态
        active && [
          "bg-key-active border-key-active text-white",
          "shadow-[0_0_12px_rgba(233,69,96,0.5)]",
          "hover:bg-[#ff5070]",
        ],
        // 宽度
        widthClasses[width]
      )}
    >
      {label}
    </button>
  )
}
