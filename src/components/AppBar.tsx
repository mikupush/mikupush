import SelectedServer from '@/components/SelectedServer'
import { Button } from '@/components/ui/button'
import { selectFiles } from '@/helpers/file'
import { UploadIcon, SidebarOpenIcon, Minus, Square, X } from 'lucide-react'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useEffect, useState } from 'react'

export default function AppBar() {
  return (
    <div className="flex place-content-between">
      <div className="flex items-center space-x-[10px] m-[15px]">
        <SelectedServer />
        <Button variant="outline" size="icon">
          <SidebarOpenIcon />
        </Button>
        <Button
          variant="outline" 
          size="icon"
          onClick={() => selectFiles()}
        >
          <UploadIcon />
        </Button>
      </div>
      <WindowControls />
    </div>
  )
}

export function WindowControls() {
  const [isMaximized, setIsMaximized] = useState(false)

  useEffect(() => {
    const window = getCurrentWindow()
    window.isMaximized().then((state) => setIsMaximized(state))
  }, [])
  
  const toggleMaximize = async () => {
    const window = getCurrentWindow()
    await window.toggleMaximize()
    setIsMaximized(await window.isMaximized());
  }

  const minimize = async () => {
    const window = getCurrentWindow()
    await window.minimize()
  }

  const close = async () => {
    const window = getCurrentWindow()
    await window.close()
  }

  return (
    <div 
      className="flex items-center justify-between h-8 select-none"
      style={{ fontFamily: 'Segoe UI, sans-serif' }}
      data-tauri-drag-region
    >
      <div className="flex">
        <button
          onClick={minimize}
          className="w-12 h-8 flex items-center justify-center hover:bg-gray-200 transition-colors text-[10px]"
          style={{fontFamily: 'Segoe Fluent Icons'}}
        >
          {'\ue921'}
        </button>

        <button
          onClick={toggleMaximize}
          className="w-12 h-8 flex items-center justify-center hover:bg-gray-200 transition-colors text-[10px]"
          style={{fontFamily: 'Segoe Fluent Icons'}}
        >
          {isMaximized ? '\ue923' : '\ue922'}
        </button>
        
        <button
          onClick={close}
          className="w-12 h-8 flex items-center justify-center hover:bg-red-600 hover:text-white transition-colors text-[10px]"
          style={{fontFamily: 'Segoe Fluent Icons'}}
        >
          {'\ue8bb'}
        </button>
      </div>
    </div>
  )
}