import { generatePath, useLocation, useParams } from 'react-router-dom';
import { Settings, Home, PanelsTopLeft } from 'lucide-react';
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
  const { organizationId } = useParams<{ organizationId: string }>();

  const location = useLocation();
  const { setOpen } = useSidebar();

  const paths = useMemo(() => {
    return {
      [Routes.ORGANIZATIONS]: Routes.ORGANIZATIONS,
      [Routes.ORGANIZATION_PROJECTS]: generatePath(
        Routes.ORGANIZATION_PROJECTS,
        {
          organizationId: organizationId!,
        }
      ),
      [Routes.ORGANIZATION_SETTINGS]: generatePath(
        Routes.ORGANIZATION_SETTINGS,
        {
          organizationId: organizationId!,
        }
      ),
    };
  }, [organizationId]);

  return (
    <SidebarComponent
      variant='sidebar'
      collapsible='icon'
      onMouseEnter={() => setOpen(true)}
      onMouseLeave={() => setOpen(false)}
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
                  isActive={
                    location.pathname === paths[Routes.ORGANIZATION_PROJECTS]
                  }
                >
                  <Link to={paths[Routes.ORGANIZATION_PROJECTS]}>
                    <PanelsTopLeft />
                    <span>Projects</span>
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
                  isActive={
                    location.pathname === paths[Routes.ORGANIZATION_SETTINGS]
                  }
                >
                  <Link to={paths[Routes.ORGANIZATION_SETTINGS]}>
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
