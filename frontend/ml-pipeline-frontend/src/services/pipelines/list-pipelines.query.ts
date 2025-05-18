import { queryOptions, useQuery } from '@tanstack/react-query'; // Import UseQueryOptions
import { QueryKeys } from '../constants';
import { ApiError, PipelineSummary } from '@/types';
import { useEffect } from 'react';
import { getHttp } from '@/lib/fetch';

export interface ListPipelinesResponse {
  pipelines: {
    id: string;
    name: string;
    description?: string;
  }[];
}

export async function listPipelines(
  projectId: string
): Promise<ListPipelinesResponse> {
  return getHttp(`/projects/${projectId}/pipelines`);
}

export function listPipelinesQuery(
  projectId?: string,
  enabled: boolean = true
) {
  return queryOptions({
    queryKey: [QueryKeys.LIST_PIPELINES, projectId],
    queryFn: () => listPipelines(projectId!),
    enabled: !!projectId && enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data.pipelines as PipelineSummary[],
  });
}

interface UseListPipelinesProps {
  projectId?: string;
  enabled?: boolean;
  onSuccess?: (pipelines: PipelineSummary[]) => void;
  onError?: (error: ApiError) => void;
}

export function useListPipelines(props?: UseListPipelinesProps) {
  const query = useQuery(listPipelinesQuery(props?.projectId, props?.enabled));

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
