import { Keyboard } from "@/components/keyboard"
import { GroupBox } from "@/components/ui"
import { Button } from "@/components/layout"

interface KeyboardPanelProps {
  enabledKeys: Set<number>
  onKeyClick: (vk: number) => void
  onClearKeys: () => void
}

/**
 * 键盘设置面板
 * 清空按钮放在键盘右上角
 */
export function KeyboardPanel({
  enabledKeys,
  onKeyClick,
  onClearKeys,
}: KeyboardPanelProps) {
  return (
    <GroupBox title="按键设置 - 【红色为启用连发 蓝色为关闭连发】" className="inline-block">
      <Keyboard
        enabledKeys={enabledKeys}
        onKeyClick={onKeyClick}
        onClearKeys={onClearKeys}
      />
    </GroupBox>
  )
}
