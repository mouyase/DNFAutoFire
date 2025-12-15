import { StrictMode } from "react"
import { createRoot } from "react-dom/client"
import "./index.css"
import App from "./app"
import { isMockMode, tauriCommands } from "./lib/tauri"
import { TooltipProvider } from "./components/ui"
import { SettingsProvider, loadSettings } from "./contexts/settings-context"

// 浏览器模式下添加特殊样式类，模拟 Tauri 窗口尺寸
const rootElement = document.getElementById("root")!
if (isMockMode()) {
  rootElement.classList.add("browser-mode")
  console.log("[Mock Mode] 浏览器开发模式已启用，窗口尺寸固定为 910x500")
}

// 应用初始化
async function init() {
  // 预加载设置
  const settings = await loadSettings()

  // 同步快捷键配置到后端
  if (!isMockMode()) {
    await tauriCommands.updateShortcuts(
      settings.shortcut.popupWindow,
      settings.shortcut.toggleAutofire
    )
  }

  // 渲染应用
  createRoot(rootElement).render(
    <StrictMode>
      <SettingsProvider initialSettings={settings}>
        <TooltipProvider delayDuration={0}>
          <App />
        </TooltipProvider>
      </SettingsProvider>
    </StrictMode>
  )
}

void init()
