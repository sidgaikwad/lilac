import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ApiError } from '@/types';
import { deleteHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';

export interface DeleteOrganizationRequest {
  organizationId: string;
}

export async function deleteOrganization(
  payload: DeleteOrganizationRequest
): Promise<void> {
  return deleteHttp(`/organizations/${payload.organizationId}`);
}

export interface UseDeleteOrganizationProps {
  onSuccess?: () => void;
  onError?: (error: ApiError) => void;
}

export function useDeleteOrganization(props?: UseDeleteOrganizationProps) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: deleteOrganization,
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_ORGANIZATIONS],
      });
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.GET_ORGANIZATION, variables.organizationId],
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
