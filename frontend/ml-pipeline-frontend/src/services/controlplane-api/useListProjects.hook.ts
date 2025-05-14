import { useQuery } from '@tanstack/react-query';
import { BASE_URL, QueryKeys } from './constants';
import { ApiError, Project } from '@/types'; 
import useAuthStore from '@/store/useAuthStore';
import { ListProjectsResponse } from './types';

interface UseListProjectsProps {
  organizationId: string | undefined; 
}


const fetchProjects = async (orgId?: string): Promise<Project[]> => {
  const queryParams = orgId ? `?organizationId=${orgId}` : '';
  const resp = await fetch(`${BASE_URL}/projects${queryParams}`, {
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
  const body: ListProjectsResponse = await resp.json();
  return body.projects.map((proj) => ({
    id: proj.id,
    name: proj.name,
    organizationId: proj.organizationId,
  }));
};

export function useListProjects(props?: UseListProjectsProps) {
  const { user } = useAuthStore();
  return useQuery<Project[], ApiError>({ 
    
    queryKey: [QueryKeys.LIST_PROJECTS, props?.organizationId],
    
    queryFn: () => fetchProjects(props?.organizationId), 
    
    enabled: !!props?.organizationId || !!user?.id,
    staleTime: 1000 * 60 * 5, 
  });
}
