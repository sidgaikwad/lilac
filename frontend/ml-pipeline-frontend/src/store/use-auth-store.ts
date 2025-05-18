import { createWithEqualityFn } from 'zustand/traditional';

interface AuthState {
  token: string | undefined;
  login: (token: string) => void;
  logout: () => void;
}

const useAuthStore = createWithEqualityFn<AuthState>((set) => ({
  token: localStorage.getItem('token') || undefined,

  login: (token) => {
    localStorage.setItem('token', token);
    set({ token });
  },

  logout: () => {
    localStorage.removeItem('token');
    set({ token: undefined });
  },
}));

export default useAuthStore;
