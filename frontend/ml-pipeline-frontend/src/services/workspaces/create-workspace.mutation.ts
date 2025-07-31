import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ServiceError } from '@/types';
import { postHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';
import type { CreateWorkspaceRequest, Workspace } from '@/types/api/workspace';

async function createWorkspace({
  projectId,
  payload,
}: {
  projectId: string;
  payload: CreateWorkspaceRequest;
}): Promise<Workspace> {
  return postHttp(`/projects/${projectId}/workspaces`, payload);
}

export interface UseCreateWorkspaceProps {
  onSuccess?: (data: Workspace) => void;
  onError?: (error: ServiceError) => void;
}

export function useCreateWorkspace(props: UseCreateWorkspaceProps) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationKey: [QueryKeys.CREATE_WORKSPACE],
    mutationFn: createWorkspace,
    onSuccess: (data) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_WORKSPACES],
      });
      if (props.onSuccess !== undefined) {
        props.onSuccess(data);
      }
    },
    onError: props.onError,
  });
}
