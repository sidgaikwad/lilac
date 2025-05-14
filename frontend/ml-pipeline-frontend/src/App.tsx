import { Routes, Route, Navigate } from 'react-router-dom';
import ProtectedRoute from './components/router/ProtectedRoute';
import { ApiError } from '@/types';
import '@tanstack/react-query';


import MainLayout from './components/layout/MainLayout';


import LoginPage from './features/auth/pages/LoginPage';
import PipelineEditorPage from './features/pipelines/pages/PipelineEditorPage';
import DataSetsPage from './features/datasets/pages/DataSetsPage';
import DataSetDetailPage from './features/datasets/pages/DataSetDetailPage';
import SettingsPage from './features/settings/pages/SettingsPage';
import AccountSettingsPage from './features/settings/pages/AccountSettingsPage';
import OrganizationSettingsPage from './features/settings/pages/OrganizationSettingsPage';
import OrganizationsOverviewPage from './features/organizations/OrganizationsOverviewPage';
import ProjectOverviewPage from './features/projects/ProjectOverviewPage';
import ProjectsByOrgPage from './features/projects/ProjectsByOrgPage'; 
import SignUpPage from './features/auth/pages/SignUpPage';
import PipelinesOverviewPage from './features/pipelines/pages/PipelinesOverviewPage';

declare module '@tanstack/react-query' {
  interface Register {
    defaultError: ApiError;
  }
}

const NotFoundPage = () => <div>404 Not Found</div>;

function App() {
  const renderAppContent = () => {
    return (
      <Routes>
        <Route element={<MainLayout />}>
          <Route index element={<OrganizationsOverviewPage />} />
          <Route path="organizations/:organizationId/projects" element={<ProjectsByOrgPage />} /> {}
          <Route path="projects/:projectId" element={<ProjectOverviewPage />} />
          <Route
            path="projects/:projectId/database"
            element={<div>Database Section Placeholder</div>}
          />
          <Route
            path="projects/:projectId/pipelines"
            element={<PipelinesOverviewPage />}
          />
          <Route
            path="projects/:projectId/pipelines/:pipelineId"
            element={<PipelineEditorPage />}
          />
          <Route
            path="projects/:projectId/datasets"
            element={<DataSetsPage />}
          />
          <Route
            path="projects/:projectId/datasets/:datasetId"
            element={<DataSetDetailPage />}
          />
          <Route
            path="projects/:projectId/auth"
            element={<div>Auth Section Placeholder</div>}
          />
          <Route
            path="projects/:projectId/storage"
            element={<div>Storage Section Placeholder</div>}
          />
          <Route path="settings" element={<SettingsPage />}>
            <Route index element={<Navigate to="account" replace />} />
            <Route path="account" element={<AccountSettingsPage />} />
            <Route path="organization" element={<OrganizationSettingsPage />} />
          </Route>
          <Route path="*" element={<NotFoundPage />} />
        </Route>
      </Routes>
    );
  };

  return (
    <Routes>
      <Route path="/login" element={<LoginPage />} />
      <Route path="/signup" element={<SignUpPage />} />
      <Route element={<ProtectedRoute />}>
        <Route path="*" element={renderAppContent()} />
      </Route>
    </Routes>
  );
}

export default App;
