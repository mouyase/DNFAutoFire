import type { KeyConfig } from "@/types"

/**
 * 标准 104 键键盘布局
 * 宽度单位：1U = 标准键宽
 *
 * 行宽度参考：
 * - 数字行：1 + 10×1 + 1 + 1 + 2 = 15U
 * - Tab行：1.5 + 10×1 + 1 + 1 + 1.5 = 15U
 * - Caps行：1.75 + 9×1 + 1 + 1 + 2.25 = 15U
 * - Shift行：2.25 + 7×1 + 1 + 1 + 1 + 2.75 = 15U
 * - 空格行：1.25×3 + 6.25 + 1.25×4 = 15U
 */

export const MAIN_KEYBOARD: KeyConfig[][] = [
  // 功能键行 (特殊布局，不是15U)
  [
    { vk: 27, label: "Esc", width: 1 },
    { vk: 0, label: "", empty: true, width: 1 }, // 空隙
    { vk: 112, label: "F1", width: 1 },
    { vk: 113, label: "F2", width: 1 },
    { vk: 114, label: "F3", width: 1 },
    { vk: 115, label: "F4", width: 1 },
    { vk: 0, label: "", empty: true, width: 0.5 }, // 空隙
    { vk: 116, label: "F5", width: 1 },
    { vk: 117, label: "F6", width: 1 },
    { vk: 118, label: "F7", width: 1 },
    { vk: 119, label: "F8", width: 1 },
    { vk: 0, label: "", empty: true, width: 0.5 }, // 空隙
    { vk: 120, label: "F9", width: 1 },
    { vk: 121, label: "F10", width: 1 },
    { vk: 122, label: "F11", width: 1 },
    { vk: 123, label: "F12", width: 1 },
  ],
  // 数字行 = 15U
  [
    { vk: 192, label: "`", width: 1 },
    { vk: 49, label: "1", width: 1 },
    { vk: 50, label: "2", width: 1 },
    { vk: 51, label: "3", width: 1 },
    { vk: 52, label: "4", width: 1 },
    { vk: 53, label: "5", width: 1 },
    { vk: 54, label: "6", width: 1 },
    { vk: 55, label: "7", width: 1 },
    { vk: 56, label: "8", width: 1 },
    { vk: 57, label: "9", width: 1 },
    { vk: 48, label: "0", width: 1 },
    { vk: 189, label: "-", width: 1 },
    { vk: 187, label: "=", width: 1 },
    { vk: 8, label: "Back", width: 2 },
  ],
  // Tab行 = 15U
  [
    { vk: 9, label: "Tab", width: 1.5 },
    { vk: 81, label: "Q", width: 1 },
    { vk: 87, label: "W", width: 1 },
    { vk: 69, label: "E", width: 1 },
    { vk: 82, label: "R", width: 1 },
    { vk: 84, label: "T", width: 1 },
    { vk: 89, label: "Y", width: 1 },
    { vk: 85, label: "U", width: 1 },
    { vk: 73, label: "I", width: 1 },
    { vk: 79, label: "O", width: 1 },
    { vk: 80, label: "P", width: 1 },
    { vk: 219, label: "[", width: 1 },
    { vk: 221, label: "]", width: 1 },
    { vk: 220, label: "\\", width: 1.5 },
  ],
  // Caps行 = 15U
  [
    { vk: 20, label: "Caps", width: 1.75 },
    { vk: 65, label: "A", width: 1 },
    { vk: 83, label: "S", width: 1 },
    { vk: 68, label: "D", width: 1 },
    { vk: 70, label: "F", width: 1 },
    { vk: 71, label: "G", width: 1 },
    { vk: 72, label: "H", width: 1 },
    { vk: 74, label: "J", width: 1 },
    { vk: 75, label: "K", width: 1 },
    { vk: 76, label: "L", width: 1 },
    { vk: 186, label: ";", width: 1 },
    { vk: 222, label: "'", width: 1 },
    { vk: 13, label: "Enter", width: 2.25 },
  ],
  // Shift行 = 15U
  [
    { vk: 160, label: "Shift", width: 2.25 },
    { vk: 90, label: "Z", width: 1 },
    { vk: 88, label: "X", width: 1 },
    { vk: 67, label: "C", width: 1 },
    { vk: 86, label: "V", width: 1 },
    { vk: 66, label: "B", width: 1 },
    { vk: 78, label: "N", width: 1 },
    { vk: 77, label: "M", width: 1 },
    { vk: 188, label: ",", width: 1 },
    { vk: 190, label: ".", width: 1 },
    { vk: 191, label: "/", width: 1 },
    { vk: 161, label: "Shift", width: 2.75 },
  ],
  // 空格行 = 15U
  [
    { vk: 162, label: "Ctrl", width: 1.25 },
    { vk: 91, label: "Win", width: 1.25 },
    { vk: 164, label: "Alt", width: 1.25 },
    { vk: 32, label: "", width: 6.25 }, // Space
    { vk: 165, label: "Alt", width: 1.25 },
    { vk: 92, label: "Win", width: 1.25 },
    { vk: 93, label: "App", width: 1.25 },
    { vk: 163, label: "Ctrl", width: 1.25 },
  ],
]

