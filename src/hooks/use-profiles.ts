import { useState, useEffect, useCallback, useRef, useMemo } from "react"
import type { Profile } from "@/types"

// 配置文件存储 key
const STORAGE_KEY = "dnf-autofire-profiles"
const ACTIVE_KEY = "dnf-autofire-active-profile"

/**
 * 检测是否在 Tauri 环境中运行
 */
function isTauriEnvironment(): boolean {
  return typeof window !== "undefined" && "__TAURI__" in window
}

/**
 * 生成唯一 ID
 */
function generateId(): string {
  return Date.now().toString(36) + Math.random().toString(36).slice(2, 9)
}

/**
 * 创建默认配置
 */
function createDefaultProfile(): Profile {
  return {
    id: generateId(),
    name: "默认",
    enabledKeys: [],
  }
}

/**
 * 浏览器端存储实现
 */
const browserStorage = {
  async loadProfiles(): Promise<{ profiles: Profile[]; activeId: string | null }> {
    const profilesJson = localStorage.getItem(STORAGE_KEY)
    const activeId = localStorage.getItem(ACTIVE_KEY)
    const profiles: Profile[] = profilesJson ? JSON.parse(profilesJson) : []
    return { profiles, activeId }
  },

  async saveProfiles(profiles: Profile[]): Promise<void> {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(profiles))
  },

  async saveActiveId(id: string): Promise<void> {
    localStorage.setItem(ACTIVE_KEY, id)
  },
}

/**
 * Tauri 端存储实现
 */
const tauriStorage = {
  async loadProfiles(): Promise<{ profiles: Profile[]; activeId: string | null }> {
    const { exists, mkdir, readTextFile } = await import("@tauri-apps/plugin-fs")
    const { appDataDir, join } = await import("@tauri-apps/api/path")

    const appDir = await appDataDir()
    const profilesDir = await join(appDir, "profiles")

    // 确保目录存在
    if (!(await exists(profilesDir))) {
      await mkdir(profilesDir, { recursive: true })
    }

    const profiles: Profile[] = []
    let activeId: string | null = null

    // 读取活动配置 ID
    const activeFilePath = await join(profilesDir, "active.json")
    if (await exists(activeFilePath)) {
      const content = await readTextFile(activeFilePath)
      const data = JSON.parse(content) as { activeId: string }
      activeId = data.activeId
    }

    // 读取配置索引
    const indexPath = await join(profilesDir, "index.json")
    if (await exists(indexPath)) {
      const indexContent = await readTextFile(indexPath)
      const index = JSON.parse(indexContent) as { profileIds: string[] }

      for (const id of index.profileIds) {
        const profilePath = await join(profilesDir, `${id}.json`)
        if (await exists(profilePath)) {
          const content = await readTextFile(profilePath)
          profiles.push(JSON.parse(content) as Profile)
        }
      }
    }

    return { profiles, activeId }
  },

  async saveProfile(profile: Profile): Promise<void> {
    const { writeTextFile } = await import("@tauri-apps/plugin-fs")
    const { appDataDir, join } = await import("@tauri-apps/api/path")

    const appDir = await appDataDir()
    const profilePath = await join(appDir, "profiles", `${profile.id}.json`)
    await writeTextFile(profilePath, JSON.stringify(profile, null, 2))
  },

  async saveIndex(profileIds: string[]): Promise<void> {
    const { writeTextFile, mkdir, exists } = await import("@tauri-apps/plugin-fs")
    const { appDataDir, join } = await import("@tauri-apps/api/path")

    const appDir = await appDataDir()
    const profilesDir = await join(appDir, "profiles")

    // 确保目录存在
    if (!(await exists(profilesDir))) {
      await mkdir(profilesDir, { recursive: true })
    }

    const indexPath = await join(profilesDir, "index.json")
    await writeTextFile(indexPath, JSON.stringify({ profileIds }, null, 2))
  },

  async saveActiveId(id: string): Promise<void> {
    const { writeTextFile, mkdir, exists } = await import("@tauri-apps/plugin-fs")
    const { appDataDir, join } = await import("@tauri-apps/api/path")

    const appDir = await appDataDir()
    const profilesDir = await join(appDir, "profiles")

    // 确保目录存在
    if (!(await exists(profilesDir))) {
      await mkdir(profilesDir, { recursive: true })
    }

    const activeFilePath = await join(profilesDir, "active.json")
    await writeTextFile(activeFilePath, JSON.stringify({ activeId: id }))
  },

  async deleteProfile(id: string): Promise<void> {
    const { remove, exists } = await import("@tauri-apps/plugin-fs")
    const { appDataDir, join } = await import("@tauri-apps/api/path")

    const appDir = await appDataDir()
    const profilePath = await join(appDir, "profiles", `${id}.json`)

    if (await exists(profilePath)) {
      await remove(profilePath)
    }
  },
}

