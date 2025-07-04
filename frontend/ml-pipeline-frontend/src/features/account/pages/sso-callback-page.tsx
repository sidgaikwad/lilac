import { useEffect } from 'react';
import { useLocation, useNavigate, useParams } from 'react-router-dom';
import { toast } from 'sonner';
import useAuthStore from '@/store/use-auth-store';

function SsoCallbackPage() {
  const location = useLocation();
  const navigate = useNavigate();
  const { provider, type } = useParams();
  const { setToken } = useAuthStore();

  useEffect(() => {
    const params = new URLSearchParams(location.search);
    const code = params.get('code');
    const state = params.get('state');
    const error = params.get('error');

    if (error) {
      navigate(`/login?error=${error}`);
      return;
    }

    if (code && state && provider && type) {
      fetch(`/api/auth/${type}/${provider}/exchange`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          code,
          state,
        }),
      })
        .then((response) => {
          if (!response.ok) {
            return response.json().then((err) => {
              throw new Error(err.error);
            });
          }
          return response.json();
        })
        .then((data) => {
          console.log('Received data:', data);
          setToken(data.access_token);
          navigate('/organizations');
        })
        .catch((err) => {
          toast.error(err.message);
          navigate('/login');
        });
    }
  }, [location, navigate, setToken, provider, type]);

  return <div>Loading...</div>;
}

export default SsoCallbackPage;
