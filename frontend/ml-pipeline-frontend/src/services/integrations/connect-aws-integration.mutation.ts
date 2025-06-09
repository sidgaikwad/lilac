import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ApiError } from '@/types';
import { postHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';

export interface ConnectAwsIntegrationRequest {
  projectId: string;
  roleArn: string;
  placeholderExternalId: string;
}

export interface ConnectAwsIntegrationResponse {
  externalId: string;
}

async function connectAwsIntegration(
  payload: ConnectAwsIntegrationRequest
): Promise<ConnectAwsIntegrationResponse> {
  const resp = await postHttp<Sn<Omit<ConnectAwsIntegrationRequest, 'projectId'>>, Sn<ConnectAwsIntegrationResponse>>(
    `/projects/${payload.projectId}/integrations/s3`,
    {
      role_arn: payload.roleArn,
      placeholder_external_id: payload.placeholderExternalId,
    }
  );
  return {
    externalId: resp.external_id,
  }
}

export interface UseConnectAwsIntegrationProps {
  onSuccess?: (data: ConnectAwsIntegrationResponse) => void;
  onError?: (error: ApiError) => void;
}

export function useConnectAwsIntegration(props: UseConnectAwsIntegrationProps) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationKey: [QueryKeys.CONNECT_AWS_INTEGRATION],
    mutationFn: connectAwsIntegration,
    onSuccess: (data, variables) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.GET_PROJECT, variables.projectId],
      });
      if (props.onSuccess !== undefined) {
        props.onSuccess(data);
      }
    },
    onError: props.onError,
  });
}
