import { queryOptions, useQuery } from '@tanstack/react-query'; // Import UseQueryOptions
import { QueryKeys } from '../constants';
import { ApiError, Project } from '@/types';
import { useEffect } from 'react';
import { getHttp } from '@/lib/fetch';

export interface ListProjectsResponse {
  projects: {
    id: string;
    name: string;
    organizationId: string;
  }[];
}

export async function listProjects(
  organizationId?: string
): Promise<ListProjectsResponse> {
  return getHttp('/projects', organizationId ? { organizationId } : undefined);
}

export function listProjectsQuery(
  organizationId?: string,
  enabled: boolean = true
) {
  return queryOptions({
    queryKey: [QueryKeys.LIST_PROJECTS, organizationId],
    queryFn: () => listProjects(organizationId!),
    enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data.projects as Project[],
  });
}

interface UseListProjectsProps {
  organizationId?: string;
  enabled?: boolean;
  onSuccess?: (projects: Project[]) => void;
  onError?: (error: ApiError) => void;
}

export function useListProjects(props?: UseListProjectsProps) {
  const query = useQuery(
    listProjectsQuery(props?.organizationId, props?.enabled)
  );

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
