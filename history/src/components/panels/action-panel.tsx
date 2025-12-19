import { GroupBox } from "@/components/ui"

interface ActionPanelProps {
  isRunning: boolean
  onToggle: () => void
  onSettings?: () => void
  onCheckUpdate?: () => void
}

/**
 * 操作按钮面板
 * 主按钮突出显示，次要操作放在顶部
 */
export function ActionPanel({ isRunning, onToggle, onSettings, onCheckUpdate }: ActionPanelProps) {
  return (
    <GroupBox title="控制" className="h-full">
      <div className="flex flex-col gap-1.5 h-full">
        {/* 次要操作按钮 */}
        <button
          type="button"
          onClick={onSettings}
          className="h-7 text-xs text-gray-700 font-medium bg-gray-100 hover:bg-gray-200 border border-gray-200 rounded transition-colors"
        >
          设置
        </button>
        <button
          type="button"
          onClick={onCheckUpdate}
          className="h-7 text-xs text-gray-700 font-medium bg-gray-100 hover:bg-gray-200 border border-gray-200 rounded transition-colors"
        >
          更新
        </button>

        {/* 主操作按钮 - 放在底部最显眼 */}
        <button
          type="button"
          onClick={onToggle}
          className={`mt-auto h-12 rounded text-sm font-medium transition-colors ${
            isRunning
              ? "bg-red-500 text-white hover:bg-red-600"
              : "bg-emerald-500 text-white hover:bg-emerald-600"
          }`}
        >
          {isRunning ? "停止" : "启动"}
        </button>
      </div>
    </GroupBox>
  )
}