/**
 * 编辑键区布局（3列 x 6行）
 */
export const EDIT_KEYS: KeyConfig[][] = [
  // 第1行：与功能键行对齐
  [
    { vk: 44, label: "PrtSc", width: 1 },
    { vk: 145, label: "ScrLk", width: 1 },
    { vk: 19, label: "Pause", width: 1 },
  ],
  // 第2行
  [
    { vk: 45, label: "Ins", width: 1 },
    { vk: 36, label: "Home", width: 1 },
    { vk: 33, label: "PgUp", width: 1 },
  ],
  // 第3行
  [
    { vk: 46, label: "Del", width: 1 },
    { vk: 35, label: "End", width: 1 },
    { vk: 34, label: "PgDn", width: 1 },
  ],
  // 第4行：空行
  [],
  // 第5行：方向键上
  [
    { vk: 0, label: "", empty: true, width: 1 },
    { vk: 38, label: "↑", width: 1 },
    { vk: 0, label: "", empty: true, width: 1 },
  ],
  // 第6行：方向键下
  [
    { vk: 37, label: "←", width: 1 },
    { vk: 40, label: "↓", width: 1 },
    { vk: 39, label: "→", width: 1 },
  ],
]

/**
 * 数字小键盘布局（CSS Grid）
 * 4列 x 5行
 */
export interface NumpadKey extends KeyConfig {
  col: number
  row: number
  colSpan?: number
  rowSpan?: number
}

export const NUMPAD_KEYS: NumpadKey[] = [
  // 第1行
  { vk: 144, label: "Num", col: 0, row: 0, width: 1 },
  { vk: 111, label: "/", col: 1, row: 0, width: 1 },
  { vk: 106, label: "*", col: 2, row: 0, width: 1 },
  { vk: 109, label: "-", col: 3, row: 0, width: 1 },
  // 第2行
  { vk: 103, label: "7", col: 0, row: 1, width: 1 },
  { vk: 104, label: "8", col: 1, row: 1, width: 1 },
  { vk: 105, label: "9", col: 2, row: 1, width: 1 },
  { vk: 107, label: "+", col: 3, row: 1, rowSpan: 2, width: 1 },
  // 第3行
  { vk: 100, label: "4", col: 0, row: 2, width: 1 },
  { vk: 101, label: "5", col: 1, row: 2, width: 1 },
  { vk: 102, label: "6", col: 2, row: 2, width: 1 },
  // 第4行
  { vk: 97, label: "1", col: 0, row: 3, width: 1 },
  { vk: 98, label: "2", col: 1, row: 3, width: 1 },
  { vk: 99, label: "3", col: 2, row: 3, width: 1 },
  { vk: 13, label: "Enter", col: 3, row: 3, rowSpan: 2, width: 1 },
  // 第5行
  { vk: 96, label: "0", col: 0, row: 4, colSpan: 2, width: 1 },
  { vk: 110, label: ".", col: 2, row: 4, width: 1 },
]
