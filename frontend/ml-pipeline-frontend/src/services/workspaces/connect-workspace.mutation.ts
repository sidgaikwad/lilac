import { useMutation } from '@tanstack/react-query';
import { getHttp } from '@/lib/fetch';

interface ConnectWorkspaceParams {
  projectId: string;
  workspaceId: string;
}

export const useConnectWorkspaceMutation = () => {
  return useMutation<string, Error, ConnectWorkspaceParams>({
    mutationFn: async ({ projectId, workspaceId }) => {
      const response = await getHttp<{ url: string }>(
        `/projects/${projectId}/workspaces/${workspaceId}/connect`
      );
      return response.url;
    },
  });
};