import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ServiceError } from '@/types';
import { deleteHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';

export interface DeleteProjectRequest {
  projectId: string;
}

export async function deleteProject(
  payload: DeleteProjectRequest
): Promise<void> {
  return deleteHttp(`/projects/${payload.projectId}`);
}

export interface UseDeleteProjectProps {
  onSuccess?: () => void;
  onError?: (error: ServiceError) => void;
}

export function useDeleteProject(props?: UseDeleteProjectProps) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: deleteProject,
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_PROJECTS],
      });
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.GET_PROJECT, variables.projectId],
      });
      if (props?.onSuccess) {
        props.onSuccess();
      }
    },
    onError: (error) => {
      if (props?.onError) {
        props.onError(error);
      }
    },
  });
}
