import { getHttp } from '@/lib/fetch';
import { queryOptions, useQuery } from '@tanstack/react-query';
import { ClusterNode, ServiceError } from '@/types';
import { useEffect } from 'react';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';
import { camelCaseObject } from '@/lib/utils';
import { QueryKeys } from '../constants';

export interface ListClusterNodesResponse {
  clusterNodes: ClusterNode[];
}

export async function listClusterNodes(
  clusterId: string
): Promise<ListClusterNodesResponse> {
  const resp = await getHttp<Sn<ListClusterNodesResponse>>(
    `/clusters/${clusterId}/nodes`
  );
  return camelCaseObject(resp);
}

export function listClusterNodesQuery(
  clusterId?: string,
  enabled: boolean = true
) {
  return queryOptions({
    queryKey: [QueryKeys.LIST_CLUSTER_NODES, clusterId],
    queryFn: () => listClusterNodes(clusterId!),
    enabled: !!clusterId && enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data.clusterNodes as ClusterNode[],
  });
}

interface UseListClusterNodesProps {
  clusterId: string | undefined;
  enabled?: boolean;
  onSuccess?: (nodes: ClusterNode[]) => void;
  onError?: (error: ServiceError) => void;
}

export function useListClusterNodes(props: UseListClusterNodesProps) {
  const query = useQuery(listClusterNodesQuery(props.clusterId, props.enabled));

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
