import { Keyboard } from "@/components/keyboard"
import { GroupBox } from "@/components/ui"
import { cn } from "@/lib/utils"

interface KeyboardPanelProps {
  enabledKeys: Set<number>
  onKeyClick: (vk: number) => void
  onClearKeys: () => void
  /** 当前配置名称 */
  activeProfileName?: string
  /** 是否有未保存的更改 */
  hasUnsavedChanges?: boolean
}

/**
 * 键盘设置面板
 * 清空按钮放在键盘右上角
 */
export function KeyboardPanel({
  enabledKeys,
  onKeyClick,
  onClearKeys,
  activeProfileName,
  hasUnsavedChanges,
}: KeyboardPanelProps) {
  // 当前配置名称显示（带未保存状态指示）
  const profileIndicator = activeProfileName ? (
    <span
      className={cn(
        "px-2 py-0.5 rounded text-xs font-medium",
        hasUnsavedChanges
          ? "bg-amber-100 text-amber-700"
          : "bg-blue-100 text-blue-700"
      )}
    >
      {activeProfileName}
      {hasUnsavedChanges && " *"}
    </span>
  ) : null

  return (
    <GroupBox title="按键设置" extra={profileIndicator} className="inline-block">
      <Keyboard enabledKeys={enabledKeys} onKeyClick={onKeyClick} onClearKeys={onClearKeys} />
    </GroupBox>
  )
}
