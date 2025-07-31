import { getHttp } from '@/lib/fetch';
import { queryOptions, useQuery } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { ClusterJob, ResourceRequirements, ServiceError } from '@/types';
import { useEffect } from 'react';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';
import { camelCaseObject } from '@/lib/utils';

export interface GetClusterJobsResponse {
  clusterJobs: {
    jobId: string;
    jobName: string;
    jobStatus: string;
    nodeId?: string;
    queueId: string;
    resourceRequirements: ResourceRequirements;
    createdAt: string;
    updatedAt: string;
  }[]
}

export async function getClusterJobs(
  clusterId: string
): Promise<GetClusterJobsResponse> {
  const resp = await getHttp<Sn<GetClusterJobsResponse>>(
    `/clusters/${clusterId}/jobs`
  );
  return camelCaseObject(resp);
}

export function getClusterJobsQuery(
  clusterId?: string,
  enabled: boolean = true
) {
  return queryOptions({
    queryKey: [QueryKeys.GET_CLUSTER_JOBS, clusterId],
    queryFn: () => getClusterJobs(clusterId!),
    enabled: !!clusterId && enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data.clusterJobs as ClusterJob[],
  });
}

interface UseGetClusterProps {
  clusterId: string | undefined;
  enabled?: boolean;
  onSuccess?: (jobs: ClusterJob[]) => void;
  onError?: (error: ServiceError) => void;
}

export function useGetClusterJobs(props: UseGetClusterProps) {
  const query = useQuery(getClusterJobsQuery(props.clusterId, props.enabled));

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
