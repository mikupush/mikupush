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
import { fetchCurrentUploads, requestUploads } from '@/helpers/upload.ts'
import Router from '@/router.tsx'
import { ServerProvider } from '@/context/ServerProvider.tsx'
import { ToastContainer } from '@/components/ToastContainer.tsx'
import { UploadPreferencesDialog } from '@/components/UploadPreferencesDialog'
import toast from 'react-hot-toast'
import { LanguageProvider } from '@/context/LanguageProvider.tsx'

await getCurrentWebview().onDragDropEvent((event) => {
  const store = useUploadsStore.getState()

  if (event.payload.type === 'over') {
    store.showDropZone(true)
  } else if (event.payload.type === 'drop') {
    requestUploads(event.payload.paths).catch(error => toast.error(String(error)))
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

// Drain after registering the listener as native requests can arrive before the UI loads.
async function takePendingUploads() {
  const batches = await invoke<string[][]>('take_pending_upload_paths')
  batches.forEach(paths => useUploadsStore.getState().queueUploadPreferences(paths))
}
await listen('upload-preferences-requested', () => {
  takePendingUploads().catch(error => toast.error(String(error)))
})
await takePendingUploads()

fetchCurrentUploads()

function MainWindow() {
  return (
    <LanguageProvider>
      <ThemeProvider>
        <ServerProvider>
          <Router />
          <ToastContainer />
          <UploadPreferencesDialog />
        </ServerProvider>
      </ThemeProvider>
    </LanguageProvider>
  )
}

export default MainWindow
