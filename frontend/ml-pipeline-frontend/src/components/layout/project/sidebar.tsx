import { generatePath, useLocation, useParams } from 'react-router-dom';
import {
  Settings,
  LayoutDashboard,
  Home,
  ArrowLeft,
  Laptop,
} from 'lucide-react';
import {
  Sidebar as SidebarComponent,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarSeparator,
  useSidebar,
} from '@/components/ui/sidebar';
import { Routes } from '@/constants/routes';
import { Link } from 'react-router-dom';
import { useMemo } from 'react';

export default function Sidebar() {
  const { projectId } = useParams<{ projectId: string }>();
  const location = useLocation();
  const { setOpen } = useSidebar();

  const paths = useMemo(() => {
    if (!projectId) {
      return null;
    }
    return {
      [Routes.PROJECT_DETAILS]: generatePath(Routes.PROJECT_DETAILS, {
        projectId,
      }),
      [Routes.PROJECT_WORKSPACES]: generatePath(Routes.PROJECT_WORKSPACES, {
        projectId: projectId!,
      }),
      [Routes.PROJECT_SETTINGS]: generatePath(Routes.PROJECT_SETTINGS, {
        projectId,
      }),
    };
  }, [projectId]);

  if (!paths) {
    return null;
  }

  return (
    <SidebarComponent
      variant='sidebar'
      collapsible='icon'
      onMouseEnter={() => setOpen(true)}
      onMouseLeave={() => setOpen(false)}
      className='top-(--header-height) !h-[calc(100svh-var(--header-height))]'
    >
      <SidebarHeader />
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem className='w-fit'>
                <SidebarMenuButton
                  asChild
                  className='hover:bg-accent-background-1 hover:text-gray-text-muted hover:cursor-pointer'
                >
                  <Link to='/'>
                    <ArrowLeft />
                    <span>Back</span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarSeparator className='mx-0' />

        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton asChild isActive={location.pathname === '/'}>
                  <Link to='/'>
                    <Home />
                    <span>Projects</span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton
                  asChild
                  isActive={location.pathname === paths[Routes.PROJECT_DETAILS]}
                >
                  <Link to={paths[Routes.PROJECT_DETAILS]}>
                    <LayoutDashboard />
                    <span>Dashboard</span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton
                  asChild
                  isActive={
                    location.pathname === paths[Routes.PROJECT_WORKSPACES]
                  }
                >
                  <Link to={paths[Routes.PROJECT_WORKSPACES]}>
                    <Laptop />
                    <span>Workspaces</span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarSeparator className='mx-0' />

        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  asChild
                  isActive={location.pathname === paths[Routes.PROJECT_SETTINGS]}
                >
                  <Link to={paths[Routes.PROJECT_SETTINGS]}>
                    <Settings />
                    <span>Project Settings</span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>

      <SidebarFooter />
    </SidebarComponent>
  );
}
