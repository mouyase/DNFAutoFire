import { Keyboard } from "@/components/keyboard"
import { GroupBox } from "@/components/ui"
import { Button } from "@/components/layout"

interface KeyboardPanelProps {
  version: string
  enabledKeys: Set<number>
  onKeyClick: (vk: number) => void
  onClearKeys: () => void
}

/**
 * 键盘设置面板
 * 版本信息显示在键盘右上角的空白区域
 */
export function KeyboardPanel({
  version,
  enabledKeys,
  onKeyClick,
  onClearKeys,
}: KeyboardPanelProps) {
  return (
    <GroupBox title="按键设置 - 【红色为启用连发 蓝色为关闭连发】">
      <div className="flex">
        {/* 键盘 */}
        <Keyboard enabledKeys={enabledKeys} onKeyClick={onKeyClick} />

        {/* 右侧信息栏 - 在键盘旁边 */}
        <div className="flex flex-col items-end ml-3 pt-1">
          <span className="text-xs text-text-secondary whitespace-nowrap">
            当前版本：{version}
          </span>
          <a
            href="https://github.com"
            target="_blank"
            rel="noopener noreferrer"
            className="text-xs text-text-link underline hover:text-blue-800 whitespace-nowrap"
          >
            Github
          </a>
          <Button
            variant="secondary"
            size="sm"
            onClick={onClearKeys}
            className="mt-2"
          >
            清空
          </Button>
        </div>
      </div>
    </GroupBox>
  )
}
