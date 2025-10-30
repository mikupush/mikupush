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