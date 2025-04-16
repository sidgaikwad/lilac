import React, { memo } from 'react';
import { Handle, Position, NodeProps } from 'reactflow';
import { Card, CardHeader, CardTitle } from '@/components/ui/card';
import { cn } from '@/lib/utils';
import { StepDefinition } from '@/types';

interface PipelineNodeData {
  label: string;
  stepType: string;
  parameters: Record<string, any>;
  stepDefinition: StepDefinition;
}

/**
 * Custom React Flow node component for displaying pipeline steps.
 * Includes input (left) and output (right) handles, with restrictions
 * for Input/Output category nodes.
 */
const PipelineNode: React.FC<NodeProps<PipelineNodeData>> = ({ data, selected }) => {
  const isInputNode = data.stepDefinition?.category === 'Input';
  const isOutputNode = data.stepDefinition?.category === 'Output';

  return (
    <Card className={cn(
      "w-48 border-2 shadow-md rounded-lg bg-white dark:bg-gray-800",
      selected ? "border-blue-500 ring-2 ring-blue-300 dark:border-blue-400 dark:ring-blue-600" : "border-gray-300 dark:border-gray-600"
    )}>
      {/* Render Target Handle only if NOT an Input node */}
      {!isInputNode && (
        <Handle
          type="target"
          position={Position.Left}
          id="input"
          className="!bg-blue-500 w-3 h-3 border-2 border-white dark:border-gray-800"
          isConnectable={true}
        />
      )}

      <CardHeader className="p-3">
        <CardTitle className="text-sm font-medium text-center truncate" title={data.label}>
          {/* TODO: Add Icon */}
          {data.label || 'Pipeline Step'}
        </CardTitle>
      </CardHeader>

      {/* Render Source Handle only if NOT an Output node */}
      {!isOutputNode && (
        <Handle
          type="source"
          position={Position.Right}
          id="output"
          className="!bg-green-500 w-3 h-3 border-2 border-white dark:border-gray-800"
          isConnectable={true}
        />
      )}
    </Card>
  );
};

export default memo(PipelineNode);