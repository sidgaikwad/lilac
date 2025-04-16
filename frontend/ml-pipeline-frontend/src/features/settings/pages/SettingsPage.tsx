import React from 'react';
import { NavLink, Outlet } from 'react-router-dom';
// Sub-pages are now imported in App.tsx for routing

const SettingsPage: React.FC = () => {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-6">Settings</h1>
      <div className="flex space-x-4 mb-6 border-b border-border pb-2">
        <NavLink
          to="account"
          className={({ isActive }) =>
            `pb-2 border-b-2 ${isActive ? 'border-primary text-primary' : 'border-transparent text-muted-foreground hover:text-foreground hover:border-gray-300 dark:hover:border-gray-700'}`
          }
        >
          Account
        </NavLink>
        <NavLink
          to="organization"
          className={({ isActive }) =>
            `pb-2 border-b-2 ${isActive ? 'border-primary text-primary' : 'border-transparent text-muted-foreground hover:text-foreground hover:border-gray-300 dark:hover:border-gray-700'}`
          }
        >
          Organization
        </NavLink>
      </div>

      {/* Outlet renders the matched sub-route (Account or Organization) */}
      <Outlet />
    </div>
  );
};

// No longer need to export sub-pages from here
export default SettingsPage;