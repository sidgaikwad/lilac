import { queryOptions, useQuery } from '@tanstack/react-query'; // Import UseQueryOptions
import { QueryKeys } from '../constants';
import { ServiceError, CredentialSummary } from '@/types';
import { useEffect } from 'react';
import { getHttp } from '@/lib/fetch';
import { camelCaseObject } from '@/lib/utils';
import { SnakeCasedPropertiesDeep as Sn } from 'type-fest';

export interface ListCredentialsResponse {
  credentials: {
    credentialId: string;
    credentialName: string;
    credentialDescription?: string;
    credentialType: string;
  }[];
}

export async function listCredentials(): Promise<ListCredentialsResponse> {
  const resp = await getHttp<Sn<ListCredentialsResponse>>('/credentials');
  return camelCaseObject(resp);
}

export function listCredentialsQuery(enabled: boolean = true) {
  return queryOptions({
    queryKey: [QueryKeys.LIST_CREDENTIALS],
    queryFn: () => listCredentials(),
    enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data.credentials as CredentialSummary[],
  });
}

interface UseListCredentialsProps {
  enabled?: boolean;
  onSuccess?: (credentials: CredentialSummary[]) => void;
  onError?: (error: ServiceError) => void;
}

export function useListCredentials(props?: UseListCredentialsProps) {
  const query = useQuery(listCredentialsQuery(props?.enabled));

  useEffect(() => {
    if (props?.onSuccess && query.data != undefined) {
      props.onSuccess(query.data);
    }
  }, [props, query.data]);

  useEffect(() => {
    if (props?.onError && query.error != null) {
      props.onError(query.error);
    }
  }, [props, query.error]);

  return query;
}
