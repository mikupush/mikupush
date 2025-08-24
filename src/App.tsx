import styles from './App.module.css'
import { getCurrentWebview } from '@tauri-apps/api/webview'
import { invoke } from '@tauri-apps/api/core'
import AppBar from '@/components/AppBar'
import Uploads from '@/components/Uploads.tsx'
import { Toaster } from 'react-hot-toast'
import { useUploadsStore } from '@/store/uploads'
import { UploadRequest } from '@/model/upload'

await getCurrentWebview().onDragDropEvent((event) => {
  const store = useUploadsStore.getState()

  // se repiten mucho los eventos habra que tener cuidado con eso
  // https://v2.tauri.app/plugin/dialog/#build-a-file-selector-dialog
  if (event.payload.type === 'over') {
    store.showDropZone(true)
  } else if (event.payload.type === 'drop') {
    console.log('User dropped', event.payload.paths)
    invoke<UploadRequest[]>('enqueue_many_uploads', { paths: event.payload.paths })
      .then((requests) => store.setInProgressUploads(requests))
      .catch((error) => console.warn('files dialog error', error))
    store.showDropZone(false)
  } else {
    store.showDropZone(false)
  }
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
