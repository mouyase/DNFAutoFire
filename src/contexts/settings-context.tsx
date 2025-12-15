import { createContext, useContext, useState, useCallback, type ReactNode } from "react"
import { settingsStorage, DEFAULT_SETTINGS, type AppSettings } from "@/lib/storage"

interface SettingsContextValue {
  settings: AppSettings
  isLoading: boolean
  updateSettings: (newSettings: AppSettings) => Promise<void>
}

const SettingsContext = createContext<SettingsContextValue | null>(null)

interface SettingsProviderProps {
  children: ReactNode
  initialSettings: AppSettings
}

/**
 * 设置提供者组件
 *
 * 需要在应用入口处使用，传入预加载的配置
 */
export function SettingsProvider({ children, initialSettings }: SettingsProviderProps) {
  const [settings, setSettings] = useState<AppSettings>(initialSettings)
  const [isLoading] = useState(false)

  const updateSettings = useCallback(async (newSettings: AppSettings) => {
    setSettings(newSettings)
    await settingsStorage.save(newSettings)
    console.log("[设置] 已保存:", newSettings)
  }, [])

  return (
    <SettingsContext.Provider value={{ settings, isLoading, updateSettings }}>
      {children}
    </SettingsContext.Provider>
  )
}

/**
 * 获取设置的 Hook
 */
export function useSettings() {
  const context = useContext(SettingsContext)
  if (!context) {
    throw new Error("useSettings must be used within a SettingsProvider")
  }
  return context
}

/**
 * 预加载设置（在应用入口调用）
 */
export async function loadSettings(): Promise<AppSettings> {
  try {
    const settings = await settingsStorage.load()
    console.log("[设置] 预加载完成:", settings)
    return settings
  } catch (error) {
    console.error("[设置] 预加载失败:", error)
    return DEFAULT_SETTINGS
  }
}
