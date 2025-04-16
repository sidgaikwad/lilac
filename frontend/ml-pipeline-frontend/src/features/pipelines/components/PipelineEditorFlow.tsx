import React, { useState, useCallback, useRef } from 'react';
import ReactFlow, {
  Controls,
  Background,
  Node,
  Edge,
  BackgroundVariant,
  ReactFlowInstance,
  NodeTypes,
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

const PipelineEditorFlow: React.FC = () => {
  const reactFlowWrapper = useRef<HTMLDivElement>(null);
  const [reactFlowInstance, setReactFlowInstance] = useState<ReactFlowInstance | null>(null);

  const {
    nodes,
    edges,
    onNodesChange,
    onEdgesChange,
    onConnect,
    addNode,
  } = usePipelineEditorState();

  const { onDragOver, onDrop } = usePipelineDropHandling({
    reactFlowWrapperRef: reactFlowWrapper,
    reactFlowInstance,
    addNode,
  });

  const {
    isDialogOpen,
    configuringNode,
    openDialog,
    closeDialog,
    handleSaveParameters,
  } = useParameterDialog();

  const onNodeClick = useCallback((event: React.MouseEvent, node: Node) => {
    openDialog(node);
  }, [openDialog]);

  return (
    <div className="flex-grow h-full bg-gray-100 dark:bg-gray-900 relative" ref={reactFlowWrapper}>
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        onInit={setReactFlowInstance}
        onDrop={onDrop}
        onDragOver={onDragOver}
        onNodeClick={onNodeClick}
        fitView
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