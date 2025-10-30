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