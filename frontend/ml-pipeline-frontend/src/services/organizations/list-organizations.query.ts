import { queryOptions, useQuery } from '@tanstack/react-query'; // Import UseQueryOptions
import { QueryKeys } from '../constants';
import { ApiError, Organization } from '@/types';
import { useEffect } from 'react';
import { getHttp } from '@/lib/fetch';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';

export interface ListOrganizationsResponse {
  organizations: {
    organizationId: string;
    organizationName: string;
  }[];
}

export async function listOrganizations(): Promise<ListOrganizationsResponse> {
  const resp = await getHttp<Sn<ListOrganizationsResponse>>('/organizations');
  return {
    organizations: resp.organizations.map((org) => ({
      organizationId: org.organization_id,
      organizationName: org.organization_name,
    })),
  };
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
