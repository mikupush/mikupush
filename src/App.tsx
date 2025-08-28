import styles from './App.module.css'
import { getCurrentWebview } from '@tauri-apps/api/webview'
import { invoke } from '@tauri-apps/api/core'
import AppBar from '@/components/AppBar'
import Uploads from '@/components/Uploads.tsx'
import { Toaster } from 'react-hot-toast'
import { useUploadsStore } from '@/store/uploads'
import { UploadRequest } from '@/model/upload'
import { listen } from '@tauri-apps/api/event'

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
    <div className={styles.app}>
      <AppBar />
      <Uploads />
      <Toaster position="bottom-right" />
    </div>
  )
}

export default App
