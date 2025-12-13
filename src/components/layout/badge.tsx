import { cn } from "@/lib/utils"

interface BadgeProps {
  /** 徽章文本 */
  children: React.ReactNode
  /** 徽章类型 */
  variant?: "admin" | "warning"
  /** 自定义类名 */
  className?: string
}

/**
 * 徽章组件
 */
export function Badge({ children, variant = "admin", className }: BadgeProps) {
  return (
    <span
      className={cn(
        "text-[11px] px-2 py-0.5 rounded font-medium",
        variant === "admin" && "bg-accent text-black",
        variant === "warning" && "bg-warning text-black",
        className
      )}
    >
      {children}
    </span>
  )
}
