import { useMutation, useQueryClient } from '@tanstack/react-query';
import { QueryKeys } from './constants';
import { post } from '@/lib/fetch';
import {
  CreateDatasetRequest,
  CreateDatasetResponse,
} from './types';
import { ApiError } from '@/types';

const createDataset = async (
    payload: CreateDatasetRequest
  ): Promise<CreateDatasetResponse> => {
    return post(`/api/datasets/${payload.projectId}`, payload);
  };
  export interface UseCreateDatasetProps {
    onSuccess?: (data: CreateDatasetResponse) => void;
    onError?: (error: ApiError) => void;
  }
  
  export function useCreateDataset(props: UseCreateDatasetProps) {
    const queryClient = useQueryClient();
  
    return useMutation({
      mutationFn: createDataset,
      onSuccess: (data, variables) => {
        queryClient.invalidateQueries({
          queryKey: [QueryKeys.LIST_DATASETS],
        });
        if (props.onSuccess !== undefined) {
          props.onSuccess(data);
        }
      },
      onError: props.onError,
    });
  }
  