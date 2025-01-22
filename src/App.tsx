import { ActionView } from './components/ActionView'
import { ConfigsView } from './components/ConfigsView'
import { KeyboardView } from './components/KeyboardView'

export default function App() {
  return (
    <div className='flex flex-col justify-center'>
      <KeyboardView activeKeys={[]} onActiveKeysChange={() => {}} />
      <div className='flex flex-1 p-2'>
        <ConfigsView configs={['config1', 'config2', 'config3']} />
        <div className='flex flex-1 justify-end items-end'>
          <ActionView />
        </div>
      </div>
    </div>
  )
}
