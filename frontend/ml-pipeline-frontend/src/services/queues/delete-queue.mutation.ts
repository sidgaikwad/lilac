import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ServiceError } from '@/types';
import { deleteHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';

export interface DeleteQueueRequest {
  queueId: string;
}

async function deleteQueue(payload: DeleteQueueRequest): Promise<void> {
  await deleteHttp(`/queues/${payload.queueId}`);
}

export interface UseDeleteQueueProps {
  onSuccess?: () => void;
  onError?: (error: ServiceError) => void;
}

export function useDeleteQueue(props: UseDeleteQueueProps) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationKey: [QueryKeys.DELETE_QUEUE],
    mutationFn: deleteQueue,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_QUEUES],
      });
      if (props.onSuccess !== undefined) {
        props.onSuccess();
      }
    },
    onError: props.onError,
  });
}
