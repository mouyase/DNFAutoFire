import { Keyboard } from "@/components/keyboard"
import { Header, StatusBar } from "@/components/layout"
import { useAutofire } from "@/hooks/use-autofire"
import { useAdmin } from "@/hooks/use-admin"

/**
 * 应用根组件
 */
function App() {
  const { isRunning, enabledKeys, toggleKey, clearKeys, toggle } = useAutofire()
  const { isElevated, restartAsAdmin } = useAdmin()

  return (
    <div className="min-h-screen flex flex-col">
      <Header
        isElevated={isElevated}
        isRunning={isRunning}
        onToggle={toggle}
        onRestartAsAdmin={restartAsAdmin}
      />

      <main className="flex-1 flex flex-col gap-5 p-5">
        <Keyboard enabledKeys={enabledKeys} onKeyClick={toggleKey} />
        <StatusBar isRunning={isRunning} enabledKeys={enabledKeys} onClear={clearKeys} />
      </main>
    </div>
  )
}

export default App
