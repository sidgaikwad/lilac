import { useEffect } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import useAuthStore from '@/store/use-auth-store';

function OidcCallbackPage() {
  const location = useLocation();
  const navigate = useNavigate();
  const { setToken } = useAuthStore();

  useEffect(() => {
    const params = new URLSearchParams(location.search);
    const token = params.get('token');

    if (token) {
      setToken(token);
      navigate('/organizations');
    }
  }, [location, navigate, setToken]);

  return <div>Loading...</div>;
}

export default OidcCallbackPage;