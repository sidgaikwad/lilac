import { create } from 'zustand';
import { User } from '@/types'; // Import User type

// Define the shape of the authentication state
interface AuthState {
  isAuthenticated: boolean;
  user: User | null;
  token: string | null;
  isLoading: boolean; // Tracks initial auth check loading
  error: string | null; // Stores login errors
  // Actions to update the state
  setAuthState: (isAuthenticated: boolean, user: User | null, token: string | null) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  logout: () => void;
}

const useAuthStore = create<AuthState>((set) => ({
  isAuthenticated: false,
  user: null,
  token: localStorage.getItem('authToken'), // Initialize token from storage
  isLoading: true, // Start loading until initial check completes
  error: null,

  // Action to update the entire auth state, typically after login or auth check
  setAuthState: (isAuthenticated, user, token) => {
    if (isAuthenticated && token) {
      localStorage.setItem('authToken', token);
    } else {
      localStorage.removeItem('authToken');
    }
    set({ isAuthenticated, user, token, isLoading: false, error: null });
  },

  // Action to explicitly set loading state
  setLoading: (loading) => {
    set({ isLoading: loading });
  },

  // Action to set login errors
  setError: (error) => {
    set({ error });
  },

  // Action for logging out
  logout: () => {
    localStorage.removeItem('authToken');
    set({ isAuthenticated: false, user: null, token: null, error: null, isLoading: false });
  },
}));

export default useAuthStore;