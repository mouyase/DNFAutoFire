import { GroupBox } from "@/components/ui"

interface ActionPanelProps {
  isRunning: boolean
  onToggle: () => void
  onSettings?: () => void
  onCheckUpdate?: () => void
}

/**
 * 操作按钮面板
 * 主按钮突出显示，次要操作放在底部
 */
export function ActionPanel({
  isRunning,
  onToggle,
  onSettings,
  onCheckUpdate,
}: ActionPanelProps) {
  return (
    <GroupBox title="控制" className="h-full">
      <div className="flex flex-col gap-2 h-full">
        {/* 次要操作按钮 */}
        <button
          type="button"
          onClick={onSettings}
          className="h-7 text-xs text-gray-600 bg-gray-100 hover:bg-gray-200 rounded transition-colors"
        >
          设置
        </button>
        <button
          type="button"
          onClick={onCheckUpdate}
          className="h-7 text-xs text-gray-600 bg-gray-100 hover:bg-gray-200 rounded transition-colors"
        >
          更新
        </button>

        {/* 主操作按钮 - 放在底部最显眼 */}
        <button
          type="button"
          onClick={onToggle}
          className={`
            mt-auto h-12 rounded-lg text-sm font-medium
            transition-all duration-200 shadow-md
            ${
              isRunning
                ? "bg-gradient-to-b from-red-500 to-red-600 text-white hover:from-red-600 hover:to-red-700 shadow-red-200"
                : "bg-gradient-to-b from-emerald-500 to-emerald-600 text-white hover:from-emerald-600 hover:to-emerald-700 shadow-emerald-200"
            }
          `}
        >
          {isRunning ? "停止" : "启动"}
        </button>
      </div>
    </GroupBox>
  )
}
