import { create } from 'zustand';
import { User } from '@/types'; // Import User type

// Define the shape of the authentication state
interface AuthState {
  user: User | undefined;
  token: string | undefined;
  // Actions to update the state
  login: (user: User, token: string) => void;
  logout: () => void;
}

const useAuthStore = create<AuthState>((set) => ({
  user:
    localStorage.getItem('user') && JSON.parse(localStorage.getItem('user')!),
  token: localStorage.getItem('token') || undefined, // Initialize token from storage

  // Action to update the entire auth state, typically after login or auth check
  login: (user, token) => {
    localStorage.setItem('token', token);
    localStorage.setItem('user', JSON.stringify(user));
    set({ user, token });
  },

  // Action for logging out
  logout: () => {
    localStorage.removeItem('token');
    set({ user: undefined, token: undefined });
  },
}));

export default useAuthStore;
