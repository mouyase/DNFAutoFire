import { cn } from "@/lib/utils"
import type { KeyConfig } from "@/types"
import { KEY_SIZE, KEY_GAP } from "./constants"
import { Tooltip, TooltipTrigger, TooltipContent } from "@/components/ui"

/** 不支持的按键 VK 码（系统修饰键） */
const DISABLED_KEYS = new Set([
  162, // Left Ctrl
  163, // Right Ctrl
  164, // Left Alt
  165, // Right Alt
  91, // Left Win
  92, // Right Win
  93, // App (Menu)
])

interface KeyProps {
  config: KeyConfig
  active?: boolean
  onClick?: (vk: number) => void
}

/**
 * 单个键盘按键组件 - 模拟物理键盘样式
 * 激活状态模拟按键按下效果
 */
export function Key({ config, active = false, onClick }: KeyProps) {
  const { vk, label, width = 1, height = 1, empty } = config

  // 计算实际像素尺寸
  const w = KEY_SIZE * width + KEY_GAP * Math.max(0, width - 1)
  const h = KEY_SIZE * height + KEY_GAP * Math.max(0, height - 1)

  // 检查是否为禁用按键
  const isDisabled = DISABLED_KEYS.has(vk)

  // 空占位
  if (empty) {
    return <div style={{ width: w, height: h, flexShrink: 0 }} />
  }

  const button = (
    <button
      type="button"
      onClick={() => !isDisabled && onClick?.(vk)}
      disabled={isDisabled}
      style={{ width: w, height: h, flexShrink: 0 }}
      className={cn(
        // 基础样式
        "relative flex items-center justify-center",
        "rounded-md text-[11px] font-medium",
        "select-none",
        "whitespace-pre-wrap leading-tight",
        "transition-all duration-75",
        // 禁用状态
        isDisabled
          ? [
              // 禁用样式 - 灰色、不可点击
              "bg-gradient-to-b from-gray-100 to-gray-200",
              "text-gray-400",
              "border border-gray-300",
              "cursor-not-allowed",
              "opacity-60",
            ]
          : // 正常状态
            active
            ? [
                // 激活状态 - 按下效果（内凹、深色）
                "bg-gradient-to-b from-gray-200 to-gray-300",
                "text-gray-600",
                "border border-gray-400",
                "cursor-pointer",
                // 内凹阴影效果
                "shadow-[inset_0_2px_4px_rgba(0,0,0,0.2),inset_0_1px_2px_rgba(0,0,0,0.1)]",
                // 按下时向下偏移
                "translate-y-[1px]",
              ]
            : [
                // 默认状态 - 立体按键效果
                "bg-gradient-to-b from-white to-gray-100",
                "text-gray-700",
                "border border-gray-300",
                "cursor-pointer",
                // 外凸阴影效果
                "shadow-[inset_0_1px_0_rgba(255,255,255,0.8),0_1px_2px_rgba(0,0,0,0.1),0_2px_4px_rgba(0,0,0,0.05)]",
                "hover:from-gray-50 hover:to-gray-150 hover:border-blue-400",
                "active:translate-y-[1px] active:shadow-[inset_0_2px_4px_rgba(0,0,0,0.15)]",
              ]
      )}
    >
      {/* 按键顶面高光效果（未激活时显示） */}
      {!active && !isDisabled && (
        <span className="absolute inset-x-1 top-0.5 h-[1px] rounded-full bg-white/60" />
      )}
      {label}
    </button>
  )

  // 禁用按键使用 Tooltip 显示提示
  if (isDisabled) {
    return (
      <Tooltip>
        <TooltipTrigger asChild>{button}</TooltipTrigger>
        <TooltipContent>该按键不支持连发</TooltipContent>
      </Tooltip>
    )
  }

  return button
}
