import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ApiError } from '@/types';
import { post } from '@/lib/fetch';
import { CreateProjectRequest, CreateProjectResponse } from './types';
import { QueryKeys } from './constants';

async function createProject(
  payload: CreateProjectRequest
): Promise<CreateProjectResponse> {
  return post('/projects', payload);
}

export interface UseCreateProjectProps {
  onSuccess?: (data: CreateProjectResponse) => void;
  onError?: (error: ApiError) => void;
}

export function useCreateProject(props: UseCreateProjectProps) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: createProject,
    onSuccess: (data, variables) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_PROJECTS, variables.organizationId],
      });
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_PROJECTS, undefined],
      });
      if (props.onSuccess !== undefined) {
        props.onSuccess(data);
      }
    },
    onError: props.onError,
  });
}
