import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import ProjectCard from '@/features/projects/components/project-card';
import CreateProjectModal from '@/features/projects/components/create-project-modal';
import EmptyCardSection from '@/components/common/empty-card-section';
import { Skeleton } from '@/components/ui/skeleton';
import { getOrganizationQuery } from '@/services';
import { useSuspenseQuery } from '@tanstack/react-query';
import { useListProjects } from '@/services';
import { toast } from 'sonner';
import useOrganizationStore from '@/store/use-organization-store';
import {
  Container,
  ContainerAction,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';
import Breadcrumbs from '@/components/common/breadcrumbs';

function ProjectsListPage() {
  const { organizationId } = useParams<{ organizationId: string }>();
  const [isCreateProjectModalOpen, setCreateProjectModalOpen] = useState(false);
  const setSelectedOrgId = useOrganizationStore(
    (state) => state.setSelectedOrganizationId
  );

  const { data: organization } = useSuspenseQuery(
    getOrganizationQuery(organizationId)
  );

  const { data: projects = [], isLoading: isLoadingProjects } = useListProjects(
    {
      organizationId,
      onError: (error) =>
        toast.error('Failed to list projects', {
          description: error.error,
        }),
    }
  );

  useEffect(() => {
    setSelectedOrgId(organizationId);
  }, [setSelectedOrgId, organizationId]);

  if (isLoadingProjects) {
    return (
      <div className='container mx-auto space-y-6 p-4 md:p-6 lg:p-8'>
        <Skeleton className='mb-4 h-10 w-1/2' /> {}
        <div className='mb-4 flex justify-end'>
          <Skeleton className='h-10 w-32' /> {}
        </div>
        <div className='grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3'>
          {[...Array(3)].map((_, i) => (
            <Skeleton key={i} className='h-48 w-full' />
          ))}
        </div>
      </div>
    );
  }

  return (
    <Container>
      <ContainerHeader>
        <div className='flex-1 shrink-0 grow-0 basis-full pb-4'>
          <Breadcrumbs
            breadcrumbs={[
              {
                content: 'Organizations',
                link: '/organizations',
              },
              {
                content: organization.name,
                link: `/organizations/${organizationId}/projects`,
              },
              {
                content: 'Projects',
                link: `/organizations/${organizationId}/projects`,
              },
            ]}
          />
        </div>
        <ContainerTitle>
          Projects
          <ContainerDescription>Select your project</ContainerDescription>
        </ContainerTitle>
        <ContainerAction>
          <CreateProjectModal
            isOpen={isCreateProjectModalOpen}
            setOpen={setCreateProjectModalOpen}
            organization={organization}
          />
        </ContainerAction>
      </ContainerHeader>

      <ContainerContent>
        {projects.length > 0 ? (
          <div className='m-auto flex flex-wrap gap-4'>
            {projects.map((project) => (
              <ProjectCard key={project.id} project={project} />
            ))}
          </div>
        ) : (
          <EmptyCardSection
            title='No Projects'
            description={`Get started by creating your first project in ${organization.name}.`}
            buttonText='Create Project'
            onClick={() => setCreateProjectModalOpen(true)}
          />
        )}
      </ContainerContent>
    </Container>
  );
}

export default ProjectsListPage;
