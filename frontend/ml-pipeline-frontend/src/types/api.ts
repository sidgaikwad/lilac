// Placeholder for common API response structures or error types

export interface ApiError {
  message: string;
  statusCode?: number;
  details?: any;
}

// Example generic response (adjust as needed)
export interface ApiResponse<T> {
  data: T;
  message?: string;
  // Add other common fields like pagination info if applicable
}