import { useQuery } from '@tanstack/react-query'; // Import UseQueryOptions
import { BASE_URL, QueryKeys } from './constants';
import { ApiError, Organization } from '@/types';
import { useEffect } from 'react';
import { ListOrganizationsResponse } from './types';

// Define options type, making 'enabled' optional
interface UseListOrganizationsOptions {
  enabled?: boolean;
  onSuccess?: (organizations: Organization[]) => void;
  onError?: (error: ApiError) => void;
}

// Mock function simulating API call
const fetchOrganizationsMock = async (): Promise<Organization[]> => {
  const resp = await fetch(`${BASE_URL}/organizations`, {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${localStorage.getItem('token')}`,
    },
  });
  if (resp.status < 200 || resp.status >= 300) {
    return Promise.reject({
      statusCode: resp.status,
      ...(await resp.json()),
    });
  }
  const body: ListOrganizationsResponse = await resp.json();
  return body.organizations.map((org) => ({
    id: org.id,
    name: org.name,
  }));
};

// Accept optional options object
export function useListOrganizations(props?: UseListOrganizationsOptions) {
  const query = useQuery<Organization[], ApiError>({
    queryKey: [QueryKeys.LIST_ORGANIZATIONS],
    queryFn: fetchOrganizationsMock,
    staleTime: 1000 * 60 * 5, // 5 minutes
    enabled: props?.enabled ?? true,
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