/**
 * 配置管理 Hook
 * 负责配置的 CRUD 和持久化
 */
export function useProfiles() {
  const [profiles, setProfiles] = useState<Profile[]>([])
  const [activeProfileId, setActiveProfileId] = useState<string | null>(null)
  const [isLoading, setIsLoading] = useState(true)
  const [hasUnsavedChanges, setHasUnsavedChanges] = useState(false)

  // 保存初始按键状态，用于检测变化
  const savedKeysRef = useRef<number[]>([])

  // 获取当前活动配置（使用 useMemo 稳定引用，只在 id 或 name 变化时更新）
  const activeProfile = useMemo(() => {
    const profile = profiles.find((p) => p.id === activeProfileId)
    return profile ?? null
  }, [profiles, activeProfileId])

  // 是否为 Tauri 环境
  const isTauri = isTauriEnvironment()

  /**
   * 加载所有配置
   */
  const loadProfiles = useCallback(async () => {
    try {
      setIsLoading(true)

      let loadedProfiles: Profile[]
      let savedActiveId: string | null

      if (isTauri) {
        const result = await tauriStorage.loadProfiles()
        loadedProfiles = result.profiles
        savedActiveId = result.activeId
      } else {
        const result = await browserStorage.loadProfiles()
        loadedProfiles = result.profiles
        savedActiveId = result.activeId
      }

      // 如果没有配置，创建默认配置
      if (loadedProfiles.length === 0) {
        const defaultProfile = createDefaultProfile()
        loadedProfiles = [defaultProfile]
        savedActiveId = defaultProfile.id

        // 保存默认配置
        if (isTauri) {
          await tauriStorage.saveProfile(defaultProfile)
          await tauriStorage.saveIndex([defaultProfile.id])
          await tauriStorage.saveActiveId(defaultProfile.id)
        } else {
          await browserStorage.saveProfiles(loadedProfiles)
          await browserStorage.saveActiveId(defaultProfile.id)
        }
      }

      setProfiles(loadedProfiles)
      console.log("[配置] 加载完成, 配置数量:", loadedProfiles.length)

      // 设置活动配置
      const validActiveId =
        loadedProfiles.find((p) => p.id === savedActiveId)?.id ??
        loadedProfiles[0]?.id ??
        null
      setActiveProfileId(validActiveId)
      console.log("[配置] 活动配置 ID:", validActiveId)

      // 更新保存的按键状态
      const active = loadedProfiles.find((p) => p.id === validActiveId)
      savedKeysRef.current = active?.enabledKeys ?? []
      setHasUnsavedChanges(false)
    } catch (error) {
      console.error("加载配置失败:", error)

      // 发生错误时创建默认配置
      const defaultProfile = createDefaultProfile()
      setProfiles([defaultProfile])
      setActiveProfileId(defaultProfile.id)
      savedKeysRef.current = []

      // 也尝试保存默认配置
      try {
        if (!isTauri) {
          await browserStorage.saveProfiles([defaultProfile])
          await browserStorage.saveActiveId(defaultProfile.id)
        }
      } catch {
        // 忽略保存错误
      }
    } finally {
      setIsLoading(false)
    }
  }, [isTauri])

  /**
   * 保存配置到存储
   */
  const saveToStorage = useCallback(
    async (newProfiles: Profile[], profile?: Profile) => {
      if (isTauri) {
        if (profile) {
          await tauriStorage.saveProfile(profile)
        }
        await tauriStorage.saveIndex(newProfiles.map((p) => p.id))
      } else {
        await browserStorage.saveProfiles(newProfiles)
      }
    },
    [isTauri]
  )

  /**
   * 保存活动配置 ID
   */
  const saveActiveIdToStorage = useCallback(
    async (id: string) => {
      if (isTauri) {
        await tauriStorage.saveActiveId(id)
      } else {
        await browserStorage.saveActiveId(id)
      }
    },
    [isTauri]
  )

  /**
   * 创建新配置
   */
  const createProfile = useCallback(async (): Promise<Profile> => {
    // 生成不重复的名称
    let name = "新配置"
    let counter = 1
    while (profiles.some((p) => p.name === name)) {
      name = `新配置${counter++}`
    }

    const newProfile: Profile = {
      id: generateId(),
      name,
      enabledKeys: [],
    }

    const newProfiles = [...profiles, newProfile]
    setProfiles(newProfiles)
    setActiveProfileId(newProfile.id)
    savedKeysRef.current = []
    setHasUnsavedChanges(false)

    // 保存到存储
    await saveToStorage(newProfiles, newProfile)
    await saveActiveIdToStorage(newProfile.id)

    console.log(`[配置] 新建: "${name}"`)
    return newProfile
  }, [profiles, saveToStorage, saveActiveIdToStorage])

  /**
   * 复制当前配置
   */
  const cloneProfile = useCallback(async (): Promise<Profile | null> => {
    if (!activeProfile) return null

    // 生成不重复的名称
    let name = `${activeProfile.name}-副本`
    let counter = 1
    while (profiles.some((p) => p.name === name)) {
      name = `${activeProfile.name}-副本${counter++}`
    }

    const clonedProfile: Profile = {
      id: generateId(),
      name,
      enabledKeys: [...activeProfile.enabledKeys],
    }

    const newProfiles = [...profiles, clonedProfile]
    setProfiles(newProfiles)
    setActiveProfileId(clonedProfile.id)
    savedKeysRef.current = [...clonedProfile.enabledKeys]
    setHasUnsavedChanges(false)

    // 保存到存储
    await saveToStorage(newProfiles, clonedProfile)
    await saveActiveIdToStorage(clonedProfile.id)

    console.log(`[配置] 复制: "${activeProfile.name}" → "${name}"`)
    return clonedProfile
  }, [activeProfile, profiles, saveToStorage, saveActiveIdToStorage])

  /**
   * 删除配置
   */
  const deleteProfile = useCallback(
    async (id: string): Promise<boolean> => {
      if (profiles.length <= 1) return false

      const profile = profiles.find((p) => p.id === id)
      if (!profile) return false

      const newProfiles = profiles.filter((p) => p.id !== id)
      setProfiles(newProfiles)

      // 如果删除的是当前活动配置，切换到第一个
      if (id === activeProfileId) {
        const newActive = newProfiles[0]
        setActiveProfileId(newActive.id)
        savedKeysRef.current = newActive.enabledKeys
        setHasUnsavedChanges(false)
        await saveActiveIdToStorage(newActive.id)
      }

      // 删除文件/更新存储
      if (isTauri) {
        await tauriStorage.deleteProfile(id)
        await tauriStorage.saveIndex(newProfiles.map((p) => p.id))
      } else {
        await browserStorage.saveProfiles(newProfiles)
      }

      console.log(`[配置] 删除: "${profile.name}"`)
      return true
    },
    [profiles, activeProfileId, isTauri, saveActiveIdToStorage]
  )

  /**
   * 重命名配置
   */
  const renameProfile = useCallback(
    async (id: string, newName: string): Promise<boolean> => {
      const trimmed = newName.trim()
      if (!trimmed) return false

      const profile = profiles.find((p) => p.id === id)
      if (!profile) return false

      // 检查名称是否重复
      if (profiles.some((p) => p.id !== id && p.name === trimmed)) {
        return false
      }

      const oldName = profile.name
      const updatedProfile = { ...profile, name: trimmed }
      const newProfiles = profiles.map((p) => (p.id === id ? updatedProfile : p))
      setProfiles(newProfiles)

      // 保存到存储
      await saveToStorage(newProfiles, updatedProfile)

      console.log(`[配置] 重命名: "${oldName}" → "${trimmed}"`)
      return true
    },
    [profiles, saveToStorage]
  )

  /**
   * 切换活动配置
   */
  const selectProfile = useCallback(
    async (id: string) => {
      const profile = profiles.find((p) => p.id === id)
      if (!profile) return

      setActiveProfileId(id)
      savedKeysRef.current = profile.enabledKeys
      setHasUnsavedChanges(false)
      await saveActiveIdToStorage(id)

      console.log(`[配置] 读取: "${profile.name}"`)
    },
    [profiles, saveActiveIdToStorage]
  )

  /**
   * 更新当前配置的按键
   */
  const updateEnabledKeys = useCallback(
    (keys: number[]) => {
      if (!activeProfileId) return

      setProfiles((prev) =>
        prev.map((p) => (p.id === activeProfileId ? { ...p, enabledKeys: keys } : p))
      )

      // 检测是否有未保存的更改
      const isSame =
        keys.length === savedKeysRef.current.length &&
        keys.every((k) => savedKeysRef.current.includes(k))
      setHasUnsavedChanges(!isSame)
    },
    [activeProfileId]
  )

  /**
   * 保存当前配置
   */
  const saveCurrentProfile = useCallback(async (): Promise<boolean> => {
    if (!activeProfile) return false

    await saveToStorage(profiles, activeProfile)
    savedKeysRef.current = [...activeProfile.enabledKeys]
    setHasUnsavedChanges(false)

    console.log(`[配置] 保存: "${activeProfile.name}"`)
    return true
  }, [activeProfile, profiles, saveToStorage])

  // 初始化加载（组件挂载时执行）
  useEffect(() => {
    console.log("[配置] 开始加载配置...")
    void loadProfiles()
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  return {
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
    loadProfiles,
  }
}

