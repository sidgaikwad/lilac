import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ServiceError } from '@/types';
import { deleteHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';

export interface DeleteCredentialRequest {
  credentialId: string;
}

export async function deleteCredential(
  payload: DeleteCredentialRequest
): Promise<void> {
  return deleteHttp(`/credentials/${payload.credentialId}`);
}

export interface UseDeleteCredentialProps {
  onSuccess?: () => void;
  onError?: (error: ServiceError) => void;
}

export function useDeleteCredential(props?: UseDeleteCredentialProps) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: [QueryKeys.DELETE_CREDENTIAL],
    mutationFn: deleteCredential,
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_CREDENTIALS],
      });
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.GET_CREDENTIAL, variables.credentialId],
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
