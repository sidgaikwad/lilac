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

const PipelineNode: React.FC<NodeProps<PipelineNodeData>> = ({ data, selected }) => {
  const isInputNode = data.stepDefinition?.category === 'Input';
  const isOutputNode = data.stepDefinition?.category === 'Output';

  return (
    // Use theme variables for card background and border
    // Use primary color for selection border/ring
    <Card className={cn(
      "w-48 border-2 shadow-md rounded-lg bg-card", // Use bg-card
      selected
        ? "border-primary ring-2 ring-primary/50" // Use primary for selection
        : "border-border" // Use theme border
    )}>
      {!isInputNode && (
        <Handle
          type="target"
          position={Position.Left}
          id="input"
          // Use theme colors for handle, maybe accent or primary? Let's try accent
          className="!bg-accent w-3 h-3 border-2 border-card" // Border matches card bg
          isConnectable={true}
        />
      )}

      <CardHeader className="p-3">
        <CardTitle className="text-sm font-medium text-center truncate" title={data.label}>
          {data.label || 'Pipeline Step'}
        </CardTitle>
      </CardHeader>

      {!isOutputNode && (
        <Handle
          type="source"
          position={Position.Right}
          id="output"
           // Use theme colors for handle, maybe secondary or primary? Let's try secondary
          className="!bg-secondary w-3 h-3 border-2 border-card" // Border matches card bg
          isConnectable={true}
        />
      )}
    </Card>
  );
};

export default memo(PipelineNode);