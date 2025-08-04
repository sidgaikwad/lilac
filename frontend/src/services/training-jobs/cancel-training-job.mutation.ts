import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ServiceError } from '@/types';
import { postHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';

export interface CancelTrainingJobRequest {
  jobId: string;
}

async function cancelTrainingJob(
  payload: CancelTrainingJobRequest
): Promise<void> {
  await postHttp(`/training_jobs/${payload.jobId}/cancel`, {});
}

export interface UseCancelTrainingJobProps {
  onSuccess?: () => void;
  onError?: (error: ServiceError) => void;
}

export function useCancelTrainingJob(props: UseCancelTrainingJobProps) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationKey: [QueryKeys.CANCEL_TRAINING_JOB],
    mutationFn: cancelTrainingJob,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_CLUSTER_JOBS],
      });
      if (props.onSuccess !== undefined) {
        props.onSuccess();
      }
    },
    onError: props.onError,
  });
}
