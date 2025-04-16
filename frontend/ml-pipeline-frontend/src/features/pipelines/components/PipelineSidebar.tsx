import React from 'react';
import { StepDefinition } from '@/types';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { cn } from '@/lib/utils';
import { useQuery } from '@tanstack/react-query'; // Import useQuery
import { fetchStepDefinitions } from '../services/stepDefinitionService'; // Import fetch function
import { Loader2Icon, AlertTriangleIcon } from 'lucide-react'; // Icons for loading/error

interface PipelineSidebarProps {
  // No longer needs availableSteps passed as prop
}

const groupStepsByCategory = (steps: StepDefinition[] = []): Record<string, StepDefinition[]> => {
  return steps.reduce((acc, step) => {
    const category = step.category || 'Uncategorized';
    if (!acc[category]) acc[category] = [];
    acc[category].push(step);
    return acc;
  }, {} as Record<string, StepDefinition[]>);
};

const buttonFocusStyle = "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 dark:focus-visible:ring-offset-gray-950";

const PipelineSidebar: React.FC<PipelineSidebarProps> = () => {

  // Fetch step definitions using React Query
  const { data: availableSteps = [], isLoading, error } = useQuery<StepDefinition[], Error>({
    queryKey: ['stepDefinitions'],
    queryFn: fetchStepDefinitions,
    staleTime: 1000 * 60 * 60, // Cache for 1 hour, as they likely don't change often
  });

  const onDragStart = (event: React.DragEvent, nodeType: string, stepDefinitionId: string, label: string) => {
    event.dataTransfer.setData('application/reactflow-stepdef-id', stepDefinitionId);
    event.dataTransfer.setData('application/reactflow-label', label);
    event.dataTransfer.setData('application/reactflow-type', nodeType);
    event.dataTransfer.effectAllowed = 'move';
  };

  const groupedSteps = groupStepsByCategory(availableSteps);

  return (
    <aside className="w-64 bg-card border-l border-border flex flex-col shrink-0 h-full">
      <CardHeader className="shrink-0">
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
             <span className="text-sm">Error loading pipes: {error.message}</span>
          </div>
        ) : Object.keys(groupedSteps).length === 0 ? (
           <div className="p-4 text-center text-muted-foreground h-full flex items-center justify-center">
             No pipes available.
           </div>
        ) : (
          Object.entries(groupedSteps).map(([category, steps]) => (
            <div key={category} className="p-4">
              <h3 className="text-sm font-semibold mb-2 text-muted-foreground">{category}</h3>
              <div className="space-y-2">
                {steps.map((step) => (
                  <Button
                    key={step.id}
                    variant="outline"
                    className={cn("w-full justify-start cursor-grab text-left h-auto py-2", buttonFocusStyle)}
                    draggable
                    onDragStart={(event) => onDragStart(event, step.type, step.id, step.label)}
                    title={step.description}
                  >
                    {step.label}
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