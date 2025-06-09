import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ApiError } from '@/types';
import { postHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';

export interface RunPipelineRequest {
  pipelineId: string;
  datasetId: string;
}

export interface RunPipelineResponse {
  id: string;
}

async function runPipeline(
  payload: RunPipelineRequest
): Promise<RunPipelineResponse> {
  const { pipelineId, ...request } = payload;
  return postHttp<
    Sn<Omit<RunPipelineRequest, 'pipelineId'>>,
    Sn<RunPipelineResponse>
  >(`/pipelines/${pipelineId}/run`, {
    dataset_id: request.datasetId,
  });
}

export interface UseRunPipelineProps {
  onSuccess?: (data: RunPipelineResponse) => void;
  onError?: (error: ApiError) => void;
}

export function useRunPipeline(props: UseRunPipelineProps) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationKey: [QueryKeys.RUN_PIPELINE],
    mutationFn: runPipeline,
    onSuccess: (data) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_JOBS],
      });
      if (props.onSuccess !== undefined) {
        props.onSuccess(data);
      }
    },
    onError: props.onError,
  });
}
