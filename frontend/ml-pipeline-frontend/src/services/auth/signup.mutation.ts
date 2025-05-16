import { useMutation } from '@tanstack/react-query';
import { ApiError } from '@/types';
import { post } from '@/lib/fetch';
import { QueryKeys } from '../constants';

export interface SignUpRequest {
  email: string;
  password: string;
}

export interface SignUpResponse {
  userId: string;
}

export async function signUp(payload: SignUpRequest): Promise<SignUpResponse> {
  return post('/auth/signup', payload, false);
}

export interface UseSignUpProps {
  onSuccess?: (userId: string) => void;
  onError?: (error: ApiError) => void;
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
