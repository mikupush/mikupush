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

export const UPLOAD_ERROR_EXISTS = 'exists'
export const UPLOAD_ERROR_NOT_EXISTS = 'not_exists'
export const UPLOAD_ERROR_MAX_FILE_SIZE_EXCEEDED = 'max_file_size_exceeded'
export const UPLOAD_ERROR_NOT_COMPLETED = 'not_completed'
export const UPLOAD_ERROR_UNKNOWN_MIME_TYPE = 'unknown_mime_type'
export const UPLOAD_ERROR_CANCELED = 'canceled'
export const UPLOAD_ERROR_INTERNAL_SERVER_ERROR = 'internal_server_error'
export const UPLOAD_ERROR_CLIENT_ERROR = 'client_error'

export type UploadErrorCode =
  | typeof UPLOAD_ERROR_EXISTS
  | typeof UPLOAD_ERROR_NOT_EXISTS
  | typeof UPLOAD_ERROR_MAX_FILE_SIZE_EXCEEDED
  | typeof UPLOAD_ERROR_NOT_COMPLETED
  | typeof UPLOAD_ERROR_UNKNOWN_MIME_TYPE
  | typeof UPLOAD_ERROR_CANCELED
  | typeof UPLOAD_ERROR_INTERNAL_SERVER_ERROR
  | typeof UPLOAD_ERROR_CLIENT_ERROR