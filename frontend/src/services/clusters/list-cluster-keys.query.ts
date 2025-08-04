import { getHttp } from '@/lib/fetch';
import { queryOptions, useQuery } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { ClusterApiKey, ServiceError } from '@/types';
import { useEffect } from 'react';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';
import { camelCaseObject } from '@/lib/utils';

export interface ListClusterKeysResponse {
  id: string;
  clusterId: string;
  prefix: string;
  createdAt: string;
  lastUsedAt: string;
  expiresAt: string;
}

export async function listClusterKeys(
  clusterId: string
): Promise<ListClusterKeysResponse[]> {
  const resp = await getHttp<Sn<ListClusterKeysResponse[]>>(
    `/clusters/${clusterId}/api-keys`
  );
  return camelCaseObject(resp);
}

export function listClusterKeysQuery(
  clusterId?: string,
  enabled: boolean = true
) {
  return queryOptions({
    queryKey: [QueryKeys.LIST_CLUSTER_KEYS, clusterId],
    queryFn: () => listClusterKeys(clusterId!),
    enabled: !!clusterId && enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data as ClusterApiKey[],
  });
}

interface UseListClusterProps {
  clusterId: string | undefined;
  enabled?: boolean;
  onSuccess?: (keys: ClusterApiKey[]) => void;
  onError?: (error: ServiceError) => void;
}

export function useListClusterKeys(props: UseListClusterProps) {
  const query = useQuery(listClusterKeysQuery(props.clusterId, props.enabled));

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
