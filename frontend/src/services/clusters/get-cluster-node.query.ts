import { queryOptions, useQuery } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { ClusterNode, ServiceError } from '@/types';
import { useEffect } from 'react';
import { getHttp } from '@/lib/fetch';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';
import { camelCaseObject } from '@/lib';

export async function getClusterNode(nodeId: string): Promise<ClusterNode> {
  const resp = await getHttp<Sn<ClusterNode>>(`/nodes/${nodeId}`);
  return camelCaseObject(resp);
}

export function getClusterNodeQuery(nodeId?: string, enabled: boolean = true) {
  return queryOptions({
    queryKey: [QueryKeys.GET_CLUSTER_NODE, nodeId],
    queryFn: () => getClusterNode(nodeId!),
    enabled: !!nodeId && enabled,
    staleTime: 1000 * 60 * 5,
  });
}

interface UseGetClusterNodeProps {
  nodeId?: string;
  enabled?: boolean;
  onSuccess?: (clusterNodes: ClusterNode) => void;
  onError?: (error: ServiceError) => void;
}

export function useGetClusterNode(props?: UseGetClusterNodeProps) {
  const query = useQuery(getClusterNodeQuery(props?.nodeId, props?.enabled));

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
