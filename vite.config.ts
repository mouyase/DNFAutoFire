import { defineConfig } from "vite"
import react from "@vitejs/plugin-react"
import path from "path"
import { resolve } from "path"

const host = process.env.TAURI_DEV_HOST

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    react({
      babel: {
        plugins: [["babel-plugin-react-compiler"]],
      },
    }),
  ],

  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },

  // 多入口配置（主窗口 + 迷你窗口）
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, "index.html"),
        mini: resolve(__dirname, "mini.html"),
      },
    },
  },

  // Vite 配置，仅用于 Tauri 开发，详见:
  // https://v2.tauri.app/start/frontend/vite/
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 忽略 Rust 源码目录
      ignored: ["**/src-tauri/**"],
    },
  },
}))
