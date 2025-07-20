import { useLocation } from 'react-router-dom';
import { Settings, HardDrive, Shapes, Server } from 'lucide-react'; // Added LayoutDashboard
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
import { Link } from 'react-router-dom';
import { useMemo } from 'react';
import { Routes } from '@/constants';

export default function Sidebar() {
  const location = useLocation();
  const { setOpen, isMobile } = useSidebar();

  const paths = useMemo(() => {
    return {
      [Routes.PROJECTS]: Routes.PROJECTS,
      [Routes.DATA_SOURCES]: Routes.DATA_SOURCES,
      [Routes.CLUSTERS]: Routes.CLUSTERS,
      [Routes.ORG_SETTINGS]: Routes.ORG_SETTINGS,
    };
  }, []);

  return (
    <SidebarComponent
      variant='sidebar'
      collapsible='icon'
      onMouseEnter={!isMobile ? () => setOpen(true) : undefined}
      onMouseLeave={!isMobile ? () => setOpen(false) : undefined}
      className='top-(--header-height) !h-[calc(100svh-var(--header-height))]'
    >
      <SidebarHeader></SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  asChild
                  isActive={location.pathname === paths[Routes.PROJECTS]}
                >
                  <Link to={paths[Routes.PROJECTS]}>
                    <Shapes />
                    <span>Projects</span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton
                  asChild
                  isActive={location.pathname === paths[Routes.DATA_SOURCES]}
                >
                  <Link to={paths[Routes.DATA_SOURCES]}>
                    <HardDrive />
                    <span>Data Sources</span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton
                  asChild
                  isActive={location.pathname === paths[Routes.CLUSTERS]}
                >
                  <Link to={paths[Routes.CLUSTERS]}>
                    <Server />
                    <span>Clusters</span>
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
                  isActive={location.pathname === paths[Routes.ORG_SETTINGS]}
                >
                  <Link to={paths[Routes.ORG_SETTINGS]}>
                    <Settings />
                    <span>Organization Settings</span>
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
