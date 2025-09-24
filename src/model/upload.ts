/**
 * Copyright 2025 Miku Push! Team
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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