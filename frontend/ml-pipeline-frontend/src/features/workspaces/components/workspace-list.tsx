import { Workspace } from '@/types/api/workspace';
import WorkspaceListItem from './workspace-list-item';

export interface WorkspaceListProps {
  workspaces: Workspace[];
  onStartWorkspace: (id: string) => void;
  onStopWorkspace: (id: string) => void;
  projectId: string;
}

function WorkspaceList({
  workspaces,
  onStartWorkspace,
  onStopWorkspace,
  projectId,
}: WorkspaceListProps) {
  return (
    <div className='inline-grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3'>
      {workspaces.map((workspace) => (
        <WorkspaceListItem
          key={workspace.id}
          workspace={workspace}
          onStart={onStartWorkspace}
          onStop={onStopWorkspace}
          projectId={projectId}
        />
      ))}
    </div>
  );
}

export default WorkspaceList;
