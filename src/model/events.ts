import { Progress } from '@/model/upload'

export interface ProgressEvent {
  uploadId: string
  progress: Progress
}
