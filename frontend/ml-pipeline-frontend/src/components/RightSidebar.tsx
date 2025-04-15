// src/components/RightSidebar.tsx
import React from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { Button } from '@/components/ui/button';
import { availablePipeTypes } from '@/config/pipelineEditorConfig'; // Import config

interface SidebarProps {
  // Potentially add props later if needed, e.g., for org pipes
}

const RightSidebar: React.FC<SidebarProps> = (/* props */) => {
  // Handler for starting the drag operation from the sidebar item
  const onDragStart = (event: React.DragEvent, nodeType: string, nodeLabel: string) => {
    // Store data about the node being dragged using the dataTransfer object
    event.dataTransfer.setData('application/reactflow-type', nodeType);
    event.dataTransfer.setData('application/reactflow-label', nodeLabel);
    event.dataTransfer.effectAllowed = 'move'; // Indicate the type of operation allowed
  };

  return (
    <Card className="h-full w-64 rounded-none border-l border-r-0 border-t-0 border-b-0 flex flex-col shrink-0">
      {/* Section 1: Available Pipes */}
      <CardHeader className="shrink-0">
        <CardTitle>Available Pipes</CardTitle>
      </CardHeader>
      <Separator className="shrink-0"/>
      <CardContent className="p-4 space-y-2 overflow-y-auto flex-grow">
        {/* Map over the pipe types defined in the config file */}
        {availablePipeTypes.map((pipe) => (
          <Button
            key={pipe.label}
            variant="outline"
            className="w-full justify-start cursor-grab"
            draggable // Enable dragging for this button
            onDragStart={(event) => onDragStart(event, pipe.type, pipe.label)}
          >
            {pipe.label} ({pipe.type})
          </Button>
        ))}
      </CardContent>

      {/* Placeholder for Section 2: My Org Pipes (Functionality to be added later) */}
      {/*
      <Separator className="shrink-0"/>
      <CardHeader className="shrink-0">
        <CardTitle>My Org Pipes</CardTitle>
      </CardHeader>
      <CardContent className="p-4 space-y-2 overflow-y-auto flex-grow">
         {/* List organization-specific or saved pipes here */}
      {/* </CardContent>
      */}
    </Card>
  );
};

export default RightSidebar;
