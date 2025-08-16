// TODO: alinear con rust
export interface UploadRequest {
  progress: number
  speed: number
  error?: string
  finished: boolean
  upload: Upload
}

export interface Upload {
    id: string
    name: string
    size: number
    mimeType: string
    uploadedAt: Date
}
