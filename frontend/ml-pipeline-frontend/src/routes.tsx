import { createBrowserRouter } from 'react-router';
import { Routes } from './constants';
import OrganizationsListPage from './features/organizations/pages/organizations-list-page';
import {
  MainLayout,
  OrganizationLayout,
  ProjectLayout,
} from './components/layout';
import ProjectsListPage from './features/projects/pages/projects-list-page';
import ProjectDashboardPage from './features/projects/pages/project-dashboard-page';
import PipelinesOverviewPage from './features/pipelines/pages/pipelines-overview-page';
import PipelineEditorPage from './features/pipelines/pages/pipeline-editor-page';
import DataSetsPage from './features/datasets/pages/datasets-page';
import DataSetDetailPage from './features/datasets/pages/dataset-details-page';
import ProtectedRoute from './components/router/protected-route';
import LoginPage from './features/account/pages/login-page';
import SignUpPage from './features/account/pages/sign-up-page';
import ErrorBoundary from './error-boundary';
import OrganizationSettingsPage from './features/organizations/pages/organization-settings-page';
import AccountSettingsPage from './features/account/pages/account-settings-page';
import ProjectSettingsPage from './features/projects/pages/project-settings-page';

export const router = createBrowserRouter([
  {
    errorElement: <ErrorBoundary />,
    children: [
      {
        path: '/login',
        element: <LoginPage />,
      },
      {
        path: '/signup',
        element: <SignUpPage />,
      },
      {
        element: <ProtectedRoute />,
        children: [
          {
            element: <MainLayout />,
            children: [
              {
                index: true,
                element: <OrganizationsListPage />,
              },
              {
                path: '/organizations',
                element: <OrganizationsListPage />,
              },
              {
                path: Routes.ACCOUNT_SETTINGS,
                element: <AccountSettingsPage />,
              },
            ],
          },
          {
            element: <OrganizationLayout />,
            children: [
              {
                path: Routes.ORGANIZATION_PROJECTS,
                element: <ProjectsListPage />,
              },
              {
                path: Routes.ORGANIZATION_SETTINGS,
                element: <OrganizationSettingsPage />,
              },
            ],
          },
          {
            element: <ProjectLayout />,
            children: [
              {
                path: Routes.PROJECT_DETAILS,
                element: <ProjectDashboardPage />,
              },
              {
                path: Routes.PROJECT_PIPELINES,
                element: <PipelinesOverviewPage />,
              },
              {
                path: Routes.PROJECT_PIPELINE_DETAILS,
                element: <PipelineEditorPage />,
              },
              {
                path: Routes.PROJECT_DATASETS,
                element: <DataSetsPage />,
              },
              {
                path: Routes.PROJECT_DATASET_DETAILS,
                element: <DataSetDetailPage />,
              },
              {
                path: Routes.PROJECT_SETTINGS,
                element: <ProjectSettingsPage />,
              },
            ],
          },
        ],
      },
      {
        path: '*',
        element: <div>404 Not Found</div>,
      },
    ],
  },
]);
