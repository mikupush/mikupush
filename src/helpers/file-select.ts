import { UploadRequest } from '@/model/upload'
import { useUploadsStore } from '@/store/uploads'
import { invoke } from '@tauri-apps/api/core'

export function selectFiles() {
  const store = useUploadsStore()

  invoke('select_files_to_upload')
    .then((requests) => store.setInProgressUploads(requests as UploadRequest[]))
    .catch((error) => console.warn('files dialog error', error))
}