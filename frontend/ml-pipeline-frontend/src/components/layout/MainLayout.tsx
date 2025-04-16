import React from 'react';
import { Outlet } from 'react-router-dom';
import Sidebar from './Sidebar';

const MainLayout: React.FC = () => {
  return (
    // Use theme variables for background and text
    <div className="flex h-screen bg-background text-foreground">
      <Sidebar />
      <main className="flex-1 flex flex-col overflow-hidden">
        {/* Optional Top Bar Placeholder - If added, style with theme colors */}
        {/* <header className="h-16 bg-card border-b border-border">Top Bar</header> */}
        <div className="flex-1 overflow-y-auto p-6">
          <Outlet />
        </div>
      </main>
    </div>
  );
};

export default MainLayout;