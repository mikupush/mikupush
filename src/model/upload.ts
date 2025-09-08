import { type UploadErrorCode } from '@/constants/upload_error'

export interface Progress {
  progress: number
  totalSize: number
  uploadedBytes: number
  rateBytes: number
}

export interface UploadRequestError {
  code: UploadErrorCode
  message: string
}

export interface UploadRequest {
  progress: Progress;
  error?: UploadRequestError;
  upload: Upload;
  finished: boolean;
  canceled: boolean;
}

export interface Upload {
  id: string;
  name: string;
  size: number;
  mimeType: string;
  path: string;
  url?: string;
  createdAt: string; // ISO string, matches Rust DateTimeUtc
  status: string;
}
