import React from 'react';
import { NavLink } from 'react-router-dom';
import { ThemeToggle } from '@/components/common/ThemeToggle';
import { Button } from '@/components/ui/button'; // Import Button
import useAuthStore from '@/store/authStore'; // Import auth store
import { LogOutIcon } from 'lucide-react'; // Import icon

const Sidebar: React.FC = () => {
  const logout = useAuthStore((state) => state.logout); // Get logout action

  const handleLogout = () => {
    logout();
    // No need to navigate here, ProtectedRoute will redirect to /login
  };

  return (
    <aside className="w-64 bg-card p-4 border-r border-border shrink-0 flex flex-col">
      <h2 className="text-lg font-semibold mb-4">ML Pipeline</h2>
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
          {/* Remove temporary login link */}
          {/*
          <li className="pt-4">
             <NavLink
              to="/login"
              className="block px-3 py-2 rounded text-sm text-muted-foreground hover:bg-accent hover:text-accent-foreground"
            >
              (Temp Login Link)
            </NavLink>
          </li>
          */}
        </ul>
      </nav>
      <div className="mt-auto pt-4 border-t border-border space-y-2">
          <Button variant="ghost" onClick={handleLogout} className="w-full justify-start text-muted-foreground hover:text-foreground">
            <LogOutIcon className="mr-2 h-4 w-4" />
            Logout
          </Button>
          <ThemeToggle />
      </div>
    </aside>
  );
};

export default Sidebar;