import React from 'react';
import { Outlet, useLocation } from 'react-router-dom'; // Import useLocation
import Sidebar from './Sidebar';
import Header from './Header';
import ProjectSectionSidebar from './ProjectSectionSidebar'; // Import the secondary sidebar
import { cn } from '@/lib/utils'; // Import cn

const MainLayout: React.FC = () => {
  const location = useLocation();

  // Determine active section based on URL path (simple example)
  // Matches /projects/:projectId/SECTION_NAME/...
  const pathSegments = location.pathname.split('/').filter(Boolean);
  let activeSection: string | undefined = undefined;
  if (pathSegments[0] === 'projects' && pathSegments.length >= 3) {
      activeSection = pathSegments[2];
  }

  // Determine if we are in a project context to show the secondary sidebar
  const isInProjectContext = pathSegments[0] === 'projects' && pathSegments[1];

  return (
    <div className="flex h-screen bg-background text-foreground overflow-hidden">
      <Sidebar /> {/* Main icon sidebar */}
      <div className="flex flex-col flex-1 overflow-hidden">
        <Header />
        <div className="flex flex-1 overflow-hidden"> {/* Container for secondary sidebar + main content */}
          {/* Conditionally render secondary sidebar */}
          {isInProjectContext && <ProjectSectionSidebar activeSection={activeSection} />}
          {/* Adjust main content padding/margin based on secondary sidebar presence */}
          <main className={cn("flex-1 overflow-y-auto p-6", isInProjectContext && "md:pl-0")}> {/* Remove left padding on md+ if secondary sidebar is shown */}
            <Outlet />
          </main>
        </div>
      </div>
    </div>
  );
};

export default MainLayout;