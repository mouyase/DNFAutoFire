import { useCallback } from "react"
import { AppLayout } from "@/components/layout"
import {
  KeyboardPanel,
  ConfigPanel,
  FeaturesPanel,
  ActionPanel,
} from "@/components/panels"
import { useAutofire } from "@/hooks/use-autofire"


/**
 * 应用根组件
 * 使用插槽式布局，各区域插入独立的面板组件
 */
function App() {
  const { isRunning, enabledKeys, toggleKey, clearKeys, toggle } = useAutofire()

  // 包装异步函数为 void 返回
  const handleKeyClick = useCallback((vk: number) => {
    void toggleKey(vk)
  }, [toggleKey])

  const handleClearKeys = useCallback(() => {
    void clearKeys()
  }, [clearKeys])

  const handleToggle = useCallback(() => {
    void toggle()
  }, [toggle])

  return (
    <AppLayout
      // 顶部：键盘设置面板
      top={
        <KeyboardPanel
          enabledKeys={enabledKeys}
          onKeyClick={handleKeyClick}
          onClearKeys={handleClearKeys}
        />
      }
      // 底部左侧：配置设置面板
      bottomLeft={<ConfigPanel />}
      // 底部中间：其他功能面板
      bottomCenter={<FeaturesPanel />}
      // 底部右侧：操作按钮面板
      bottomRight={
        <ActionPanel
          isRunning={isRunning}
          onToggle={handleToggle}
        />
      }
    />
  )
}

export default App
