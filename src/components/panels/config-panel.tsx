import { useState } from "react"
import { GroupBox } from "@/components/ui"

interface ConfigPanelProps {
  configs?: string[]
  selectedConfig?: string
  onSelectConfig?: (name: string) => void
  onLoadConfig?: () => void
  onSaveConfig?: () => void
  onDeleteConfig?: () => void
  onCloneConfig?: () => void
  onRenameConfig?: (newName: string) => void
}

/**
 * 配置管理面板
 * 左侧配置列表，右侧操作按钮
 */
export function ConfigPanel({
  configs = ["默认", "临时", "测试"],
  selectedConfig: initialSelected = "默认",
  onSelectConfig,
  onLoadConfig,
  onSaveConfig,
  onDeleteConfig,
  onCloneConfig,
  onRenameConfig,
}: ConfigPanelProps) {
  const [selected, setSelected] = useState(initialSelected)
  const [isRenaming, setIsRenaming] = useState(false)
  const [newName, setNewName] = useState("")

  const handleSelect = (name: string) => {
    setSelected(name)
    onSelectConfig?.(name)
  }

  const handleStartRename = () => {
    setNewName(selected)
    setIsRenaming(true)
  }

  const handleConfirmRename = () => {
    if (newName.trim() && newName !== selected) {
      onRenameConfig?.(newName.trim())
      setSelected(newName.trim())
    }
    setIsRenaming(false)
  }

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      handleConfirmRename()
    } else if (e.key === "Escape") {
      setIsRenaming(false)
    }
  }

  return (
    <GroupBox title="配置管理" className="h-full">
      <div className="flex gap-3 h-full">
        {/* 左侧：配置列表 */}
        <div className="w-16 shrink-0 border border-gray-300 rounded bg-white overflow-auto">
          {configs.map((config) => (
            <div
              key={config}
              onClick={() => handleSelect(config)}
              onDoubleClick={onLoadConfig}
              className={`
                px-2 py-1 text-xs cursor-pointer truncate
                ${config === selected
                  ? "bg-blue-500 text-white"
                  : "hover:bg-blue-50 border-b border-gray-100 last:border-b-0"
                }
              `}
            >
              {config}
            </div>
          ))}
        </div>

        {/* 右侧：操作区 */}
        <div className="flex-1 flex flex-col gap-2 min-w-0">
          {/* 当前配置名 / 重命名输入 */}
          {isRenaming ? (
            <div className="flex gap-1">
              <input
                type="text"
                value={newName}
                onChange={(e) => setNewName(e.target.value)}
                onKeyDown={handleKeyDown}
                className="flex-1 h-6 px-2 text-xs border border-blue-500 rounded focus:outline-none min-w-0"
                placeholder="新名称"
                autoFocus
              />
              <button
                type="button"
                onClick={handleConfirmRename}
                className="h-6 px-2 text-xs bg-blue-500 text-white rounded hover:bg-blue-600"
              >
                确定
              </button>
              <button
                type="button"
                onClick={() => setIsRenaming(false)}
                className="h-6 px-2 text-xs text-gray-500 bg-gray-100 rounded hover:bg-gray-200"
              >
                取消
              </button>
            </div>
          ) : (
            <div className="h-6 px-2 flex items-center text-xs bg-gray-50 rounded border border-gray-200 truncate">
              <span className="text-gray-400 mr-1">当前:</span>
              <span className="font-medium">{selected}</span>
            </div>
          )}

          {/* 操作按钮 - 紧凑布局 */}
          <div className="grid grid-cols-2 gap-1">
            <button
              type="button"
              onClick={onLoadConfig}
              className="h-6 text-xs text-gray-700 bg-gray-100 rounded hover:bg-gray-200 transition-colors"
            >
              读取
            </button>
            <button
              type="button"
              onClick={onSaveConfig}
              className="h-6 text-xs text-white bg-blue-500 rounded hover:bg-blue-600 transition-colors"
            >
              保存
            </button>
            <button
              type="button"
              onClick={handleStartRename}
              disabled={isRenaming}
              className="h-6 text-xs text-gray-700 bg-gray-100 rounded hover:bg-gray-200 transition-colors disabled:opacity-50"
            >
              重命名
            </button>
            <button
              type="button"
              onClick={onCloneConfig}
              className="h-6 text-xs text-gray-700 bg-gray-100 rounded hover:bg-gray-200 transition-colors"
            >
              复制
            </button>
            <button
              type="button"
              onClick={onDeleteConfig}
              disabled={configs.length <= 1}
              className="col-span-2 h-6 text-xs text-red-500 border border-red-200 rounded hover:bg-red-50 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              删除
            </button>
          </div>
        </div>
      </div>
    </GroupBox>
  )
}
