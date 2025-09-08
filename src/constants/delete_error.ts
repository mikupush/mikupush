export const DELETE_ERROR_NOT_EXISTS = 'not_exists'
export const DELETE_ERROR_INTERNAL_SERVER_ERROR = 'internal_server_error'
export const DELETE_ERROR_CLIENT_ERROR = 'client_error'

export type DeleteErrorCode =
  | typeof DELETE_ERROR_NOT_EXISTS
  | typeof DELETE_ERROR_INTERNAL_SERVER_ERROR
  | typeof DELETE_ERROR_CLIENT_ERROR
