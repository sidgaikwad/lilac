import React from 'react';
import { NavLink, Outlet } from 'react-router-dom';

// Placeholder Setting Pages
const AccountSettingsPage = () => <div>Account Settings Form Placeholder</div>;
const OrganizationSettingsPage = () => <div>Organization Settings Placeholder (Members, etc.)</div>;

const SettingsPage: React.FC = () => {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-6">Settings</h1>
      {/* Use theme border color */}
      <div className="flex space-x-4 mb-6 border-b border-border pb-2">
        <NavLink
          to="account"
          className={({ isActive }) =>
            // Use theme colors for active/inactive/hover states
            `pb-2 border-b-2 ${isActive ? 'border-primary text-primary' : 'border-transparent text-muted-foreground hover:text-foreground hover:border-gray-300 dark:hover:border-gray-700'}`
          }
        >
          Account
        </NavLink>
        <NavLink
          to="organization"
          className={({ isActive }) =>
            // Use theme colors for active/inactive/hover states
            `pb-2 border-b-2 ${isActive ? 'border-primary text-primary' : 'border-transparent text-muted-foreground hover:text-foreground hover:border-gray-300 dark:hover:border-gray-700'}`
          }
        >
          Organization
        </NavLink>
      </div>

      <Outlet />
    </div>
  );
};

export { AccountSettingsPage, OrganizationSettingsPage };
export default SettingsPage;