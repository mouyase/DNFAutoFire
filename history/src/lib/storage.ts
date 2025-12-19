/**
 * 软件配置存储工具
 *
 * 负责管理应用设置（快捷键、行为配置等）
 */

import { isTauriEnvironment } from "./tauri-mock"

/**
 * 应用设置接口
 */
export interface AppSettings {
  /** 快捷键配置 */
  shortcut: {
    /** 打开/切换迷你窗口的快捷键 */
    popupWindow: string
    /** 全局开关连发的快捷键 */
    toggleAutofire: string
  }
  /** 行为配置 */
  behavior: {
    /** 启动/停止连发时播放音效 */
    playSoundOnToggle: boolean
  }
}

/**
 * 默认设置
 */
export const DEFAULT_SETTINGS: AppSettings = {
  shortcut: {
    popupWindow: "Alt+`",
    toggleAutofire: "Pause",
  },
  behavior: {
    playSoundOnToggle: true,
  },
}

const SETTINGS_KEY = "dnf-autofire-settings"

/**
 * 浏览器端存储实现
 */
const browserStorage = {
  async load(): Promise<AppSettings> {
    const json = localStorage.getItem(SETTINGS_KEY)
    if (json) {
      try {
        return { ...DEFAULT_SETTINGS, ...JSON.parse(json) }
      } catch {
        return DEFAULT_SETTINGS
      }
    }
    return DEFAULT_SETTINGS
  },

  async save(settings: AppSettings): Promise<void> {
    localStorage.setItem(SETTINGS_KEY, JSON.stringify(settings))
  },
}

/**
 * Tauri 端存储实现
 */
const tauriStorage = {
  async load(): Promise<AppSettings> {
    try {
      const { exists, readTextFile } = await import("@tauri-apps/plugin-fs")
      const { appDataDir, join } = await import("@tauri-apps/api/path")

      const appDir = await appDataDir()
      const settingsPath = await join(appDir, "settings.json")

      if (await exists(settingsPath)) {
        const content = await readTextFile(settingsPath)
        return { ...DEFAULT_SETTINGS, ...JSON.parse(content) }
      }
    } catch (error) {
      console.error("[设置] 加载失败:", error)
    }
    return DEFAULT_SETTINGS
  },

  async save(settings: AppSettings): Promise<void> {
    try {
      const { writeTextFile, mkdir, exists } = await import("@tauri-apps/plugin-fs")
      const { appDataDir, join } = await import("@tauri-apps/api/path")

      const appDir = await appDataDir()

      // 确保目录存在
      if (!(await exists(appDir))) {
        await mkdir(appDir, { recursive: true })
      }

      const settingsPath = await join(appDir, "settings.json")
      await writeTextFile(settingsPath, JSON.stringify(settings, null, 2))
      console.log("[设置] 已保存")
    } catch (error) {
      console.error("[设置] 保存失败:", error)
    }
  },
}

/**
 * 统一的设置存储接口
 */
export const settingsStorage = {
  /**
   * 加载设置
   */
  async load(): Promise<AppSettings> {
    if (isTauriEnvironment()) {
      return tauriStorage.load()
    }
    return browserStorage.load()
  },

  /**
   * 保存设置
   */
  async save(settings: AppSettings): Promise<void> {
    if (isTauriEnvironment()) {
      await tauriStorage.save(settings)
    } else {
      await browserStorage.save(settings)
    }
  },
}
