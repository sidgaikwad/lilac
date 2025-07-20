import { getHttp } from '@/lib/fetch';
import { queryOptions, useQuery } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { ServiceError } from '@/types';
import { useEffect } from 'react';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';
import { camelCaseObject } from '@/lib/utils';
import { AwsCredentials } from './types';

export interface GetCredentialResponse {
  credentialId: string;
  credentialName: string;
  credentialDescription?: string;
  credentials: AwsCredentials;
}

export async function getCredential(
  credentialId: string
): Promise<GetCredentialResponse> {
  const resp = await getHttp<Sn<GetCredentialResponse>>(
    `/credentials/${credentialId}`
  );
  return camelCaseObject(resp);
}

export function getCredentialQuery(
  credentialId?: string,
  enabled: boolean = true
) {
  return queryOptions({
    queryKey: [QueryKeys.GET_CREDENTIAL, credentialId],
    queryFn: () => getCredential(credentialId!),
    enabled: !!credentialId && enabled,
    staleTime: 1000 * 60 * 5,
  });
}

interface UseGetCredentialProps {
  credentialId: string | undefined;
  enabled?: boolean;
  onSuccess?: (credential: GetCredentialResponse) => void;
  onError?: (error: ServiceError) => void;
}

export function useGetCredential(props: UseGetCredentialProps) {
  const query = useQuery(getCredentialQuery(props.credentialId, props.enabled));

  useEffect(() => {
    if (props?.onSuccess && query.data !== undefined) {
      props.onSuccess(query.data);
    }
  }, [props, query.data]);

  useEffect(() => {
    if (props?.onError && query.error !== null) {
      props.onError(query.error);
    }
  }, [props, query.error]);

  return query;
}
