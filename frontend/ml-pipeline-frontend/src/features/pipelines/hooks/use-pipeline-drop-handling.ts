import { useCallback, RefObject } from 'react';
import { ReactFlowInstance, XYPosition } from '@xyflow/react'; // Import Project type
import { StepDefinition } from '@/types';

interface UsePipelineDropHandlingProps {
  reactFlowWrapperRef: RefObject<HTMLDivElement | null>;
  reactFlowInstance: ReactFlowInstance | null;
  addNode: (position: XYPosition, stepDefinition: StepDefinition) => void;
  stepDefinitions: StepDefinition[];
}

/**
 * Custom hook to handle dragging steps from the sidebar and dropping them onto the canvas.
 */
export function usePipelineDropHandling({
  reactFlowWrapperRef,
  reactFlowInstance,
  addNode,
  stepDefinitions,
}: UsePipelineDropHandlingProps) {
  const onDragOver = useCallback((event: React.DragEvent) => {
    event.preventDefault();
    event.dataTransfer.dropEffect = 'move';
  }, []);

  const onDrop = useCallback(
    (event: React.DragEvent) => {
      event.preventDefault();

      if (!reactFlowWrapperRef.current || !reactFlowInstance) {
        console.warn(
          'Drop event occurred before React Flow was fully initialized or wrapper ref is null.'
        );
        return;
      }

      const reactFlowBounds =
        reactFlowWrapperRef.current.getBoundingClientRect();
      const stepDefinitionId = event.dataTransfer.getData(
        'application/reactflow-stepdef-id'
      );
      const label = event.dataTransfer.getData('application/reactflow-label');
      const type = event.dataTransfer.getData('application/reactflow-type');

      if (!stepDefinitionId || !label || !type) {
        console.warn('Drop event missing required dataTransfer items.');
        return;
      }

      const stepDef = stepDefinitions.find(
        (def) => def.id === stepDefinitionId
      );
      if (!stepDef) {
        console.error(`Dropped step definition not found: ${stepDefinitionId}`);
        return;
      }

      const position = reactFlowInstance.screenToFlowPosition({
        // Use project from props
        x: event.clientX - reactFlowBounds.left,
        y: event.clientY - reactFlowBounds.top,
      });

      addNode(position, stepDef);
    },
    [reactFlowInstance, addNode, reactFlowWrapperRef, stepDefinitions]
  );

  return { onDragOver, onDrop };
}
