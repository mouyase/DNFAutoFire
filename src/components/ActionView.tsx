import { Button } from "@heroui/react"
import { invoke } from "@tauri-apps/api/core"
import { AutoFire } from "../lib/autofire";

// 在按钮点击事件中调用
const handleStart = async () => {

// 使用示例
const autoFire = new AutoFire();
// 启动连发，监控 J, K, L, H 键
await autoFire.start([
  AutoFire.KEY_CODES.J,
  AutoFire.KEY_CODES.K,
  AutoFire.KEY_CODES.L,
  AutoFire.KEY_CODES.H,
]); 
}

const handleStop = async () => {
  try {
    await invoke('stop_auto_fire')
  } catch (error) {
    console.error(error)
  }
}

export const ActionView = () => {
  // const { activeKeys, onActiveKeysChange } = props

  return (
    <div>
      <Button onPress={handleStart}>启动</Button>
      <Button onPress={handleStop}>停止</Button>
    </div>
  )
}

