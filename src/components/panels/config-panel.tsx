import { GroupBox } from "@/components/ui"
import { Button } from "@/components/layout"

interface ConfigPanelProps {
  configs?: string[]
  selectedConfig?: string
  onSelectConfig?: (name: string) => void
  onLoadConfig?: () => void
  onSaveConfig?: () => void
  onCloneConfig?: () => void
  onDeleteConfig?: () => void
}

/**
 * 配置设置面板
 */
export function ConfigPanel({
  configs = ["默认", "临时", "测试"],
  selectedConfig = "测试",
}: ConfigPanelProps) {
  return (
    <GroupBox title="配置设置 - 【双击读取配置】" className="h-full">
      <div className="flex gap-3 h-full">
        {/* 左侧：配置列表 */}
        <div className="w-[70px] shrink-0 border border-gray-300 bg-white rounded overflow-auto">
          {configs.map((config) => (
            <div
              key={config}
              className={`px-2 py-0.5 text-xs cursor-pointer ${
                config === selectedConfig
                  ? "bg-blue-500 text-white"
                  : "hover:bg-blue-50"
              }`}
            >
              {config}
            </div>
          ))}
        </div>

        {/* 右侧：配置操作 */}
        <div className="flex-1 flex flex-col gap-2">
          {/* 配置名称 */}
          <div className="flex items-center gap-2">
            <span className="text-xs text-text-secondary whitespace-nowrap">当前配置名称</span>
            <input
              type="text"
              className="flex-1 h-6 px-2 text-xs border border-gray-300 rounded focus:outline-none focus:border-blue-500 min-w-0"
              defaultValue={selectedConfig}
            />
          </div>

          {/* 配置按钮 */}
          <div className="grid grid-cols-2 gap-1.5">
            <Button variant="secondary" size="sm">读取配置</Button>
            <Button variant="secondary" size="sm">保存配置</Button>
            <Button variant="secondary" size="sm">克隆配置</Button>
            <Button variant="secondary" size="sm">删除配置</Button>
          </div>

          {/* 快速切换热键 */}
          <div className="mt-auto">
            <div className="text-xs text-text-secondary mb-1">快速切换热键</div>
            <input
              type="text"
              className="w-full h-6 px-2 text-xs border border-gray-300 rounded focus:outline-none focus:border-blue-500"
              defaultValue="Alt + `"
              readOnly
            />
          </div>
        </div>
      </div>
    </GroupBox>
  )
}
