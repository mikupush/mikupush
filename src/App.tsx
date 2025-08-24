import styles from './App.module.css'
import { getCurrentWebview } from '@tauri-apps/api/webview'
import { invoke } from '@tauri-apps/api/core'
import AppBar from '@/components/AppBar'
import Uploads from '@/components/Uploads.tsx'
import { Toaster } from 'react-hot-toast'

await getCurrentWebview().onDragDropEvent(async (event) => {
  const dropArea = document.querySelectorAll('.file-drop-area');

  const showActive = () => dropArea.forEach(el => {
    el.classList.remove('active')
    el.classList.add('active')
  })

  const hideActive = () => dropArea.forEach(el => {
    el.classList.remove('active')
  })
  // añadir aqui clase de css al input del fichero y llamar a un comando de tauri
  // se repiten mucho los eventos habra que tener cuidado con eso
  // https://v2.tauri.app/plugin/dialog/#build-a-file-selector-dialog
  if (event.payload.type === 'over') {
    showActive()
  } else if (event.payload.type === 'drop') {
    await invoke('enqueue_many_uploads', { paths: event.payload.paths })
  } else {
    hideActive()
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
