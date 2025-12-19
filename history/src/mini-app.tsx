import { useCallback, useEffect, useRef, useState } from "react"
import { ProfileSelector } from "./components/mini-window/profile-selector"
import { useProfiles } from "./hooks/use-profiles"
import { useAutofire } from "./hooks/use-autofire"
import { tauriCommands } from "./lib/tauri"
import { listen } from "@tauri-apps/api/event"
import { getCurrentWindow } from "@tauri-apps/api/window"

/**
 * 迷你窗口主组件
 *
 * 功能：
 * - 显示配置列表和运行状态
 * - 键盘/鼠标切换配置
 * - 切换后自动启动连发并关闭窗口
 * - 可以停止连发
 */
export default function MiniApp() {
  const { profiles, activeProfileId, selectProfile, loadProfiles } = useProfiles()
  const { isRunning, start, stop } = useAutofire()

  // 选中索引状态，用于键盘导航
  const [manualSelectedIndex, setManualSelectedIndex] = useState<number | null>(null)

  // 计算当前选中索引
  const activeIndex = profiles.findIndex((p) => p.id === activeProfileId)
  const selectedIndex = manualSelectedIndex ?? (activeIndex >= 0 ? activeIndex : 0)

  // 使用 ref 保存最新的 profiles 和 selectedIndex
  const profilesRef = useRef(profiles)
  const selectedIndexRef = useRef(selectedIndex)
  useEffect(() => {
    profilesRef.current = profiles
    selectedIndexRef.current = selectedIndex
  }, [profiles, selectedIndex])

  // 窗口获得焦点时强制聚焦到 document（解决无边框窗口键盘事件问题）
  useEffect(() => {
    const focusDocument = () => {
      // 强制聚焦到 body 元素
      document.body.focus()
      // 如果有 activeElement，模拟点击来激活键盘事件
      if (document.activeElement instanceof HTMLElement) {
        document.activeElement.blur()
      }
      document.body.click()
    }

    // 初始聚焦
    focusDocument()

    // 监听窗口焦点事件
    const unlisten = listen("tauri://focus", () => {
      focusDocument()
    })

    return () => {
      void unlisten.then((fn) => fn())
    }
  }, [])

  // 关闭迷你窗口
  const closeWindow = useCallback(async () => {
    try {
      console.log("[MiniApp] 正在关闭窗口...")
      const win = getCurrentWindow()
      await win.hide()
      console.log("[MiniApp] 窗口已隐藏")
    } catch (error) {
      console.error("[MiniApp] 关闭窗口失败:", error)
    }
  }, [])

  // 处理选择配置
  const handleSelectAsync = useCallback(
    async (profileId: string) => {
      console.log("[MiniApp] 开始切换配置:", profileId)
      try {
        const profile = profilesRef.current.find((p) => p.id === profileId)
        if (!profile) {
          console.log("[MiniApp] 配置不存在:", profileId)
          return
        }

        console.log("[MiniApp] 选择配置:", profile.name)
        await selectProfile(profileId)

        console.log("[MiniApp] 设置按键:", profile.enabledKeys)
        await tauriCommands.setKeys(profile.enabledKeys)

        console.log("[MiniApp] 启动连发...")
        await start()

        console.log("[MiniApp] 发送配置切换事件...")
        const win = getCurrentWindow()
        await win.emit("profile-switched", { profileId })

        console.log("[MiniApp] 关闭窗口...")
        await closeWindow()
        console.log("[MiniApp] 切换配置完成")
      } catch (error) {
        console.error("[MiniApp] 切换配置失败:", error)
        // 即使出错也尝试关闭窗口
        try {
          await closeWindow()
        } catch {
          // 忽略关闭窗口的错误
        }
      }
    },
    [selectProfile, start, closeWindow]
  )

  // 存储最新的函数引用到 ref（在 effect 中更新）
  const handleSelectAsyncRef = useRef(handleSelectAsync)
  const closeWindowRef = useRef(closeWindow)
  useEffect(() => {
    handleSelectAsyncRef.current = handleSelectAsync
    closeWindowRef.current = closeWindow
  }, [handleSelectAsync, closeWindow])

  // 使用 document 级别的键盘事件监听（解决无边框窗口焦点问题）
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      const currentProfiles = profilesRef.current
      const currentIndex = selectedIndexRef.current

      switch (e.key) {
        case "ArrowUp":
          e.preventDefault()
          setManualSelectedIndex((prev) => {
            const current = prev ?? currentIndex
            return current > 0 ? current - 1 : currentProfiles.length - 1
          })
          break
        case "ArrowDown":
          e.preventDefault()
          setManualSelectedIndex((prev) => {
            const current = prev ?? currentIndex
            return current < currentProfiles.length - 1 ? current + 1 : 0
          })
          break
        case "Enter":
          e.preventDefault()
          if (currentProfiles[currentIndex]) {
            void handleSelectAsyncRef.current(currentProfiles[currentIndex].id)
          }
          break
        case "Escape":
          e.preventDefault()
          void closeWindowRef.current()
          break
      }
    }

    document.addEventListener("keydown", handleKeyDown)
    return () => {
      document.removeEventListener("keydown", handleKeyDown)
    }
  }, [])

  // 监听配置切换事件（从主窗口广播）
  useEffect(() => {
    const unlisten = listen("profile-switched", () => {
      void loadProfiles()
    })
    return () => {
      void unlisten.then((fn) => fn())
    }
  }, [loadProfiles])

  // 监听选择下一个配置事件（Alt+` 再次按下时触发）
  useEffect(() => {
    const unlisten = listen("select-next-profile", () => {
      setManualSelectedIndex((prev) => {
        const currentIndex = selectedIndexRef.current
        const currentProfiles = profilesRef.current
        const current = prev ?? currentIndex
        return current < currentProfiles.length - 1 ? current + 1 : 0
      })
    })
    return () => {
      void unlisten.then((fn) => fn())
    }
  }, [])

  // 点击选择配置
  const handleSelect = useCallback(
    (profileId: string) => {
      void handleSelectAsync(profileId)
    },
    [handleSelectAsync]
  )

  // 停止连发
  const handleStop = useCallback(async () => {
    await stop()
  }, [stop])

  // 处理悬停
  const handleHover = useCallback((index: number) => {
    setManualSelectedIndex(index)
  }, [])

  return (
    <div className="flex h-screen flex-col bg-surface text-text-primary">
      {/* 标题栏 */}
      <div className="flex items-center justify-between border-b border-border px-4 py-3">
        <div className="flex items-center gap-2">
          <h1 className="text-sm font-medium">切换配置</h1>
          <span
            className={`inline-block h-2 w-2 rounded-full ${isRunning ? "bg-green-500" : "bg-gray-400"}`}
          />
        </div>
        <span className="text-xs text-text-secondary">
          {isRunning ? "运行中" : "已停止"}
        </span>
      </div>

      {/* 配置列表 */}
      <div className="flex-1 overflow-y-auto p-2">
        <ProfileSelector
          profiles={profiles}
          activeProfileId={activeProfileId}
          selectedIndex={selectedIndex}
          onSelect={handleSelect}
          onHover={handleHover}
        />
      </div>

      {/* 底部操作栏 - 固定高度避免内容变化导致高度变化 */}
      <div className="flex h-12 items-center justify-between border-t border-border px-4">
        <div className="w-16">
          {isRunning && (
            <button
              onClick={() => void handleStop()}
              className="rounded bg-red-500 px-3 py-1.5 text-xs font-medium text-white transition-colors hover:bg-red-600"
            >
              停止
            </button>
          )}
        </div>
        <div className="text-xs text-text-secondary">
          <kbd className="rounded bg-gray-200 px-1">↑↓</kbd> 选择
          <span className="mx-1">|</span>
          <kbd className="rounded bg-gray-200 px-1">Enter</kbd> 确认
          <span className="mx-1">|</span>
          <kbd className="rounded bg-gray-200 px-1">Esc</kbd> 关闭
        </div>
      </div>
    </div>
  )
}
