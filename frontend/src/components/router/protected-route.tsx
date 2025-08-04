import { Outlet, useNavigate } from 'react-router-dom';
import useAuthStore from '@/store/use-auth-store';
import { useGetAccountDetails } from '@/services';
import { useEffect } from 'react';

function ProtectedRoute() {
  const { token, clearToken } = useAuthStore();
  const { isError, error } = useGetAccountDetails({});
  const navigate = useNavigate();

  useEffect(() => {
    if (!token || (isError && error.statusCode === 401)) {
      console.error(error);
      clearToken();
      navigate('/login', { replace: true });
    }
  }, [token, clearToken, isError, error, navigate]);

  return <Outlet />;
}

export default ProtectedRoute;
