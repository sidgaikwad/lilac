// src/components/TopBar.tsx
import React from 'react';
import { Button } from '@/components/ui/button';
import { Menu } from 'lucide-react';

const TopBar: React.FC = () => {
  return (
    <div className="flex h-14 items-center border-b bg-background px-4 shrink-0">
      {/* Placeholder for Logo/Nav */}
      <Button variant="ghost" size="icon" className="mr-4">
        <Menu className="h-5 w-5" />
        <span className="sr-only">Toggle Menu</span>
      </Button>
      <h1 className="text-lg font-semibold">Pipeline Editor</h1>
      {/* Add other top bar elements like user profile, save button etc. here */}
      <div className="ml-auto"> {/* Pushes subsequent items to the right */}
        {/* Example: <Button>Save Pipeline</Button> */}
      </div>
    </div>
  );
};

export default TopBar;