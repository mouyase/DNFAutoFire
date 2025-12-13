import { useState, useEffect, useCallback } from "react"
import { tauriCommands } from "@/lib/tauri"

/**
 * 管理员权限 Hook
 */
export function useAdmin() {
  const [isElevated, setIsElevated] = useState(false)
  const [isChecking, setIsChecking] = useState(true)

  // 检查权限
  const checkAdmin = useCallback(async () => {
    setIsChecking(true)
    try {
      const elevated = await tauriCommands.isElevated()
      setIsElevated(elevated)
    } catch (error) {
      console.error("检查管理员权限失败:", error)
      setIsElevated(false)
    } finally {
      setIsChecking(false)
    }
  }, [])

  // 以管理员权限重启
  const restartAsAdmin = useCallback(async () => {
    try {
      await tauriCommands.restartAsAdmin()
    } catch (error) {
      console.error("以管理员权限重启失败:", error)
    }
  }, [])

  // 初始化时检查权限
  useEffect(() => {
    void checkAdmin()
  }, [checkAdmin])

  return {
    isElevated,
    isChecking,
    restartAsAdmin,
  }
}
