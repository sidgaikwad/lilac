import React from 'react';
import { NavLink, useParams } from 'react-router-dom';
import { cn } from '@/lib/utils';
import { ScrollArea } from '@/components/ui/scroll-area'; // Use ScrollArea for potentially long lists

interface ProjectSectionSidebarProps {
  // Props to determine which section's links to show (e.g., 'database', 'pipelines', 'datasets')
  // This will likely come from the parent layout or routing state
  activeSection?: string; // Example prop
}

// Mock data structure for links - replace with actual logic
const sectionLinks: Record<
  string,
  { title: string; items: { href: string; label: string }[] }
> = {
  // Removed 'database' section for now
  pipelines: {
    title: 'Pipelines',
    items: [
      { href: '', label: 'All Pipelines' }, // Link to base pipelines page
      { href: 'editor', label: 'New Pipeline' }, // Example link
      // Add other pipeline related links...
    ],
  },
  datasets: {
    // Ensure 'datasets' section exists
    title: 'Data Sets',
    items: [
      { href: '', label: 'All Data Sets' }, // Link to base datasets page
      // Removed Visualizer link
      // Add other dataset related links... e.g., specific dataset views?
    ],
  },
  // Add other sections like 'settings' if needed (though settings might be global)
};

const ProjectSectionSidebar: React.FC<ProjectSectionSidebarProps> = ({
  activeSection = 'pipelines' /* Default to pipelines for now */,
}) => {
  const { projectId } = useParams<{ projectId: string }>();
  const currentSection = sectionLinks[activeSection];

  if (!projectId || !currentSection) {
    // Don't render sidebar if no project or section is active/found
    return null;
  }

  const getNavLinkClass = ({ isActive }: { isActive: boolean }) =>
    cn(
      'block px-4 py-1.5 text-sm rounded-md transition-colors', // Base styling
      isActive
        ? 'bg-muted text-foreground font-medium' // Active style
        : 'text-muted-foreground hover:text-foreground hover:bg-muted/50' // Inactive style
    );

  return (
    <aside className="w-64 bg-background border-r border-border p-4 mx-4 shrink-0 hidden md:block">
      {' '}
      {/* Hide on small screens */}
      <ScrollArea className="h-full">
        <h2 className="text-lg font-semibold mb-4 px-4">
          {currentSection.title}
        </h2>
        <nav className="flex flex-col gap-1">
          {currentSection.items.map((item) => (
            <NavLink
              key={item.href}
              // Construct full path relative to the current section base
              // e.g., /projects/:projectId/pipelines/editor
              to={`/projects/${projectId}/${activeSection}/${item.href}`.replace(
                /\/$/,
                ''
              )} // Remove trailing slash if href is empty
              end={item.href === ''} // `end` prop for exact match on base path
              className={getNavLinkClass}
            >
              {item.label}
            </NavLink>
          ))}
        </nav>
      </ScrollArea>
    </aside>
  );
};

export default ProjectSectionSidebar;
