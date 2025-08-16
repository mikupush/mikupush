import { UploadRequest } from '@/model/upload'
import { useUploadsStore } from '@/store/uploads'
import { invoke } from '@tauri-apps/api/core'

export function selectFiles() {
  const store = useUploadsStore.getState()

  invoke<UploadRequest[]>('select_files_to_upload')
    .then((requests) => store.setInProgressUploads(requests))
    .catch((error) => console.warn('files dialog error', error))
}

export function extractExtension(path: string): string {
  const baseName = (path.split(/[\\/]/).pop() ?? '').trim();
  const lastDot = baseName.lastIndexOf('.');

  if (lastDot <= 0 || lastDot === baseName.length - 1) {
    return ''
  }

  return baseName.slice(lastDot + 1).toLowerCase();
};
