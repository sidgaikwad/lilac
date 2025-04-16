import React, { useState, useCallback, useRef, useEffect } from 'react';
import ReactFlow, {
  Controls,
  Background,
  Node,
  Edge,
  BackgroundVariant,
  ReactFlowInstance,
  NodeTypes,
  Viewport,
  useReactFlow, // Keep useReactFlow here at the top level
} from 'reactflow';
import PipelineNode from './PipelineNode';
import ParameterEditDialog from './ParameterEditDialog';
import { Toaster } from "@/components/ui/sonner";
import { usePipelineEditorState } from '../hooks/usePipelineEditorState';
import { usePipelineDropHandling } from '../hooks/usePipelineDropHandling';
import { useParameterDialog } from '../hooks/useParameterDialog';

import 'reactflow/dist/style.css';

const nodeTypes: NodeTypes = {
  pipelineNode: PipelineNode,
};

interface PipelineEditorFlowProps {
  initialNodes?: Node[];
  initialEdges?: Edge[];
  initialViewport?: Viewport;
  onFlowInit?: (instance: ReactFlowInstance) => void;
}

const PipelineEditorFlow: React.FC<PipelineEditorFlowProps> = ({
  initialNodes = [],
  initialEdges = [],
  initialViewport,
  onFlowInit,
}) => {
  const reactFlowWrapper = useRef<HTMLDivElement>(null);
  const [reactFlowInstance, setReactFlowInstance] = useState<ReactFlowInstance | null>(null);
  // Call useReactFlow at the top level of the component
  const { project } = useReactFlow();

  const {
    nodes,
    edges,
    setNodes, // Get setNodes from the state hook
    onNodesChange,
    onEdgesChange,
    onConnect,
    isValidConnection,
    addNode,
  } = usePipelineEditorState(initialNodes, initialEdges);

  // Pass the required 'project' function to the drop handling hook
  const { onDragOver, onDrop } = usePipelineDropHandling({
    reactFlowWrapperRef: reactFlowWrapper,
    reactFlowInstance,
    addNode,
    project, // Pass project down
  });

  // Pass the required 'setNodes' function to the dialog hook
  const {
    isDialogOpen,
    configuringNode,
    openDialog,
    closeDialog,
    handleSaveParameters,
  } = useParameterDialog({ setNodes }); // Pass setNodes down

  const onNodeClick = useCallback((event: React.MouseEvent, node: Node) => {
    openDialog(node);
  }, [openDialog]);

  const handleInit = (instance: ReactFlowInstance) => {
    setReactFlowInstance(instance);
    if (onFlowInit) {
      onFlowInit(instance);
    }
  };

  useEffect(() => {
    if (reactFlowInstance && initialViewport) {
      reactFlowInstance.setViewport(initialViewport, { duration: 0 });
    }
  }, [reactFlowInstance, initialViewport]);


  return (
    <div className="flex-grow h-full bg-gray-100 dark:bg-gray-900 relative" ref={reactFlowWrapper}>
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        isValidConnection={isValidConnection}
        onInit={handleInit}
        onDrop={onDrop}
        onDragOver={onDragOver}
        onNodeClick={onNodeClick}
        defaultViewport={initialViewport}
        proOptions={{ hideAttribution: true }}
      >
        <Controls />
        <Background variant={BackgroundVariant.Dots} gap={12} size={1} />
        <Toaster position="bottom-left" richColors />
      </ReactFlow>

      {configuringNode && (
        <ParameterEditDialog
          isOpen={isDialogOpen}
          onClose={closeDialog}
          onSave={handleSaveParameters}
          nodeLabel={configuringNode.data.label}
          stepDefinition={configuringNode.data.stepDefinition}
          initialParamValues={configuringNode.data.parameters || {}}
        />
      )}
    </div>
  );
};

export default PipelineEditorFlow;