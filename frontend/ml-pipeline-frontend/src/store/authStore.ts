import { create } from 'zustand';

// TODO: Define a proper User type based on API response
interface User {
  id: string;
  name: string;
  email: string;
  // Add other relevant user fields
}

interface AuthState {
  isAuthenticated: boolean;
  user: User | null;
  token: string | null;
  isLoading: boolean;
  error: string | null;
  login: (/* credentials */) => Promise<void>; // Placeholder for login action
  logout: () => void;
  checkAuthStatus: () => Promise<void>; // Placeholder for checking token validity
}

const useAuthStore = create<AuthState>((set, get) => ({
  isAuthenticated: false,
  user: null,
  token: localStorage.getItem('authToken'), // Initialize token from storage
  isLoading: false, // Initially not loading
  error: null,

  // Example Login Action (replace with actual API call)
  login: async (/* credentials */) => {
    set({ isLoading: true, error: null });
    try {
      // const response = await apiClient.post('/auth/login', credentials);
      // const { token, user } = response.data;
      const token = 'fake-jwt-token'; // Replace with actual token
      const user: User = { id: '1', name: 'Test User', email: 'test@example.com' }; // Replace with actual user

      localStorage.setItem('authToken', token);
      set({ isAuthenticated: true, user, token, isLoading: false });
    } catch (err) {
      const errorMessage = (err as Error).message || 'Login failed';
      set({ error: errorMessage, isLoading: false });
      console.error('Login error:', err);
    }
  },

  // Logout Action
  logout: () => {
    localStorage.removeItem('authToken');
    set({ isAuthenticated: false, user: null, token: null });
  },

  // Example Check Auth Status Action (e.g., on app load)
  checkAuthStatus: async () => {
    const token = get().token;
    if (!token) {
      set({ isAuthenticated: false, user: null, isLoading: false });
      return;
    }
    set({ isLoading: true });
    try {
      // TODO: Add an API endpoint to validate the token (e.g., GET /users/me)
      // const response = await apiClient.get('/users/me'); // Assuming token is sent via interceptor
      // const user = response.data;
      const user: User = { id: '1', name: 'Test User', email: 'test@example.com' }; // Replace with actual user from validation
      set({ isAuthenticated: true, user, isLoading: false });
    } catch (err) {
      console.error('Auth status check failed:', err);
      get().logout(); // Log out if token is invalid
      set({ isLoading: false });
    }
  },
}));

// Call checkAuthStatus on initial load (outside the store definition)
// This might be better placed in your main App component's useEffect
// useAuthStore.getState().checkAuthStatus();

export default useAuthStore;