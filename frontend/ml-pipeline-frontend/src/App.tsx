import { Routes, Route, Navigate, Outlet } from 'react-router-dom';
import MainLayout from './components/layout/MainLayout';

// Import Feature Pages
import LoginPage from './features/auth/pages/LoginPage';
import DashboardPage from './features/pipelines/pages/DashboardPage';
import PipelineEditorPage from './features/pipelines/pages/PipelineEditorPage';
import SettingsPage, { AccountSettingsPage, OrganizationSettingsPage } from './features/settings/pages/SettingsPage';

const NotFoundPage = () => <div>404 Not Found</div>;

// TODO: Replace with actual auth logic
const isAuthenticated = () => true;

// Wrapper for protected routes using MainLayout
const ProtectedRoute = () => {
  if (!isAuthenticated()) {
    return <Navigate to="/login" replace />;
  }
  return <MainLayout />; // Renders Outlet for nested routes
};


function App() {
  return (
    <Routes>
      {/* Public Route */}
      <Route path="/login" element={<LoginPage />} />

      {/* Protected Routes using Layout */}
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