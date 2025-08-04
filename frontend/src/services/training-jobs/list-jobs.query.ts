import { queryOptions, useQuery } from '@tanstack/react-query'; // Import UseQueryOptions
import { QueryKeys } from '../constants';
import { ServiceError, Job, ResourceRequirements } from '@/types';
import { useEffect } from 'react';
import { getHttp } from '@/lib/fetch';
import { camelCaseObject } from '@/lib/utils';
import { SnakeCasedPropertiesDeep as Sn } from 'type-fest';

export interface ListJobsResponse {
  jobs: {
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

export async function listJobs(): Promise<ListJobsResponse> {
  const resp = await getHttp<Sn<ListJobsResponse>>('/training_jobs');
  return camelCaseObject(resp);
}

export function listJobsQuery(enabled: boolean = true) {
  return queryOptions({
    queryKey: [QueryKeys.LIST_JOBS],
    queryFn: () => listJobs(),
    enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data.jobs as Job[],
  });
}

interface UseListJobsProps {
  enabled?: boolean;
  onSuccess?: (clusters: Job[]) => void;
  onError?: (error: ServiceError) => void;
}

export function useListJobs(props?: UseListJobsProps) {
  const query = useQuery(listJobsQuery(props?.enabled));

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
