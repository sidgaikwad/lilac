import { useMutation, useQueryClient } from '@tanstack/react-query';
import { QueryKeys } from './constants';
import { post } from '@/lib/fetch';
import { ApiError } from '@/types';

export interface CreateDatasetRequest {
  datasetName: string;
  description?: string;
  images: {
    metadata: {
      fileName: string,
      fileType: string,
      size: number,
      createdAt: string,
    }
    contents: string,
  }[];
  projectId: string;
}

export interface CreateDatasetResponse {
  id: string;
}

const createDataset = async (
    payload: CreateDatasetRequest
  ): Promise<CreateDatasetResponse> => {
    return post(`/projects/${payload.projectId}/datasets`, payload);
  };
  export interface UseCreateDatasetProps {
    onSuccess?: (data: CreateDatasetResponse) => void;
    onError?: (error: ApiError) => void;
  }
  
  export function useCreateDataset(props: UseCreateDatasetProps) {
    const queryClient = useQueryClient();
  
    return useMutation({
      mutationFn: createDataset,
      onSuccess: (data, _variables) => {
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
  