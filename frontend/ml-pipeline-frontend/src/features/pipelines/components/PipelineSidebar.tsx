import React from 'react';
import { StepDefinition } from '@/types';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { cn } from '@/lib/utils'; // Import cn

interface PipelineSidebarProps {
  availableSteps: StepDefinition[];
}

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

// Consistent focus style for buttons
const buttonFocusStyle = "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 dark:focus-visible:ring-offset-gray-950";


const PipelineSidebar: React.FC<PipelineSidebarProps> = ({ availableSteps }) => {

  const onDragStart = (event: React.DragEvent, nodeType: string, stepDefinitionId: string, label: string) => {
    event.dataTransfer.setData('application/reactflow-stepdef-id', stepDefinitionId);
    event.dataTransfer.setData('application/reactflow-label', label);
    event.dataTransfer.setData('application/reactflow-type', nodeType);
    event.dataTransfer.effectAllowed = 'move';
  };

  const groupedSteps = groupStepsByCategory(availableSteps);

  return (
    // Use theme variables for background and border
    <aside className="w-64 bg-card border-l border-border flex flex-col shrink-0 h-full">
      <CardHeader className="shrink-0">
        <CardTitle>Available Pipes</CardTitle>
      </CardHeader>
      <Separator className="shrink-0" /> {/* Separator uses border color */}
      <CardContent className="p-0 overflow-y-auto flex-grow">
        {Object.entries(groupedSteps).map(([category, steps]) => (
          <div key={category} className="p-4">
            {/* Use theme muted foreground color */}
            <h3 className="text-sm font-semibold mb-2 text-muted-foreground">{category}</h3>
            <div className="space-y-2">
              {steps.map((step) => (
                // Button uses theme variables via variants, add focus style
                <Button
                  key={step.id}
                  variant="outline"
                  className={cn("w-full justify-start cursor-grab text-left h-auto py-2", buttonFocusStyle)}
                  draggable
                  onDragStart={(event) => onDragStart(event, step.type, step.id, step.label)}
                  title={step.description}
                >
                  {/* TODO: Add icon based on category/type */}
                  {step.label}
                </Button>
              ))}
            </div>
            <Separator className="mt-4" />
          </div>
        ))}
      </CardContent>
    </aside>
  );
};

export default PipelineSidebar;