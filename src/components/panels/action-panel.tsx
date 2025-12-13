import { Button } from "@/components/layout"

interface ActionPanelProps {
  isRunning: boolean
  onToggle: () => void
  onSettings?: () => void
  onCheckUpdate?: () => void
}

/**
 * 操作按钮面板
 */
export function ActionPanel({
  isRunning,
  onToggle,
  onSettings,
  onCheckUpdate,
}: ActionPanelProps) {
  return (
    <div className="flex flex-col gap-2 h-full">
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
