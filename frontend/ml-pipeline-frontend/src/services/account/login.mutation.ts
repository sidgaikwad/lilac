import { useMutation } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { AuthToken } from '@/types/api/auth';
import useAuthStore from '@/store/use-auth-store';
import { ApiError } from '@/types';
import { postHttp } from '@/lib/fetch';

interface LoginRequest {
  email: string;
  password: string;
}

async function login(payload: LoginRequest): Promise<AuthToken> {
  return postHttp('/auth/login', payload, false);
}

export interface UseLoginProps {
  onSuccess?: (token: AuthToken) => void;
  onError?: (error: ApiError) => void;
}

export function useLogin(props: UseLoginProps) {
  const { login: loginAuthStore } = useAuthStore();

  const mutation = useMutation({
    mutationKey: [QueryKeys.LOGIN],
    mutationFn: login, // Use the mock function
    onSuccess: (data) => {
      loginAuthStore(data.accessToken);

      if (props.onSuccess) {
        props.onSuccess(data);
      }
    },
    onError: (error) => {
      if (props.onError) {
        props.onError(error);
      }
    },
  });

  return mutation;
}
