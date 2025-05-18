import { useEffect } from 'react';
import useOrganizationStore from '@/store/use-organization-store';
import { useParams } from 'react-router-dom';
import { getProjectQuery } from '@/services';
import {
  Container,
  ContainerAction,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';
import Breadcrumbs from '@/components/common/breadcrumbs';
import { useSuspenseQuery } from '@tanstack/react-query';

type ProjectParams = {
  projectId: string;
};

function ProjectDashboardPage() {
  const { projectId } = useParams<ProjectParams>();
  const { data: project } = useSuspenseQuery(getProjectQuery(projectId));
  const { setSelectedProjectId, setSelectedOrganizationId } =
    useOrganizationStore();

  useEffect(() => {
    setSelectedOrganizationId(project?.organizationId);
    setSelectedProjectId(project?.id);
  }, [
    setSelectedOrganizationId,
    setSelectedProjectId,
    project?.id,
    project?.organizationId,
  ]);

  return (
    <Container>
      <ContainerHeader>
        <div className="flex-1 shrink-0 grow-0 basis-full pb-4">
          <Breadcrumbs
            breadcrumbs={[
              {
                content: 'Projects',
                link: `/organizations/${project?.organizationId}/projects`,
              },
              {
                content: project.name,
                link: `/projects/${projectId}`,
              },
            ]}
          />
        </div>
        <ContainerTitle>
          {project?.name}
          <ContainerDescription></ContainerDescription>
        </ContainerTitle>
        <ContainerAction></ContainerAction>
      </ContainerHeader>

      <ContainerContent>
        <p>Select an option from the menu on the left to begin!</p>
      </ContainerContent>
    </Container>
  );
}
export default ProjectDashboardPage;
