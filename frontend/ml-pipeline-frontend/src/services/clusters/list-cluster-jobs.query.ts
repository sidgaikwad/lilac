import { getHttp } from '@/lib/fetch';
import { queryOptions, useQuery } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { ClusterJob, ResourceRequirements, ServiceError } from '@/types';
import { useEffect } from 'react';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';
import { camelCaseObject } from '@/lib/utils';

export interface ListClusterJobsResponse {
  clusterJobs: {
    jobId: string;
    jobName: string;
    jobStatus: string;
    nodeId?: string;
    queueId: string;
    resourceRequirements: ResourceRequirements;
    createdAt: string;
    updatedAt: string;
  }[];
}

export async function listClusterJobs(
  clusterId: string
): Promise<ListClusterJobsResponse> {
  const resp = await getHttp<Sn<ListClusterJobsResponse>>(
    `/clusters/${clusterId}/jobs`
  );
  return camelCaseObject(resp);
}

export function listClusterJobsQuery(
  clusterId?: string,
  enabled: boolean = true
) {
  return queryOptions({
    queryKey: [QueryKeys.LIST_CLUSTER_JOBS, clusterId],
    queryFn: () => listClusterJobs(clusterId!),
    enabled: !!clusterId && enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data.clusterJobs as ClusterJob[],
  });
}

interface UseListClusterProps {
  clusterId: string | undefined;
  enabled?: boolean;
  onSuccess?: (jobs: ClusterJob[]) => void;
  onError?: (error: ServiceError) => void;
}

export function useListClusterJobs(props: UseListClusterProps) {
  const query = useQuery(listClusterJobsQuery(props.clusterId, props.enabled));

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
