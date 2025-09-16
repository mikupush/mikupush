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
  const baseName = (path.split(/[\\/]/).pop() ?? '').trim()
  const lastDot = baseName.lastIndexOf('.')

  if (lastDot <= 0 || lastDot === baseName.length - 1) {
    return ''
  }

  return baseName.slice(lastDot + 1).toLowerCase()
};
