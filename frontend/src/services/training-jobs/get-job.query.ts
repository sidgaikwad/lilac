import { getHttp } from '@/lib/fetch';
import { queryOptions, useQuery } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { Job, ResourceRequirements, ServiceError } from '@/types';
import { useEffect } from 'react';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';
import { camelCaseObject } from '@/lib/utils';

export interface GetJobResponse {
  jobId: string;
  jobName: string;
  jobStatus: string;
  nodeId?: string;
  queueId?: string;
  resourceRequirements: ResourceRequirements;
  createdAt: string;
  updatedAt: string;
}

export async function getJob(
  jobId: string
): Promise<GetJobResponse> {
  const resp = await getHttp<Sn<GetJobResponse>>(`/training_jobs/${jobId}`);
  return camelCaseObject(resp);
}

export function getJobQuery(jobId?: string, enabled: boolean = true) {
  return queryOptions({
    queryKey: [QueryKeys.GET_JOB, jobId],
    queryFn: () => getJob(jobId!),
    enabled: !!jobId && enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data as Job,
  });
}

interface UseGetJobProps {
  jobId: string | undefined;
  enabled?: boolean;
  onSuccess?: (job: Job) => void;
  onError?: (error: ServiceError) => void;
}

export function useGetJob(props: UseGetJobProps) {
  const query = useQuery(getJobQuery(props.jobId, props.enabled));

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
