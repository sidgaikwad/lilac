import { getHttp } from '@/lib/fetch';
import { queryOptions, useQuery } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { ApiError, Pipeline } from '@/types';
import { useEffect } from 'react';

export interface GetPipelineResponse {
  id: string;
  name: string;
  description?: string;
  projectId: string;
  steps: {
    stepId: string;
    stepDefinitionId: string;
    stepType: string;
    stepParameters: Record<string, string | number | boolean | object>;
  }[];
  stepConnections: [string, string][];
}

export async function getPipeline(
  pipelineId: string
): Promise<GetPipelineResponse> {
  return getHttp(`/pipelines/${pipelineId}`);
}

export function getPipelineQuery(pipelineId?: string, enabled: boolean = true) {
  return queryOptions({
    queryKey: [QueryKeys.GET_PIPELINE, pipelineId],
    queryFn: () => getPipeline(pipelineId!),
    enabled: !!pipelineId && enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data as Pipeline,
  });
}

interface UseGetPipelineProps {
  pipelineId: string | undefined;
  enabled?: boolean;
  onSuccess?: (pipeline: Pipeline) => void;
  onError?: (error: ApiError) => void;
}

export function useGetPipeline(props: UseGetPipelineProps) {
  const query = useQuery(getPipelineQuery(props.pipelineId, props.enabled));

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
