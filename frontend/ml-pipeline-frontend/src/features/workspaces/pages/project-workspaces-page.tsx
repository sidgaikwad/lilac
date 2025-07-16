import { useState } from 'react';
import { useParams } from 'react-router-dom';
import {
  Container,
  ContainerAction,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';
import { mockWorkspaces } from '@/features/workspaces/mock-data';
import WorkspaceList from '@/features/workspaces/components/workspace-list';
import CreateWorkspaceModal from '../components/create-workspace-modal';

function ProjectWorkspacesPage() {
  const { projectId } = useParams<{ projectId: string }>();
  const [isModalOpen, setModalOpen] = useState(false);
  const [workspaces] = useState(mockWorkspaces);
  const [isLoading] = useState(false); // Mock loading state

  if (isLoading) {
    return <div>Loading...</div>;
  }

  return (
    <Container>
      <ContainerHeader>
        <ContainerTitle>
          Workspaces
          <ContainerDescription>
            Manage your development environments
          </ContainerDescription>
        </ContainerTitle>
        <ContainerAction>
          <CreateWorkspaceModal
            isOpen={isModalOpen}
            setOpen={setModalOpen}
            projectId={projectId ?? ''}
          />
        </ContainerAction>
      </ContainerHeader>
      <ContainerContent>
        <WorkspaceList
          workspaces={workspaces}
          onStartWorkspace={(id) => console.log('Start:', id)}
          onStopWorkspace={(id) => console.log('Stop:', id)}
        />
      </ContainerContent>
    </Container>
  );
}

export default ProjectWorkspacesPage;