import React, { useEffect, useState } from 'react';
import { Link, useNavigate, useParams, useLocation } from 'react-router-dom';
import { useQueryClient } from '@tanstack/react-query';
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
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip';
import {
  ChevronDown,
  LifeBuoy,
  LogOut,
  Settings,
  User,
  Box,
  PlusIcon,
} from 'lucide-react';
import useAuthStore from '@/store/useAuthStore';
import useOrganizationStore from '@/store/useOrganizationStore';
import { Organization, Project } from '@/types';
import { useListOrganizations } from '@/services/controlplane-api/useListOrganizations.hook';
import { useListProjects } from '@/services/controlplane-api/useListProjects.hook';
import { QueryKeys } from '@/services/controlplane-api/constants';
import { Spinner } from '../ui';
import { Skeleton } from '../ui/skeleton';

const Header: React.FC = () => {
  const logout = useAuthStore((state) => state.logout);
  const user = useAuthStore((state) => state.user);
  const { projectId } = useParams();
  const navigate = useNavigate();
  const location = useLocation();
  const queryClient = useQueryClient();

  const [localDisplayOrgId, setLocalDisplayOrgId] = useState<string | undefined>();

  const {
    data: organizations = [],
    isLoading: isLoadingOrgs,
    isFetching: isFetchingOrgs,
  } = useListOrganizations();

  const {
    data: projects = [],
    isLoading: isLoadingProjects,
    isFetching: isFetchingProjects,
  } = useListProjects({ organizationId: localDisplayOrgId });

  const {
    selectedOrganizationId: globalSelectedOrgId,
    setSelectedOrganizationId: setGlobalSelectedOrgId,
  } = useOrganizationStore();

  useEffect(() => {
    let newEffectiveOrgId: string | undefined = globalSelectedOrgId;

    if (!newEffectiveOrgId && projectId && projects && projects.length > 0) {
      const orgIdFromProject = projects.find(p => p.id === projectId)?.organizationId;
      if (orgIdFromProject) {
        newEffectiveOrgId = orgIdFromProject;
        if (globalSelectedOrgId !== orgIdFromProject) {
          setGlobalSelectedOrgId(orgIdFromProject); 
        }
      }
    }

    if (localDisplayOrgId !== newEffectiveOrgId) {
      setLocalDisplayOrgId(newEffectiveOrgId);
    }
  }, [globalSelectedOrgId, projectId, projects, localDisplayOrgId, setGlobalSelectedOrgId]);


  const handleLogout = () => {
    logout();
    setGlobalSelectedOrgId(undefined);
    setLocalDisplayOrgId(undefined);
  };

  const handleOrgSelect = (org: Organization) => {
    setGlobalSelectedOrgId(org.id);
    setLocalDisplayOrgId(org.id); 
    navigate(`/organizations/${org.id}/projects`);
  };

  const handleProjectSelect = (newSelectedProject: Project) => {
    const currentPath = location.pathname;
    const projectPathRegex = /^\/projects\/([^/]+)(\/.*)?$/;
    const match = currentPath.match(projectPathRegex);

    let suffix = '';
    if (match && match[2]) {
      const knownSuffixes = ['/pipelines', '/datasets', '/database', '/auth', '/storage'];
      const sectionCandidate = '/' + match[2].split('/')[1];
      if (knownSuffixes.includes(sectionCandidate)) {
        suffix = sectionCandidate;
      }
    }

    if (globalSelectedOrgId !== newSelectedProject.organizationId) {
      setGlobalSelectedOrgId(newSelectedProject.organizationId);
    } else if (localDisplayOrgId !== newSelectedProject.organizationId) {
      setLocalDisplayOrgId(newSelectedProject.organizationId);
    }
    
    navigate(`/projects/${newSelectedProject.id}${suffix}`);
    
    queryClient.invalidateQueries({ queryKey: [QueryKeys.LIST_DATASETS, newSelectedProject.id] });
    queryClient.invalidateQueries({ queryKey: [QueryKeys.LIST_PIPELINE, newSelectedProject.id] });
  };

  const orgDropdownLoading =
    isLoadingOrgs || (isFetchingOrgs && organizations.length === 0);
  const projectDropdownLoading =
    isLoadingProjects || (isFetchingProjects && projects.length === 0);

  return (
    <header className="sticky top-0 z-30 flex h-14 items-center gap-4 border-b bg-background px-4 sm:static sm:h-auto sm:border-0 sm:bg-transparent sm:px-6 py-2">
      <div className="flex items-center gap-3">
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Link
                to="/"
                className="flex items-center justify-center text-foreground hover:text-foreground/80"
              >
                <Box className="h-6 w-6" />
              </Link>
            </TooltipTrigger>
            <TooltipContent>
              <p>Organizations Overview</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>

        <Separator orientation="vertical" className="h-6" />

        {orgDropdownLoading ? (
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
                  {organizations.find((org) => org.id === localDisplayOrgId)
                    ?.name ?? 'Select Org'}
                </span>
                <ChevronDown className="h-4 w-4 text-muted-foreground ml-1" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="start">
              {organizations.length > 0 ? (
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

        {projectDropdownLoading || !localDisplayOrgId ? (
          <Skeleton className="w-24 h-6 bg-muted-background" />
        ) : (
          <DropdownMenu>
            <DropdownMenuTrigger
              asChild
              disabled={!localDisplayOrgId || projects.length === 0 && !isLoadingProjects}
            >
              <Button
                variant="ghost"
                size="sm"
                className="flex items-center gap-1 px-2 h-7 text-xs font-medium"
              >
                <span className="truncate max-w-[120px]">
                  {projects.find((proj) => proj.id === projectId)?.name ??
                    'Select Project'}
                </span>
                <ChevronDown className="h-4 w-4 text-muted-foreground ml-1" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="start">
              {projects.length > 0 ? (
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
            <DropdownMenuItem asChild>
              <a href="mailto:support@example.com">
                <LifeBuoy className="mr-2 h-4 w-4" /> Support
              </a>
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
