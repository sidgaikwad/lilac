import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ApiError } from '@/types';
import { postHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';

export interface CreateOrganizationRequest {
  organizationName: string;
}

export interface CreateOrganizationResponse {
  organizationId: string;
}

async function createOrganization(
  payload: CreateOrganizationRequest
): Promise<CreateOrganizationResponse> {
  const resp = await postHttp<
    Sn<Omit<CreateOrganizationRequest, 'projectId'>>,
    Sn<CreateOrganizationResponse>
  >('/organizations', {
    organization_name: payload.organizationName,
  });
  return {
    organizationId: resp.organization_id,
  }
}

export interface UseCreateOrganizationProps {
  onSuccess?: (data: CreateOrganizationResponse) => void;
  onError?: (error: ApiError) => void;
}

export function useCreateOrganization(props: UseCreateOrganizationProps) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationKey: [QueryKeys.CREATE_ORGANIZATION],
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
