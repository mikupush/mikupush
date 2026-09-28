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
import { UploadPreferences, UploadRequest } from '@/model/upload.ts'
import { applyConfig, getConfig } from '@/helpers/config.ts'
import {
  CONFIG_ASK_UPLOAD_PREFERENCES,
  CONFIG_TRUE_VALUE,
  CONFIG_UPLOAD_PREFERENCES
} from '@/constants/config.ts'

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
  invoke<UploadRequest[]>('list_active_uploads')
    .then((requests) => store.setInProgressUploads(requests))
    .finally(() => store.setIsLoading(false))
}

export async function getUploadPreferences(): Promise<UploadPreferences> {
  const value: unknown = JSON.parse(await getConfig(CONFIG_UPLOAD_PREFERENCES))

  if (typeof value !== 'object' || value === null
    || !('encryption' in value) || typeof value.encryption !== 'boolean') {
    throw new Error('Invalid upload preferences')
  }

  return { encryption: value.encryption }
}

export async function saveUploadPreferences(preferences: UploadPreferences) {
  await applyConfig(CONFIG_UPLOAD_PREFERENCES, JSON.stringify(preferences))
}

export async function startUploads(paths: string[], preferences: UploadPreferences) {
  if (paths.length === 0) return

  const requests = await invoke<UploadRequest[]>('enqueue_uploads', {
    paths: [...paths],
    preferences: { ...preferences }
  })

  useUploadsStore.getState().setInProgressUploads(requests)
}

export async function requestUploads(paths: string[]) {
  if (paths.length === 0) return

  if (await getConfig(CONFIG_ASK_UPLOAD_PREFERENCES) === CONFIG_TRUE_VALUE) {
    useUploadsStore.getState().queueUploadPreferences(paths)
    return
  }

  await startUploads(paths, await getUploadPreferences())
}
