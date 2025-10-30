/**
 * Miku Push! is a simple, lightweight, and open-source WeTransfer alternative for desktop.
 * Copyright (C) 2025  Miku Push! Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import { Button } from '@/components/ui/button'
import { selectFiles } from '@/helpers/file'
import { UploadIcon, SidebarOpenIcon } from 'lucide-react'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { platform } from '@tauri-apps/plugin-os'
import { useEffect, useState } from 'react'
import { SidebarTrigger, useSidebar } from '@/components/ui/sidebar.tsx'
import { Separator } from '@/components/ui/separator.tsx'
import { SelectedServerDropdown } from '@/components/SelectedServer.tsx'

export default function AppTitleBar() {
  const { open, isMobile } = useSidebar()

  const leftSafeArea = ((!open || open && isMobile) && platform() === 'macos') ? 'ml-[90px]' : ''
  const margin = (platform() === 'macos') ? 'm-[10px]' : 'm-[15px]'
  const isDragArea = ['windows', 'macos'].includes(platform())

  return (
    <div className="flex place-content-between" data-tauri-drag-region={isDragArea}>
      <div className={`flex items-center space-x-3 ${leftSafeArea} ${margin}`}>
        <Button variant="outline" size="icon" hidden={true}>
          <SidebarOpenIcon/>
        </Button>
        <SidebarTrigger />
        <Separator orientation="vertical" className="data-[orientation=vertical]:h-4" />
        {(!open || isMobile) && (
          <>
            <SelectedServerDropdown />
            <Separator orientation="vertical" className="data-[orientation=vertical]:h-4" />
          </>
        )}
        <Button
          variant="outline"
          size="icon"
          onClick={() => selectFiles()}
        >
          <UploadIcon/>
        </Button>
      </div>
      <WindowControls/>
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
    setIsMaximized(await window.isMaximized())
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
    <>
      {(platform() === 'windows') && (
        <div
          className="flex items-center justify-between h-8 select-none"
          style={{ fontFamily: 'Segoe UI, sans-serif' }}
          data-tauri-drag-region
        >
          <div className="flex">
            <button
              onClick={minimize}
              className="w-12 h-8 flex items-center justify-center hover:bg-accent transition-colors text-[10px]"
              style={{ fontFamily: 'Segoe Fluent Icons' }}
            >
              {'\ue921'}
            </button>

            <button
              onClick={toggleMaximize}
              className="w-12 h-8 flex items-center justify-center hover:bg-accent transition-colors text-[10px]"
              style={{ fontFamily: 'Segoe Fluent Icons' }}
            >
              {isMaximized ? '\ue923' : '\ue922'}
            </button>

            <button
              onClick={close}
              className="w-12 h-8 flex items-center justify-center hover:bg-red-600 hover:text-white transition-colors text-[10px]"
              style={{ fontFamily: 'Segoe Fluent Icons' }}
            >
              {'\ue8bb'}
            </button>
          </div>
        </div>
      )}
    </>
  )
}