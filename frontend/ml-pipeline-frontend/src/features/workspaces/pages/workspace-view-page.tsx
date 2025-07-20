import { useParams } from 'react-router-dom';
import { useWorkspaceConnection } from '../hooks/use-workspace-connection';

function WorkspaceViewPage() {
  const { projectId, workspaceId } = useParams<{ projectId: string; workspaceId: string }>();
  const { data, isLoading } = useWorkspaceConnection(projectId, workspaceId);

  const iframeUrl = data?.url && data.token ? `${data.url}/lab?token=${data.token}` : null;

  return (
    <div className="h-full w-full">
      {isLoading || !iframeUrl ? (
        <p>Loading workspace...</p>
      ) : (
        <iframe
          src={iframeUrl}
          className="h-full w-full border-0"
          title="Workspace"
        />
      )}
    </div>
  );
}

export default WorkspaceViewPage;