import React from 'react';
import { NavLink } from 'react-router-dom';
import { ThemeToggle } from '@/components/common/ThemeToggle';
import { Button } from '@/components/ui/button';
import useAuthStore from '@/store/authStore';
import { LogOutIcon } from 'lucide-react';

const Sidebar: React.FC = () => {
  const logout = useAuthStore((state) => state.logout);

  const handleLogout = () => {
    logout();
  };

  return (
    <aside className="w-64 bg-card p-4 border-r border-border shrink-0 flex flex-col">
      {/* Container for Title and Theme Toggle */}
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-lg font-semibold">Mario Bros</h2> {/* Updated Title */}
        <ThemeToggle /> {/* Moved Toggle Here */}
      </div>

      <nav className="flex-grow">
        <ul className="space-y-2">
          <li>
            <NavLink
              to="/pipelines"
              className={({ isActive }) =>
                `block px-3 py-2 rounded hover:bg-accent hover:text-accent-foreground ${isActive ? 'bg-accent text-accent-foreground font-semibold' : 'text-foreground/80'}`
              }
            >
              Dashboard
            </NavLink>
          </li>
          <li>
            <NavLink
              to="/settings"
              className={({ isActive }) =>
                `block px-3 py-2 rounded hover:bg-accent hover:text-accent-foreground ${isActive ? 'bg-accent text-accent-foreground font-semibold' : 'text-foreground/80'}`
              }
            >
              Settings
            </NavLink>
          </li>
        </ul>
      </nav>
      {/* Add Logout Button at the bottom */}
      <div className="mt-auto pt-4 border-t border-border">
          <Button variant="ghost" onClick={handleLogout} className="w-full justify-start text-muted-foreground hover:text-foreground">
            <LogOutIcon className="mr-2 h-4 w-4" />
            Logout
          </Button>
      </div>
      {/* Removed the div that previously held only the ThemeToggle */}
    </aside>
  );
};

export default Sidebar;