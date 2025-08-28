import { useUploadsStore } from '@/store/uploads.ts'
import { invoke } from '@tauri-apps/api/core'
import { UploadRequest } from '@/model/upload.ts'

export function cancelUpload(uploadId: string) {
  const store = useUploadsStore.getState()

  invoke<UploadRequest[]>('cancel_upload', { uploadId })
    .then((result) => store.setInProgressUploads(result))
    .catch((error) => console.warn(error))
}

export function retryUpload(uploadId: string) {
  console.log('retry upload')
  invoke<void>('retry_upload', { uploadId })
    .then(() => console.log('retry upload success'))
    .catch((error) => console.warn(error))
}