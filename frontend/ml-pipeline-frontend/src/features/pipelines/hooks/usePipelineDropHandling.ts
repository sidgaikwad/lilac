import { useCallback, RefObject } from 'react';
import { Node, ReactFlowInstance, Project } from 'reactflow'; // Import Project type
import { hardcodedStepDefinitions } from '@/config/stepDefinitions';

let idCounter = 0;
const getNewNodeId = () => `dndnode_${idCounter++}`;

interface UsePipelineDropHandlingProps {
  reactFlowWrapperRef: RefObject<HTMLDivElement | null>;
  reactFlowInstance: ReactFlowInstance | null;
  addNode: (node: Node) => void;
  project: Project; // Accept project function as prop
}

/**
 * Custom hook to handle dragging steps from the sidebar and dropping them onto the canvas.
 */
export function usePipelineDropHandling({
  reactFlowWrapperRef,
  reactFlowInstance,
  addNode,
  project, // Use project from props
}: UsePipelineDropHandlingProps) {
  // Removed internal useReactFlow call

  const onDragOver = useCallback((event: React.DragEvent) => {
    event.preventDefault();
    event.dataTransfer.dropEffect = 'move';
  }, []);

  const onDrop = useCallback(
    (event: React.DragEvent) => {
      event.preventDefault();

      if (!reactFlowWrapperRef.current || !reactFlowInstance) {
        console.warn("Drop event occurred before React Flow was fully initialized or wrapper ref is null.");
        return;
      }

      const reactFlowBounds = reactFlowWrapperRef.current.getBoundingClientRect();
      const stepDefinitionId = event.dataTransfer.getData('application/reactflow-stepdef-id');
      const label = event.dataTransfer.getData('application/reactflow-label');
      const type = event.dataTransfer.getData('application/reactflow-type');

      if (!stepDefinitionId || !label || !type) {
        console.warn("Drop event missing required dataTransfer items.");
        return;
      }

      const stepDef = hardcodedStepDefinitions.find(def => def.id === stepDefinitionId);
      if (!stepDef) {
        console.error(`Dropped step definition not found: ${stepDefinitionId}`);
        return;
      }

      const position = project({ // Use project from props
        x: event.clientX - reactFlowBounds.left,
        y: event.clientY - reactFlowBounds.top,
      });

      const defaultParams = stepDef.parameters.reduce((acc, param) => {
        if (param.defaultValue !== undefined) acc[param.name] = param.defaultValue;
        return acc;
      }, {} as Record<string, any>);

      const newNode: Node = {
        id: getNewNodeId(),
        type: 'pipelineNode',
        position,
        data: { label, stepType: type, parameters: defaultParams, stepDefinition: stepDef },
      };

      addNode(newNode);
    },
    [reactFlowInstance, project, addNode, reactFlowWrapperRef] // Add project to dependencies
  );

  return { onDragOver, onDrop };
}