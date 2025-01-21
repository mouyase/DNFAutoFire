import { ActionView } from './components/ActionView'
import { KeyboardView } from './components/KeyboardView'

export default function App() {
  return (
    <div className='flex flex-col justify-center'>
      <KeyboardView activeKeys={[]} onActiveKeysChange={() => {}} />
      <ActionView />
    </div>
  )
}
