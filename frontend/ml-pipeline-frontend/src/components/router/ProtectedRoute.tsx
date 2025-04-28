import React from 'react';
import { Outlet } from 'react-router-dom'; // Import Outlet
import { Navigate } from 'react-router-dom';
import useAuthStore from '@/store/useAuthStore';

/**
 * A route guard that checks authentication status.
 * Renders the MainLayout with nested routes (via Outlet) if authenticated,
 * otherwise redirects to the login page.
 * Shows a loading indicator during the initial auth check.
 */
const ProtectedRoute: React.FC = () => {
  const { user, token } = useAuthStore();

  if (!user || !token) {
    return <Navigate to="/login" replace />;
  }

  // User is authenticated, render the nested routes via Outlet.
  // The correct layout (MainLayout or ProjectLayout) will be rendered by the
  // specific route configuration within App.tsx.
  return <Outlet />;
};

export default ProtectedRoute;
