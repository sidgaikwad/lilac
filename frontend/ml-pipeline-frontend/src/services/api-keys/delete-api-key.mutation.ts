import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ServiceError } from '@/types';
import { deleteHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';

export interface DeleteApiKeyRequest {
  keyId: string;
}

async function deleteApiKey(payload: DeleteApiKeyRequest): Promise<void> {
  await deleteHttp(`/account/api-keys/${payload.keyId}`);
}

export interface UseDeleteApiKeyProps {
  onSuccess?: () => void;
  onError?: (error: ServiceError) => void;
}

export function useDeleteApiKey(props: UseDeleteApiKeyProps) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationKey: [QueryKeys.DELETE_API_KEY],
    mutationFn: deleteApiKey,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_API_KEYS],
      });
      if (props.onSuccess !== undefined) {
        props.onSuccess();
      }
    },
    onError: props.onError,
  });
}