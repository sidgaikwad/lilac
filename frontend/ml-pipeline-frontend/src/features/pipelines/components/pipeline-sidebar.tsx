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

function PipelineSidebar() {
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

  const groupedSteps = groupStepsByCategory(stepDefinitions ?? []);

  return (
    <aside className="bg-card flex h-full w-64 shrink-0 flex-col space-y-2 border border-l">
      <CardHeader className="shrink-0 pt-4">
        <CardTitle>Available Pipes</CardTitle>
      </CardHeader>
      <Separator className="shrink-0" />
      <CardContent className="flex-grow overflow-y-auto p-0">
        {isLoading ? (
          <div className="text-muted-foreground flex h-full items-center justify-center p-4 text-center">
            <Loader2Icon className="mr-2 h-4 w-4 animate-spin" /> Loading...
          </div>
        ) : error ? (
          <div className="text-destructive flex h-full flex-col items-center justify-center p-4 text-center">
            <AlertTriangleIcon className="mb-2 h-6 w-6" />
            <span className="text-sm">Error loading pipes: {error.error}</span>
          </div>
        ) : Object.keys(groupedSteps).length === 0 ? (
          <div className="text-muted-foreground flex h-full items-center justify-center p-4 text-center">
            No pipes available.
          </div>
        ) : (
          Object.entries(groupedSteps).map(([category, steps]) => (
            <div key={category} className="p-4">
              <h3 className="text-foreground mb-2 text-sm font-semibold">
                {category}
              </h3>
              <div className="space-y-2">
                {steps.map((step) => (
                  <Button
                    key={step.id}
                    variant="outline"
                    className={cn(
                      'h-auto w-full cursor-grab justify-start py-2 text-left [&.is-dragging]:cursor-grabbing',
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
