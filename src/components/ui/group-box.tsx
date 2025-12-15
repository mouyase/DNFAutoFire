import { cn } from "@/lib/utils"
import type { ReactNode } from "react"

interface GroupBoxProps {
  /** 左上角标题 */
  title?: ReactNode
  /** 右上角额外内容 */
  extra?: ReactNode
  children: ReactNode
  className?: string
  /** 内容区域的样式 */
  contentClassName?: string
}

/**
 * 分组框组件 (类似 Windows GroupBox)
 *
 * 标题和额外内容浮动在边框之上，边框保持完整连续
 */
export function GroupBox({ title, extra, children, className, contentClassName }: GroupBoxProps) {
  return (
    <div
      className={cn(
        "relative border border-border rounded-md",
        "bg-surface flex flex-col min-h-0",
        className
      )}
    >
      {/* 左上角标题 - 浮动在边框之上，带边框更整体 */}
      {title && (
        <div className="absolute -top-[9px] left-2 px-1.5 bg-surface rounded border border-border">
          {typeof title === "string" ? (
            <span className="text-xs text-text-secondary font-medium">{title}</span>
          ) : (
            title
          )}
        </div>
      )}

      {/* 右上角额外内容 - 浮动在边框之上，内容自带样式，不需要额外背景 */}
      {extra && (
        <div className="absolute -top-[9px] right-2">
          {extra}
        </div>
      )}

      {/* 内容区域 */}
      <div className={cn("flex-1 min-h-0 p-3 pt-3.5", contentClassName)}>{children}</div>
    </div>
  )
}
