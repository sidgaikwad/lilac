import { queryOptions, useQuery } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { Queue, ServiceError } from '@/types';
import { useEffect } from 'react';
import { getHttp } from '@/lib/fetch';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';

export async function getQueue(queueId: string): Promise<Queue> {
  const resp = await getHttp<Sn<Queue>>(`/queues/${queueId}`);
  return {
    id: resp.id,
    name: resp.name,
    priority: resp.priority,
    clusterTargets: resp.cluster_targets,
  };
}

export function getQueueQuery(queueId?: string, enabled: boolean = true) {
  return queryOptions({
    queryKey: [QueryKeys.GET_QUEUE, queueId],
    queryFn: () => getQueue(queueId!),
    enabled: !!queueId && enabled,
    staleTime: 1000 * 60 * 5,
  });
}

interface UseGetQueueProps {
  queueId?: string;
  enabled?: boolean;
  onSuccess?: (queues: Queue) => void;
  onError?: (error: ServiceError) => void;
}

export function useGetQueue(props?: UseGetQueueProps) {
  const query = useQuery(getQueueQuery(props?.queueId, props?.enabled));

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
