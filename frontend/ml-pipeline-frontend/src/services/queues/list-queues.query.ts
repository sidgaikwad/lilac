import { queryOptions, useQuery } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { ServiceError } from '@/types';
import { useEffect } from 'react';
import { getHttp } from '@/lib/fetch';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';
import { Queue } from '@/model/queue';

export async function listQueues(): Promise<Queue[]> {
  const resp = await getHttp<Sn<Queue[]>>('/queues');
  return resp.map((queue) => ({
    id: queue.id,
    name: queue.name,
    priority: queue.priority,
    cluster_targets: queue.cluster_targets,
  }));
}

export function listQueuesQuery(enabled: boolean = true) {
  return queryOptions({
    queryKey: [QueryKeys.LIST_QUEUES],
    queryFn: () => listQueues(),
    enabled,
    staleTime: 1000 * 60 * 5,
  });
}

interface UseListQueuesProps {
  enabled?: boolean;
  onSuccess?: (queues: Queue[]) => void;
  onError?: (error: ServiceError) => void;
}

export function useListQueues(props?: UseListQueuesProps) {
  const query = useQuery(listQueuesQuery(props?.enabled));

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