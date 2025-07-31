import { useQuery } from '@tanstack/react-query';
import { getHttp } from '@/lib/fetch';

interface ConnectionDetails {
  url: string | null;
  token: string | null;
}

const getConnectionDetails = async (
  projectId: string,
  workspaceId: string
): Promise<ConnectionDetails> => {
  return await getHttp(
    `/projects/${projectId}/workspaces/${workspaceId}/connection`
  );
};

export const useWorkspaceConnection = (
  projectId?: string,
  workspaceId?: string
) => {
  return useQuery({
    queryKey: ['workspaces', projectId, workspaceId, 'connection'],
    queryFn: () => getConnectionDetails(projectId!, workspaceId!),
    enabled: !!projectId && !!workspaceId,
  });
};
