import React from 'react';
import { Navigate, Outlet } from 'react-router-dom';
import useAuthStore from '@/store/authStore';
import MainLayout from '@/components/layout/MainLayout';

/**
 * A route guard that checks authentication status.
 * Renders the MainLayout with nested routes (via Outlet) if authenticated,
 * otherwise redirects to the login page.
 * Shows a loading indicator during the initial auth check.
 */
const ProtectedRoute: React.FC = () => {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);
  const isLoading = useAuthStore((state) => state.isLoading);

  if (isLoading) {
    // TODO: Replace with a proper loading spinner component (e.g., shadcn Skeleton or custom)
    return <div className="flex h-screen items-center justify-center">Checking authentication...</div>;
  }

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />;
  }

  // User is authenticated, render the main layout which contains the <Outlet />
  // for the matched nested route defined in App.tsx
  return <MainLayout />;
};

export default ProtectedRoute;