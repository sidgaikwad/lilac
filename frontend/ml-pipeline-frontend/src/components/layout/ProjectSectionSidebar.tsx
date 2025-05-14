import React from 'react';
import { NavLink, useParams } from 'react-router-dom';
import { cn } from '@/lib/utils';
import { ScrollArea } from '@/components/ui/scroll-area';
import { useListPipelines } from '@/services/controlplane-api/useListPipelines.hook';
import { WorkflowIcon } from 'lucide-react';

interface ProjectSectionSidebarProps {
  activeSection?: string;
}


const baseSectionLinks: Record<
  string,
  { title: string; baseHref: string; baseLabel: string }
> = {
  pipelines: {
    title: 'Pipelines',
    baseHref: '', 
    baseLabel: 'All Pipelines',
  },
  datasets: {
    title: 'Data Sets',
    baseHref: '', 
    baseLabel: 'All Data Sets',
  },
};

const ProjectSectionSidebar: React.FC<ProjectSectionSidebarProps> = ({
  activeSection = 'pipelines',
}) => {
  const { projectId } = useParams<{ projectId: string }>();
  const currentBaseSection = baseSectionLinks[activeSection];

  const { data: pipelines, isLoading: isLoadingPipelines } = useListPipelines({
    projectId,
  });

  if (!projectId || !currentBaseSection) {
    return null;
  }

  const getNavLinkClass = ({ isActive }: { isActive: boolean }) =>
    cn(
      'block px-4 py-1.5 text-sm rounded-md transition-colors truncate',
      isActive
        ? 'bg-muted text-foreground font-medium'
        : 'text-muted-foreground hover:text-foreground hover:bg-muted/50'
    );

  return (
    <aside className="w-64 bg-background border-r border-border p-4 mx-4 shrink-0 hidden md:block">
      <ScrollArea className="h-full">
        <h2 className="text-lg font-semibold mb-4 px-4">
          {currentBaseSection.title}
        </h2>
        <nav className="flex flex-col gap-1">
          {}
          <NavLink
            to={`/projects/${projectId}/${activeSection}/${currentBaseSection.baseHref}`.replace(
              /\/$/,
              ''
            )}
            end={currentBaseSection.baseHref === ''}
            className={getNavLinkClass}
          >
            {currentBaseSection.baseLabel}
          </NavLink>

          {}
          {activeSection === 'pipelines' && (
            <>
              {isLoadingPipelines && (
                <span className="px-4 py-1.5 text-sm text-muted-foreground">
                  Loading pipelines...
                </span>
              )}
              {!isLoadingPipelines &&
                pipelines?.map((pipeline) => (
                  <NavLink
                    key={pipeline.id}
                    to={`/projects/${projectId}/pipelines/${pipeline.id}`}
                    className={getNavLinkClass}
                    title={pipeline.name}
                  >
                    <WorkflowIcon className="inline-block mr-2 h-4 w-4 flex-shrink-0" />
                    {pipeline.name}
                  </NavLink>
                ))}
            </>
          )}
          {}
        </nav>
      </ScrollArea>
    </aside>
  );
};

export default ProjectSectionSidebar;
