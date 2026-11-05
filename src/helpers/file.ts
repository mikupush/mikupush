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

import { invoke } from '@tauri-apps/api/core'
import { requestUploads } from '@/helpers/upload.ts'
import toast from 'react-hot-toast'

export function selectFiles() {
  invoke<string[]>('select_files_to_upload')
    .then(requestUploads)
    .catch(error => toast.error(String(error)))
}

export function selectFolders() {
  invoke<string[]>('select_folders_to_upload')
    .then(requestUploads)
    .catch(error => toast.error(String(error)))
}

export function extractExtension(path: string): string {
  const baseName = (path.split(/[\\/]/).pop() ?? '').trim()
  const lastDot = baseName.lastIndexOf('.')

  if (lastDot <= 0 || lastDot === baseName.length - 1) {
    return ''
  }

  return baseName.slice(lastDot + 1).toLowerCase()
};

export function uploadDisplayName(name: string, directory: boolean): string {
  if (directory && name.toLowerCase().endsWith('.zip')) {
    return name.slice(0, -4)
  }

  return name
}
