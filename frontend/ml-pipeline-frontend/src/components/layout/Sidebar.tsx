import React from 'react';
import { NavLink } from 'react-router-dom';

const Sidebar: React.FC = () => (
  <aside className="w-64 bg-gray-100 p-4 border-r border-gray-200 dark:bg-gray-800 dark:border-gray-700 shrink-0">
    <h2 className="text-lg font-semibold mb-4">ML Pipeline</h2>
    <nav>
      <ul className="space-y-2">
        <li>
          <NavLink
            to="/pipelines"
            className={({ isActive }) =>
              `block px-3 py-2 rounded hover:bg-gray-200 dark:hover:bg-gray-700 ${isActive ? 'bg-gray-300 dark:bg-gray-600 font-semibold' : ''}`
            }
          >
            Dashboard
          </NavLink>
        </li>
        <li>
          <NavLink
            to="/settings"
            className={({ isActive }) =>
              `block px-3 py-2 rounded hover:bg-gray-200 dark:hover:bg-gray-700 ${isActive ? 'bg-gray-300 dark:bg-gray-600 font-semibold' : ''}`
            }
          >
            Settings
          </NavLink>
        </li>
        {/* TODO: Remove temporary login link when auth flow is implemented */}
        <li className="pt-4">
           <NavLink
            to="/login"
            className="block px-3 py-2 rounded text-sm text-gray-500 hover:bg-gray-200 dark:hover:bg-gray-700"
          >
            (Temp Login Link)
          </NavLink>
        </li>
      </ul>
    </nav>
  </aside>
);

export default Sidebar;