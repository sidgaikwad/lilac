import { useQuery } from '@tanstack/react-query';
import { getHttp } from '@/lib/fetch';
import { User } from '@/types/api/user';

export const useCurrentUser = () => {
  return useQuery<User>({
    queryKey: ['currentUser'],
    queryFn: () => getHttp('/account/details'),
  });
};