import type { Profile } from "@/types"

interface ProfileSelectorProps {
  profiles: Profile[]
  activeProfileId: string | null
  selectedIndex: number
  onSelect: (profileId: string) => void | Promise<void>
  onHover: (index: number) => void
}

/**
 * 配置选择器组件
 *
 * 显示配置列表，支持：
 * - 鼠标点击选择
 * - 鼠标悬停高亮
 * - 键盘导航
 */
export function ProfileSelector({
  profiles,
  activeProfileId,
  selectedIndex,
  onSelect,
  onHover,
}: ProfileSelectorProps) {
  if (profiles.length === 0) {
    return (
      <div className="flex h-full items-center justify-center text-text-secondary">
        暂无配置
      </div>
    )
  }

  return (
    <div className="space-y-1">
      {profiles.map((profile, index) => {
        const isActive = profile.id === activeProfileId
        const isSelected = index === selectedIndex

        return (
          <button
            key={profile.id}
            onClick={() => void onSelect(profile.id)}
            onMouseEnter={() => onHover(index)}
            className={`flex w-full items-center justify-between rounded-md px-3 py-2 text-left text-sm transition-colors ${
              isSelected
                ? "bg-blue-100 text-blue-900"
                : "hover:bg-gray-100"
            }`}
          >
            <span className="flex items-center gap-2">
              {isActive && (
                <span className="inline-block h-2 w-2 rounded-full bg-blue-500" />
              )}
              <span className={isActive ? "font-medium" : ""}>{profile.name}</span>
            </span>
            <span className="text-xs text-text-secondary">
              {profile.enabledKeys.length} 个按键
            </span>
          </button>
        )
      })}
    </div>
  )
}
