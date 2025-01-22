import { Button } from '@heroui/button'

type KeyButtonProps = {
  label: string
  className?: string
  isActive?: boolean
  id?: string
}

const KeyButton = ({
  label,
  className = '',
  isActive = false,
  id = label.toLowerCase(),
}: KeyButtonProps) => {
  return (
    <Button
      onPress={() => console.log(id)}
      className={`h-9 w-9 min-w-9 rounded-[4px] ${className}`}
      color={isActive ? 'success' : 'default'}>
      {label}
    </Button>
  )
}

type KeyboardViewProps = {
  activeKeys: string[]
  onActiveKeysChange: (keys: string[]) => void
}

export const KeyboardView = (props: KeyboardViewProps) => {
  const { activeKeys, onActiveKeysChange } = props

  return (
    <div className='flex flex-1 p-2 justify-between'>
      <div className='flex flex-col w-[600px]'>
        <div className='flex justify-between'>
          <div className='flex'>
            <KeyButton label='ESC' />
          </div>
          <div className='flex gap-1'>
            <KeyButton label='F1' />
            <KeyButton label='F2' />
            <KeyButton label='F3' />
            <KeyButton label='F4' />
          </div>
          <div className='flex gap-1'>
            <KeyButton label='F5' />
            <KeyButton label='F6' />
            <KeyButton label='F7' />
            <KeyButton label='F8' />
          </div>
          <div className='flex gap-1'>
            <KeyButton label='F9' />
            <KeyButton label='F10' />
            <KeyButton label='F11' />
            <KeyButton label='F12' />
          </div>
        </div>
        <div className='flex mt-1 gap-1'>
          <KeyButton label='`' />
          <KeyButton label='1' />
          <KeyButton label='2' />
          <KeyButton label='3' />
          <KeyButton label='4' />
          <KeyButton label='5' />
          <KeyButton label='6' />
          <KeyButton label='7' />
          <KeyButton label='8' />
          <KeyButton label='9' />
          <KeyButton label='0' />
          <KeyButton label='-' />
          <KeyButton label='=' />
          <KeyButton label='Backspace' className='flex flex-1 text-xs' />
        </div>
        <div className='flex mt-1 gap-1'>
          <KeyButton label='Tab' className='flex flex-1' />
          <KeyButton label='Q' />
          <KeyButton label='W' />
          <KeyButton label='E' />
          <KeyButton label='R' />
          <KeyButton label='T' />
          <KeyButton label='Y' />
          <KeyButton label='U' />
          <KeyButton label='I' />
          <KeyButton label='O' />
          <KeyButton label='P' />
          <KeyButton label='[' />
          <KeyButton label=']' />
          <KeyButton label='\' className='flex flex-1' />
        </div>
        <div className='flex mt-1 gap-1'>
          <KeyButton label='Caps' className='flex flex-1' />
          <KeyButton label='A' />
          <KeyButton label='S' />
          <KeyButton label='D' />
          <KeyButton label='F' />
          <KeyButton label='G' />
          <KeyButton label='H' />
          <KeyButton label='J' />
          <KeyButton label='K' />
          <KeyButton label='L' />
          <KeyButton label=';' />
          <KeyButton label="'" />
          <KeyButton label='Enter' className='flex flex-1' />
        </div>
        <div className='flex mt-1 gap-1'>
          <KeyButton label='Shift' className='flex flex-1' />
          <KeyButton label='Z' />
          <KeyButton label='X' />
          <KeyButton label='C' />
          <KeyButton label='V' />
          <KeyButton label='B' />
          <KeyButton label='N' />
          <KeyButton label='M' />
          <KeyButton label=',' />
          <KeyButton label='.' />
          <KeyButton label='/' />
          <KeyButton label='Shift' className='flex flex-1' />
        </div>
        <div className='flex mt-1 gap-1'>
          <KeyButton label='Ctrl' className='flex flex-1' />
          <KeyButton label='Win' className='flex flex-1' />
          <KeyButton label='Alt' className='flex flex-1' />
          <KeyButton label='Space' className='w-[200px]' />
          <KeyButton label='Alt' className='flex flex-1' />
          <KeyButton label='Win' className='flex flex-1' />
          <KeyButton label='Menu' className='flex flex-1' />
          <KeyButton label='Ctrl' className='flex flex-1' />
        </div>
      </div>
      <div className='flex flex-col justify-between'>
        <div className='flex flex-col'>
          <div className='flex gap-1'>
            <KeyButton label='PrtSc' className='text-[10px]' />
            <KeyButton label='ScrLk' className='text-[10px]' />
            <KeyButton label='Pause' className='text-[10px]' />
          </div>
          <div className='flex mt-1 gap-1'>
            <KeyButton label='Insert' className='text-[10px]' />
            <KeyButton label='Home' className='text-[10px]' />
            <KeyButton label='PgUp' className='text-[10px]' />
          </div>
          <div className='flex mt-1 gap-1'>
            <KeyButton label='Delete' className='text-[10px]' />
            <KeyButton label='End' className='text-[10px]' />
            <KeyButton label='PgDn' className='text-[10px]' />
          </div>
        </div>
        <div className='flex flex-col'>
          <div className='flex justify-center'>
            <KeyButton label='↑' />
          </div>
          <div className='flex mt-1 gap-1'>
            <KeyButton label='←' />
            <KeyButton label='↓' />
            <KeyButton label='→' />
          </div>
        </div>
      </div>
      <div className='flex flex-col'>
        <div className='flex flex-1'></div>
        <div className='flex gap-1'>
          <div className='flex flex-col'>
            <div className='flex gap-1'>
              <KeyButton label='Num' className='text-xs' />
              <KeyButton label='/' />
              <KeyButton label='*' />
            </div>
            <div className='flex mt-1 gap-1'>
              <KeyButton label='7' />
              <KeyButton label='8' />
              <KeyButton label='9' />
            </div>
            <div className='flex mt-1 gap-1'>
              <KeyButton label='4' />
              <KeyButton label='5' />
              <KeyButton label='6' />
            </div>
            <div className='flex mt-1 gap-1'>
              <KeyButton label='1' />
              <KeyButton label='2' />
              <KeyButton label='3' />
            </div>
            <div className='flex mt-1 gap-1'>
              <KeyButton label='0' className='flex flex-1' />
              <KeyButton label='.' />
            </div>
          </div>
          <div className='flex flex-col justify-end gap-1'>
            <KeyButton label='-' />
            <KeyButton label='+' className='flex flex-1' />
            <KeyButton label='Enter' className='flex flex-1 text-xs' />
          </div>
        </div>
      </div>
    </div>
  )
}

export default KeyboardView
