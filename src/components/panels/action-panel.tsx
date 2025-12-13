import { Button } from "@/components/layout"

interface ActionPanelProps {
  isRunning: boolean
  onToggle: () => void
  onSettings?: () => void
  onCheckUpdate?: () => void
}

/**
 * 操作按钮面板
 * 不使用 GroupBox，直接显示按钮，与左侧 GroupBox 的内容区域对齐
 */
export function ActionPanel({
  isRunning,
  onToggle,
  onSettings,
  onCheckUpdate,
}: ActionPanelProps) {
  return (
    <div className="flex flex-col gap-2 h-full pt-4">
      <Button
        variant="secondary"
        size="md"
        className="w-full whitespace-nowrap"
        onClick={onSettings}
      >
        软件设置
      </Button>
      <Button
        variant="secondary"
        size="md"
        className="w-full whitespace-nowrap"
        onClick={onCheckUpdate}
      >
        检查更新
      </Button>
      <div className="flex-1" />
      <Button
        variant={isRunning ? "danger" : "success"}
        size="lg"
        className="w-full whitespace-nowrap"
        onClick={onToggle}
      >
        {isRunning ? "停止连发" : "启动连发"}
      </Button>
    </div>
  )
}
