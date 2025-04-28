import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'; // Import QueryClient stuff
import App from './App.tsx';
import { ThemeProvider } from './components/providers/ThemeProvider.tsx';
import './index.css';

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
      {' '}
      {/* Wrap with QueryClientProvider */}
      <BrowserRouter>
        <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
          <App />
        </ThemeProvider>
      </BrowserRouter>
    </QueryClientProvider>
  </React.StrictMode>
);
