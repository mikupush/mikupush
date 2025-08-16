export interface UploadRequest {
  progress: number;
  uploadedBytes: number;
  rateBytes: number;
  error?: string;
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
