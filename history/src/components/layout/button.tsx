import { cn } from "@/lib/utils"

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "primary" | "secondary" | "danger" | "success" | "admin" | "start" | "stop"
  size?: "sm" | "md" | "lg"
}

/**
 * 按钮组件
 */
export function Button({
  variant = "secondary",
  size = "md",
  className,
  children,
  ...props
}: ButtonProps) {
  return (
    <button
      type="button"
      className={cn(
        "inline-flex items-center justify-center font-medium rounded",
        "border transition-colors duration-150",
        "focus:outline-none focus:ring-2 focus:ring-offset-1",
        // 尺寸
        size === "sm" && "h-7 px-3 text-xs",
        size === "md" && "h-8 px-4 text-xs",
        size === "lg" && "h-10 px-6 text-sm",
        // 样式
        variant === "primary" && [
          "bg-blue-500 text-white border-blue-600",
          "hover:bg-blue-600 focus:ring-blue-300",
        ],
        variant === "secondary" && [
          "bg-white text-gray-700 border-gray-300",
          "hover:bg-gray-50 focus:ring-gray-200",
        ],
        variant === "danger" && [
          "bg-red-500 text-white border-red-600",
          "hover:bg-red-600 focus:ring-red-300",
        ],
        variant === "success" && [
          "bg-green-500 text-white border-green-600",
          "hover:bg-green-600 focus:ring-green-300",
        ],
        variant === "admin" && [
          "bg-yellow-500 text-white border-yellow-600",
          "hover:bg-yellow-600 focus:ring-yellow-300",
        ],
        variant === "start" && [
          "bg-green-500 text-white border-green-600",
          "hover:bg-green-600 focus:ring-green-300",
        ],
        variant === "stop" && [
          "bg-red-500 text-white border-red-600",
          "hover:bg-red-600 focus:ring-red-300",
        ],
        className
      )}
      {...props}
    >
      {children}
    </button>
  )
}
