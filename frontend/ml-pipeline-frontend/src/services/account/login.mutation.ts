import { useMutation } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { AuthToken } from '@/types/api/auth';
import useAuthStore from '@/store/use-auth-store';
import { ApiError } from '@/types';
import { postHttp } from '@/lib/fetch';
import { SnakeCasedPropertiesDeep as Sn } from 'type-fest';

interface LoginRequest {
  email: string;
  password: string;
}

async function login(payload: LoginRequest): Promise<AuthToken> {
  const resp = await postHttp<Sn<LoginRequest>, Sn<AuthToken>>(
    '/auth/login',
    payload
  );
  return {
    tokenType: resp.token_type,
    accessToken: resp.access_token,
  };
}

export interface UseLoginProps {
  onSuccess?: (token: AuthToken) => void;
  onError?: (error: ApiError) => void;
}

export function useLogin(props: UseLoginProps) {
  const { setToken } = useAuthStore();

  const mutation = useMutation({
    mutationKey: [QueryKeys.LOGIN],
    mutationFn: login,
    onSuccess: (data) => {
      setToken(data.accessToken);

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
