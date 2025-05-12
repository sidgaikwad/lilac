import { useMutation, useQueryClient } from '@tanstack/react-query';
import { QueryKeys } from './constants';
import { post } from '@/lib/fetch';
import {
  CreatePipelineRequest,
  CreatePipelineResponse,
  RunPipelinePayload,
  RunPipelineResponse,
  UpdatePipelineRequest,
} from './types';
import { ApiError } from '@/types';

const createPipeline = async (
  payload: CreatePipelineRequest
): Promise<CreatePipelineResponse> => {
  return post(`/pipelines`, payload);
};
export interface UseCreatePipelineProps {
  onSuccess?: (data: CreatePipelineResponse) => void;
  onError?: (error: ApiError) => void;
}

export function useCreatePipeline(props: UseCreatePipelineProps) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: createPipeline,
    onSuccess: (data, variables) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_PIPELINE, variables.projectId],
      });
      if (props.onSuccess !== undefined) {
        props.onSuccess(data);
      }
    },
    onError: props.onError,
  });
}

const updatePipeline = async (
  payload: UpdatePipelineRequest
): Promise<void> => {
  const { pipelineId, ...rest } = payload;
  return post(`/pipelines/${pipelineId}`, rest);
};
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

const runPipeline = async (payload: RunPipelinePayload): Promise<RunPipelineResponse> => {
  return post(`/pipelines/${payload.pipelineId}/run`, { datasetId: payload.datasetId });
};
export interface UseRunPipelineProps {
  onSuccess?: (data: RunPipelineResponse, variables: RunPipelinePayload) => void;
  onError?: (error: ApiError, variables: RunPipelinePayload) => void;
}

export function useRunPipeline(props: UseRunPipelineProps) {
  return useMutation<RunPipelineResponse, ApiError, RunPipelinePayload>({
    mutationFn: runPipeline,
    onSuccess: (data, variables) => {
      if (props.onSuccess !== undefined) {
        props.onSuccess(data, variables);
      }
    },
    onError: (error, variables) => {
      if (props.onError !== undefined) {
        props.onError(error, variables);
      }
    },
  });
}
