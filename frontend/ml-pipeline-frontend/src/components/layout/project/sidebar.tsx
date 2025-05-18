import { generatePath, useLocation, useParams } from 'react-router-dom';
import { Settings, Home, PanelsTopLeft, HardDrive, LayoutDashboard } from 'lucide-react'; // Added LayoutDashboard
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
import { Routes } from '@/constants';
import { Link } from 'react-router-dom';
import { useMemo } from 'react';

export default function Sidebar() {
  const { projectId } = useParams<{ projectId: string }>();

  const location = useLocation();
  const { setOpen } = useSidebar();

  const paths = useMemo(() => {
    return {
      [Routes.PROJECT_DETAILS]: generatePath(Routes.PROJECT_DETAILS, {
        projectId: projectId!,
      }),
      [Routes.PROJECT_PIPELINES]: generatePath(Routes.PROJECT_PIPELINES, {
        projectId: projectId!,
      }),
      [Routes.PROJECT_DATASETS]: generatePath(Routes.PROJECT_DATASETS, {
        projectId: projectId!,
      }),
      [Routes.PROJECT_SETTINGS]: generatePath(Routes.PROJECT_SETTINGS, {
        projectId: projectId!,
      }),
    };
  }, [projectId]);

  if (projectId === undefined) {
    console.error('Expected projectId in URL params');
    return undefined;
  }

  return (
    <SidebarComponent
      variant="sidebar"
      collapsible="icon"
      onMouseEnter={() => setOpen(true)}
      onMouseLeave={() => setOpen(false)}
      className="top-(--header-height) !h-[calc(100svh-var(--header-height))]"
    >
      <SidebarHeader></SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  asChild
                  isActive={location.pathname === Routes.ORGANIZATIONS}
                >
                  <Link to={Routes.ORGANIZATIONS}>
                    <Home />
                    <span>Organizations</span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton
                  asChild
                  isActive={location.pathname === paths[Routes.PROJECT_DETAILS]}
                >
                  <Link to={paths[Routes.PROJECT_DETAILS]}>
                    <LayoutDashboard /> {/* Changed Icon */}
                    <span>Dashboard</span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton
                  asChild
                  isActive={
                    location.pathname === paths[Routes.PROJECT_PIPELINES]
                  }
                >
                  <Link to={paths[Routes.PROJECT_PIPELINES]}>
                    <PanelsTopLeft />
                    <span>Pipelines</span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton
                  asChild
                  isActive={
                    location.pathname === paths[Routes.PROJECT_DATASETS]
                  }
                >
                  <Link to={paths[Routes.PROJECT_DATASETS]}>
                    <HardDrive />
                    <span>Datasets</span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarSeparator className="mx-0" />

        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  asChild
                  isActive={
                    location.pathname === paths[Routes.PROJECT_SETTINGS]
                  }
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

      <SidebarFooter></SidebarFooter>
    </SidebarComponent>
  );
}
