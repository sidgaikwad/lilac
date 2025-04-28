import React, { useCallback, useEffect, useRef } from 'react';
import {
  ReactFlow,
  Controls,
  Background,
  BackgroundVariant,
  NodeTypes,
  EdgeTypes,
  useReactFlow,
  OnConnect,
} from '@xyflow/react';
import PipelineNode, { PipelineNodeType } from './PipelineNode';

import '@xyflow/react/dist/style.css';
import useReactFlowStore from '@/store/useReactFlowStore';
import { shallow } from 'zustand/shallow';
import PipelineEdge from './PipelineEdge';
import { Pipeline } from '@/types';
import { usePipelineDropHandling } from '../hooks/usePipelineDropHandling';
import { useListStepDefinitions } from '@/services/controlplane-api/useListStepDefinitions.hook';
import { DevTools } from '@/components/devtools';

const nodeTypes: NodeTypes = {
  pipelineNode: PipelineNode,
};

const edgeTypes: EdgeTypes = {
  pipelineEdge: PipelineEdge,
};

interface PipelineEditorFlowProps {
  pipeline: Pipeline;
}

const PipelineEditorFlow: React.FC<PipelineEditorFlowProps> = (
  props: PipelineEditorFlowProps
) => {
  const reactFlowInstance = useReactFlow();
  const reactFlowWrapper = useRef<HTMLDivElement>(null);
  const {
    data: stepDefinitions,
    isLoading: _isLoading,
    error: _error,
  } = useListStepDefinitions();
  const {
    nodes,
    edges,
    onNodesChange,
    onEdgesChange,
    initialize,
    addNode,
    addEdge,
  } = useReactFlowStore(
    (state) => ({
      nodes: state.nodes,
      edges: state.edges,
      onNodesChange: state.onNodesChange,
      onEdgesChange: state.onEdgesChange,
      initialize: state.initialize,
      addNode: state.addNode,
      addEdge: state.addEdge,
    }),
    shallow
  );

  useEffect(() => {
    console.log(props.pipeline);
    initialize(
      props.pipeline.steps.map(
        (step, idx) =>
          ({
            id: step.stepId,
            position: {
              x: 500 * idx,
              y: 0,
            },
            data: {
              stepDefinition: stepDefinitions?.stepDefinitions.find(
                (sd) => sd.id === step.stepDefinitionId
              ),
              parameters: step.stepParameters,
            },
            type: 'pipelineNode',
          }) as PipelineNodeType
      ),
      props.pipeline.stepConnections.map((conn) => ({
        id: crypto.randomUUID(),
        source: conn[0],
        target: conn[1],
        data: {
          fromStepId: conn[0],
          toStepId: conn[1],
        },
      }))
    );
  }, [props.pipeline]);

  const { onDragOver, onDrop } = usePipelineDropHandling({
    stepDefinitions: stepDefinitions?.stepDefinitions ?? [],
    reactFlowInstance: reactFlowInstance,
    addNode,
    reactFlowWrapperRef: reactFlowWrapper,
  });

  const onConnect: OnConnect = useCallback((params) => {
    addEdge(params);
  }, []);

  return (
    // Use theme background color
    <div
      className="flex-grow h-full bg-background relative"
      ref={reactFlowWrapper}
    >
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onDragOver={onDragOver}
        onDrop={onDrop}
        onConnect={onConnect}
        nodeOrigin={[0, 0]}
        fitView
      >
        <DevTools position="top-left" />
        <Controls showInteractive={false} />
        <Background variant={BackgroundVariant.Dots} gap={12} size={1} />
      </ReactFlow>
    </div>
  );
};

export default PipelineEditorFlow;
