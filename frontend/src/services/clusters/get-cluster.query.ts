import { getHttp } from '@/lib/fetch';
import { queryOptions, useQuery } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { Cluster, ServiceError } from '@/types';
import { useEffect } from 'react';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';
import { camelCaseObject } from '@/lib/utils';

export interface GetClusterResponse {
  clusterId: string;
  clusterName: string;
  clusterDescription?: string;
}

export async function getCluster(
  clusterId: string
): Promise<GetClusterResponse> {
  const resp = await getHttp<Sn<GetClusterResponse>>(`/clusters/${clusterId}`);
  return camelCaseObject(resp);
}

export function getClusterQuery(clusterId?: string, enabled: boolean = true) {
  return queryOptions({
    queryKey: [QueryKeys.GET_CLUSTER, clusterId],
    queryFn: () => getCluster(clusterId!),
    enabled: !!clusterId && enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data as Cluster,
  });
}

interface UseGetClusterProps {
  clusterId: string | undefined;
  enabled?: boolean;
  onSuccess?: (cluster: Cluster) => void;
  onError?: (error: ServiceError) => void;
}

export function useGetCluster(props: UseGetClusterProps) {
  const query = useQuery(getClusterQuery(props.clusterId, props.enabled));

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
