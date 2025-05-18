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
import useOrganizationStore from '@/store/use-organization-store';
import { shallow } from 'zustand/shallow';
import { useNavigate, useParams } from 'react-router-dom';
import { useEffect, useMemo } from 'react';

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

  const { data: projects, isLoading } = useListProjects({
    organizationId: selectedOrganizationId,
  });

  const selectedProject = useMemo(() => {
    return projects?.find((proj) => proj.id === projectId);
  }, [projectId, projects]);

  useEffect(() => {
    if (selectedProject !== undefined) {
      setSelectedOrganizationId(selectedProject?.organizationId);
    }
  }, [selectedProject, setSelectedOrganizationId]);

  return (
    <div className="flex flex-1">
      {!projects || isLoading ? (
        <Skeleton className="bg-muted h-6 w-24" />
      ) : (
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button
              variant="ghost"
              size="sm"
              className="flex h-7 items-center gap-1 px-2 text-xs"
            >
              <span className="max-w-[100px] truncate">
                {projects.find((project) => project.id === projectId)?.name ??
                  'Select Project'}
              </span>
              <ChevronDown className="text-muted-foreground ml-1 h-4 w-4" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="start">
            {projects.length > 0 ? (
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
            <DropdownMenuItem disabled>
              <PlusIcon /> Create project
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      )}
    </div>
  );
}
