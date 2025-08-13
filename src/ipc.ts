import { invoke } from '@tauri-apps/api/core'

export function selectFiles() {
  invoke('select_files_to_upload')
    .then(() => console.log('files dialog opened successfully'))
    .catch((error) => console.warn('files dialog error', error))
}