import { cn } from "@/lib/utils"
import type { ReactNode } from "react"

interface GroupBoxProps {
  title: string
  /** 标题栏右侧额外内容 */
  extra?: ReactNode
  children: ReactNode
  className?: string
}

/**
 * 分组框组件 (类似 Windows GroupBox)
 */
export function GroupBox({ title, extra, children, className }: GroupBoxProps) {
  return (
    <fieldset
      className={cn(
        "border border-border rounded-md",
        "bg-surface p-3 pt-2",
        "flex flex-col min-h-0",
        className
      )}
    >
      {extra ? (
        <legend className="px-2 text-xs text-text-secondary font-medium w-full">
          <div className="flex items-center justify-between w-full pr-2">
            <span>{title}</span>
            {extra}
          </div>
        </legend>
      ) : (
        <legend className="px-2 text-xs text-text-secondary font-medium">{title}</legend>
      )}
      <div className="flex-1 min-h-0">{children}</div>
    </fieldset>
  )
}
