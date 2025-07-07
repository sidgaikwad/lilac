import React, { useState, useMemo, useRef } from 'react';
import { useListProjects } from '@/services';
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
import { useNavigate, useParams } from 'react-router-dom';
import CreateProjectModal from '@/features/projects/components/create-project-modal';

export default function ProjectSelectionDropdown() {
  const { projectId } = useParams<'projectId'>();
  const navigate = useNavigate();
  const { data: projects, isLoading: isLoadingProjects } = useListProjects();
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

  return (
    <>
      <div className='flex flex-1'>
        {isLoadingProjects ? (
          <Skeleton className='bg-muted h-6 w-24' />
        ) : (
          <DropdownMenu modal={false}>
            <DropdownMenuTrigger asChild>
              <Button
                ref={triggerRef}
                variant='ghost'
                size='sm'
                className='flex h-7 items-center gap-1 px-2 text-xs'
              >
                <span className='max-w-[100px] truncate'>
                  {selectedProject?.name ?? 'Select Project'}
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
                <DropdownMenuItem disabled>No projects found</DropdownMenuItem>
              )}
              <DropdownMenuSeparator />
              <DropdownMenuItem
                onSelect={() => {
                  setCreateProjectModalOpen(true);
                }}
              >
                <PlusIcon className='mr-2 h-4 w-4' />
                <span>Create project</span>
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        )}
      </div>
      <CreateProjectModal
        isOpen={isCreateProjectModalOpen}
        setOpen={handleModalOpenChange}
        showTrigger={false}
      />
    </>
  );
}
