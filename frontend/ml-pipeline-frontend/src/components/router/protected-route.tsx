import { Outlet } from 'react-router-dom'; // Import Outlet
import { Navigate } from 'react-router-dom';
import useAuthStore from '@/store/use-auth-store';
import { useGetAccountDetails } from '@/services';
import { Spinner } from '../ui/spinner';

/**
 * A route guard that checks authentication status.
 * Renders the MainLayout with nested routes (via Outlet) if authenticated,
 * otherwise redirects to the login page.
 * Shows a loading indicator during the initial auth check.
 */
function ProtectedRoute() {
  const { token, logout } = useAuthStore();
  const { isLoading, isError, error } = useGetAccountDetails({});

  if (!token || (isError && error.statusCode === 401)) {
    logout();
    return <Navigate to='/login' replace />;
  }

  if (isLoading) {
    return (<div className='w-full h-full'>
     <Spinner size='large'>Loading</Spinner>
    </div>);
  }

  // User is authenticated, render the nested routes via Outlet.
  // The correct layout (MainLayout or ProjectLayout) will be rendered by the
  // specific route configuration within App.tsx.
  return <Outlet />;
}

export default ProtectedRoute;
