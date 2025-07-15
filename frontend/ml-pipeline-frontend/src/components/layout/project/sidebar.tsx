import { generatePath, useLocation, useParams } from 'react-router-dom';
import {
  Settings,
  HardDrive,
  LayoutDashboard,
  Home,
  FlaskConical,
  BookText,
  Shapes,
} from 'lucide-react'; // Added LayoutDashboard
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
} from '@/components/ui/sidebar';
import { Routes } from '@/services/constants/routes';
import { Link } from 'react-router-dom';
import { useMemo } from 'react';
import ProjectSelectionDropdown from '@/components/common/project-selection-dropdown';

export default function Sidebar() {
  const { projectId } = useParams<{ projectId: string }>();

  const location = useLocation();

  const paths = useMemo(() => {
    return {
      [Routes.PROJECT_DETAILS]: generatePath(Routes.PROJECT_DETAILS, {
        projectId: projectId!,
      }),
      [Routes.PROJECT_DATASETS]: generatePath(Routes.PROJECT_DATASETS, {
        projectId: projectId!,
      }),
      [Routes.PROJECT_EXPERIMENTS]: generatePath(Routes.PROJECT_EXPERIMENTS, {
        projectId: projectId!,
      }),
      [Routes.PROJECT_SETTINGS]: generatePath(Routes.PROJECT_SETTINGS, {
        projectId: projectId!,
      }),
      [Routes.PROJECT_NOTEBOOKS]: generatePath(Routes.PROJECT_NOTEBOOKS, {
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
      variant='sidebar'
      collapsible='none'
      className='top-(--header-height) !h-[calc(100svh-var(--header-height))]'
    >
      <SidebarHeader></SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem className='bg-accent-secondary border-accent-border rounded-md border'>
                <SidebarMenuButton asChild>
                  <div>
                    <Shapes />
                    <ProjectSelectionDropdown />
                  </div>
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
                    <LayoutDashboard /> {/* Changed Icon */}
                    <span>Dashboard</span>
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
              <SidebarMenuItem>
                <SidebarMenuButton
                  asChild
                  isActive={
                    location.pathname === paths[Routes.PROJECT_EXPERIMENTS]
                  }
                >
                  <Link to={paths[Routes.PROJECT_EXPERIMENTS]}>
                    <FlaskConical />
                    <span>Experiments</span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton
                  asChild
                  isActive={
                    location.pathname === paths[Routes.PROJECT_NOTEBOOKS]
                  }
                >
                  <Link to={paths[Routes.PROJECT_NOTEBOOKS]}>
                    <BookText />
                    <span>Notebooks</span>
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
