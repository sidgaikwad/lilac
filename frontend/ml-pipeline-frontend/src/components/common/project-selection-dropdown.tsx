import React, { useState, useEffect, useMemo, useRef } from 'react';
import { useListProjects, useListOrganizations } from '@/services';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Button } from '@/components/ui/button';
import { Skeleton } from '@/components/ui/skeleton';
import { ChevronDown, PlusIcon } from 'lucide-react';
import useOrganizationStore from '@/store/use-organization-store';
import { shallow } from 'zustand/shallow';
import { useNavigate, useParams } from 'react-router-dom';
import CreateProjectModal from '@/features/projects/components/create-project-modal';
import { toast } from 'sonner';

export default function ProjectSelectionDropdown() {
  const { projectId } = useParams<'projectId'>();
  const navigate = useNavigate();
  const { selectedOrganizationId, setSelectedOrganizationId } =
    useOrganizationStore(
      (state) => ({
        selectedOrganizationId: state.selectedOrganizationId,
        setSelectedOrganizationId: state.setSelectedOrganizationId,
      }),
      shallow
    );

  const { data: projects, isLoading: isLoadingProjects } = useListProjects({
    organizationId: selectedOrganizationId,
    enabled: !!selectedOrganizationId,
  });

  const { data: organizations, isLoading: isLoadingOrganizations } =
    useListOrganizations();

  const selectedOrgObject = React.useMemo(
    () =>
      organizations?.find(
        (org) => org.organizationId === selectedOrganizationId
      ),
    [organizations, selectedOrganizationId]
  );

  const [isCreateProjectModalOpen, setCreateProjectModalOpen] = useState(false);
  const triggerRef = useRef<HTMLButtonElement>(null);

  const handleModalOpenChange = (open: boolean) => {
    setCreateProjectModalOpen(open);
    if (!open && triggerRef.current) {
      setTimeout(() => triggerRef.current?.focus(), 0);
    }
  };

  const selectedProject = useMemo(() => {
    return projects?.find((proj) => proj.id === projectId);
  }, [projectId, projects]);

  useEffect(() => {
    if (selectedProject !== undefined) {
      setSelectedOrganizationId(selectedProject?.organizationId);
    }
  }, [selectedProject, setSelectedOrganizationId]);

  return (
    <>
      <div className='flex flex-1'>
        {isLoadingProjects ||
        (selectedOrganizationId && isLoadingOrganizations) ? (
          <Skeleton className='bg-muted h-6 w-24' />
        ) : (
          <DropdownMenu modal={false}>
            <DropdownMenuTrigger asChild>
              <Button
                ref={triggerRef}
                variant='ghost'
                size='sm'
                className='flex h-7 items-center gap-1 px-2 text-xs'
                disabled={!selectedOrganizationId}
              >
                <span className='max-w-[100px] truncate'>
                  {selectedProject?.name ??
                    (selectedOrganizationId
                      ? 'Select Project'
                      : 'No Org Selected')}
                </span>
                <ChevronDown className='text-muted-foreground ml-1 h-4 w-4' />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align='start'>
              {projects && projects.length > 0 ? (
                projects.map((project) => (
                  <DropdownMenuItem
                    key={project.id}
                    onSelect={() => {
                      navigate(`/projects/${project.id}`);
                    }}
                  >
                    {project.name}
                  </DropdownMenuItem>
                ))
              ) : (
                <DropdownMenuItem disabled>
                  {selectedOrganizationId
                    ? 'No projects found'
                    : 'Select an organization first'}
                </DropdownMenuItem>
              )}
              <DropdownMenuSeparator />
              <DropdownMenuItem
                onSelect={() => {
                  if (selectedOrgObject) {
                    setCreateProjectModalOpen(true);
                  } else {
                    toast.error(
                      'Please select an organization or wait for it to load.'
                    );
                  }
                }}
                disabled={!selectedOrgObject || isLoadingOrganizations}
              >
                <PlusIcon className='mr-2 h-4 w-4' />
                <span>Create project</span>
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        )}
      </div>
      {selectedOrgObject && (
        <CreateProjectModal
          isOpen={isCreateProjectModalOpen}
          setOpen={handleModalOpenChange}
          organization={selectedOrgObject}
          showTrigger={false}
        />
      )}
    </>
  );
}
