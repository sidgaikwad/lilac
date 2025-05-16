import { createBrowserRouter } from 'react-router';
import { Routes } from './constants';
import OrganizationsOverviewPage from './features/organizations/OrganizationsOverviewPage';
import { MainLayout } from './components/layout';
import ProjectsByOrgPage from './features/projects/ProjectsByOrgPage';
import ProjectOverviewPage from './features/projects/ProjectOverviewPage';
import PipelinesOverviewPage from './features/pipelines/pages/PipelinesOverviewPage';
import PipelineEditorPage from './features/pipelines/pages/PipelineEditorPage';
import DataSetsPage from './features/datasets/pages/DataSetsPage';
import DataSetDetailPage from './features/datasets/pages/DataSetDetailPage';
import ProtectedRoute from './components/router/ProtectedRoute';
import LoginPage from './features/auth/pages/LoginPage';
import SignUpPage from './features/auth/pages/SignUpPage';
import ErrorBoundary from './error-boundary';

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
                element: <OrganizationsOverviewPage />,
              },
              {
                path: Routes.HOME,
                element: <OrganizationsOverviewPage />,
              },
              {
                path: Routes.PROJECTS,
                element: <ProjectsByOrgPage />,
              },
              {
                path: Routes.PROJECT_DETAILS,
                element: <ProjectOverviewPage />,
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
