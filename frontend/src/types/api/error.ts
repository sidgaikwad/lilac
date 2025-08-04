export interface ServiceError {
  error: string;
  statusCode: number;
}

export function isServiceError(error: unknown): error is ServiceError {
  if (
    !!error &&
    typeof error === 'object' &&
    'error' in error &&
    'statusCode' in error
  ) {
    return true;
  }
  return false;
}
