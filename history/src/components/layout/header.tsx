import { Badge } from "./badge"
import { Button } from "./button"

interface HeaderProps {
  /** 是否以管理员权限运行 */
  isElevated: boolean
  /** 是否正在运行连发 */
  isRunning: boolean
  /** 切换连发状态 */
  onToggle: () => void
  /** 以管理员权限重启 */
  onRestartAsAdmin: () => void
}

/**
 * 标题栏组件
 */
export function Header({ isElevated, isRunning, onToggle, onRestartAsAdmin }: HeaderProps) {
  return (
    <header className="flex justify-between items-center px-5 py-3 bg-gradient-to-br from-surface to-background border-b border-white/10">
      <div className="flex items-center gap-3">
        <span className="text-lg font-semibold bg-gradient-to-r from-accent to-[#00ccff] bg-clip-text text-transparent">
          DNF 连发工具
        </span>
        {isElevated ? (
          <Badge variant="admin">管理员</Badge>
        ) : (
          <Badge variant="warning">非管理员</Badge>
        )}
      </div>

      <div className="flex gap-2.5">
        {!isElevated && (
          <Button variant="admin" onClick={onRestartAsAdmin}>
            以管理员重启
          </Button>
        )}
        <Button variant={isRunning ? "stop" : "start"} onClick={onToggle}>
          {isRunning ? "停止连发" : "启动连发"}
        </Button>
      </div>
    </header>
  )
}
