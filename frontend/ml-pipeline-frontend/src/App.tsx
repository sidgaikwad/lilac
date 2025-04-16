import React, { useEffect } from 'react';
import { Routes, Route, Navigate } from 'react-router-dom';
import useAuthStore from './store/authStore';
import ProtectedRoute from './components/router/ProtectedRoute';
import { useQuery } from '@tanstack/react-query';
import { checkAuth } from './features/auth/services/authService';
import { User } from '@/types'; // Import User type

// Import Feature Pages
import LoginPage from './features/auth/pages/LoginPage';
import DashboardPage from './features/pipelines/pages/DashboardPage';
import PipelineEditorPage from './features/pipelines/pages/PipelineEditorPage';
import SettingsPage from './features/settings/pages/SettingsPage';
import AccountSettingsPage from './features/settings/pages/AccountSettingsPage';
import OrganizationSettingsPage from './features/settings/pages/OrganizationSettingsPage';


const NotFoundPage = () => <div>404 Not Found</div>;

function App() {
  const setAuthState = useAuthStore((state) => state.setAuthState);
  const setLoading = useAuthStore((state) => state.setLoading);
  const token = useAuthStore((state) => state.token);

  // Use React Query to check auth status on load if token exists
  const { data: user, isSuccess, isError, isLoading: isCheckingAuth } = useQuery<User, Error>({ // Add explicit types
    queryKey: ['checkAuth'],
    queryFn: checkAuth,
    enabled: !!token,
    retry: false,
    refetchOnWindowFocus: false,
    // Remove onSuccess and onError callbacks
  });

  // Effect to update Zustand store based on query result
  useEffect(() => {
    if (isSuccess && user) {
      // On successful API validation, update Zustand store
      setAuthState(true, user, localStorage.getItem('authToken')); // Re-read token
    } else if (isError) {
      // On API error (invalid token), update Zustand store
      setAuthState(false, null, null); // This also removes token from localStorage
    }
    // If query is disabled (no token) or still loading, Zustand state remains unchanged here
    // We handle the initial no-token loading state below
  }, [isSuccess, isError, user, setAuthState]);

  // Effect to handle the initial loading state when there's no token
  useEffect(() => {
    if (!token && !isCheckingAuth) { // Ensure query isn't running
      setLoading(false);
    }
    // If token exists, loading state is managed by useQuery's isLoading flag via ProtectedRoute
  }, [token, isCheckingAuth, setLoading]);


  return (
    <Routes>
      {/* Public Route */}
      <Route path="/login" element={<LoginPage />} />

      {/* Protected Routes using Layout via ProtectedRoute component */}
      <Route element={<ProtectedRoute />}>
        <Route index element={<Navigate to="/pipelines" replace />} />
        <Route path="pipelines" element={<DashboardPage />} />
        <Route path="pipelines/:pipelineId" element={<PipelineEditorPage />} />
        {/* TODO: Optional: Route for specific versions */}
        {/* <Route path="pipelines/:pipelineId/versions/:versionId" element={<PipelineEditorPage />} /> */}

        <Route path="settings" element={<SettingsPage />}>
          <Route index element={<Navigate to="account" replace />} />
          <Route path="account" element={<AccountSettingsPage />} />
          <Route path="organization" element={<OrganizationSettingsPage />} />
        </Route>

        <Route path="*" element={<NotFoundPage />} />
      </Route>

      {/* Catch-all for routes outside layout */}
      <Route path="*" element={<NotFoundPage />} />
    </Routes>
  );
}

export default App;