import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ServiceError } from '@/types';
import { deleteHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';

export interface DeleteClusterRequest {
  clusterId: string;
}

export async function deleteCluster(
  payload: DeleteClusterRequest
): Promise<void> {
  return deleteHttp(`/clusters/${payload.clusterId}`);
}

export interface UseDeleteClusterProps {
  onSuccess?: () => void;
  onError?: (error: ServiceError) => void;
}

export function useDeleteCluster(props?: UseDeleteClusterProps) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: deleteCluster,
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_CLUSTERS],
      });
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.GET_CLUSTER, variables.clusterId],
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
