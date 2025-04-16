import { useState, useCallback } from 'react';
import {
  Node,
  Edge,
  OnNodesChange,
  OnEdgesChange,
  OnConnect,
  Connection,
  NodeChange,
  EdgeChange,
  applyNodeChanges,
  applyEdgeChanges,
  addEdge,
} from 'reactflow';

const initialNodes: Node[] = [];
const initialEdges: Edge[] = [];

/**
 * Custom hook to manage the state of nodes and edges in the React Flow editor.
 * Handles changes, connections, and provides a function to add nodes programmatically.
 */
export function usePipelineEditorState(
  initialNodesProp: Node[] = initialNodes,
  initialEdgesProp: Edge[] = initialEdges
) {
  const [nodes, setNodes] = useState<Node[]>(initialNodesProp);
  const [edges, setEdges] = useState<Edge[]>(initialEdgesProp);

  const onNodesChange: OnNodesChange = useCallback(
    (changes: NodeChange[]) => setNodes((nds) => applyNodeChanges(changes, nds)),
    [setNodes]
  );

  const onEdgesChange: OnEdgesChange = useCallback(
    (changes: EdgeChange[]) => setEdges((eds) => applyEdgeChanges(changes, eds)),
    [setEdges]
  );

  const onConnect: OnConnect = useCallback(
    (connection: Connection) => setEdges((eds) => addEdge(connection, eds)),
    [setEdges]
  );

  const addNode = useCallback((newNode: Node) => {
    setNodes((nds) => nds.concat(newNode));
  }, [setNodes]);

  return {
    nodes,
    edges,
    setNodes, // Needed for direct updates like parameter changes
    setEdges,
    onNodesChange,
    onEdgesChange,
    onConnect,
    addNode,
  };
}