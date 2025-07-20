import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ServiceError } from '@/types';
import { postHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';

export interface CreateProjectRequest {
  projectName: string;
}

export interface CreateProjectResponse {
  projectId: string;
}

async function createProject(
  payload: CreateProjectRequest
): Promise<CreateProjectResponse> {
  const resp = await postHttp<
    Sn<CreateProjectRequest>,
    Sn<CreateProjectResponse>
  >('/projects', {
    project_name: payload.projectName,
  });
  return {
    projectId: resp.project_id,
  };
}

export interface UseCreateProjectProps {
  onSuccess?: (data: CreateProjectResponse) => void;
  onError?: (error: ServiceError) => void;
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
