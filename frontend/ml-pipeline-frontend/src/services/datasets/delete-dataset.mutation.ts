import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ApiError } from '@/types';
import { deleteHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';

export interface DeleteDatasetRequest {
  projectId: string;
  datasetId: string;
}

export async function deleteDataset(
  payload: DeleteDatasetRequest
): Promise<void> {
  return deleteHttp(`/datasets/${payload.datasetId}`);
}

export interface UseDeleteDatasetProps {
  onSuccess?: () => void;
  onError?: (error: ApiError) => void;
}

export function useDeleteDataset(props?: UseDeleteDatasetProps) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: deleteDataset,
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_DATASETS, variables.projectId],
      });
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.GET_DATASET, variables.datasetId],
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
