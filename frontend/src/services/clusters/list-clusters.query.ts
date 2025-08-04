import { queryOptions, useQuery } from '@tanstack/react-query'; // Import UseQueryOptions
import { QueryKeys } from '../constants';
import { ServiceError, ClusterSummary } from '@/types';
import { useEffect } from 'react';
import { getHttp } from '@/lib/fetch';
import { camelCaseObject } from '@/lib/utils';
import { SnakeCasedPropertiesDeep as Sn } from 'type-fest';

export interface ListClustersResponse {
  clusters: {
    clusterId: string;
    clusterName: string;
    clusterDescription?: string;
    totalNodes: number;
    busyNodes: number;
    totalRunningJobs: number;
  }[];
}

export async function listClusters(): Promise<ListClustersResponse> {
  const resp = await getHttp<Sn<ListClustersResponse>>('/clusters');
  return camelCaseObject(resp);
}

export function listClustersQuery(enabled: boolean = true) {
  return queryOptions({
    queryKey: [QueryKeys.LIST_CLUSTERS],
    queryFn: () => listClusters(),
    enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data.clusters as ClusterSummary[],
  });
}

interface UseListClustersProps {
  enabled?: boolean;
  onSuccess?: (clusters: ClusterSummary[]) => void;
  onError?: (error: ServiceError) => void;
}

export function useListClusters(props?: UseListClustersProps) {
  const query = useQuery(listClustersQuery(props?.enabled));

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
