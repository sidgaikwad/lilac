import React, { useEffect } from 'react';
import { Routes, Route, Navigate } from 'react-router-dom';
import useAuthStore from './store/authStore';
import ProtectedRoute from './components/router/ProtectedRoute';

// Import Feature Pages
import LoginPage from './features/auth/pages/LoginPage';
import DashboardPage from './features/pipelines/pages/DashboardPage';
import PipelineEditorPage from './features/pipelines/pages/PipelineEditorPage';
// Import Settings pages from their own files
import SettingsPage from './features/settings/pages/SettingsPage';
import AccountSettingsPage from './features/settings/pages/AccountSettingsPage';
import OrganizationSettingsPage from './features/settings/pages/OrganizationSettingsPage';


const NotFoundPage = () => <div>404 Not Found</div>;

function App() {
  const checkAuth = useAuthStore((state) => state.checkAuthStatus);
  const setLoading = useAuthStore((state) => state.setLoading);

  useEffect(() => {
    const tokenExists = !!localStorage.getItem('authToken');
    if (tokenExists) {
        checkAuth();
    } else {
        setLoading(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

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