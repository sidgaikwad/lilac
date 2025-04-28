import { useMutation } from '@tanstack/react-query';
import { BASE_URL } from './constants';
import { ApiError } from '@/types';

async function registerUser(payload: {
  email: string;
  password: string;
}): Promise<{ userId: string }> {
  const resp = await fetch(`${BASE_URL}/users`, {
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
  return await resp.json();
}

export interface UseRegisterUserProps {
  onSuccess?: (userId: string) => void;
  onError?: (error: ApiError) => void;
}

export function useRegisterUser(props: UseRegisterUserProps) {
  return useMutation({
    mutationFn: registerUser,
    onSuccess:
      props.onSuccess !== undefined
        ? (data) => props.onSuccess!(data.userId)
        : undefined,
    onError:
      props.onError !== undefined
        ? (error) => props.onError!(error)
        : undefined,
  });
}
