import { createBrowserRouter } from 'react-router';
import { Routes } from './constants/routes';
import { MainLayout, ProjectLayout } from './components/layout';
import ProjectsListPage from './features/projects/pages/projects-list-page';
import ExperimentsPage from './features/experiments/pages/experiments-page';
import NotebooksPage from './features/notebooks/pages/notebooks-page';
import ProjectWorkspacesPage from './features/workspaces/pages/project-workspaces-page';
import ProjectDashboardPage from './features/projects/pages/project-dashboard-page';
import ProtectedRoute from './components/router/protected-route';
import LoginPage from './features/account/pages/login-page';
import SignUpPage from './features/account/pages/sign-up-page';
import SsoCallbackPage from './features/account/pages/sso-callback-page';
import ErrorBoundary from './error-boundary';
import AccountSettingsPage from './features/account/pages/account-settings-page';
import ProjectSettingsPage from './features/projects/pages/project-settings-page';
import DataSetsPage from './features/datasets/pages/datasets-page';

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
        path: '/auth/callback/:type/:provider',
        element: <SsoCallbackPage />,
      },
      {
        element: <ProtectedRoute />,
        children: [
          {
            element: <MainLayout />,
            children: [
              {
                index: true,
                element: <ProjectsListPage />,
              },
              {
                path: Routes.PROJECTS,
                element: <ProjectsListPage />,
              },
              {
                path: Routes.DATA_SOURCES,
                element: <div>TODO</div>,
              },
              {
                path: Routes.CLUSTERS,
                element: <div>TODO</div>,
              },
              {
                path: Routes.ACCOUNT_SETTINGS,
                element: <AccountSettingsPage />,
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
                path: Routes.PROJECT_WORKSPACES,
                element: <div>TODO</div>,
              },
              {
                path: Routes.PROJECT_DATASETS,
                element: <DataSetsPage />,
              },
              {
                path: Routes.PROJECT_SETTINGS,
                element: <ProjectSettingsPage />,
              },
              {
                path: Routes.PROJECT_EXPERIMENTS,
                element: <ExperimentsPage />,
              },
              {
                path: Routes.PROJECT_NOTEBOOKS,
                element: <NotebooksPage />,
              },
              {
                path: Routes.PROJECT_WORKSPACES,
                element: <ProjectWorkspacesPage />,
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
