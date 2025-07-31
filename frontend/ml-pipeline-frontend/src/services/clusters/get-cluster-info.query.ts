import { getHttp } from '@/lib/fetch';
import { queryOptions, useQuery } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { Cluster, ClusterInfo, ServiceError } from '@/types';
import { useEffect } from 'react';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';
import { camelCaseObject } from '@/lib/utils';

export interface GetClusterInfoResponse {
  clusterId: string;
  clusterName: string;
  clusterDescription?: string;
  totalNodes: number,
  busyNodes: number,
  memoryInfo: {
    totalMemoryMb: number,
    usedMemoryMb: number,
  },
  cpuInfo: {
    totalMillicores: number,
    usedMillicores: number,
  },
  gpuInfo: {
    totalGpus: number,
    usedGpus: number,
  },
  jobInfo: {
    totalRunningJobs: number,
  },
  createdAt: string,
  updatedAt: string,
}

export async function getClusterInfo(
  clusterId: string
): Promise<GetClusterInfoResponse> {
  const resp = await getHttp<Sn<GetClusterInfoResponse>>(`/clusters/${clusterId}/info`);
  return camelCaseObject(resp);
}

export function getClusterInfoQuery(clusterId?: string, enabled: boolean = true) {
  return queryOptions({
    queryKey: [QueryKeys.GET_CLUSTER_INFO, clusterId],
    queryFn: () => getClusterInfo(clusterId!),
    enabled: !!clusterId && enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data as ClusterInfo,
  });
}

interface UseGetClusterProps {
  clusterId: string | undefined;
  enabled?: boolean;
  onSuccess?: (cluster: Cluster) => void;
  onError?: (error: ServiceError) => void;
}

export function useGetClusterInfo(props: UseGetClusterProps) {
  const query = useQuery(getClusterInfoQuery(props.clusterId, props.enabled));

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
