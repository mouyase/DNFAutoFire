import { useState } from "react"
import {
  GroupBox,
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
  Button,
} from "@/components/ui"
import { cn } from "@/lib/utils"
import type { Profile } from "@/types"

interface ConfigPanelProps {
  profiles: Profile[]
  activeProfileId: string | null
  hasUnsavedChanges: boolean
  isLoading: boolean
  onSelectProfile: (id: string) => Promise<void>
  onResetProfile: () => void | Promise<void>
  onSaveProfile: () => Promise<boolean>
  onDeleteProfile: (id: string) => Promise<boolean>
  onCloneProfile: () => Promise<Profile | null>
  onRenameProfile: (id: string, newName: string) => Promise<boolean>
  onCreateProfile: () => Promise<Profile>
}

type DialogType = "unsaved" | "delete" | null

interface PendingAction {
  type: "switch" | "delete"
  targetId: string
}

/**
 * 配置管理面板
 *
 * 操作说明：
 * - 点击配置项：切换配置并自动加载按键
 * - 保存：把当前键盘按键保存到配置（未保存时高亮）
 * - 重置：放弃未保存的更改，恢复到上次保存的状态
 * - 新建/复制/改名/删除：配置管理操作
 */
export function ConfigPanel({
  profiles,
  activeProfileId,
  hasUnsavedChanges,
  isLoading,
  onSelectProfile,
  onResetProfile,
  onSaveProfile,
  onDeleteProfile,
  onCloneProfile,
  onRenameProfile,
  onCreateProfile,
}: ConfigPanelProps) {
  const [editingId, setEditingId] = useState<string | null>(null)
  const [dialogType, setDialogType] = useState<DialogType>(null)
  const [pendingAction, setPendingAction] = useState<PendingAction | null>(null)

  const activeProfile = profiles?.find((p) => p.id === activeProfileId)

  // 处理选择配置（切换并自动加载按键）
  const handleSelect = async (id: string) => {
    if (id === activeProfileId) return

    // 如果有未保存的更改，先弹出确认对话框
    if (hasUnsavedChanges) {
      setPendingAction({ type: "switch", targetId: id })
      setDialogType("unsaved")
      return
    }

    await onSelectProfile(id)
  }

  // 处理双击进入编辑模式
  const handleDoubleClick = (id: string) => {
    setEditingId(id)
  }

  // 处理重命名确认
  const handleRenameConfirm = async (id: string, newName: string) => {
    const trimmed = newName.trim()
    const profile = profiles?.find((p) => p.id === id)

    if (trimmed && profile && trimmed !== profile.name) {
      await onRenameProfile(id, trimmed)
    }
    setEditingId(null)
  }

  // 处理重命名键盘事件
  const handleRenameKeyDown = (e: React.KeyboardEvent<HTMLInputElement>, id: string) => {
    if (e.key === "Enter") {
      void handleRenameConfirm(id, e.currentTarget.value)
    } else if (e.key === "Escape") {
      setEditingId(null)
    }
  }

  // 处理新建
  const handleCreate = async () => {
    await onCreateProfile()
  }

  // 处理复制
  const handleClone = async () => {
    await onCloneProfile()
  }

  // 处理删除（弹出确认对话框）
  const handleDeleteClick = () => {
    if (!profiles || profiles.length <= 1 || !activeProfileId) return
    setPendingAction({ type: "delete", targetId: activeProfileId })
    setDialogType("delete")
  }

  // 确认删除
  const handleDeleteConfirm = async () => {
    if (pendingAction?.type === "delete") {
      await onDeleteProfile(pendingAction.targetId)
    }
    setDialogType(null)
    setPendingAction(null)
  }

  // 处理重置（放弃未保存的更改）
  const handleReset = () => {
    void onResetProfile()
  }

  // 处理保存
  const handleSave = async () => {
    await onSaveProfile()
  }

  // 处理未保存对话框 - 保存并继续
  const handleSaveAndContinue = async () => {
    await onSaveProfile()
    if (pendingAction?.type === "switch") {
      await onSelectProfile(pendingAction.targetId)
    }
    setDialogType(null)
    setPendingAction(null)
  }

  // 处理未保存对话框 - 不保存继续
  const handleDiscardAndContinue = async () => {
    if (pendingAction?.type === "switch") {
      await onSelectProfile(pendingAction.targetId)
    }
    setDialogType(null)
    setPendingAction(null)
  }

  // 处理对话框关闭
  const handleDialogClose = () => {
    setDialogType(null)
    setPendingAction(null)
  }

  if (isLoading) {
    return (
      <GroupBox title="配置管理" className="h-full">
        <div className="flex items-center justify-center h-full text-gray-400 text-xs">
          加载中...
        </div>
      </GroupBox>
    )
  }

  return (
    <>
      <GroupBox title="配置管理" className="h-full">
        <div className="flex gap-2 h-full min-h-0">
          {/* 左侧：配置列表 */}
          <div className="flex-1 min-w-0 border border-gray-200 rounded bg-gray-50/50 overflow-y-auto [scrollbar-gutter:stable]">
            <div className="flex flex-col gap-1 py-2 pl-2 pr-0.5">
              {(profiles ?? []).map((profile) => (
                <div key={profile.id} className="shrink-0">
                  {editingId === profile.id ? (
                    <input
                      type="text"
                      defaultValue={profile.name}
                      onKeyDown={(e) => handleRenameKeyDown(e, profile.id)}
                      onBlur={(e) => void handleRenameConfirm(profile.id, e.target.value)}
                      className="w-full h-7 px-2 text-xs border border-blue-400 rounded bg-white focus:outline-none"
                      autoFocus
                    />
                  ) : (
                    <button
                      type="button"
                      onClick={() => void handleSelect(profile.id)}
                      onDoubleClick={() => handleDoubleClick(profile.id)}
                      className={cn(
                        "w-full h-7 px-2 text-xs text-left rounded transition-colors truncate",
                        profile.id === activeProfileId
                          ? "bg-blue-500 text-white"
                          : "bg-white text-gray-700 hover:bg-gray-100 border border-gray-200"
                      )}
                    >
                      {profile.name}
                    </button>
                  )}
                </div>
              ))}
            </div>
          </div>

          {/* 右侧：操作按钮 */}
          <div className="w-12 shrink-0 flex flex-col gap-1">
            {/* 保存按钮：未保存时高亮 */}
            <ConfigButton
              onClick={() => void handleSave()}
              variant={hasUnsavedChanges ? "warning" : "default"}
            >
              保存
            </ConfigButton>
            <ConfigButton onClick={handleReset} disabled={!hasUnsavedChanges}>
              重置
            </ConfigButton>
            <ConfigButton onClick={() => void handleCreate()}>新建</ConfigButton>
            <ConfigButton onClick={() => void handleClone()}>复制</ConfigButton>
            <ConfigButton onClick={() => activeProfileId && setEditingId(activeProfileId)}>
              改名
            </ConfigButton>
            <ConfigButton
              onClick={handleDeleteClick}
              disabled={!profiles || profiles.length <= 1}
              variant="danger"
            >
              删除
            </ConfigButton>
          </div>
        </div>
      </GroupBox>

      {/* 未保存更改对话框 */}
      <Dialog open={dialogType === "unsaved"} onOpenChange={handleDialogClose}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>未保存的更改</DialogTitle>
            <DialogDescription>
              当前配置 "{activeProfile?.name}" 有未保存的更改。是否保存？
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="ghost" onClick={handleDialogClose}>
              取消
            </Button>
            <Button variant="default" onClick={() => void handleDiscardAndContinue()}>
              不保存
            </Button>
            <Button variant="primary" onClick={() => void handleSaveAndContinue()}>
              保存
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* 删除确认对话框 */}
      <Dialog open={dialogType === "delete"} onOpenChange={handleDialogClose}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>确认删除</DialogTitle>
            <DialogDescription>
              确定要删除配置 "{activeProfile?.name}" 吗？此操作不可撤销。
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="ghost" onClick={handleDialogClose}>
              取消
            </Button>
            <Button variant="destructive" onClick={() => void handleDeleteConfirm()}>
              删除
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  )
}

/** 配置面板按钮 */
interface ConfigButtonProps {
  children: React.ReactNode
  onClick?: () => void
  disabled?: boolean
  variant?: "default" | "primary" | "warning" | "danger"
}

function ConfigButton({ children, onClick, disabled, variant = "default" }: ConfigButtonProps) {
  const baseStyles =
    "h-6 px-2 text-xs rounded transition-colors font-medium disabled:opacity-40 disabled:cursor-not-allowed"

  const variantStyles = {
    default: "bg-gray-100 text-gray-700 hover:bg-gray-200 border border-gray-200",
    primary: "bg-blue-500 text-white hover:bg-blue-600",
    warning: "bg-amber-500 text-white hover:bg-amber-600",
    danger: "bg-white text-red-500 hover:bg-red-50 border border-red-200 hover:border-red-300",
  }

  return (
    <button
      type="button"
      onClick={onClick}
      disabled={disabled}
      className={cn(baseStyles, variantStyles[variant])}
    >
      {children}
    </button>
  )
}
