import { cn } from "@/lib/utils"
import { getVkName } from "@/lib/vk-codes"
import { Button } from "./button"

interface StatusBarProps {
  /** 是否正在运行 */
  isRunning: boolean
  /** 已启用的按键集合 */
  enabledKeys: Set<number>
  /** 清空所有按键 */
  onClear: () => void
}

/**
 * 状态栏组件
 */
export function StatusBar({ isRunning, enabledKeys, onClear }: StatusBarProps) {
  const enabledKeyNames =
    enabledKeys.size === 0
      ? "无"
      : Array.from(enabledKeys)
          .map((vk) => getVkName(vk))
          .join(", ")

  return (
    <footer className="flex items-center gap-5 px-5 py-3 bg-surface rounded-lg">
      <div className="flex items-center gap-2">
        <span className="text-text-muted text-sm">状态:</span>
        <span
          className={cn("font-medium", isRunning ? "text-accent animate-pulse" : "text-danger")}
        >
          {isRunning ? "运行中" : "已停止"}
        </span>
      </div>

      <div className="flex items-center gap-2">
        <span className="text-text-muted text-sm">已启用按键:</span>
        <span className="font-medium">{enabledKeyNames}</span>
      </div>

      <Button variant="secondary" className="ml-auto" onClick={onClear}>
        清空所有
      </Button>
    </footer>
  )
}
