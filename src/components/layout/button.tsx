import { cn } from "@/lib/utils"

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  /** 按钮类型 */
  variant?: "start" | "stop" | "secondary" | "admin"
}

/**
 * 按钮组件
 */
export function Button({ variant = "secondary", className, children, ...props }: ButtonProps) {
  return (
    <button
      type="button"
      className={cn(
        "px-5 py-2 rounded-md text-sm font-medium cursor-pointer transition-all duration-200",
        // 启动按钮
        variant === "start" && [
          "bg-gradient-to-br from-[#00cc66] to-accent text-black",
          "hover:-translate-y-0.5 hover:shadow-lg hover:shadow-accent/30",
        ],
        // 停止按钮
        variant === "stop" && [
          "bg-gradient-to-br from-[#cc3366] to-danger text-white",
          "hover:-translate-y-0.5 hover:shadow-lg hover:shadow-danger/30",
        ],
        // 次要按钮
        variant === "secondary" && ["bg-key text-text", "hover:bg-key-hover"],
        // 管理员按钮
        variant === "admin" && ["bg-warning text-black", "hover:-translate-y-0.5"],
        className
      )}
      {...props}
    >
      {children}
    </button>
  )
}
