import React, { useState } from 'react';
import { useParams, Link } from 'react-router-dom';
import { useListProjects } from '@/services/controlplane-api/useListProjects.hook';
import { useGetOrganization } from '@/services/controlplane-api/useGetOrganization.hook';
import { toast } from 'sonner';
import { Button, Spinner } from '@/components/ui';
import ProjectCard from '@/features/organizations/components/ProjectCard';
import CreateProjectModal from '@/features/organizations/components/CreateProjectModal';
import EmptyCardSection from '@/components/common/EmptyCardSection';
import { Skeleton } from '@/components/ui/skeleton';
import { AlertTriangle } from 'lucide-react';

const ProjectsByOrgPage: React.FC = () => {
  const { organizationId } = useParams<{ organizationId: string }>();
  const [isCreateProjectModalOpen, setCreateProjectModalOpen] = useState(false);

  const {
    data: organization,
    isLoading: isLoadingOrganization,
    error: orgError,
  } = useGetOrganization({ organizationId });

  const {
    data: projects = [],
    isLoading: isLoadingProjects,
    error: projectsError,
  } = useListProjects({ organizationId });

  if (isLoadingOrganization || isLoadingProjects) {
    return (
      <div className="container mx-auto p-4 md:p-6 lg:p-8 space-y-6">
        <Skeleton className="h-10 w-1/2 mb-4" /> {}
        <div className="flex justify-end mb-4">
          <Skeleton className="h-10 w-32" /> {}
        </div>
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
          {[...Array(3)].map((_, i) => (
            <Skeleton key={i} className="h-48 w-full" /> 
          ))}
        </div>
      </div>
    );
  }

  if (orgError || projectsError) {
    return (
      <div className="container mx-auto p-4 md:p-6 lg:p-8 text-center">
        <AlertTriangle className="h-12 w-12 text-destructive mx-auto mb-4" />
        <h2 className="text-xl font-semibold text-destructive mb-2">
          Error Loading Page
        </h2>
        <p className="text-muted-foreground">
          {orgError?.error || projectsError?.error || 'An unexpected error occurred.'}
        </p>
        <Button asChild variant="outline">
          <Link to="/">Go to Organizations</Link>
        </Button>
      </div>
    );
  }

  if (!organization) {
    return (
      <div className="container mx-auto p-4 md:p-6 lg:p-8 text-center">
        <AlertTriangle className="h-12 w-12 text-muted-foreground mx-auto mb-4" />
        <h2 className="text-xl font-semibold mb-2">Organization Not Found</h2>
        <Button asChild variant="outline">
          <Link to="/">Go to Organizations</Link>
        </Button>
      </div>
    );
  }

  return (
    <div className="container mx-auto p-4 md:p-6 lg:p-8 space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold">
          Projects in {organization.name}
        </h1>
        <CreateProjectModal
          isOpen={isCreateProjectModalOpen}
          setOpen={setCreateProjectModalOpen}
          organizations={[organization]} 
          organizationId={organization.id} 
        />
      </div>

      {projects.length > 0 ? (
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
          {projects.map((project) => (
            <ProjectCard
              key={project.id}
              organization={organization}
              project={project}
            />
          ))}
        </div>
      ) : (
        <EmptyCardSection
          title="No Projects"
          description={`Get started by creating your first project in ${organization.name}.`}
          buttonText="Create Project"
          onClick={() => setCreateProjectModalOpen(true)}
        />
      )}
    </div>
  );
};

export default ProjectsByOrgPage;