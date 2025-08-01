import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ServiceError } from '@/types';
import { deleteHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';

export interface DeleteClusterKeyRequest {
  clusterId: string;
  keyId: string;
}

export async function deleteClusterKey(
  payload: DeleteClusterKeyRequest
): Promise<void> {
  return deleteHttp(`/clusters/${payload.clusterId}/api-keys/${payload.keyId}`);
}

export interface UseDeleteClusterKeyProps {
  onSuccess?: () => void;
  onError?: (error: ServiceError) => void;
}

export function useDeleteClusterKey(props?: UseDeleteClusterKeyProps) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: deleteClusterKey,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.GET_CLUSTER_KEYS],
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
