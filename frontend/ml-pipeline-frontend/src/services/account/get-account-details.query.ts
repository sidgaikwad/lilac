import { queryOptions, useQuery } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { ApiError, User } from '@/types';
import { useEffect } from 'react';
import { get } from '@/lib/fetch';

export interface GetAccountDetailsResponse {
  id: string;
  email: string;
  emailVerified: boolean;
}

export async function getAccountDetails(): Promise<GetAccountDetailsResponse> {
  return get('/account/details');
}

export function getAccountDetailsQuery(enabled: boolean = true) {
  return queryOptions({
    queryKey: [QueryKeys.GET_ACCOUNT_DETAILS],
    queryFn: getAccountDetails,
    enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data as User,
  });
}

interface UseGetAccountDetailsProps {
  enabled?: boolean;
  onSuccess?: (user: User) => void;
  onError?: (error: ApiError) => void;
}

export function useGetAccountDetails(props: UseGetAccountDetailsProps) {
  const query = useQuery(getAccountDetailsQuery(props.enabled));

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
