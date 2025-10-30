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

import { getCurrentWebview } from '@tauri-apps/api/webview'
import { invoke } from '@tauri-apps/api/core'
import { useUploadsStore } from '@/store/uploads'
import { UploadRequest } from '@/model/upload'
import { listen } from '@tauri-apps/api/event'
import { ThemeProvider } from '@/context/ThemeProvider.tsx'
import { fetchCurrentUploads } from '@/helpers/upload.ts'
import Router from '@/router.tsx'
import { ServerProvider } from '@/context/ServerProvider.tsx'
import { ToastContainer } from '@/components/ToastContainer.tsx'

await getCurrentWebview().onDragDropEvent((event) => {
  const store = useUploadsStore.getState()

  if (event.payload.type === 'over') {
    store.showDropZone(true)
  } else if (event.payload.type === 'drop') {
    invoke<UploadRequest[]>('enqueue_many_uploads', { paths: event.payload.paths })
      .then((requests) => store.setInProgressUploads(requests))
      .catch((error) => console.warn('files dialog error', error))
    store.showDropZone(false)
  } else {
    store.showDropZone(false)
  }
})

await listen<UploadRequest[]>('uploads-changed', (event) => {
  const store = useUploadsStore.getState()

  setTimeout(() => {
    store.setInProgressUploads(event.payload)
  }, 100)
})

fetchCurrentUploads()

function MainWindow() {
  return (
    <ThemeProvider>
      <ServerProvider>
        <Router />
        <ToastContainer />
      </ServerProvider>
    </ThemeProvider>
  )
}

export default MainWindow