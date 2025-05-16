import { useLocation, useParams } from 'react-router-dom';
import useOrganizationStore from '@/store/useOrganizationStore';
import {
  Settings,
  Home,
  HardDrive,
  Wrench,
  LogOut,
  PanelsTopLeft,
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
import { Routes } from '@/constants';

export default function Sidebar() {
  const organizationId = useOrganizationStore(
    (state) => state.selectedOrganizationId
  );
  const { projectId } = useParams<{ projectId?: string }>();
  const location = useLocation();

  const { setOpen } = useSidebar();

  return (
    <SidebarComponent
      variant="sidebar"
      collapsible="icon"
      onMouseEnter={() => setOpen(true)}
      onMouseLeave={() => setOpen(false)}
      className="top-(--header-height) !h-[calc(100svh-var(--header-height))]"
    >
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton
              asChild
              isActive={location.pathname === Routes.HOME}
            >
              <a href={Routes.HOME}>
                <Home />
                <span>Home</span>
              </a>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton
              asChild
              isActive={
                location.pathname ===
                `/organizations/${organizationId}/projects`
              }
            >
              <a
                href={
                  organizationId
                    ? `/organizations/${organizationId}/projects`
                    : '/'
                }
              >
                <PanelsTopLeft />
                <span>Projects</span>
              </a>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarSeparator className="mx-0" />
      <SidebarContent className="">
        <SidebarGroup>
          <SidebarGroupContent>
            {projectId ? (
              <>
                <SidebarMenu>
                  <SidebarMenuItem>
                    <SidebarMenuButton
                      asChild
                      isActive={
                        location.pathname === `/projects/${projectId}/pipelines`
                      }
                    >
                      <a href={`/projects/${projectId}/pipelines`}>
                        <Wrench />
                        <span>Pipelines</span>
                      </a>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                </SidebarMenu>
                <SidebarMenu>
                  <SidebarMenuItem>
                    <SidebarMenuButton
                      asChild
                      isActive={
                        location.pathname === `/projects/${projectId}/datasets`
                      }
                    >
                      <a href={`/projects/${projectId}/datasets`}>
                        <HardDrive />
                        <span>Datasets</span>
                      </a>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                </SidebarMenu>
              </>
            ) : undefined}
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarSeparator className="mx-0" />
      <SidebarFooter>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton asChild>
              <a href={`/settings`}>
                <Settings />
                <span>Settings</span>
              </a>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton asChild>
              <a href={`/logout`}>
                <LogOut />
                <span>Logout</span>
              </a>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarFooter>
    </SidebarComponent>
  );
}
