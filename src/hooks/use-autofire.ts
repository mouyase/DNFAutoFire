import { useState, useEffect, useCallback } from "react"
import { tauriCommands } from "@/lib/tauri"

/**
 * 连发状态 Hook
 */
export function useAutofire() {
  const [isRunning, setIsRunning] = useState(false)
  const [enabledKeys, setEnabledKeys] = useState<Set<number>>(new Set())

  // 刷新状态
  const refresh = useCallback(async () => {
    try {
      const running = await tauriCommands.isRunning()
      const keys = await tauriCommands.getEnabledKeys()
      setIsRunning(running)
      setEnabledKeys(new Set(keys))
    } catch (error) {
      console.error("刷新状态失败:", error)
    }
  }, [])

  // 切换按键
  const toggleKey = useCallback(async (vk: number) => {
    try {
      const enabled = await tauriCommands.toggleKey(vk)
      setEnabledKeys((prev) => {
        const next = new Set(prev)
        if (enabled) {
          next.add(vk)
        } else {
          next.delete(vk)
        }
        return next
      })
      return enabled
    } catch (error) {
      console.error("切换按键失败:", error)
      return false
    }
  }, [])

  // 清空所有按键
  const clearKeys = useCallback(async () => {
    try {
      await tauriCommands.clearKeys()
      setEnabledKeys(new Set())
    } catch (error) {
      console.error("清空按键失败:", error)
    }
  }, [])

  // 启动连发
  const start = useCallback(async () => {
    try {
      await tauriCommands.startAutofire()
      setIsRunning(true)
    } catch (error) {
      console.error("启动连发失败:", error)
    }
  }, [])

  // 停止连发
  const stop = useCallback(async () => {
    try {
      await tauriCommands.stopAutofire()
      setIsRunning(false)
    } catch (error) {
      console.error("停止连发失败:", error)
    }
  }, [])

  // 切换连发状态
  const toggle = useCallback(async () => {
    if (isRunning) {
      await stop()
    } else {
      await start()
    }
  }, [isRunning, start, stop])

  // 初始化和定时刷新状态
  useEffect(() => {
    // 初始化获取状态
    const init = async () => {
      try {
        const running = await tauriCommands.isRunning()
        const keys = await tauriCommands.getEnabledKeys()
        setIsRunning(running)
        setEnabledKeys(new Set(keys))
      } catch (error) {
        console.error("初始化状态失败:", error)
      }
    }
    void init()

    // 定时刷新
    const interval = setInterval(refresh, 500)
    return () => clearInterval(interval)
  }, [refresh])

  return {
    isRunning,
    enabledKeys,
    toggleKey,
    clearKeys,
    start,
    stop,
    toggle,
    refresh,
  }
}
