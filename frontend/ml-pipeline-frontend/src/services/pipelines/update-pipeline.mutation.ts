import { postHttp } from '@/lib/fetch';
import { ApiError } from '@/types';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';

export interface UpdatePipelineRequest {
  pipelineId: string;
  name?: string;
  description?: string;
  steps?: {
    stepId: string;
    stepParameters: Record<string, string | number | boolean | object>;
    stepDefinitionId: string;
  }[];
  stepConnections?: [string, string][];
}

export async function updatePipeline(
  payload: UpdatePipelineRequest
): Promise<void> {
  const { pipelineId, ...request } = payload;
  return postHttp<Sn<Omit<UpdatePipelineRequest, 'pipelineId'>>, void>(
    `/pipelines/${pipelineId}`,
    {
      name: request.name,
      description: request.description,
      steps: request.steps?.map((step) => ({
        step_id: step.stepId,
        step_parameters: step.stepParameters,
        step_definition_id: step.stepDefinitionId,
      })),
      step_connections: request.stepConnections,
    }
  );
}

export interface UseUpdatePipelineProps {
  onSuccess?: () => void;
  onError?: (error: ApiError) => void;
}

export function useUpdatePipeline(props: UseUpdatePipelineProps) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: updatePipeline,
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.GET_PIPELINE, variables.pipelineId],
      });
      if (props.onSuccess !== undefined) {
        props.onSuccess();
      }
    },
    onError: props.onError,
  });
}
