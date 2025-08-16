import { Upload, UploadRequest } from '@/model/upload'
import { create } from 'zustand'

interface UploadsStoreState {
  inProgressUploads: UploadRequest[]
  archivedUploads: Upload[]
}

interface UploadsStoreActions {
  setInProgressUploads(uploadsRequests: UploadRequest[]): void
}

type UploadsStore = UploadsStoreState & UploadsStoreActions

export const useUploadsStore = create<UploadsStore>((set) => ({
  inProgressUploads: [],
  archivedUploads: [],

  setInProgressUploads: (inProgressUploads: UploadRequest[]) => {
    console.log('set uploads in progress', inProgressUploads)
    set((state) => ({ ...state, inProgressUploads }))
  }
}))