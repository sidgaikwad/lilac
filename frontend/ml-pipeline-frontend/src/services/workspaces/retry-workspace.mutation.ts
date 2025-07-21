import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ServiceError } from '@/types';
import { postHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';

export interface RetryWorkspaceRequest {
  projectId: string;
  workspaceId: string;
}

async function retryWorkspace(
  payload: RetryWorkspaceRequest
): Promise<void> {
  await postHttp<null, null>(
    `/projects/${payload.projectId}/workspaces/${payload.workspaceId}/retry`,
    null
  );
}

export interface UseRetryWorkspaceProps {
  onSuccess?: () => void;
  onError?: (error: ServiceError) => void;
}

export function useRetryWorkspace(props?: UseRetryWorkspaceProps) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationKey: [QueryKeys.RETRY_WORKSPACE],
    mutationFn: retryWorkspace,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_WORKSPACES],
      });
      if (props?.onSuccess !== undefined) {
        props.onSuccess();
      }
    },
    onError: props?.onError,
  });
}