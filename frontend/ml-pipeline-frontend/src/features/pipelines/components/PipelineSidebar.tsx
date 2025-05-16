import React from 'react';
import { Button } from '@/components/ui/button';
import { CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { cn } from '@/lib/utils';
import { Loader2Icon, AlertTriangleIcon } from 'lucide-react'; // Icons for loading/error
import { useListStepDefinitions } from '@/services';
import { StepDefinition } from '@/types';

// interface PipelineSidebarProps {}

const groupStepsByCategory = (
  stepDefinitions: StepDefinition[] = []
): Record<string, StepDefinition[]> => {
  return stepDefinitions.reduce(
    (acc, step) => {
      if (!acc[step.category]) acc[step.category] = [];
      acc[step.category].push(step);
      return acc;
    },
    {} as Record<string, StepDefinition[]>
  );
};

const buttonFocusStyle =
  'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 dark:focus-visible:ring-offset-gray-950';

const PipelineSidebar: React.FC = () => {
  // Fetch step definitions using React Query
  const { data: stepDefinitions, isLoading, error } = useListStepDefinitions();

  const onDragStart = (
    event: React.DragEvent,
    nodeType: string,
    stepDefinitionId: string,
    label: string
  ) => {
    event.dataTransfer.setData(
      'application/reactflow-stepdef-id',
      stepDefinitionId
    );
    event.dataTransfer.setData('application/reactflow-label', label);
    event.dataTransfer.setData('application/reactflow-type', nodeType);
    event.dataTransfer.effectAllowed = 'move';
  };

  const groupedSteps = groupStepsByCategory(
    stepDefinitions ?? []
  );

  return (
    <aside className="w-64 bg-card border-l border-border flex flex-col shrink-0 h-full space-y-2">
      <CardHeader className="shrink-0 pt-4">
        <CardTitle>Available Pipes</CardTitle>
      </CardHeader>
      <Separator className="shrink-0" />
      <CardContent className="p-0 overflow-y-auto flex-grow">
        {isLoading ? (
          <div className="p-4 text-center text-muted-foreground flex items-center justify-center h-full">
            <Loader2Icon className="mr-2 h-4 w-4 animate-spin" /> Loading...
          </div>
        ) : error ? (
          <div className="p-4 text-destructive flex flex-col items-center justify-center h-full text-center">
            <AlertTriangleIcon className="h-6 w-6 mb-2" />
            <span className="text-sm">Error loading pipes: {error.error}</span>
          </div>
        ) : Object.keys(groupedSteps).length === 0 ? (
          <div className="p-4 text-center text-muted-foreground h-full flex items-center justify-center">
            No pipes available.
          </div>
        ) : (
          Object.entries(groupedSteps).map(([category, steps]) => (
            <div key={category} className="p-4">
              <h3 className="text-sm font-semibold mb-2 text-muted-foreground">
                {category}
              </h3>
              <div className="space-y-2">
                {steps.map((step) => (
                  <Button
                    key={step.id}
                    variant="outline"
                    className={cn(
                      'w-full justify-start cursor-grab text-left h-auto py-2 [&.is-dragging]:cursor-grabbing',
                      buttonFocusStyle
                    )}
                    title={step.name}
                    draggable
                    onDragStart={(event) =>
                      onDragStart(event, step.stepType, step.id, step.name)
                    }
                  >
                    {step.name}
                  </Button>
                ))}
              </div>
              <Separator className="mt-4" />
            </div>
          ))
        )}
      </CardContent>
    </aside>
  );
};

export default PipelineSidebar;
