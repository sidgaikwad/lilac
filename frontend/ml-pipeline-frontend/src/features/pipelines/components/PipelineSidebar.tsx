import React from 'react';
import { StepDefinition } from '@/types';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';

interface PipelineSidebarProps {
  availableSteps: StepDefinition[];
}

/**
 * Groups StepDefinition objects by their category.
 */
const groupStepsByCategory = (steps: StepDefinition[]): Record<string, StepDefinition[]> => {
  return steps.reduce((acc, step) => {
    const category = step.category || 'Uncategorized';
    if (!acc[category]) {
      acc[category] = [];
    }
    acc[category].push(step);
    return acc;
  }, {} as Record<string, StepDefinition[]>);
};

/**
 * Sidebar component displaying available pipeline steps (pipes) grouped by category.
 * Allows dragging steps onto the editor canvas.
 */
const PipelineSidebar: React.FC<PipelineSidebarProps> = ({ availableSteps }) => {

  const onDragStart = (event: React.DragEvent, nodeType: string, stepDefinitionId: string, label: string) => {
    // Data needed by PipelineEditorFlow's onDrop handler
    event.dataTransfer.setData('application/reactflow-stepdef-id', stepDefinitionId);
    event.dataTransfer.setData('application/reactflow-label', label);
    event.dataTransfer.setData('application/reactflow-type', nodeType);
    event.dataTransfer.effectAllowed = 'move';
  };

  const groupedSteps = groupStepsByCategory(availableSteps);

  return (
    <aside className="w-64 bg-gray-50 dark:bg-gray-800 border-l border-gray-200 dark:border-gray-700 flex flex-col shrink-0 h-full">
      <CardHeader className="shrink-0">
        <CardTitle>Available Pipes</CardTitle>
      </CardHeader>
      <Separator className="shrink-0" />
      <CardContent className="p-0 overflow-y-auto flex-grow">
        {Object.entries(groupedSteps).map(([category, steps]) => (
          <div key={category} className="p-4">
            <h3 className="text-sm font-semibold mb-2 text-gray-600 dark:text-gray-400">{category}</h3>
            <div className="space-y-2">
              {steps.map((step) => (
                <Button
                  key={step.id}
                  variant="outline"
                  className="w-full justify-start cursor-grab text-left h-auto py-2"
                  draggable
                  onDragStart={(event) => onDragStart(event, step.type, step.id, step.label)}
                  title={step.description}
                >
                  {/* TODO: Add icon based on category/type */}
                  {step.label}
                </Button>
              ))}
            </div>
            {/* Add separator unless it's the last category? Could be complex. */}
            <Separator className="mt-4" />
          </div>
        ))}
      </CardContent>
    </aside>
  );
};

export default PipelineSidebar;