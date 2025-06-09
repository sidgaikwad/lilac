export interface ApiError {
  error: string;
  statusCode: number;
}

export function isApiError(error: unknown): error is ApiError {
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
