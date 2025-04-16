import apiClient from '@/services/apiClient';
import { User } from '@/types'; // Assuming User type is defined in types

interface LoginCredentials {
  email: string;
  password?: string;
}

interface LoginResponse {
  token: string;
  user: User;
}

// TODO: Replace mock with actual API call
export const loginUser = async (credentials: LoginCredentials): Promise<LoginResponse> => {
  console.log("Attempting login via authService with:", credentials);
  // const response = await apiClient.post<LoginResponse>('/auth/login', credentials);
  // return response.data;

  // Mock Implementation
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      if (credentials.email === 'admin@example.com' && credentials.password === 'admin') {
        const mockResponse: LoginResponse = {
          token: 'mock-jwt-token-12345',
          user: { id: 'user-admin', name: 'Admin User', email: 'admin@example.com' },
        };
        console.log("Mock login successful in service");
        resolve(mockResponse);
      } else {
        console.error("Mock login failed in service");
        reject(new Error("Invalid email or password"));
      }
    }, 500);
  });
};

// TODO: Replace mock with actual API call to validate token and get user
export const checkAuth = async (): Promise<User> => {
  console.log("Checking auth status via authService");
  // Assumes token is sent via apiClient interceptor
  // const response = await apiClient.get<User>('/users/me'); // Or similar validation endpoint
  // return response.data;

   // Mock Implementation
   return new Promise((resolve, reject) => {
     setTimeout(() => {
        const token = localStorage.getItem('authToken');
        if (token === 'mock-jwt-token-12345') {
            const mockUser: User = { id: 'user-admin', name: 'Admin User', email: 'admin@example.com' };
            console.log("Mock auth check successful in service");
            resolve(mockUser);
        } else {
             console.error("Mock auth check failed in service: Invalid/missing token");
             reject(new Error("Invalid or missing token"));
        }
     }, 200);
   });
};

// TODO: Add logout API call if needed
// export const logoutUser = async (): Promise<void> => {
//   await apiClient.post('/auth/logout');
// };