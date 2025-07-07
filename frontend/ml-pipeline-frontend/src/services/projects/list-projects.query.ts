import { queryOptions, useQuery } from '@tanstack/react-query'; // Import UseQueryOptions
import { QueryKeys } from '../constants';
import { ApiError, Project } from '@/types';
import { useEffect } from 'react';
import { getHttp } from '@/lib/fetch';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';

export interface ListProjectsResponse {
  projects: {
    id: string;
    name: string;
  }[];
}

export async function listProjects(
): Promise<ListProjectsResponse> {
  const resp = await getHttp<Sn<ListProjectsResponse>>('/projects');
  return {
    projects: resp.projects.map((proj) => ({
      id: proj.id,
      name: proj.name,
    })),
  };
}

export function listProjectsQuery(
  enabled: boolean = true
) {
  return queryOptions({
    queryKey: [QueryKeys.LIST_PROJECTS],
    queryFn: () => listProjects(),
    enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data.projects as Project[],
  });
}

interface UseListProjectsProps {
  enabled?: boolean;
  onSuccess?: (projects: Project[]) => void;
  onError?: (error: ApiError) => void;
}

export function useListProjects(props?: UseListProjectsProps) {
  const query = useQuery(
    listProjectsQuery(props?.enabled)
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
