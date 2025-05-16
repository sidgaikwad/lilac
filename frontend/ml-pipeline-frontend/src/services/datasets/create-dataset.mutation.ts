import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ApiError } from '@/types';
import { post } from '@/lib/fetch';
import { QueryKeys } from '../constants';

export interface CreateDatasetRequest {
  datasetName: string;
  description?: string;
  projectId: string;
  images: {
    metadata: {
      fileName: string;
      fileType: string;
      size: number;
      createdAt: string;
    };
    contents: string;
  }[];
}

export interface CreateDatasetResponse {
  id: string;
}

async function createDataset(
  payload: CreateDatasetRequest
): Promise<CreateDatasetResponse> {
  const { projectId, ...request } = payload;
  return post(`/projects/${projectId}/datasets`, request);
}

export interface UseCreateDatasetProps {
  onSuccess?: (data: CreateDatasetResponse) => void;
  onError?: (error: ApiError) => void;
}

export function useCreateDataset(props: UseCreateDatasetProps) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationKey: [QueryKeys.CREATE_DATASET],
    mutationFn: createDataset,
    onSuccess: (data) => {
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
