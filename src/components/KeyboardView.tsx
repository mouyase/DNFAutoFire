import { Button } from '@heroui/button'

interface KeyProps {
  label: string
  className?: string
}

const Key = ({ label, className = '' }: KeyProps) => {
  return (
    <Button className={'h-9 w-9 min-w-9'} color='default'>
      {label}
    </Button>
  )
}

export const KeyboardView = () => {
  return (
    <div>
      {/* 第二行: 主键盘区域 + 功能区域 + 小键盘区域 */}
      <div className='flex justify-between'>
        <div className='flex flex-col gap-2'>
          {/* 第一行: 功能键区域 */}
          <div className='flex'>
            <div className='flex gap-[2px]'>
              <Key label='ESC' />
              <div className='w-6' />
              <div className='flex gap-[2px]'>
                <Key label='F1' />
                <Key label='F2' />
                <Key label='F3' />
                <Key label='F4' />
              </div>
              <div className='w-4' />
              <div className='flex gap-[2px]'>
                <Key label='F5' />
                <Key label='F6' />
                <Key label='F7' />
                <Key label='F8' />
              </div>
              <div className='w-4' />
              <div className='flex gap-[2px]'>
                <Key label='F9' />
                <Key label='F10' />
                <Key label='F11' />
                <Key label='F12' />
              </div>
            </div>
          </div>
          {/* 主键盘区域 */}
          <div className='flex flex-col gap-[2px] w-[580px]'>
            <div className='flex gap-[2px] justify-between'>
              <Key label='`' />
              <Key label='1' />
              <Key label='2' />
              <Key label='3' />
              <Key label='4' />
              <Key label='5' />
              <Key label='6' />
              <Key label='7' />
              <Key label='8' />
              <Key label='9' />
              <Key label='0' />
              <Key label='-' />
              <Key label='=' />
              <Key label='Backspace' className='w-[72px]' />
            </div>
            <div className='flex gap-[2px] justify-between'>
              <Key label='Tab' className='w-[54px]' />
              <Key label='Q' />
              <Key label='W' />
              <Key label='E' />
              <Key label='R' />
              <Key label='T' />
              <Key label='Y' />
              <Key label='U' />
              <Key label='I' />
              <Key label='O' />
              <Key label='P' />
              <Key label='[' />
              <Key label=']' />
              <Key label='\' className='w-[54px]' />
            </div>
            <div className='flex gap-[2px] justify-between'>
              <Key label='Caps' className='w-[63px]' />
              <Key label='A' />
              <Key label='S' />
              <Key label='D' />
              <Key label='F' />
              <Key label='G' />
              <Key label='H' />
              <Key label='J' />
              <Key label='K' />
              <Key label='L' />
              <Key label=';' />
              <Key label="'" />
              <Key label='Enter' className='w-[81px]' />
            </div>
            <div className='flex gap-[2px] justify-between'>
              <Key label='Shift' className='w-[81px]' />
              <Key label='Z' />
              <Key label='X' />
              <Key label='C' />
              <Key label='V' />
              <Key label='B' />
              <Key label='N' />
              <Key label='M' />
              <Key label=',' />
              <Key label='.' />
              <Key label='/' />
              <Key label='Shift' className='w-[99px]' />
            </div>
            <div className='flex gap-[2px] justify-between'>
              <Key label='Ctrl' className='w-[45px]' />
              <Key label='Win' className='w-[45px]' />
              <Key label='Alt' className='w-[45px]' />
              <Key label='' className='w-[225px]' />
              <Key label='Alt' className='w-[45px]' />
              <Key label='Win' className='w-[45px]' />
              <Key label='Menu' className='w-[45px]' />
              <Key label='Ctrl' className='w-[45px]' />
            </div>
          </div>

          {/* 功能区域 */}
          <div className='flex gap-6'>
            {/* 编辑键区域 + 截图等功能键 */}
            <div className='flex flex-col gap-[2px]'>
              <div className='flex gap-[2px]'>
                <Key label='PrtSc' />
                <Key label='ScrLk' />
                <Key label='Pause' />
              </div>
              <div className='flex gap-[2px]'>
                <Key label='Insert' />
                <Key label='Home' />
                <Key label='PgUp' />
              </div>
              <div className='flex gap-[2px]'>
                <Key label='Delete' />
                <Key label='End' />
                <Key label='PgDn' />
              </div>
              <div className='h-9' />
              <div className='flex justify-center'>
                <Key label='↑' />
              </div>
              <div className='flex gap-[2px]'>
                <Key label='←' />
                <Key label='↓' />
                <Key label='→' />
              </div>
            </div>

            {/* 小键盘区域 */}
            <div className='flex flex-col gap-[2px]'>
              <div className='flex gap-[2px]'>
                <Key label='Num' />
                <Key label='/' />
                <Key label='*' />
                <Key label='-' />
              </div>
              <div className='flex'>
                <div className='flex flex-col gap-[2px]'>
                  <div className='flex gap-[2px]'>
                    <Key label='7' />
                    <Key label='8' />
                    <Key label='9' />
                  </div>
                  <div className='flex gap-[2px]'>
                    <Key label='4' />
                    <Key label='5' />
                    <Key label='6' />
                  </div>
                  <div className='flex gap-[2px]'>
                    <Key label='1' />
                    <Key label='2' />
                    <Key label='3' />
                  </div>
                  <div className='flex gap-[2px]'>
                    <Key label='0' className='w-[74px]' />
                    <Key label='.' />
                  </div>
                </div>
                <div className='flex flex-col gap-[2px] ml-[2px]'>
                  <Key label='+' className='h-[74px]' />
                  <Key label='Enter' className='h-[74px]' />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
