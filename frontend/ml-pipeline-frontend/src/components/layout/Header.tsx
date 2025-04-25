import React, { useEffect } from 'react'; // Removed useState
import { Link } from 'react-router-dom';
import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';
import { ThemeToggle } from '@/components/common/ThemeToggle';
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { ChevronDown, LifeBuoy, LogOut, Settings, User, Box, Loader2Icon } from 'lucide-react'; // Added Loader2Icon
import useAuthStore from '@/store/authStore';
import useOrganizationStore from '@/store/organizationStore';
import { Organization, Project } from '@/types';
// Import TanStack Query hooks
import { useListOrganizations } from '@/services/controlplane-api/useListOrganizations.hook';
import { useListProjects } from '@/services/controlplane-api/useListProjects.hook';

// Removed mock service imports and mockUser definition

// TODO: Fetch actual user data
const mockUser = { name: "Luew2", email: "luew2@example.com", avatarUrl: "" };

const Header: React.FC = () => {
    const logout = useAuthStore(state => state.logout);
    const {
        selectedOrganization,
        selectedProject,
        setSelectedOrganization,
        setSelectedProject
    } = useOrganizationStore();

    // Fetch organizations using the hook
    const { data: organizations = [], isLoading: isLoadingOrgs, isFetching: isFetchingOrgs } = useListOrganizations();

    // Fetch projects using the hook, dependent on selectedOrganization
    const { data: projects = [], isLoading: isLoadingProjects, isFetching: isFetchingProjects } = useListProjects({
        organizationId: selectedOrganization?.id,
    });

    // Effect to automatically select the first project when org changes or projects load
    useEffect(() => {
        // Only run if projects have loaded for the selected org and we aren't still fetching them
        if (selectedOrganization && !isFetchingProjects && projects) {
            // Check if current selection is invalid or null
            if (!selectedProject || selectedProject.organization_id !== selectedOrganization.id) {
                console.log("Header: Auto-selecting first project:", projects[0]?.name ?? 'None');
                setSelectedProject(projects[0] || null);
            }
        }
        // If no org is selected, the project should already be null due to store logic,
        // but we ensure it if projects becomes empty while an org is technically selected
        else if (selectedOrganization && !isFetchingProjects && projects.length === 0) {
             if (selectedProject) {
                 console.log("Header: Clearing project selection as none exist for the org.");
                 setSelectedProject(null);
             }
        }
    }, [selectedOrganization, projects, selectedProject, isFetchingProjects, setSelectedProject]);


    const handleLogout = () => {
        logout();
        // Optionally clear selected org/project on logout
        // setSelectedOrganization(null);
    };

    const handleOrgSelect = (org: Organization) => {
        // Setting the org in the store will trigger the useListProjects hook
        // and the useEffect above will handle selecting the first project.
        setSelectedOrganization(org);
    };

    const handleProjectSelect = (project: Project) => {
        setSelectedProject(project);
    };

    // Determine loading states for dropdowns
    const orgDropdownLoading = isLoadingOrgs || (isFetchingOrgs && organizations.length === 0);
    const projectDropdownLoading = isLoadingProjects || (isFetchingProjects && projects.length === 0);

    return (
        <header className="sticky top-0 z-30 flex h-14 items-center gap-4 border-b bg-background px-4 sm:static sm:h-auto sm:border-0 sm:bg-transparent sm:px-6 py-2">
            {/* Left side: Logo -> Org -> Project */}
            <div className="flex items-center gap-3">
                 <Link to="/" className="flex items-center justify-center text-foreground hover:text-foreground/80">
                    <Box className="h-6 w-6" />
                 </Link>

                 <Separator orientation="vertical" className="h-6" />

                 {/* Org Dropdown */}
                 <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                        <Button variant="ghost" size="sm" className="flex items-center gap-1 px-2 h-7 text-xs">
                            {orgDropdownLoading ? <Loader2Icon className="h-4 w-4 animate-spin mr-1" /> : null}
                            <span className="truncate max-w-[100px]">{selectedOrganization?.name ?? 'Select Org'}</span>
                            <ChevronDown className="h-4 w-4 text-muted-foreground ml-1" />
                        </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="start">
                        <DropdownMenuLabel>Organizations</DropdownMenuLabel>
                        <DropdownMenuSeparator />
                        {orgDropdownLoading ? (
                            <DropdownMenuItem disabled>Loading...</DropdownMenuItem>
                        ) : organizations.length > 0 ? (
                            organizations.map(org => (
                                <DropdownMenuItem key={org.id} onSelect={() => handleOrgSelect(org)}>
                                    {org.name}
                                </DropdownMenuItem>
                            ))
                        ) : (
                             <DropdownMenuItem disabled>No organizations found</DropdownMenuItem>
                        )}
                        <DropdownMenuSeparator />
                        <DropdownMenuItem disabled>+ Create New Org</DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>

                <Separator orientation="vertical" className="h-6" />

                {/* Project Dropdown */}
                <DropdownMenu>
                    <DropdownMenuTrigger asChild disabled={!selectedOrganization || projectDropdownLoading}>
                        <Button variant="ghost" size="sm" className="flex items-center gap-1 px-2 h-7 text-sm font-medium">
                             {projectDropdownLoading ? <Loader2Icon className="h-4 w-4 animate-spin mr-1" /> : null}
                            <span className="truncate max-w-[120px]">
                                {selectedProject ? selectedProject.name : "Select Project"}
                            </span>
                            <ChevronDown className="h-4 w-4 text-muted-foreground ml-1" />
                        </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="start">
                        <DropdownMenuLabel>Projects in {selectedOrganization?.name ?? '...'}</DropdownMenuLabel>
                        <DropdownMenuSeparator />
                        {projectDropdownLoading ? (
                             <DropdownMenuItem disabled>Loading...</DropdownMenuItem>
                        ) : projects.length > 0 ? (
                            projects.map(proj => (
                                <DropdownMenuItem key={proj.id} onSelect={() => handleProjectSelect(proj)}>
                                    {proj.name}
                                </DropdownMenuItem>
                            ))
                        ) : (
                            <DropdownMenuItem disabled>No projects found</DropdownMenuItem>
                        )}
                        <DropdownMenuSeparator />
                        <DropdownMenuItem disabled>+ Create New Project</DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            </div>

            {/* Right side: Theme Toggle, User Menu */}
            <div className="ml-auto flex items-center gap-4">
                <ThemeToggle />
                <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                        <Button variant="ghost" size="icon" className="rounded-full h-8 w-8">
                            <Avatar className="h-8 w-8">
                                <AvatarImage src={mockUser.avatarUrl} alt={mockUser.name} />
                                <AvatarFallback>{mockUser.name.substring(0, 1).toUpperCase()}</AvatarFallback>
                            </Avatar>
                        </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                        <DropdownMenuLabel>My Account</DropdownMenuLabel>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem asChild><Link to="/settings/account"><User className="mr-2 h-4 w-4" />Profile</Link></DropdownMenuItem>
                        <DropdownMenuItem asChild><Link to="/settings"><Settings className="mr-2 h-4 w-4" />Settings</Link></DropdownMenuItem>
                        <DropdownMenuItem> <LifeBuoy className="mr-2 h-4 w-4" /> Support</DropdownMenuItem>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem onClick={handleLogout}><LogOut className="mr-2 h-4 w-4" />Logout</DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            </div>
        </header>
    );
};

export default Header;