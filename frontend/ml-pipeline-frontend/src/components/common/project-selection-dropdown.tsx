import { useState, useMemo, useRef } from 'react';
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
import { Link, useParams } from 'react-router-dom';
import CreateProjectModal from '@/features/projects/components/create-project-modal';

export default function ProjectSelectionDropdown() {
  const { projectId } = useParams<'projectId'>();
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
    return projects?.find((proj) => proj.projectId === projectId);
  }, [projectId, projects]);

  return (
    <div>
      <div className='flex w-full'>
        {isLoadingProjects ? (
          <Skeleton className='h-6' />
        ) : (
          <DropdownMenu modal={false}>
            <DropdownMenuTrigger asChild>
              <Button
                ref={triggerRef}
                variant='ghost'
                size='icon'
                className='w-fit justify-around'
              >
                <span className='truncate'>
                  {selectedProject?.projectName ?? 'Select Project'}
                </span>
                <ChevronDown className='text-gray-text-muted' />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent side='bottom' align='start'>
              {projects && projects.length > 0 ? (
                projects.map((project) => (
                  <DropdownMenuItem asChild key={project.projectId}>
                    <Link to={`/projects/${project.projectId}`}>
                      {project.projectName}
                    </Link>
                  </DropdownMenuItem>
                ))
              ) : (
                <DropdownMenuItem disabled>No projects found</DropdownMenuItem>
              )}
              <DropdownMenuSeparator />
              <DropdownMenuItem asChild>
                <Button
                  variant='ghost'
                  onClick={() => {
                    setCreateProjectModalOpen(true);
                  }}
                >
                  <PlusIcon className='mr-2 h-4 w-4' />
                  <span>Create project</span>
                </Button>
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
    </div>
  );
}
