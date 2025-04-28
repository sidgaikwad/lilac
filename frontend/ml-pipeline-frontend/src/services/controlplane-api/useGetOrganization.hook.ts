import { useQuery } from '@tanstack/react-query';
import { BASE_URL, QueryKeys } from './constants';
import { ApiError, Organization } from '@/types';
import { useEffect } from 'react';

interface UseGetOrganizationProps {
  organizationId: string | undefined;
  enabled?: boolean;
  onSuccess?: (project: Organization) => void;
  onError?: (error: ApiError) => void;
}

// Mock function simulating API call for pipeline details
const fetchOrganizationDetail = async (
  id: string
): Promise<Organization | undefined> => {
  const resp = await fetch(`${BASE_URL}/organizations/${id}`, {
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
  return await resp.json();
};

export function useGetOrganization(props: UseGetOrganizationProps) {
  const query = useQuery<Organization | undefined, ApiError>({
    queryKey: [QueryKeys.GET_ORGANIZATION, props.organizationId],
    queryFn: () => fetchOrganizationDetail(props.organizationId!),
    enabled: !!props.organizationId,
    staleTime: 1000 * 60 * 5, // 5 minutes
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
