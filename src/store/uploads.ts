import { Upload, UploadRequest } from '@/model/upload'
import { create } from 'zustand'

interface UploadsStoreState {
  inProgressUploads: UploadRequest[]
  archivedUploads: Upload[]
  activeDropZone: boolean
}

interface UploadsStoreActions {
  setInProgressUploads(uploadsRequests: UploadRequest[]): void
  showDropZone(show: boolean): void
}

type UploadsStore = UploadsStoreState & UploadsStoreActions

export const useUploadsStore = create<UploadsStore>((set) => ({
  inProgressUploads: [],
  archivedUploads: [],
  activeDropZone: false,

  setInProgressUploads: (inProgressUploads: UploadRequest[]) => {
    set((state) => ({ ...state, inProgressUploads }))
  },
  showDropZone: (show: boolean) => {
    set((state) => ({ ...state, activeDropZone: show }))
  }
}))