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