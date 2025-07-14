import { useMutation } from '@tanstack/react-query';
import { toast } from '@/components/toast';
import useAuthStore from '@/store/use-auth-store';
import { useNavigate } from 'react-router-dom';
import { postHttp } from '@/lib/fetch';
interface SsoLoginRequest {
  provider: string;
  type: string;
}

interface SsoLoginResponse {
  authorization_url: string;
}

export const useSsoLogin = () => {
  return useMutation<SsoLoginResponse, Error, SsoLoginRequest>({
    mutationFn: (request) =>
      postHttp(`/auth/${request.type}/${request.provider}/login`, {}),
    onSuccess: (data) => {
      if (data.authorization_url) {
        window.location.href = data.authorization_url;
      }
    },
    onError: (error) => {
      toast.error('Login failed', {
        description: error.message,
      });
    },
  });
};

interface SsoExchangeRequest {
  code: string;
  state: string;
  provider: string;
  type: string;
}

interface SsoExchangeResponse {
  access_token: string;
}

interface ServiceError {
  error: string;
  statusCode: number;
}

export const useSsoExchange = () => {
  const navigate = useNavigate();
  const { setToken } = useAuthStore();

  return useMutation<SsoExchangeResponse, ServiceError, SsoExchangeRequest>({
    mutationFn: (request) =>
      postHttp(`/auth/${request.type}/${request.provider}/exchange`, {
        code: request.code,
        state: request.state,
      }),
    onSuccess: (data) => {
      setToken(data.access_token);
      navigate('/');
    },
    onError: (error) => {
      useAuthStore.getState().clearToken();
      setTimeout(() => {
        toast.error('Login failed', {
          description: error.error,
        });
      }, 50);
      navigate(`/login`);
    },
  });
};
