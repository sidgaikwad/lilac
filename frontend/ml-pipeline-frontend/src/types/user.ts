// Placeholder for User related types

export interface User {
  id: string;
  name: string;
  email: string;
  // Add other fields returned by the API (e.g., GET /users/{user_id})
  created_at?: string;
  // Role within an organization might be here or in a separate Membership type
}

// Type for login response (adjust based on actual API)
export interface LoginResponse {
  token: string;
  user: User;
}