import { useQuery } from '@tanstack/react-query';
import { BASE_URL, QueryKeys } from './constants';
import { ApiError, Project } from '@/types';
import { useEffect } from 'react';
import { GetProjectResponse } from './types';

interface UseGetProjectProps {
  projectId: string | undefined;
  enabled?: boolean;
  onSuccess?: (project: Project) => void;
  onError?: (error: ApiError) => void;
}

// Mock function simulating API call for pipeline details
const fetchProject = async (projectId: string): Promise<Project> => {
  const resp = await fetch(`${BASE_URL}/projects/${projectId}`, {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${localStorage.getItem('token')}`,
    },
  });
  if (resp.status < 200 || resp.status >= 300) {
    return Promise.reject({
      statusCode: resp.status,
      ...(await resp.json()),
    });
  }
  const body: GetProjectResponse = await resp.json();
  return {
    id: body.id,
    name: body.name,
    organizationId: body.organizationId,
  };
};

export function useGetProject(props: UseGetProjectProps) {
  const query = useQuery<Project | undefined, ApiError>({
    queryKey: [QueryKeys.GET_PROJECT, props.projectId],
    queryFn: () => fetchProject(props.projectId!),
    enabled: !!props.projectId,
    staleTime: 1000 * 60 * 5, // 5 minutes
  });

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
