import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ApiError } from '@/types';
import { postHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';

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
  const resp = await postHttp<
    Sn<CreatePipelineRequest>,
    Sn<CreatePipelineResponse>
  >('/pipelines', {
    name: payload.name,
    project_id: payload.projectId,
  });
  return {
    id: resp.id,
  };
}

export interface UseCreatePipelineProps {
  onSuccess?: (data: CreatePipelineResponse) => void;
  onError?: (error: ApiError) => void;
}

export function useCreatePipeline(props?: UseCreatePipelineProps) {
  const queryClient = useQueryClient();
  return useMutation<CreatePipelineResponse, ApiError, CreatePipelineRequest>({
    mutationKey: [QueryKeys.CREATE_PIPELINE],
    mutationFn: createPipeline,
    onSuccess: (data, variables) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_PIPELINES, variables.projectId],
      });

      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_PIPELINES],
      });

      if (props?.onSuccess) {
        props.onSuccess(data);
      }
    },
    onError: (error) => {
      if (props?.onError) {
        props.onError(error);
      }
    },
  });
}
