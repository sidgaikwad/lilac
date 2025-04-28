import React, { useEffect, useState } from 'react'; // Removed useState
import { Link, useNavigate, useParams } from 'react-router-dom';
import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';
import { ThemeToggle } from '@/components/common/ThemeToggle';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import {
  ChevronDown,
  LifeBuoy,
  LogOut,
  Settings,
  User,
  Box,
  PlusIcon,
} from 'lucide-react'; // Added Loader2Icon
import useAuthStore from '@/store/useAuthStore';
import { Organization, Project } from '@/types';
// Import TanStack Query hooks
import { useListOrganizations } from '@/services/controlplane-api/useListOrganizations.hook';
import { useListProjects } from '@/services/controlplane-api/useListProjects.hook';
import { Spinner } from '../ui';
import { Skeleton } from '../ui/skeleton';

// Removed mock service imports and user definition

const Header: React.FC = () => {
  const logout = useAuthStore((state) => state.logout);
  const user = useAuthStore((state) => state.user);
  const { projectId } = useParams();
  const navigate = useNavigate();

  const [organizationId, setOrganizationId] = useState<string>();

  // Fetch organizations using the hook
  const {
    data: organizations = [],
    isLoading: isLoadingOrgs,
    isFetching: isFetchingOrgs,
  } = useListOrganizations();

  // Fetch projects using the hook, dependent on selectedOrganization
  const {
    data: projects = [],
    isLoading: isLoadingProjects,
    isFetching: isFetchingProjects,
  } = useListProjects({ organizationId });

  useEffect(() => {
    setOrganizationId(
      projects.find((proj) => proj.id === projectId)?.organizationId
    );
  }, [projects, projectId]);

  const handleLogout = () => {
    logout();
    // Optionally clear selected org/project on logout
    setOrganizationId(undefined);
  };

  const handleOrgSelect = (org: Organization) => {
    // Setting the org in the store will trigger the useListProjects hook
    // and the useEffect above will handle selecting the first project.
    setOrganizationId(org.id);
  };

  const handleProjectSelect = (project: Project) => {
    navigate(`/projects/${project.id}`);
  };

  // Determine loading states for dropdowns
  const orgDropdownLoading =
    isLoadingOrgs || (isFetchingOrgs && organizations.length === 0);
  const projectDropdownLoading =
    isLoadingProjects || (isFetchingProjects && projects.length === 0);

  return (
    <header className="sticky top-0 z-30 flex h-14 items-center gap-4 border-b bg-background px-4 sm:static sm:h-auto sm:border-0 sm:bg-transparent sm:px-6 py-2">
      {/* Left side: Logo -> Org -> Project */}
      <div className="flex items-center gap-3">
        <Link
          to="/"
          className="flex items-center justify-center text-foreground hover:text-foreground/80"
        >
          <Box className="h-6 w-6" />
        </Link>

        <Separator orientation="vertical" className="h-6" />

        {/* Org Dropdown */}
        {orgDropdownLoading || projectDropdownLoading ? (
          <Skeleton className="w-24 h-6 bg-muted-background" />
        ) : (
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button
                variant="ghost"
                size="sm"
                className="flex items-center gap-1 px-2 h-7 text-xs"
              >
                <span className="truncate max-w-[100px]">
                  {organizations.find((org) => org.id === organizationId)
                    ?.name ?? 'Select Org'}
                </span>
                <ChevronDown className="h-4 w-4 text-muted-foreground ml-1" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="start">
              {orgDropdownLoading ? (
                <DropdownMenuItem disabled>
                  <Spinner show={orgDropdownLoading} />
                </DropdownMenuItem>
              ) : organizations.length > 0 ? (
                organizations.map((org) => (
                  <DropdownMenuItem
                    key={org.id}
                    onSelect={() => handleOrgSelect(org)}
                  >
                    {org.name}
                  </DropdownMenuItem>
                ))
              ) : (
                <DropdownMenuItem disabled>
                  No organizations found
                </DropdownMenuItem>
              )}
              <DropdownMenuSeparator />
              <DropdownMenuItem disabled>
                <PlusIcon /> Create organization
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        )}

        <Separator orientation="vertical" className="h-6" />

        {/* Project Dropdown */}
        {orgDropdownLoading || projectDropdownLoading ? (
          <Skeleton className="w-24 h-6 bg-muted-background" />
        ) : (
          <DropdownMenu>
            <DropdownMenuTrigger
              asChild
              disabled={!organizations || projectDropdownLoading}
            >
              <Button
                variant="ghost"
                size="sm"
                className="flex items-center gap-1 px-2 h-7 text-sm font-medium"
              >
                <span className="truncate max-w-[120px]">
                  {projects.find((proj) => proj.id === projectId)?.name ??
                    'Select Project'}
                </span>
                <ChevronDown className="h-4 w-4 text-muted-foreground ml-1" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="start">
              {projectDropdownLoading ? (
                <DropdownMenuItem disabled>
                  <Spinner show={projectDropdownLoading} />
                </DropdownMenuItem>
              ) : projects.length > 0 ? (
                projects.map((proj) => (
                  <DropdownMenuItem
                    key={proj.id}
                    onSelect={() => handleProjectSelect(proj)}
                  >
                    {proj.name}
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

      {/* Right side: Theme Toggle, User Menu */}
      <div className="ml-auto flex items-center gap-4">
        <ThemeToggle />
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button
              variant="ghost"
              size="icon"
              className="rounded-full h-8 w-8"
            >
              <Avatar className="h-8 w-8">
                <AvatarImage src={''} alt={user?.email} />
                <AvatarFallback>
                  {user?.email.substring(0, 1).toUpperCase()}
                </AvatarFallback>
              </Avatar>
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuLabel>My Account</DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuItem asChild>
              <Link to="/settings/account">
                <User className="mr-2 h-4 w-4" />
                Profile
              </Link>
            </DropdownMenuItem>
            <DropdownMenuItem asChild>
              <Link to="/settings">
                <Settings className="mr-2 h-4 w-4" />
                Settings
              </Link>
            </DropdownMenuItem>
            <DropdownMenuItem>
              {' '}
              <LifeBuoy className="mr-2 h-4 w-4" /> Support
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem onClick={handleLogout}>
              <LogOut className="mr-2 h-4 w-4" />
              Logout
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </header>
  );
};

export default Header;
