import { useEffect, useState } from 'react'; // Added useState back
import { Routes, Route, Navigate, useNavigate } from 'react-router-dom';
import useAuthStore from './store/authStore';
import useOrganizationStore from './store/organizationStore';
import ProtectedRoute from './components/router/ProtectedRoute';
import { useQuery } from '@tanstack/react-query'; // Re-added useQuery
import { checkAuth } from './features/auth/services/authService'; // Re-added checkAuth
import { User, ApiError } from '@/types';
import '@tanstack/react-query'
// import { toast } from 'sonner'; // Keep removed

// Import hook for initial org check
import { useListOrganizations } from '@/services/controlplane-api/useListOrganizations.hook';

// Import Layouts
import MainLayout from './components/layout/MainLayout';

// Import Feature Pages
import LoginPage from './features/auth/pages/LoginPage';
import ProjectPipelinesPage from './features/pipelines/pages/ProjectPipelinesPage';
import PipelineEditorPage from './features/pipelines/pages/PipelineEditorPage';
import DataSetsPage from './features/datasets/pages/DataSetsPage';
import DataSetDetailPage from './features/datasets/pages/DataSetDetailPage';
import SettingsPage from './features/settings/pages/SettingsPage';
import AccountSettingsPage from './features/settings/pages/AccountSettingsPage';
import OrganizationSettingsPage from './features/settings/pages/OrganizationSettingsPage';
import { Loader2Icon } from 'lucide-react';
import { Button } from './components/ui/button';

declare module '@tanstack/react-query' {
  interface Register {
    defaultError: ApiError
  }
}

const NotFoundPage = () => <div>404 Not Found</div>;

// Simplified InitialRedirect
const InitialRedirect: React.FC = () => {
    const selectedProject = useOrganizationStore(state => state.selectedProject);
    const isAuthenticated = useAuthStore(state => state.isAuthenticated);
    const navigate = useNavigate();
    const [hasNavigated, setHasNavigated] = useState(false);

    useEffect(() => {
        // Only navigate if authenticated, selectedProject is determined (not undefined), and navigation hasn't happened
        if (isAuthenticated && selectedProject !== undefined && !hasNavigated) {
            if (selectedProject) {
                console.log(`InitialRedirect: Navigating to project ${selectedProject.id}`);
                navigate(`/projects/${selectedProject.id}/pipelines`, { replace: true });
                setHasNavigated(true);
            } else {
                console.log("InitialRedirect: No project selected after init, navigating to settings.");
                navigate('/settings', { replace: true });
                setHasNavigated(true);
            }
        }
    }, [isAuthenticated, selectedProject, navigate, hasNavigated]);

    // Show loader only while waiting for project state to be determined after authentication
    if (!isAuthenticated || selectedProject === undefined || !hasNavigated) {
        return (
            <div className="flex justify-center items-center h-screen">
                <Loader2Icon className="h-8 w-8 animate-spin text-primary" />
            </div>
        );
    }
    return null; // Render nothing once navigation is attempted or project state is known
};

// Placeholder for No Orgs component
const NoOrganizationsFound: React.FC = () => {
    const navigate = useNavigate();
    return (
        <div className="flex flex-col items-center justify-center h-screen gap-4 p-4 text-center">
            <h2 className="text-xl font-semibold">No Organizations Found</h2>
            <p className="text-muted-foreground">Create or be invited to an organization.</p>
            <Button onClick={() => navigate('/settings/organization')}>Go to Organization Settings</Button>
        </div>
    );
};

// Placeholder Full Page Loader
const FullPageLoader: React.FC<{ message?: string }> = ({ message = "Loading..." }) => (
    <div className="flex justify-center items-center h-screen">
        <Loader2Icon className="h-8 w-8 animate-spin text-primary" />
        <span className="ml-3 text-lg">{message}</span>
    </div>
);


