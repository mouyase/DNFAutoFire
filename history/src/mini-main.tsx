import { StrictMode } from "react"
import { createRoot } from "react-dom/client"
import "./index.css"
import MiniApp from "./mini-app"

const rootElement = document.getElementById("root")!

createRoot(rootElement).render(
  <StrictMode>
    <MiniApp />
  </StrictMode>
)
