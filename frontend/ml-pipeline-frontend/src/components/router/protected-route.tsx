import { Outlet } from 'react-router-dom';
import { Navigate } from 'react-router-dom';
import useAuthStore from '@/store/use-auth-store';

function ProtectedRoute() {
  const { token } = useAuthStore();

  if (!token) {
    return <Navigate to='/login' replace />;
  }

  return <Outlet />;
}

export default ProtectedRoute;