function App() {
  const setAuthState = useAuthStore(state => state.setAuthState);
  const setLoading = useAuthStore(state => state.setLoading);
  const token = useAuthStore(state => state.token); // Use token to enable/disable checkAuth
  const isAuthenticated = useAuthStore(state => state.isAuthenticated);

  const setSelectedOrganization = useOrganizationStore(state => state.setSelectedOrganization);
  const setSelectedProject = useOrganizationStore(state => state.setSelectedProject);

  // Restore checkAuth query - it will run if a token exists (even mock) but likely fail
  const { data: user, isSuccess: isAuthSuccess, isError: isAuthError, isLoading: isCheckingAuth } = useQuery<User, Error>({
    queryKey: ['checkAuth'],
    queryFn: checkAuth,
    enabled: !!token, // Enable if token exists
    retry: false,
    refetchOnWindowFocus: false,
  });

  // Fetch organizations only if authenticated
  const { data: organizations, isLoading: isLoadingOrgs, isError: isOrgError } = useListOrganizations({
      enabled: isAuthenticated,
  });

  // Update auth state based on query result or lack of token
  useEffect(() => {
    if (isAuthSuccess && user) {
      // If checkAuth succeeds (e.g., valid token from previous session), update state
      setAuthState(true, user, localStorage.getItem('token'));
    } else if (isAuthError) {
      // If checkAuth fails (invalid token), clear auth state only if not already authenticated (by mock login)
       if (!isAuthenticated) {
            console.log("checkAuth failed, clearing auth state.");
            setAuthState(false, null, null);
            setSelectedOrganization(null);
            setSelectedProject(null);
       } else {
            console.log("checkAuth failed, but already authenticated (likely mock login). Ignoring.");
       }
    } else if (!token && !isCheckingAuth) {
      // If no token and not checking, ensure logged out state
       if (isAuthenticated) { // Clear state if somehow authenticated without token
           console.log("No token found, clearing auth state.");
           setAuthState(false, null, null);
           setSelectedOrganization(null);
           setSelectedProject(null);
       }
    }
  }, [isAuthSuccess, isAuthError, user, token, isCheckingAuth, isAuthenticated, setAuthState, setSelectedOrganization, setSelectedProject]);

  // Update global loading state
  useEffect(() => {
    // Loading is true if checking auth OR if authenticated but still loading initial orgs
    const overallLoading = isCheckingAuth || (isAuthenticated && isLoadingOrgs);
    setLoading(overallLoading);
  }, [isCheckingAuth, isAuthenticated, isLoadingOrgs, setLoading]);


  // Determine main content based on auth and org state
  const renderAppContent = () => {
      // If authenticated and failed to load orgs
      if (isAuthenticated && isOrgError) {
          return <FullPageLoader message="Error loading organizations..." />;
      }
      // If authenticated and orgs loaded, but none exist
      if (isAuthenticated && !isLoadingOrgs && organizations?.length === 0) {
          return <NoOrganizationsFound />;
      }
      // If authenticated and orgs exist (or still loading them), render main layout
      // The ProtectedRoute already handles the main loading state via useAuthStore().isLoading
      if (isAuthenticated) {
          return (
              <Routes>
                  <Route element={<MainLayout />}>
                      <Route index element={<InitialRedirect />} />
                      <Route path="projects/:projectId/home" element={<div>Project Home Placeholder</div>} />
                      <Route path="projects/:projectId/database" element={<div>Database Section Placeholder</div>} />
                      <Route path="projects/:projectId/pipelines" element={<ProjectPipelinesPage />} />
                      <Route path="projects/:projectId/pipelines/:pipelineId" element={<PipelineEditorPage />} />
                      <Route path="projects/:projectId/datasets" element={<DataSetsPage />} />
                      <Route path="projects/:projectId/datasets/:datasetId" element={<DataSetDetailPage />} />
                      <Route path="projects/:projectId/auth" element={<div>Auth Section Placeholder</div>} />
                      <Route path="projects/:projectId/storage" element={<div>Storage Section Placeholder</div>} />
                      <Route path="settings" element={<SettingsPage />}>
                          <Route index element={<Navigate to="account" replace />} />
                          <Route path="account" element={<AccountSettingsPage />} />
                          <Route path="organization" element={<OrganizationSettingsPage />} />
                      </Route>
                      <Route path="*" element={<NotFoundPage />} />
                  </Route>
              </Routes>
          );
      }
      // This case should ideally not be reached if ProtectedRoute works correctly
      return <FullPageLoader message="Verifying session..." />;
  }

  return (
    <Routes>
      <Route path="/login" element={<LoginPage />} />
      {/* ProtectedRoute uses isLoading and isAuthenticated from store */}
      <Route element={<ProtectedRoute />}>
          <Route path="/*" element={renderAppContent()} />
      </Route>
    </Routes>
  );
}

export default App;