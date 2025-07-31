import { useQuery } from '@tanstack/react-query';
import { getHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';
import type { Workspace } from '@/types/api/workspace';

async function listWorkspaces(projectId: string): Promise<Workspace[]> {
  return getHttp(`/projects/${projectId}/workspaces`);
}

export function useListWorkspaces(projectId: string) {
  return useQuery({
    queryKey: [QueryKeys.LIST_WORKSPACES, projectId],
    queryFn: () => listWorkspaces(projectId),
  });
}
