export interface ProgressEvent {
  uploadId: string
  progress: number
  totalSize: number
  uploadedBytes: number
  rateBytes: number
}
