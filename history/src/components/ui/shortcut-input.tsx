import { useState, useCallback, useRef, useEffect } from "react"
import { cn } from "@/lib/utils"

interface ShortcutInputProps {
  value: string
  onChange: (value: string) => void
  className?: string
  disabled?: boolean
}

/**
 * 快捷键录入组件
 *
 * 点击后进入录入模式，按下按键组合后自动保存
 */
export function ShortcutInput({ value, onChange, className, disabled }: ShortcutInputProps) {
  const [isRecording, setIsRecording] = useState(false)
  const [currentKeys, setCurrentKeys] = useState<string[]>([])
  const inputRef = useRef<HTMLButtonElement>(null)

  // 键盘事件处理
  useEffect(() => {
    if (!isRecording) return

    const handleKeyDown = (e: KeyboardEvent) => {
      e.preventDefault()
      e.stopPropagation()

      const key = normalizeKey(e)
      if (!key) return

      // 收集修饰键和主键
      const modifiers: string[] = []
      if (e.ctrlKey) modifiers.push("Ctrl")
      if (e.altKey) modifiers.push("Alt")
      if (e.shiftKey) modifiers.push("Shift")
      if (e.metaKey) modifiers.push("Meta")

      // 如果按下的是修饰键本身，只更新显示
      if (["Control", "Alt", "Shift", "Meta"].includes(e.key)) {
        setCurrentKeys(modifiers)
        return
      }

      // 组合完整的快捷键
      const shortcut = [...modifiers, key].join("+")
      onChange(shortcut)
      setIsRecording(false)
      setCurrentKeys([])
    }

    const handleKeyUp = (e: KeyboardEvent) => {
      e.preventDefault()
      e.stopPropagation()

      // 如果松开了所有键且有内容，结束录入
      if (!e.ctrlKey && !e.altKey && !e.shiftKey && !e.metaKey) {
        if (currentKeys.length > 0) {
          // 只有修饰键的情况，不保存
          setCurrentKeys([])
        }
      }
    }

    const handleBlur = () => {
      setIsRecording(false)
      setCurrentKeys([])
    }

    document.addEventListener("keydown", handleKeyDown, true)
    document.addEventListener("keyup", handleKeyUp, true)
    inputRef.current?.addEventListener("blur", handleBlur)

    return () => {
      document.removeEventListener("keydown", handleKeyDown, true)
      document.removeEventListener("keyup", handleKeyUp, true)
      inputRef.current?.removeEventListener("blur", handleBlur)
    }
  }, [isRecording, currentKeys, onChange])

  // 开始录入
  const startRecording = useCallback(() => {
    if (disabled) return
    setIsRecording(true)
    setCurrentKeys([])
  }, [disabled])

  // 取消录入
  const cancelRecording = useCallback(() => {
    setIsRecording(false)
    setCurrentKeys([])
  }, [])

  // 显示内容
  const displayValue = isRecording
    ? currentKeys.length > 0
      ? currentKeys.join("+") + "+..."
      : "按下快捷键..."
    : value

  return (
    <button
      ref={inputRef}
      type="button"
      onClick={isRecording ? cancelRecording : startRecording}
      disabled={disabled}
      className={cn(
        "inline-flex items-center justify-center rounded border px-2 py-1 text-xs font-mono transition-colors",
        isRecording
          ? "border-blue-500 bg-blue-50 text-blue-600"
          : "border-gray-200 bg-gray-100 text-gray-700 hover:border-gray-300 hover:bg-gray-200",
        disabled && "cursor-not-allowed opacity-50",
        className
      )}
    >
      {displayValue}
    </button>
  )
}

/**
 * 将键盘事件的 key 转换为标准格式
 */
function normalizeKey(e: KeyboardEvent): string | null {
  const { key, code } = e

  // 忽略修饰键
  if (["Control", "Alt", "Shift", "Meta"].includes(key)) {
    return null
  }

  // 特殊键映射
  const specialKeys: Record<string, string> = {
    " ": "Space",
    "Escape": "Esc",
    "ArrowUp": "Up",
    "ArrowDown": "Down",
    "ArrowLeft": "Left",
    "ArrowRight": "Right",
    "Backquote": "`",
    "`": "`",
  }

  if (specialKeys[key]) {
    return specialKeys[key]
  }

  // 处理 Backquote（`键）
  if (code === "Backquote") {
    return "`"
  }

  // Pause 键
  if (key === "Pause") {
    return "Pause"
  }

  // 功能键 F1-F12
  if (/^F\d+$/.test(key)) {
    return key
  }

  // 数字键
  if (/^\d$/.test(key)) {
    return key
  }

  // 字母键（大写）
  if (/^[a-zA-Z]$/.test(key)) {
    return key.toUpperCase()
  }

  // 其他键
  return key
}
