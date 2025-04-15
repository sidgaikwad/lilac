// src/components/CustomNode.tsx
import React, { memo } from 'react';
import { Handle, Position, NodeProps } from 'reactflow';
import { Card, CardHeader, CardTitle } from '@/components/ui/card'; // Use Shadcn Card for styling
import { cn } from '@/lib/utils'; // Import cn for conditional classes

// Using memo for performance optimization
const CustomNode: React.FC<NodeProps> = ({ data, isConnectable, selected }) => {
  return (
    // Card styling includes min-width and highlights border on selection
    <Card className={cn(
        "shadow-md min-w-36 cursor-pointer", // Make it clear it's clickable
        selected ? "border-primary ring-2 ring-primary/30" : "border-border" // Enhanced selection feedback
     )}>
      {/* Input Handle (Left) */}
      <Handle
        type="target"
        position={Position.Left}
        id="left-target"
        isConnectable={isConnectable} // isConnectable={1} could also work here for simple cases
        className="!bg-teal-500 w-2 h-2"
      />

      {/* Node Content */}
      <CardHeader className="px-3 py-2">
        <CardTitle className="text-sm font-medium text-center break-words">
            {/* Display node label */}
            {data.label}
        </CardTitle>
      </CardHeader>

      {/* Output Handle (Right) */}
      <Handle
        type="source"
        position={Position.Right}
        id="right-source"
        isConnectable={isConnectable} // isConnectable={1} could also work here
        className="!bg-rose-500 w-2 h-2"
      />
    </Card>
  );
};

export default memo(CustomNode);
