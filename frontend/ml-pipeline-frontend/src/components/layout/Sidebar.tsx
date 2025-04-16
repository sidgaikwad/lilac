import React from 'react';
import { NavLink } from 'react-router-dom';
import { ThemeToggle } from '@/components/common/ThemeToggle';

const Sidebar: React.FC = () => (
  // Use theme variables for sidebar background and border
  <aside className="w-64 bg-card p-4 border-r border-border shrink-0 flex flex-col">
    <h2 className="text-lg font-semibold mb-4">ML Pipeline</h2>
    <nav className="flex-grow">
      <ul className="space-y-2">
        <li>
          <NavLink
            to="/pipelines"
            className={({ isActive }) =>
              // Use theme variables for hover and active states
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
              // Use theme variables for hover and active states
              `block px-3 py-2 rounded hover:bg-accent hover:text-accent-foreground ${isActive ? 'bg-accent text-accent-foreground font-semibold' : 'text-foreground/80'}`
            }
          >
            Settings
          </NavLink>
        </li>
        {/* TODO: Remove temporary login link when auth flow is implemented */}
        <li className="pt-4">
           <NavLink
            to="/login"
            // Use theme variables for hover state
            className="block px-3 py-2 rounded text-sm text-muted-foreground hover:bg-accent hover:text-accent-foreground"
          >
            (Temp Login Link)
          </NavLink>
        </li>
      </ul>
    </nav>
    {/* Use theme border color */}
    <div className="mt-auto pt-4 border-t border-border">
        <ThemeToggle />
    </div>
  </aside>
);

export default Sidebar;