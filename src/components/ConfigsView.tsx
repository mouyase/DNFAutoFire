import { Button, Input, Listbox, ListboxItem } from '@heroui/react'
import { useMemo, useState } from 'react'

type ConfigsViewViewProps = {
  nowConfig: string
  configs: string[]
}

export const ConfigsView = (props: ConfigsViewViewProps) => {
  const { configs, nowConfig } = props
  const [selectedKeys, setSelectedKeys] = useState(new Set([nowConfig]))

  const nowSelectValue = useMemo(() => {
    return selectedKeys.values().next().value
  }, [selectedKeys])

  const items = useMemo(
    () => configs.map((item) => ({ label: item, key: item })),
    [configs],
  )
  return (
    <div className='flex'>
      <div className='w-[240px] border rounded-md'>
        <Listbox
          aria-label='configs'
          disallowEmptySelection={true}
          selectedKeys={selectedKeys}
          selectionMode='single'
          items={items}
          onSelectionChange={(keys) => {
            setSelectedKeys(keys as Set<string>)
          }}>
          {(item) => <ListboxItem key={item.key}>{item.label}</ListboxItem>}
        </Listbox>
      </div>
      <div className='flex flex-col ml-3 w-[120px] gap-2'>
        <Input value={nowSelectValue} />
        <Button>读取</Button>
        <Button>保存</Button>
        <Button>删除</Button>
      </div>
    </div>
  )
}
