import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ServiceError } from '@/types';
import { postHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';
import { camelCaseObject, snakeCaseObject } from '@/lib/utils';
import { AwsCredentials, GcpCredentials } from './types';

export interface CreateCredentialRequest {
  credentialName: string;
  credentialDescription?: string;
  credentials: AwsCredentials | GcpCredentials;
}

export interface CreateCredentialResponse {
  credentialId: string;
}

async function createCredential(
  payload: CreateCredentialRequest
): Promise<CreateCredentialResponse> {
  const request = snakeCaseObject<CreateCredentialRequest>(payload, false);
  const resp = await postHttp<
    Sn<CreateCredentialRequest, { splitOnNumbers: false }>,
    Sn<CreateCredentialResponse>
  >(`/credentials`, request);
  return camelCaseObject(resp);
}

export interface UseCreateCredentialProps {
  onSuccess?: (data: CreateCredentialResponse) => void;
  onError?: (error: ServiceError) => void;
}

export function useCreateCredential(props?: UseCreateCredentialProps) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationKey: [QueryKeys.CREATE_CREDENTIAL],
    mutationFn: createCredential,
    onSuccess: (data) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_CREDENTIALS],
      });
      if (props?.onSuccess !== undefined) {
        props.onSuccess(data);
      }
    },
    onError: props?.onError,
  });
}
