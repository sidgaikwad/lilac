import { queryOptions, useQuery } from '@tanstack/react-query'; // Import UseQueryOptions
import { QueryKeys } from '../constants';
import { ApiError, Organization } from '@/types';
import { useEffect } from 'react';
import { get } from '@/lib/fetch';

export interface ListOrganizationsResponse {
  organizations: {
    id: string;
    name: string;
  }[];
}

export async function listOrganizations(): Promise<ListOrganizationsResponse> {
  return get('/organizations');
}

export function listOrganizationsQuery(enabled: boolean = true) {
  return queryOptions({
    queryKey: [QueryKeys.LIST_ORGANIZATIONS],
    queryFn: () => listOrganizations(),
    enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data.organizations as Organization[],
  });
}

interface UseListOrganizationsProps {
  enabled?: boolean;
  onSuccess?: (organizations: Organization[]) => void;
  onError?: (error: ApiError) => void;
}

export function useListOrganizations(props?: UseListOrganizationsProps) {
  const query = useQuery(listOrganizationsQuery(props?.enabled));

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
