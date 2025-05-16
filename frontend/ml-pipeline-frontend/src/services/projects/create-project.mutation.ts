import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ApiError } from '@/types';
import { post } from '@/lib/fetch';
import { QueryKeys } from '../constants';

export interface CreateProjectRequest {
  name: string;
  organizationId: string;
}

export interface CreateProjectResponse {
  id: string;
}

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
    mutationKey: [QueryKeys.CREATE_PROJECT],
    mutationFn: createProject,
    onSuccess: (data) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_PROJECTS],
      });
      if (props.onSuccess !== undefined) {
        props.onSuccess(data);
      }
    },
    onError: props.onError,
  });
}
