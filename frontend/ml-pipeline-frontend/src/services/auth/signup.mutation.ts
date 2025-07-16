import { useMutation } from '@tanstack/react-query';
import { ServiceError } from '@/types';
import { postHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';
import { camelCaseObject } from '@/lib/utils';

export interface SignUpRequest {
  email: string;
  password: string;
}

export interface SignUpResponse {
  userId: string;
}

export async function signUp(payload: SignUpRequest): Promise<SignUpResponse> {
  return camelCaseObject(await postHttp<Sn<SignUpRequest>, Sn<SignUpResponse>>('/auth/signup', payload));
}

export interface UseSignUpProps {
  onSuccess?: (userId: string) => void;
  onError?: (error: ServiceError) => void;
}

export function useSignUp(props: UseSignUpProps) {
  return useMutation({
    mutationKey: [QueryKeys.SIGN_UP],
    mutationFn: signUp,
    onSuccess: (data) => {
      if (props.onSuccess) {
        props.onSuccess(data.userId);
      }
    },
    onError: props.onError,
  });
}
