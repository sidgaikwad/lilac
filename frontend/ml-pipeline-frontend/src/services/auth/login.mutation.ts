import { useMutation } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { AuthToken } from '@/types/api/auth';
import useAuthStore from '@/store/useAuthStore';
import { ApiError } from '@/types';
import { post } from '@/lib/fetch';

interface LoginRequest {
  email: string;
  password: string;
}

async function login(payload: LoginRequest): Promise<AuthToken> {
  return post('/auth/login', payload, false);
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
