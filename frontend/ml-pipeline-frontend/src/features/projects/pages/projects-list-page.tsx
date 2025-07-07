import { useState } from 'react';
import ProjectCard from '@/features/projects/components/project-card';
import CreateProjectModal from '@/features/projects/components/create-project-modal';
import EmptyCardSection from '@/components/common/empty-card-section';
import { Skeleton } from '@/components/ui/skeleton';
import { useListProjects } from '@/services';
import { toast } from 'sonner';
import {
  Container,
  ContainerAction,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';

function ProjectsListPage() {
  const [isCreateProjectModalOpen, setCreateProjectModalOpen] = useState(false);

  const { data: projects = [], isLoading: isLoadingProjects } = useListProjects(
    {
      onError: (error) =>
        toast.error('Failed to list projects', {
          description: error.error,
        }),
    }
  );

  if (isLoadingProjects) {
    return (
      <div className='container mx-auto space-y-6 p-4 md:p-6 lg:p-8'>
        <Skeleton className='mb-4 h-10 w-1/2' />
        <div className='mb-4 flex justify-end'>
          <Skeleton className='h-10 w-32' />
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
        <ContainerTitle>
          Projects
          <ContainerDescription>Select your project</ContainerDescription>
        </ContainerTitle>
        <ContainerAction>
          <CreateProjectModal
            isOpen={isCreateProjectModalOpen}
            setOpen={setCreateProjectModalOpen}
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
            description='Get started by creating your first project.'
            buttonText='Create Project'
            onClick={() => setCreateProjectModalOpen(true)}
          />
        )}
      </ContainerContent>
    </Container>
  );
}

export default ProjectsListPage;
