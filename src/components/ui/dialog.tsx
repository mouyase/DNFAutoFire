import { useEffect, useRef, type ReactNode } from "react"
import { cn } from "@/lib/utils"

interface DialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  children: ReactNode
}

/**
 * 对话框根组件
 */
export function Dialog({ open, onOpenChange, children }: DialogProps) {
  const overlayRef = useRef<HTMLDivElement>(null)

  // ESC 键关闭
  useEffect(() => {
    if (!open) return

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        onOpenChange(false)
      }
    }

    document.addEventListener("keydown", handleKeyDown)
    return () => document.removeEventListener("keydown", handleKeyDown)
  }, [open, onOpenChange])

  // 点击遮罩关闭
  const handleOverlayClick = (e: React.MouseEvent) => {
    if (e.target === overlayRef.current) {
      onOpenChange(false)
    }
  }

  if (!open) return null

  return (
    <div
      ref={overlayRef}
      onClick={handleOverlayClick}
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 animate-in fade-in-0"
    >
      {children}
    </div>
  )
}

interface DialogContentProps {
  children: ReactNode
  className?: string
}

/**
 * 对话框内容
 */
export function DialogContent({ children, className }: DialogContentProps) {
  return (
    <div
      className={cn(
        "bg-white rounded-lg shadow-lg border border-gray-200",
        "p-5 w-80 max-w-[90vw]",
        "animate-in zoom-in-95 fade-in-0",
        className
      )}
    >
      {children}
    </div>
  )
}

interface DialogHeaderProps {
  children: ReactNode
  className?: string
}

/**
 * 对话框标题区域
 */
export function DialogHeader({ children, className }: DialogHeaderProps) {
  return <div className={cn("mb-4", className)}>{children}</div>
}

interface DialogTitleProps {
  children: ReactNode
  className?: string
}

/**
 * 对话框标题
 */
export function DialogTitle({ children, className }: DialogTitleProps) {
  return <h2 className={cn("text-base font-semibold text-gray-900", className)}>{children}</h2>
}

interface DialogDescriptionProps {
  children: ReactNode
  className?: string
}

/**
 * 对话框描述
 */
export function DialogDescription({ children, className }: DialogDescriptionProps) {
  return <p className={cn("text-sm text-gray-500 mt-1", className)}>{children}</p>
}

interface DialogFooterProps {
  children: ReactNode
  className?: string
}

/**
 * 对话框底部按钮区域
 */
export function DialogFooter({ children, className }: DialogFooterProps) {
  return <div className={cn("flex justify-end gap-2 mt-5", className)}>{children}</div>
}

interface ButtonProps {
  children: ReactNode
  onClick?: () => void
  variant?: "default" | "primary" | "danger" | "ghost"
  disabled?: boolean
  className?: string
  type?: "button" | "submit"
}

/**
 * 通用按钮组件
 */
export function Button({
  children,
  onClick,
  variant = "default",
  disabled,
  className,
  type = "button",
}: ButtonProps) {
  const baseStyles =
    "h-8 px-4 text-sm font-medium rounded-md transition-colors disabled:opacity-50 disabled:cursor-not-allowed"

  const variantStyles = {
    default: "bg-gray-100 text-gray-700 hover:bg-gray-200 border border-gray-200",
    primary: "bg-blue-500 text-white hover:bg-blue-600",
    danger: "bg-red-500 text-white hover:bg-red-600",
    ghost: "text-gray-600 hover:bg-gray-100",
  }

  return (
    <button
      type={type}
      onClick={onClick}
      disabled={disabled}
      className={cn(baseStyles, variantStyles[variant], className)}
    >
      {children}
    </button>
  )
}
