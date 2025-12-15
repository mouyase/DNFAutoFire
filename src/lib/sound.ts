/**
 * 音效工具模块
 *
 * 使用系统音效播放提示音
 */

import { tauriCommands, isMockMode } from "./tauri"

/**
 * 播放启动音效
 */
export function playStartSound(): void {
  if (isMockMode()) {
    console.log("[音效] 启动音效 (Mock)")
    return
  }
  void tauriCommands.playSound("start")
}

/**
 * 播放停止音效
 */
export function playStopSound(): void {
  if (isMockMode()) {
    console.log("[音效] 停止音效 (Mock)")
    return
  }
  void tauriCommands.playSound("stop")
}
