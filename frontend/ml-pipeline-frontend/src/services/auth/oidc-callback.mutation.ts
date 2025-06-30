import { useMutation } from '@tanstack/react-query';
import { postHttp } from '@/lib/fetch';

export const oidcCallback = async (params: { code: string; state: string }) => {
  return postHttp('/auth/oidc/callback', params);
};

export const useOidcCallback = (options: any = {}) => {
  return useMutation({
    mutationFn: oidcCallback,
    ...options,
  });
};