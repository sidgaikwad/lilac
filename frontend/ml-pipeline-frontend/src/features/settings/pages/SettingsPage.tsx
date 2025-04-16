import React from 'react';
import { NavLink, Outlet } from 'react-router-dom'; // Use NavLink for active styling

// Placeholder Setting Pages (defined here for simplicity, could be separate files)
const AccountSettingsPage = () => <div>Account Settings Form Placeholder</div>;
const OrganizationSettingsPage = () => <div>Organization Settings Placeholder (Members, etc.)</div>;

const SettingsPage: React.FC = () => {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-6">Settings</h1>
      <div className="flex space-x-4 mb-6 border-b pb-2">
        <NavLink
          to="account"
          className={({ isActive }) =>
            `pb-2 border-b-2 ${isActive ? 'border-blue-500 text-blue-600' : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}`
          }
        >
          Account
        </NavLink>
        <NavLink
          to="organization"
          className={({ isActive }) =>
            `pb-2 border-b-2 ${isActive ? 'border-blue-500 text-blue-600' : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}`
          }
        >
          Organization
        </NavLink>
      </div>

      {/* Render matched nested route (Account or Organization) */}
      <Outlet />
    </div>
  );
};

// Export sub-pages for routing in App.tsx
export { AccountSettingsPage, OrganizationSettingsPage };
export default SettingsPage;