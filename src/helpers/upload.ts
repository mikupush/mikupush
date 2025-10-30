/**
 * Miku Push! is a simple, lightweight, and open-source WeTransfer alternative for desktop.
 * Copyright (C) 2025  Miku Push! Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
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

export function fetchCurrentUploads() {
  const store = useUploadsStore.getState()

  store.setIsLoading(true)
  invoke<UploadRequest[]>('get_all_in_progress_uploads')
    .then((requests) => store.setInProgressUploads(requests))
    .finally(() => store.setIsLoading(false))
}