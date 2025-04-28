import { StepDefinition } from '@/types';
import {
  Edge,
  EdgeChange,
  Node,
  NodeChange,
  OnNodesChange,
  OnEdgesChange,
  applyNodeChanges,
  applyEdgeChanges,
  XYPosition,
  Connection,
} from '@xyflow/react';
import { createWithEqualityFn } from 'zustand/traditional';

export type RFState = {
  nodes: Node[];
  edges: Edge[];
  onNodesChange: OnNodesChange;
  onEdgesChange: OnEdgesChange;
  reset: () => void;
  initialize: (nodes: Node[], edges: Edge[]) => void;
  addNode: (position: XYPosition, stepDefinition: StepDefinition) => void;
  updateNode: (nodeId: string, newParameters: object) => void;
  addEdge: (connection: Connection) => void;
};

const useReactFlowStore = createWithEqualityFn<RFState>((set, get) => ({
  nodes: [],
  edges: [],
  onNodesChange: (changes: NodeChange[]) => {
    set({
      nodes: applyNodeChanges(changes, get().nodes),
    });
  },
  onEdgesChange: (changes: EdgeChange[]) => {
    set({
      edges: applyEdgeChanges(changes, get().edges),
    });
  },
  reset: () => {
    set({
      edges: [],
      nodes: [],
    });
  },
  initialize: (nodes: Node[], edges: Edge[]) => {
    console.log(nodes, edges);
    set({
      edges,
      nodes,
    });
  },
  addNode: (position: XYPosition, stepDefinition: StepDefinition) => {
    const newNode = {
      id: crypto.randomUUID(),
      type: 'pipelineNode',
      data: { parameters: {}, stepDefinition },
      position,
    };

    set({
      nodes: [...get().nodes, newNode],
    });
  },
  updateNode: (nodeId: string, newParameters: object) => {
    set({
      nodes: get().nodes.map((node) =>
        node.id === nodeId
          ? {
              ...node,
              data: {
                ...node.data,
                parameters: {
                  ...(node.data.parameters as object),
                  ...newParameters,
                },
              },
            }
          : node
      ),
    });
  },
  addEdge: (connection: Connection) => {
    const newEdge: Edge = {
      id: crypto.randomUUID(),
      source: connection.source,
      target: connection.target,
      sourceHandle: connection.sourceHandle,
      targetHandle: connection.targetHandle,
      data: {
        fromStepId: connection.source,
        toStepId: connection.target,
      },
    };
    set({
      edges: [...get().edges, newEdge],
    });
  },
}));

export default useReactFlowStore;
