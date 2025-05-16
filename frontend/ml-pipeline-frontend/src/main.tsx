import './index.css';
import '@tanstack/react-query';
import { ApiError } from '@/types';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'; // Import QueryClient stuff
import { router } from './routes.tsx';
import { RouterProvider } from 'react-router-dom';
import { ThemeProvider } from './components/providers/ThemeProvider.tsx';
import React from 'react';
import ReactDOM from 'react-dom/client';

declare module '@tanstack/react-query' {
  interface Register {
    defaultError: ApiError;
  }
}

// Create a client
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      // Configure default query options if needed (e.g., staleTime, gcTime)
      // staleTime: 1000 * 60 * 5, // 5 minutes
      refetchOnWindowFocus: false,
      retryDelay: (attemptIndex) => Math.min(200 * 2 ** attemptIndex, 5000),
    },
  },
});

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
        <RouterProvider router={router} />
      </ThemeProvider>
    </QueryClientProvider>
  </React.StrictMode>
);
