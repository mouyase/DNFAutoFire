import type { ReactNode } from "react"

interface AppLayoutProps {
  /** 顶部区域（键盘设置） */
  top: ReactNode
  /** 底部左侧区域（配置设置） */
  bottomLeft: ReactNode
  /** 底部中间区域（其他功能） */
  bottomCenter: ReactNode
  /** 底部右侧区域（操作按钮） */
  bottomRight: ReactNode
}

/**
 * 应用主布局组件
 * 提供插槽式的布局结构，各区域可插入不同组件
 */
export function AppLayout({
  top,
  bottomLeft,
  bottomCenter,
  bottomRight,
}: AppLayoutProps) {
  return (
    <div className="h-full flex flex-col bg-bg p-2.5 gap-2.5 overflow-hidden">
      {/* 顶部区域 */}
      <div className="shrink-0">{top}</div>

      {/* 底部区域 */}
      <div className="flex gap-2 flex-1 min-h-0">
        {/* 左侧 - 配置设置 */}
        <div className="shrink-0" style={{ width: 280 }}>{bottomLeft}</div>

        {/* 中间 - 其他功能 */}
        <div className="shrink-0" style={{ width: 140 }}>{bottomCenter}</div>

        {/* 右侧 - 操作按钮 */}
        <div className="shrink-0" style={{ width: 76 }}>{bottomRight}</div>
      </div>
    </div>
  )
}
