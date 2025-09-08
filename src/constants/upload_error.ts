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
