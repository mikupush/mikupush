import { Progress } from "@/model/upload.ts"

export interface ProgressEvent {
  uploadId: string
  progress: Progress
}
