import { MAIN_KEYBOARD, EDIT_KEYS, NUMPAD_KEYS } from "@/lib/keyboard-layout"
import type { NumpadKey } from "@/lib/keyboard-layout"
import { Key } from "./key"
import { KEY_SIZE, KEY_GAP, FUNCTION_ROW_GAP } from "./constants"

interface KeyboardProps {
  enabledKeys: Set<number>
  onKeyClick: (vk: number) => void
  onClearKeys?: () => void
}

/** 渲染主键盘区 */
function MainKeyboard({
  enabledKeys,
  onKeyClick,
}: {
  enabledKeys: Set<number>
  onKeyClick: (vk: number) => void
}) {
  return (
    <div className="flex flex-col">
      {MAIN_KEYBOARD.map((row, rowIndex) => (
        <div
          key={rowIndex}
          className="flex"
          style={{
            gap: KEY_GAP,
            height: KEY_SIZE,
            // 第一行（功能键行）后面有额外间距
            marginBottom: rowIndex === 0 ? FUNCTION_ROW_GAP : KEY_GAP,
          }}
        >
          {row.map((key, keyIndex) => (
            <Key
              key={`${key.vk}-${keyIndex}`}
              config={key}
              active={enabledKeys.has(key.vk)}
              onClick={onKeyClick}
            />
          ))}
        </div>
      ))}
    </div>
  )
}

/** 渲染编辑键区 */
function EditKeys({
  enabledKeys,
  onKeyClick,
}: {
  enabledKeys: Set<number>
  onKeyClick: (vk: number) => void
}) {
  const sectionWidth = KEY_SIZE * 3 + KEY_GAP * 2

  return (
    <div className="flex flex-col">
      {EDIT_KEYS.map((row, rowIndex) => {
        // 空行
        if (row.length === 0) {
          return (
            <div
              key={rowIndex}
              style={{
                height: KEY_SIZE,
                width: sectionWidth,
                marginBottom: KEY_GAP,
              }}
            />
          )
        }

        return (
          <div
            key={rowIndex}
            className="flex"
            style={{
              gap: KEY_GAP,
              height: KEY_SIZE,
              // 第一行后有额外间距
              marginBottom: rowIndex === 0 ? FUNCTION_ROW_GAP : KEY_GAP,
            }}
          >
            {row.map((key, keyIndex) => (
              <Key
                key={`${key.vk}-${keyIndex}`}
                config={key}
                active={enabledKeys.has(key.vk)}
                onClick={onKeyClick}
              />
            ))}
          </div>
        )
      })}
    </div>
  )
}

/** 渲染数字小键盘 */
function Numpad({
  enabledKeys,
  onKeyClick,
  onClearKeys,
}: {
  enabledKeys: Set<number>
  onKeyClick: (vk: number) => void
  onClearKeys?: () => void
}) {
  // 计算小键盘总宽度：4个按键 + 3个间距
  const numpadWidth = KEY_SIZE * 4 + KEY_GAP * 3

  return (
    <div className="flex flex-col">
      {/* 第一行：清空按钮靠右对齐，与功能键行对齐 */}
      <div
        className="flex justify-end"
        style={{ width: numpadWidth, height: KEY_SIZE, marginBottom: FUNCTION_ROW_GAP }}
      >
        <button
          type="button"
          onClick={onClearKeys}
          style={{ width: KEY_SIZE, height: KEY_SIZE }}
          className="flex items-center justify-center rounded-md text-[11px] font-medium cursor-pointer select-none transition-all duration-75 bg-gradient-to-b from-red-50 to-red-100 text-red-600 border border-red-300 shadow-[inset_0_1px_0_rgba(255,255,255,0.8),0_1px_2px_rgba(0,0,0,0.1)] hover:border-red-400 hover:from-red-100 hover:to-red-200"
        >
          清空
        </button>
      </div>

      {/* 数字小键盘主体 */}
      <div
        className="grid"
        style={{
          gridTemplateColumns: `repeat(4, ${KEY_SIZE}px)`,
          gridTemplateRows: `repeat(5, ${KEY_SIZE}px)`,
          gap: KEY_GAP,
        }}
      >
        {NUMPAD_KEYS.map((key: NumpadKey) => {
          const colSpan = key.colSpan ?? 1
          const rowSpan = key.rowSpan ?? 1

          return (
            <button
              key={`numpad-${key.vk}-${key.col}-${key.row}`}
              type="button"
              onClick={() => onKeyClick(key.vk)}
              style={{
                gridColumn: `${key.col + 1} / span ${colSpan}`,
                gridRow: `${key.row + 1} / span ${rowSpan}`,
              }}
              className={`
                relative flex items-center justify-center
                rounded-md text-[11px] font-medium
                cursor-pointer select-none transition-all duration-75
                ${
                  enabledKeys.has(key.vk)
                    ? "bg-gradient-to-b from-gray-200 to-gray-300 text-gray-600 border border-gray-400 shadow-[inset_0_2px_4px_rgba(0,0,0,0.2),inset_0_1px_2px_rgba(0,0,0,0.1)] translate-y-[1px]"
                    : "bg-gradient-to-b from-white to-gray-100 text-gray-700 border border-gray-300 shadow-[inset_0_1px_0_rgba(255,255,255,0.8),0_1px_2px_rgba(0,0,0,0.1),0_2px_4px_rgba(0,0,0,0.05)] hover:border-blue-400"
                }
              `}
            >
              {key.label}
            </button>
          )
        })}
      </div>
    </div>
  )
}

/**
 * 完整键盘组件 (104键布局)
 */
export function Keyboard({ enabledKeys, onKeyClick, onClearKeys }: KeyboardProps) {
  return (
    <div className="inline-flex items-start" style={{ gap: KEY_SIZE * 0.4 }}>
      {/* 主键盘区 */}
      <MainKeyboard enabledKeys={enabledKeys} onKeyClick={onKeyClick} />

      {/* 编辑键区 */}
      <EditKeys enabledKeys={enabledKeys} onKeyClick={onKeyClick} />

      {/* 数字小键盘 */}
      <Numpad enabledKeys={enabledKeys} onKeyClick={onKeyClick} onClearKeys={onClearKeys} />
    </div>
  )
}
