import React, { memo } from 'react';
import { Handle, Position, NodeProps } from 'reactflow';
import { Card, CardHeader, CardTitle } from '@/components/ui/card';
import { cn } from '@/lib/utils';
import { StepDefinition } from '@/types'; // Import StepDefinition type

// Define the expected structure of the data prop for this node type
interface PipelineNodeData {
  label: string;
  stepType: string;
  parameters: Record<string, any>;
  stepDefinition: StepDefinition; // Used by the parameter dialog
}

/**
 * Custom React Flow node component for displaying pipeline steps.
 * Includes input (left) and output (right) handles.
 */
const PipelineNode: React.FC<NodeProps<PipelineNodeData>> = ({ data, selected }) => {
  return (
    <Card className={cn(
      "w-48 border-2 shadow-md rounded-lg bg-white dark:bg-gray-800",
      // Apply distinct styling when the node is selected
      selected ? "border-blue-500 ring-2 ring-blue-300 dark:border-blue-400 dark:ring-blue-600" : "border-gray-300 dark:border-gray-600"
    )}>
      <Handle
        type="target" // Input handle
        position={Position.Left}
        id="input" // Unique ID for the handle
        className="!bg-blue-500 w-3 h-3 border-2 border-white dark:border-gray-800"
        isConnectable={true} // TODO: Add connection validation logic
      />

      <CardHeader className="p-3">
        <CardTitle className="text-sm font-medium text-center truncate" title={data.label}>
          {/* TODO: Add Icon based on data.stepDefinition.category or data.stepType */}
          {data.label || 'Pipeline Step'}
        </CardTitle>
        {/* Optionally display stepType or other info here */}
        {/* <p className="text-xs text-muted-foreground text-center">{data.stepType}</p> */}
      </CardHeader>

      <Handle
        type="source" // Output handle
        position={Position.Right}
        id="output" // Unique ID for the handle
        className="!bg-green-500 w-3 h-3 border-2 border-white dark:border-gray-800"
        isConnectable={true} // TODO: Add connection validation logic
      />
    </Card>
  );
};

// Memoize the component for performance optimization, preventing unnecessary re-renders
export default memo(PipelineNode);