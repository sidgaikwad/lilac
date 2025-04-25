import React from 'react';
import { Outlet } from 'react-router-dom'; // Import Outlet
import { Navigate } from 'react-router-dom';
import MainLayout from '@/components/layout/MainLayout';
import { useGetUser } from '@/services/controlplane-api/user/use-get-user.hook';

/**
 * A route guard that checks authentication status.
 * Renders the MainLayout with nested routes (via Outlet) if authenticated,
 * otherwise redirects to the login page.
 * Shows a loading indicator during the initial auth check.
 */
const ProtectedRoute: React.FC = () => {
  const { isError, isLoading } = useGetUser({});

  if (isLoading) {
    // TODO: Replace with a proper loading spinner component (e.g., shadcn Skeleton or custom)
    return <div className="flex h-screen items-center justify-center">Checking authentication...</div>;
  }

  if (isError) {
    return <Navigate to="/login" replace />;
  }

  // User is authenticated, render the nested routes via Outlet.
  // The correct layout (MainLayout or ProjectLayout) will be rendered by the
  // specific route configuration within App.tsx.
  return <Outlet />;
};

export default ProtectedRoute;