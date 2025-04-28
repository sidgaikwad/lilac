import { useMutation } from '@tanstack/react-query';
import { BASE_URL, QueryKeys } from './constants';
import { AuthToken } from '@/types/api/auth';
import { useNavigate } from 'react-router-dom';
import { toast } from 'sonner';
import useAuthStore from '@/store/useAuthStore';
import { ApiError, User } from '@/types';
import { LoginResponse } from './types';

export interface UseAuthProps {
  redirectTo?: string;
  onSuccess?: (token: AuthToken[]) => void;
  onError?: (error: ApiError) => void;
}

// Mock function for login - always succeeds for dev
async function fetchLogin(payload: {
  email: string;
  password: string;
}): Promise<{ token: AuthToken; user: User }> {
  const resp = await fetch(`${BASE_URL}/auth/login`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      email: payload.email,
      password: payload.password,
    }),
  });
  if (resp.status < 200 || resp.status >= 300) {
    return Promise.reject({
      statusCode: resp.status,
      ...(await resp.json()),
    });
  }
  const loginResp: LoginResponse = await resp.json();
  const userResp = await fetch(`${BASE_URL}/account/details`, {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${loginResp.accessToken}`,
    },
  });
  if (userResp.status < 200 || userResp.status >= 300) {
    return Promise.reject({
      statusCode: userResp.status,
      ...(await userResp.json()),
    });
  }
  const user: User = await userResp.json();
  return { token: loginResp, user: user };
}

export function useLogin(props: UseAuthProps) {
  const { login } = useAuthStore();
  const navigate = useNavigate();

  const mutation = useMutation({
    mutationKey: [QueryKeys.LOGIN],
    mutationFn: fetchLogin, // Use the mock function
    onSuccess: (data) => {
      // Set auth state directly here as well, using email from input if needed
      login(data.user, data.token.accessToken);

      if (props.redirectTo) {
        navigate(props.redirectTo, { replace: true });
      } else {
        navigate('/', { replace: true });
      }
    },
    onError: (error) => {
      // This shouldn't happen with the mock function, but keep for safety
      if (error.statusCode === 401) {
        toast.error('Login failed', {
          description: 'Incorrect username or password',
        });
      } else {
        toast.error('Login failed', {
          description: 'Something went wrong',
        });
      }
    },
  });

  return mutation;
}
