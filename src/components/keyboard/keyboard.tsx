import { KEYBOARD_LAYOUT } from "@/lib/keyboard-layout"
import { KeyboardRow } from "./keyboard-row"

interface KeyboardProps {
  /** 已启用的按键集合 */
  enabledKeys: Set<number>
  /** 按键点击回调 */
  onKeyClick: (vk: number) => void
}

/**
 * 键盘组件
 */
export function Keyboard({ enabledKeys, onKeyClick }: KeyboardProps) {
  return (
    <section className="bg-surface rounded-xl p-5">
      <h3 className="text-center text-text-muted font-normal text-sm mb-4">
        点击按键启用/禁用连发
      </h3>
      {KEYBOARD_LAYOUT.map((row, index) => (
        <KeyboardRow key={index} row={row} enabledKeys={enabledKeys} onKeyClick={onKeyClick} />
      ))}
    </section>
  )
}
