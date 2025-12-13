import type { RowConfig } from "@/types"

/**
 * 键盘布局配置
 */
export const KEYBOARD_LAYOUT: RowConfig[] = [
  // 功能键行
  {
    type: "function",
    keys: [
      { vk: 27, label: "Esc", width: "func" },
      "spacer",
      { vk: 112, label: "F1", width: "func" },
      { vk: 113, label: "F2", width: "func" },
      { vk: 114, label: "F3", width: "func" },
      { vk: 115, label: "F4", width: "func" },
      "spacer-small",
      { vk: 116, label: "F5", width: "func" },
      { vk: 117, label: "F6", width: "func" },
      { vk: 118, label: "F7", width: "func" },
      { vk: 119, label: "F8", width: "func" },
      "spacer-small",
      { vk: 120, label: "F9", width: "func" },
      { vk: 121, label: "F10", width: "func" },
      { vk: 122, label: "F11", width: "func" },
      { vk: 123, label: "F12", width: "func" },
    ],
  },
  // 数字行
  {
    keys: [
      { vk: 192, label: "`" },
      { vk: 49, label: "1" },
      { vk: 50, label: "2" },
      { vk: 51, label: "3" },
      { vk: 52, label: "4" },
      { vk: 53, label: "5" },
      { vk: 54, label: "6" },
      { vk: 55, label: "7" },
      { vk: 56, label: "8" },
      { vk: 57, label: "9" },
      { vk: 48, label: "0" },
      { vk: 189, label: "-" },
      { vk: 187, label: "=" },
      { vk: 8, label: "Backspace", width: "wide-2" },
    ],
  },
  // QWERTY 行
  {
    keys: [
      { vk: 9, label: "Tab", width: "wide-1-5" },
      { vk: 81, label: "Q" },
      { vk: 87, label: "W" },
      { vk: 69, label: "E" },
      { vk: 82, label: "R" },
      { vk: 84, label: "T" },
      { vk: 89, label: "Y" },
      { vk: 85, label: "U" },
      { vk: 73, label: "I" },
      { vk: 79, label: "O" },
      { vk: 80, label: "P" },
      { vk: 219, label: "[" },
      { vk: 221, label: "]" },
      { vk: 220, label: "\\", width: "wide-1-5" },
    ],
  },
  // ASDF 行
  {
    keys: [
      { vk: 20, label: "Caps", width: "wide-1-75" },
      { vk: 65, label: "A" },
      { vk: 83, label: "S" },
      { vk: 68, label: "D" },
      { vk: 70, label: "F" },
      { vk: 71, label: "G" },
      { vk: 72, label: "H" },
      { vk: 74, label: "J" },
      { vk: 75, label: "K" },
      { vk: 76, label: "L" },
      { vk: 186, label: ";" },
      { vk: 222, label: "'" },
      { vk: 13, label: "Enter", width: "wide-2-25" },
    ],
  },
  // ZXCV 行
  {
    keys: [
      { vk: 160, label: "LShift", width: "wide-2-25" },
      { vk: 90, label: "Z" },
      { vk: 88, label: "X" },
      { vk: 67, label: "C" },
      { vk: 86, label: "V" },
      { vk: 66, label: "B" },
      { vk: 78, label: "N" },
      { vk: 77, label: "M" },
      { vk: 188, label: "," },
      { vk: 190, label: "." },
      { vk: 191, label: "/" },
      { vk: 161, label: "RShift", width: "wide-2-75" },
    ],
  },
  // 空格行
  {
    keys: [
      { vk: 162, label: "LCtrl", width: "wide-1-25" },
      { vk: 91, label: "Win", width: "wide-1-25" },
      { vk: 164, label: "LAlt", width: "wide-1-25" },
      { vk: 32, label: "Space", width: "space" },
      { vk: 165, label: "RAlt", width: "wide-1-25" },
      { vk: 92, label: "Win", width: "wide-1-25" },
      { vk: 93, label: "Menu", width: "wide-1-25" },
      { vk: 163, label: "RCtrl", width: "wide-1-25" },
    ],
  },
  // 编辑键行
  {
    type: "edit",
    keys: [
      { vk: 45, label: "Ins", width: "small" },
      { vk: 36, label: "Home", width: "small" },
      { vk: 33, label: "PgUp", width: "small" },
      "spacer-small",
      { vk: 46, label: "Del", width: "small" },
      { vk: 35, label: "End", width: "small" },
      { vk: 34, label: "PgDn", width: "small" },
      "spacer",
      { vk: 38, label: "↑", width: "arrow" },
    ],
  },
  // 方向键行
  {
    type: "arrow",
    keys: [
      { vk: 37, label: "←", width: "arrow" },
      { vk: 40, label: "↓", width: "arrow" },
      { vk: 39, label: "→", width: "arrow" },
    ],
  },
]
