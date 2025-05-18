import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ApiError } from '@/types';
import { postHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';

export interface CreatePipelineRequest {
  name: string;
  projectId: string;
}

export interface CreatePipelineResponse {
  id: string;
}

async function createPipeline(
  payload: CreatePipelineRequest
): Promise<CreatePipelineResponse> {
  const { projectId, ...request } = payload;
  return postHttp(`/projects/${projectId}/pipelines`, request);
}

export interface UseCreatePipelineProps {
  onSuccess?: (data: CreatePipelineResponse) => void;
  onError?: (error: ApiError) => void;
}

export function useCreatePipeline(props: UseCreatePipelineProps) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationKey: [QueryKeys.CREATE_PIPELINE],
    mutationFn: createPipeline,
    onSuccess: (data) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_PROJECTS],
      });
      if (props.onSuccess !== undefined) {
        props.onSuccess(data);
      }
    },
    onError: props.onError,
  });
}
