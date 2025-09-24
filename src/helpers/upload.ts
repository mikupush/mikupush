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