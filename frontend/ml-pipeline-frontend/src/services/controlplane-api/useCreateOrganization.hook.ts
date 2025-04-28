import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ApiError } from '@/types';
import { post } from '@/lib/fetch';
import { CreateOrganizationRequest, CreateOrganizationResponse } from './types';
import { QueryKeys } from './constants';

async function createOrganization(
  payload: CreateOrganizationRequest
): Promise<CreateOrganizationResponse> {
  return post('/organizations', payload);
}

export interface UseCreateOrganizationProps {
  onSuccess?: (data: CreateOrganizationResponse) => void;
  onError?: (error: ApiError) => void;
}

export function useCreateOrganization(props: UseCreateOrganizationProps) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: createOrganization,
    onSuccess: (data) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_ORGANIZATIONS],
      });
      if (props.onSuccess !== undefined) {
        props.onSuccess(data);
      }
    },
    onError: props.onError,
  });
}
