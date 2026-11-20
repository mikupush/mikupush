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
import { create } from 'zustand'

interface UploadsStoreState {
  preferenceDialogs: { id: number, paths: string[] | null }[]
  inProgressUploads: UploadRequest[]
  activeDropZone: boolean
  isLoading: boolean
}

interface UploadsStoreActions {
  queueUploadPreferences(paths?: string[]): void
  closeUploadPreferences(id: number): void
  setInProgressUploads(uploadsRequests: UploadRequest[]): void
  showDropZone(show: boolean): void
  setIsLoading(loading: boolean): void
}

type UploadsStore = UploadsStoreState & UploadsStoreActions

let nextPreferenceDialogId = 0

export const useUploadsStore = create<UploadsStore>((set) => ({
  preferenceDialogs: [],
  queueUploadPreferences: (paths) => {
    set(state => ({ preferenceDialogs: [...state.preferenceDialogs, {
      id: nextPreferenceDialogId++, paths: paths ? [...paths] : null
    }] }))
  },
  closeUploadPreferences: (id) => {
    set(state => ({ preferenceDialogs: state.preferenceDialogs.filter(dialog => dialog.id !== id) }))
  },
  inProgressUploads: [],
  activeDropZone: false,
  isLoading: false,

  setInProgressUploads: (inProgressUploads: UploadRequest[]) => {
    set((state) => ({ ...state, inProgressUploads }))
  },
  showDropZone: (show: boolean) => {
    set((state) => ({ ...state, activeDropZone: show }))
  },
  setIsLoading: (loading: boolean) => {
    set((state) => ({ ...state, isLoading: loading }))
  }
}))
