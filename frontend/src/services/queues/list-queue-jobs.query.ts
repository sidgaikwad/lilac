import { queryOptions, useQuery } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { ResourceRequirements, ServiceError } from '@/types';
import { useEffect } from 'react';
import { getHttp } from '@/lib/fetch';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';
import { QueueJob } from '@/types/api/queue';
import { camelCaseObject } from '@/lib';

export interface ListQueueJobsResponse {
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

export async function listQueueJobs(
  queueId: string
): Promise<ListQueueJobsResponse> {
  const resp = await getHttp<Sn<ListQueueJobsResponse>>(
    `/queues/${queueId}/jobs`
  );
  return camelCaseObject(resp);
}

export function listQueueJobsQuery(queueId?: string, enabled: boolean = true) {
  return queryOptions({
    queryKey: [QueryKeys.LIST_QUEUE_JOBS],
    queryFn: () => listQueueJobs(queueId!),
    enabled: !!queueId && enabled,
    select: (data) => data.jobs,
    staleTime: 1000 * 60 * 5,
  });
}

interface UseListQueueJobsProps {
  queueId?: string;
  enabled?: boolean;
  onSuccess?: (queues: QueueJob[]) => void;
  onError?: (error: ServiceError) => void;
}

export function useListQueueJobs(props?: UseListQueueJobsProps) {
  const query = useQuery(listQueueJobsQuery(props?.queueId, props?.enabled));

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
