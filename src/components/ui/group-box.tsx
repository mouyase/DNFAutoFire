import { cn } from "@/lib/utils"
import type { ReactNode } from "react"

interface GroupBoxProps {
  title: string
  children: ReactNode
  className?: string
}

/**
 * 分组框组件 (类似 Windows GroupBox)
 */
export function GroupBox({ title, children, className }: GroupBoxProps) {
  return (
    <fieldset
      className={cn(
        "border border-border rounded-md",
        "bg-surface p-3 pt-2",
        className
      )}
    >
      <legend className="px-2 text-xs text-text-secondary font-medium">
        {title}
      </legend>
      {children}
    </fieldset>
  )
}
