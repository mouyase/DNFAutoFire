import { StrictMode } from "react"
import { createRoot } from "react-dom/client"
import "./index.css"
import App from "./app"
import { isMockMode } from "./lib/tauri"

// 浏览器模式下添加特殊样式类，模拟 Tauri 窗口尺寸
const rootElement = document.getElementById("root")!
if (isMockMode()) {
  rootElement.classList.add("browser-mode")
  console.log("[Mock Mode] 浏览器开发模式已启用，窗口尺寸固定为 910x500")
}

createRoot(rootElement).render(
  <StrictMode>
    <App />
  </StrictMode>
)
