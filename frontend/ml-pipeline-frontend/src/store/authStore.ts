import { create } from 'zustand';

interface LoginCredentials {
  email: string;
  password?: string;
}

interface User {
  id: string;
  name: string;
  email: string;
}

interface AuthState {
  isAuthenticated: boolean;
  user: User | null;
  token: string | null;
  isLoading: boolean;
  error: string | null;
  login: (credentials: LoginCredentials) => Promise<void>;
  logout: () => void;
  checkAuthStatus: () => Promise<void>;
  setLoading: (loading: boolean) => void;
}

const MOCK_USER: User = { id: 'user-admin', name: 'Admin User', email: 'admin@example.com' };
const MOCK_TOKEN = 'mock-jwt-token-12345';

const useAuthStore = create<AuthState>((set, get) => ({
  isAuthenticated: false,
  user: null,
  token: localStorage.getItem('authToken'),
  isLoading: true,
  error: null,

  login: async (credentials: LoginCredentials) => {
    set({ isLoading: true, error: null });
    // TODO: API Call - POST /auth/login with credentials
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        if (credentials.email === 'admin@example.com' && credentials.password === 'admin') {
          localStorage.setItem('authToken', MOCK_TOKEN);
          // TODO: Set user/token based on actual API response
          set({ isAuthenticated: true, user: MOCK_USER, token: MOCK_TOKEN, isLoading: false });
          console.log("Mock login successful");
          resolve();
        } else {
          const errorMsg = "Invalid email or password";
          set({ error: errorMsg, isLoading: false, isAuthenticated: false, user: null, token: null });
          console.error("Mock login failed");
          reject(new Error(errorMsg));
        }
      }, 500);
    });
  },

  logout: () => {
    // TODO: API Call - POST /auth/logout (optional, if backend needs session invalidation)
    localStorage.removeItem('authToken');
    set({ isAuthenticated: false, user: null, token: null, error: null, isLoading: false });
  },

  checkAuthStatus: async () => {
    const token = get().token;
    if (!token) { // Removed mock token check for simplicity, rely on API validation
      get().logout();
      set({ isLoading: false }); // Ensure loading is false if no token
      return;
    }
    // Don't set isLoading: true here, let ProtectedRoute handle initial loading state
    try {
      // TODO: API Call - GET /users/me (or similar validation endpoint) using the token
      // If valid, set user from response and isAuthenticated: true
      await new Promise(res => setTimeout(res, 200)); // Simulate API call
      set({ isAuthenticated: true, user: MOCK_USER, isLoading: false }); // Assume success for now
    } catch (err) {
      console.error('Auth status check failed:', err);
      get().logout(); // Logout if token validation fails
    }
  },

  setLoading: (loading: boolean) => {
    set({ isLoading: loading });
  },
}));

export default useAuthStore;