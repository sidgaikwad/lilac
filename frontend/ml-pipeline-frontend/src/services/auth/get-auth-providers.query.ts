import { useQuery } from '@tanstack/react-query';
import { API_URL } from '../constants';
import { Provider } from '@/types';

const fetchAuthProviders = async (): Promise<Provider[]> => {
  const response = await fetch(`${API_URL}/auth/providers`);
  if (!response.ok) {
    throw new Error('Failed to fetch auth providers');
  }
  return response.json();
};

export const useGetAuthProviders = () => {
  return useQuery({
    queryKey: ['authProviders'],
    queryFn: fetchAuthProviders,
  });
};
