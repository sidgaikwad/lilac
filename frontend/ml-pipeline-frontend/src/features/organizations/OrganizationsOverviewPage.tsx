import React, { useEffect, useState } from 'react';
import { useListOrganizations } from '@/services/controlplane-api/useListOrganizations.hook';
import useOrganizationStore from '@/store/useOrganizationStore';
import { useListProjects } from '@/services/controlplane-api/useListProjects.hook';
import { toast } from 'sonner';
import { Spinner } from '@/components/ui';
import ProjectCard from './components/ProjectCard';
import CreateOrganizationModal from './components/CreateOrganizationModal';
import EmptyProjectsCard from './components/EmptyProjectsCard';
import CreateProjectModal from './components/CreateProjectModal';
import EmptyCardSection from '@/components/common/EmptyCardSection';

const OrganizationsOverviewPage: React.FC = () => {
  const [isCreateOrgModalOpen, setCreateOrgModalOpen] = useState(false);
  const [isCreateProjectModalOpen, setCreateProjectModalOpen] = useState(false);
  const [orgId, setOrgId] = useState<string>();

  const { setSelectedOrganizationId } = useOrganizationStore();
  const { data: organizations = [], isLoading: isLoadingOrganizations } =
    useListOrganizations({
      onError: (error) =>
        toast.error(
          `Error listing organizations: ${error.statusCode} ${error.error}`,
          {
            dismissible: true,
            duration: Infinity,
          }
        ),
    });

  const { data: projects = [], isLoading: isLoadingProjects } = useListProjects(
    { organizationId: undefined }
  );

  useEffect(() => {
    setSelectedOrganizationId(undefined);
  }, []);

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold">Organizations</h1>
        <div className="space-x-4">
          <CreateOrganizationModal
            isOpen={isCreateOrgModalOpen}
            setOpen={setCreateOrgModalOpen}
          />
          <CreateProjectModal
            isOpen={isCreateProjectModalOpen}
            setOpen={setCreateProjectModalOpen}
            organizations={organizations}
            organizationId={orgId}
          />
        </div>
      </div>
      <Spinner show={isLoadingOrganizations || isLoadingProjects} />
      {organizations.length > 0 ? (
        organizations.map((org) => (
          <div key={org.id}>
            <h3 className="text-lg font-bold mx-auto px-2 py-2">{org.name}</h3>
            <div className="container flex flex-wrap gap-2 md:gap-4">
              {projects.filter((proj) => proj.organizationId === org.id)
                .length !== 0 ? (
                projects
                  .filter((proj) => proj.organizationId === org.id)
                  .map((project) => (
                    <ProjectCard
                      key={project.id}
                      organization={org}
                      project={project}
                    />
                  ))
              ) : (
                <EmptyProjectsCard
                  key={'empty-project'}
                  onClick={() => {
                    setOrgId(org.id);
                    setCreateProjectModalOpen(true);
                  }}
                />
              )}
            </div>
          </div>
        ))
      ) : (
        <EmptyCardSection
          title="No Organizations"
          description="Get started by creating a new organization."
          buttonText="Create Organization"
          onClick={() => setCreateOrgModalOpen(true)}
        />
      )}
    </div>
  );
};
export default OrganizationsOverviewPage;
