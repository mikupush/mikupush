/**
 * Copyright 2025 Miku Push! Team
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import styles from './App.module.css'
import { getCurrentWebview } from '@tauri-apps/api/webview'
import { invoke } from '@tauri-apps/api/core'
import AppBar from '@/components/AppBar'
import Uploads from '@/components/Uploads.tsx'
import { Toaster } from 'react-hot-toast'
import { useUploadsStore } from '@/store/uploads'
import { UploadRequest } from '@/model/upload'
import { listen } from '@tauri-apps/api/event'
import { ThemeProvider } from '@/components/ThemeProvider.tsx'

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

function App() {
  return (
    <ThemeProvider>
      <div className={styles.app}>
        <AppBar />
        <Uploads />
        <Toaster
          position="bottom-right"
          toastOptions={{
            style: {
              background: 'var(--background)',
              color: 'var(--foreground)',
              border: '1px solid var(--border)',
            }
          }}
        />
      </div>
    </ThemeProvider>
  )
}

export default App