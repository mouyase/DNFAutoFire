import { useCallback, useEffect, useRef } from "react"
import { AppLayout } from "@/components/layout"
import { KeyboardPanel, ConfigPanel, FeaturesPanel, ActionPanel } from "@/components/panels"
import { useAutofire } from "@/hooks/use-autofire"
import { useProfiles } from "@/hooks/use-profiles"
import { tauriCommands } from "@/lib/tauri"

/**
 * 应用根组件
 *
 * 配置管理逻辑：
 * - 选择配置：切换配置并自动加载该配置的按键
 * - 保存：把键盘上当前的按键保存到配置（未保存时按钮高亮）
 * - 重置：放弃未保存的更改，恢复到上次保存的状态
 */
function App() {
  const { isRunning, enabledKeys, toggleKey, clearKeys, toggle, refresh } = useAutofire()
  const {
    profiles,
    activeProfile,
    activeProfileId,
    isLoading,
    hasUnsavedChanges,
    createProfile,
    cloneProfile,
    deleteProfile,
    renameProfile,
    selectProfile,
    updateEnabledKeys,
    saveCurrentProfile,
  } = useProfiles()

  // 用于跟踪按键变化，避免不必要的更新
  const prevEnabledKeysRef = useRef<string>("")
  // 标记是否正在加载配置，防止循环
  const isLoadingProfileRef = useRef(false)

  // 同步键盘按键到配置状态（用户点击按键时）
  useEffect(() => {
    if (!activeProfileId || isLoadingProfileRef.current) return

    const keysArray = [...enabledKeys].sort((a, b) => a - b)
    const keysString = keysArray.join(",")

    if (keysString !== prevEnabledKeysRef.current) {
      prevEnabledKeysRef.current = keysString
      updateEnabledKeys(keysArray)
    }
  }, [enabledKeys, activeProfileId, updateEnabledKeys])

  // 点击键盘按键
  const handleKeyClick = useCallback(
    (vk: number) => {
      void toggleKey(vk)
    },
    [toggleKey]
  )

  // 清空所有按键
  const handleClearKeys = useCallback(() => {
    void clearKeys()
  }, [clearKeys])

  // 切换连发状态
  const handleToggle = useCallback(() => {
    void toggle()
  }, [toggle])

  // 加载配置的按键到键盘
  const loadProfileKeys = useCallback(
    async (keys: number[]) => {
      isLoadingProfileRef.current = true

      const keysArray = [...keys].sort((a, b) => a - b)
      prevEnabledKeysRef.current = keysArray.join(",")

      await tauriCommands.setKeys(keys)
      await refresh()

      requestAnimationFrame(() => {
        isLoadingProfileRef.current = false
      })
    },
    [refresh]
  )

  // 重置：放弃未保存的更改，恢复到上次保存的状态
  const handleResetProfile = useCallback(async () => {
    if (activeProfile) {
      await loadProfileKeys(activeProfile.enabledKeys)
    }
  }, [activeProfile, loadProfileKeys])

  // 选择配置：切换并自动加载按键
  const handleSelectProfile = useCallback(
    async (id: string) => {
      await selectProfile(id)

      // 找到新配置并加载其按键
      const profile = profiles.find((p) => p.id === id)
      if (profile) {
        await loadProfileKeys(profile.enabledKeys)
      }
    },
    [selectProfile, profiles, loadProfileKeys]
  )

  // 初始加载：应用启动时加载默认配置的按键
  const hasInitializedRef = useRef(false)
  useEffect(() => {
    if (hasInitializedRef.current || isLoading || !activeProfile) return
    hasInitializedRef.current = true

    void loadProfileKeys(activeProfile.enabledKeys)
  }, [isLoading, activeProfile, loadProfileKeys])

  return (
    <AppLayout
      top={
        <KeyboardPanel
          enabledKeys={enabledKeys}
          onKeyClick={handleKeyClick}
          onClearKeys={handleClearKeys}
          activeProfileName={activeProfile?.name}
          hasUnsavedChanges={hasUnsavedChanges}
        />
      }
      bottomLeft={
        <ConfigPanel
          profiles={profiles}
          activeProfileId={activeProfileId}
          hasUnsavedChanges={hasUnsavedChanges}
          isLoading={isLoading}
          onSelectProfile={handleSelectProfile}
          onResetProfile={handleResetProfile}
          onSaveProfile={saveCurrentProfile}
          onDeleteProfile={deleteProfile}
          onCloneProfile={cloneProfile}
          onRenameProfile={renameProfile}
          onCreateProfile={createProfile}
        />
      }
      bottomCenter={<FeaturesPanel />}
      bottomRight={<ActionPanel isRunning={isRunning} onToggle={handleToggle} />}
    />
  )
}

export default App
