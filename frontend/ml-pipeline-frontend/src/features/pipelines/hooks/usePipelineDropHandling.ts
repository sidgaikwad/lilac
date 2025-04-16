import { useCallback, RefObject } from 'react';
import { Node, ReactFlowInstance, useReactFlow } from 'reactflow';
import { hardcodedStepDefinitions } from '@/config/stepDefinitions';

// TODO: Replace simple counter with a more robust ID generation strategy (e.g., uuid)
let idCounter = 0;
const getNewNodeId = () => `dndnode_${idCounter++}`;

interface UsePipelineDropHandlingProps {
  reactFlowWrapperRef: RefObject<HTMLDivElement | null>;
  reactFlowInstance: ReactFlowInstance | null;
  addNode: (node: Node) => void;
}

/**
 * Custom hook to handle dragging steps from the sidebar and dropping them onto the canvas.
 */
export function usePipelineDropHandling({
  reactFlowWrapperRef,
  reactFlowInstance,
  addNode,
}: UsePipelineDropHandlingProps) {
  const { project } = useReactFlow();

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

      const position = project({
        x: event.clientX - reactFlowBounds.left,
        y: event.clientY - reactFlowBounds.top,
      });

      // Extract default parameters from the step definition
      const defaultParams = stepDef.parameters.reduce((acc, param) => {
        if (param.defaultValue !== undefined) acc[param.name] = param.defaultValue;
        return acc;
      }, {} as Record<string, any>);

      const newNode: Node = {
        id: getNewNodeId(),
        type: 'pipelineNode', // Matches the key in nodeTypes map
        position,
        // Store relevant data for the node and parameter editor
        data: { label, stepType: type, parameters: defaultParams, stepDefinition: stepDef },
      };

      addNode(newNode);
    },
    [reactFlowInstance, project, addNode, reactFlowWrapperRef]
  );

  return { onDragOver, onDrop };
}