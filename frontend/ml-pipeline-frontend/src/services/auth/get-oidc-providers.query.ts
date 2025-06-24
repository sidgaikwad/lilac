import { useQuery } from '@tanstack/react-query';
import { API_URL } from '../constants';

const fetchOidcProviders = async (): Promise<string[]> => {
  const response = await fetch(`${API_URL}/auth/oidc/providers`);
  if (!response.ok) {
    throw new Error('Failed to fetch OIDC providers');
  }
  return response.json();
};

export const useGetOidcProviders = () => {
  return useQuery({
    queryKey: ['oidcProviders'],
    queryFn: fetchOidcProviders,
  });
};