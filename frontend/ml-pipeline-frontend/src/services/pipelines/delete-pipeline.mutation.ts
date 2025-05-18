import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ApiError } from '@/types';
import { deleteHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';

export interface DeletePipelineRequest {
  projectId: string;
  pipelineId: string;
}

export async function deletePipeline(
  payload: DeletePipelineRequest
): Promise<void> {
  return deleteHttp(`/pipelines/${payload.pipelineId}`);
}

export interface UseDeletePipelineProps {
  onSuccess?: () => void;
  onError?: (error: ApiError) => void;
}

export function useDeletePipeline(props?: UseDeletePipelineProps) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: deletePipeline,
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_PIPELINES, variables.projectId],
      });
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.GET_PIPELINE, variables.pipelineId],
      });
      if (props?.onSuccess) {
        props.onSuccess();
      }
    },
    onError: (error) => {
      if (props?.onError) {
        props.onError(error);
      }
    },
  });
}
