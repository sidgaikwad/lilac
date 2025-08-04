import { useLocation } from 'react-router-dom';
import { Server, KeyRound, Layers, Briefcase } from 'lucide-react';
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
      [Routes.CLUSTERS]: Routes.CLUSTERS,
      [Routes.QUEUES]: Routes.QUEUES,
      [Routes.JOBS]: Routes.JOBS,
      [Routes.API_KEYS]: Routes.API_KEYS,
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
                  isActive={location.pathname === paths[Routes.CLUSTERS]}
                >
                  <Link to={paths[Routes.CLUSTERS]}>
                    <Server />
                    <span>Clusters</span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton
                  asChild
                  isActive={location.pathname === paths[Routes.JOBS]}
                >
                  <Link to={paths[Routes.JOBS]}>
                    <Briefcase />
                    <span>Jobs</span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton
                  asChild
                  isActive={location.pathname === paths[Routes.QUEUES]}
                >
                  <Link to={paths[Routes.QUEUES]}>
                    <Layers />
                    <span>Queues</span>
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
                  isActive={location.pathname === paths[Routes.API_KEYS]}
                >
                  <Link to={paths[Routes.API_KEYS]}>
                    <KeyRound />
                    <span>API Keys</span>
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
