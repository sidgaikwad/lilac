import { createBrowserRouter, Navigate } from 'react-router';
import { Routes } from './constants/routes';
import { MainLayout, ProjectLayout } from './components/layout';
import ProjectsListPage from './features/projects/pages/projects-list-page';
import ExperimentsPage from './features/experiments/pages/experiments-page';
import ProjectWorkspacesPage from './features/workspaces/pages/project-workspaces-page';
import ProjectDashboardPage from './features/projects/pages/project-dashboard-page';
import WorkspaceViewPage from './features/workspaces/pages/workspace-view-page';
import ProtectedRoute from './components/router/protected-route';
import LoginPage from './features/account/pages/login-page';
import SignUpPage from './features/account/pages/sign-up-page';
import ErrorBoundary from './error-boundary';
import AccountSettingsPage from './features/account/pages/account-settings-page';
import ProjectSettingsPage from './features/projects/pages/project-settings-page';
import ClustersPage from './features/clusters/pages/clusters-page';
import { OrgSettings } from './features/settings/pages/org-settings';

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
                element: <Navigate to={Routes.PROJECTS} replace />,
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
                element: <ClustersPage />,
              },
              {
                path: Routes.ORG_SETTINGS,
                element: <OrgSettings />,
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
                path: Routes.PROJECT_SETTINGS,
                element: <ProjectSettingsPage />,
              },
              {
                path: Routes.PROJECT_EXPERIMENTS,
                element: <ExperimentsPage />,
              },
              {
                path: Routes.PROJECT_WORKSPACES,
                element: <ProjectWorkspacesPage />,
              },
              {
                path: Routes.PROJECT_WORKSPACES + '/:workspaceId',
                element: <WorkspaceViewPage />,
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
