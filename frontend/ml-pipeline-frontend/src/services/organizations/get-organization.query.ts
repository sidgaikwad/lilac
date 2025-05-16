import { get } from '@/lib/fetch';
import { queryOptions, useQuery } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { ApiError, Organization } from '@/types';
import { useEffect } from 'react';

interface GetOrganizationResponse {
  id: string;
  name: string;
}

export async function getOrganization(
  organizationId: string
): Promise<GetOrganizationResponse> {
  return get(`/organizations/${organizationId}`);
}

export function getOrganizationQuery(
  organizationId?: string,
  enabled: boolean = true
) {
  return queryOptions({
    queryKey: [QueryKeys.GET_ORGANIZATION, organizationId],
    queryFn: () => getOrganization(organizationId!),
    enabled: !!organizationId && enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data as Organization,
  });
}

interface UseGetOrganizationProps {
  organizationId: string | undefined;
  enabled?: boolean;
  onSuccess?: (project: Organization) => void;
  onError?: (error: ApiError) => void;
}

export function useGetOrganization(props: UseGetOrganizationProps) {
  const query = useQuery(
    getOrganizationQuery(props.organizationId, props.enabled)
  );

  useEffect(() => {
    if (props?.onSuccess && query.data !== undefined) {
      props.onSuccess(query.data);
    }
  }, [props, query.data]);

  useEffect(() => {
    if (props?.onError && query.error !== null) {
      props.onError(query.error);
    }
  }, [props, query.error]);

  return query;
}
