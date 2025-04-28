import { useQuery } from '@tanstack/react-query';
import { BASE_URL, QueryKeys } from './constants';
import { Project } from '@/types';
import useAuthStore from '@/store/useAuthStore';
import { ListProjectsResponse } from './types';

interface UseListProjectsProps {
  organizationId: string | undefined; // Accept undefined to disable query
}

// Mock function simulating API call
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
  return useQuery<Project[], Error>({
    // Query key depends on the organizationId
    queryKey: [QueryKeys.LIST_PROJECTS, props?.organizationId],
    // Pass orgId to the mock function
    queryFn: () => fetchProjects(props?.organizationId), // Non-null assertion ok due to 'enabled'
    // Only run the query if organizationId is truthy
    enabled: !!props?.organizationId || !!user?.id,
    staleTime: 1000 * 60 * 5, // 5 minutes
  });
}
