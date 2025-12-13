import { cn } from "@/lib/utils"
import type { RowConfig, KeyConfig } from "@/types"
import { Key } from "./key"

interface KeyboardRowProps {
  /** 行配置 */
  row: RowConfig
  /** 已启用的按键集合 */
  enabledKeys: Set<number>
  /** 按键点击回调 */
  onKeyClick: (vk: number) => void
}

/**
 * 键盘行组件
 */
export function KeyboardRow({ row, enabledKeys, onKeyClick }: KeyboardRowProps) {
  return (
    <div
      className={cn(
        "flex justify-center gap-1 mb-1 last:mb-0",
        // 功能键行额外间距
        row.type === "function" && "mb-3",
        // 编辑键行
        row.type === "edit" && "mt-3 justify-end pr-[50px]",
        // 方向键行
        row.type === "arrow" && "justify-end pr-[50px]"
      )}
    >
      {row.keys.map((item, index) => {
        // 间隔符
        if (item === "spacer") {
          return <span key={`spacer-${index}`} className="w-[30px]" />
        }
        if (item === "spacer-small") {
          return <span key={`spacer-small-${index}`} className="w-[15px]" />
        }

        // 按键
        const key = item as KeyConfig
        return (
          <Key
            key={key.vk}
            vk={key.vk}
            label={key.label}
            width={key.width}
            active={enabledKeys.has(key.vk)}
            onClick={onKeyClick}
          />
        )
      })}
    </div>
  )
}
