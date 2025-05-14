import React, { useState } from 'react';
import { NavLink, useParams } from 'react-router-dom';
import { Button } from '@/components/ui/button';
import useAuthStore from '@/store/useAuthStore';
import useOrganizationStore from '@/store/useOrganizationStore'; 
import {
  LogOutIcon,
  Settings,
  Home,
  HardDrive,
  Zap,
  LucideProps, 
} from 'lucide-react';
import { cn } from '@/lib/utils';


type IconComponent = React.ComponentType<LucideProps>;

const Sidebar: React.FC = () => {
  const logout = useAuthStore((state) => state.logout);
  const { projectId } = useParams<{ projectId?: string }>(); 
  const { selectedOrganizationId: globalSelectedOrgId } = useOrganizationStore(); 
  const [isExpanded, setIsExpanded] = useState(false);

  const handleLogout = () => {
    logout();
  };

  const handleMouseEnter = () => setIsExpanded(true);
  const handleMouseLeave = () => setIsExpanded(false);

  const getNavLinkClass = (
    { isActive }: { isActive: boolean },
    isDisabled: boolean = false
  ) =>
    cn(
      'flex flex-nowrap items-center gap-2 rounded-md text-left outline-none transition-colors focus-visible:ring-2 focus-visible:ring-ring',
      'h-8 text-sm font-medium',
      isDisabled
        ? 'cursor-not-allowed opacity-50 pointer-events-none'
        : 'hover:bg-accent hover:text-accent-foreground',
      isActive && !isDisabled
        ? 'bg-accent text-accent-foreground font-semibold'
        : 'text-muted-foreground',
      isExpanded
        ? 'w-full justify-start py-2 px-1.5'
        : 'size-8 justify-center pl-1.5 pr-2'
    );

  
  const navLinkContent = (Icon: IconComponent, label: string) => (
    <>
      <Icon className="size-5 shrink-0" aria-hidden="true" />
      <span className={cn('whitespace-nowrap', !isExpanded && 'hidden')}>
        {label}
      </span>
    </>
  );

  return (
    <aside
      className={cn(
        'bg-card border-r border-border flex flex-col transition-[width] duration-100 ease-linear shrink-0',
        isExpanded ? 'w-56' : 'w-16'
      )}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
    >
      {}
      <nav className="flex-grow flex flex-col gap-0.5 p-2 overflow-auto">
        <NavLink
          to={globalSelectedOrgId ? `/organizations/${globalSelectedOrgId}/projects` : '/'}
          className={({ isActive }) => getNavLinkClass({ isActive }, false) 
          }
          
        >
          {navLinkContent(Home, 'Home')}
        </NavLink>
        <NavLink
          to={projectId ? `/projects/${projectId}/pipelines` : '#'}
          className={({ isActive }) =>
            getNavLinkClass({ isActive }, !projectId) 
          }
          aria-disabled={!projectId}
          onClick={(e) => !projectId && e.preventDefault()}
          tabIndex={!projectId ? -1 : undefined}
        >
          {navLinkContent(Zap, 'Pipelines')}
        </NavLink>
        <NavLink
          to={projectId ? `/projects/${projectId}/datasets` : '#'}
          className={({ isActive }) =>
            getNavLinkClass({ isActive }, !projectId)
          }
          aria-disabled={!projectId}
          onClick={(e) => !projectId && e.preventDefault()}
          tabIndex={!projectId ? -1 : undefined}
        >
          {navLinkContent(HardDrive, 'Data Sets')}
        </NavLink>
      </nav>

      {}
      <div className="mt-auto flex flex-col gap-0.5 p-2 border-t border-border">
        <NavLink
          to="/settings"
          className={({ isActive }) => getNavLinkClass({ isActive }, false)}
        >
          {navLinkContent(Settings, 'Settings')}
        </NavLink>
        <Button
          variant="ghost"
          onClick={handleLogout}
          className={getNavLinkClass({ isActive: false }, false)}
          aria-label="Logout"
        >
          {navLinkContent(LogOutIcon, 'Logout')}
        </Button>
      </div>
    </aside>
  );
};

export default Sidebar;
