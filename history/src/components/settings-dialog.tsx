import { useState, useCallback } from "react"
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
  DialogDescription,
} from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { ShortcutInput } from "@/components/ui/shortcut-input"
import type { AppSettings } from "@/lib/storage"

interface SettingsDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  settings: AppSettings
  onSave: (settings: AppSettings) => void
}

/**
 * 设置弹窗组件
 *
 * 允许用户配置：
 * - 启动后是否自动最小化到托盘
 * - 全局唤醒弹窗的快捷键
 * - 全局开关连发的快捷键
 */
export function SettingsDialog({ open, onOpenChange, settings, onSave }: SettingsDialogProps) {
  const [localSettings, setLocalSettings] = useState<AppSettings>(settings)

  // 当弹窗打开时同步外部设置
  const handleOpenChange = useCallback(
    (newOpen: boolean) => {
      if (newOpen) {
        setLocalSettings(settings)
      }
      onOpenChange(newOpen)
    },
    [settings, onOpenChange]
  )

  // 更新行为设置
  const updateBehavior = useCallback((key: keyof AppSettings["behavior"], value: boolean) => {
    setLocalSettings((prev) => ({
      ...prev,
      behavior: {
        ...prev.behavior,
        [key]: value,
      },
    }))
  }, [])

  // 更新快捷键设置
  const updateShortcut = useCallback((key: keyof AppSettings["shortcut"], value: string) => {
    setLocalSettings((prev) => ({
      ...prev,
      shortcut: {
        ...prev.shortcut,
        [key]: value,
      },
    }))
  }, [])

  // 保存设置
  const handleSave = useCallback(() => {
    onSave(localSettings)
    onOpenChange(false)
  }, [localSettings, onSave, onOpenChange])

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent className="w-96">
        <DialogHeader>
          <DialogTitle>设置</DialogTitle>
          <DialogDescription>配置应用行为和快捷键</DialogDescription>
        </DialogHeader>

        <div className="mt-4 space-y-4">
          {/* 行为设置 */}
          <div className="space-y-3">
            <h3 className="text-sm font-medium text-gray-700">行为</h3>

            <label className="flex items-center gap-3">
              <input
                type="checkbox"
                checked={localSettings.behavior.playSoundOnToggle}
                onChange={(e) => updateBehavior("playSoundOnToggle", e.target.checked)}
                className="h-4 w-4 rounded border-gray-300"
              />
              <span className="text-sm text-gray-600">启动/停止连发时播放音效</span>
            </label>
          </div>

          {/* 快捷键设置 */}
          <div className="space-y-3">
            <h3 className="text-sm font-medium text-gray-700">快捷键</h3>

            <div className="flex items-center justify-between">
              <span className="text-sm text-gray-600">唤醒配置弹窗</span>
              <ShortcutInput
                value={localSettings.shortcut.popupWindow}
                onChange={(value) => updateShortcut("popupWindow", value)}
              />
            </div>

            <div className="flex items-center justify-between">
              <span className="text-sm text-gray-600">全局开关连发</span>
              <ShortcutInput
                value={localSettings.shortcut.toggleAutofire}
                onChange={(value) => updateShortcut("toggleAutofire", value)}
              />
            </div>

            <p className="text-xs text-gray-400">
              点击快捷键区域，按下新的按键组合进行修改
            </p>
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            取消
          </Button>
          <Button variant="primary" onClick={handleSave}>
            保存
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
