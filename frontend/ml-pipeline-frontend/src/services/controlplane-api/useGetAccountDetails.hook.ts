import { useQuery } from '@tanstack/react-query';
import { QueryKeys } from './constants';
import { ApiError, User } from '@/types';
import { useEffect } from 'react';
import { get } from '@/lib/fetch';

interface UseGetAccountDetailsProps {
  onSuccess?: (user: User) => void;
  onError?: (error: ApiError) => void;
}

// Mock function simulating API call for pipeline details
const fetchAccountDetails = async (): Promise<User | undefined> => {
  return await get('/account/details');
};

export function useGetAccountDetails(props: UseGetAccountDetailsProps) {
  const query = useQuery<User | undefined, ApiError>({
    queryKey: [QueryKeys.GET_ACCOUNT_DETAILS],
    queryFn: () => fetchAccountDetails(),
    enabled: true,
  });

  useEffect(() => {
    if (props?.onSuccess && query.data != undefined) {
      props.onSuccess(query.data);
    }
  }, [props, query.data]);

  useEffect(() => {
    if (props?.onError && query.error != null) {
      props.onError(query.error);
    }
  }, [props, query.error]);

  return query;
}
