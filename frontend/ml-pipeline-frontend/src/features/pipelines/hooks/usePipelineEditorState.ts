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

  // Simplified validation: Only prevent connecting to the *same* node.
  // Allow multiple connections per handle temporarily, cleanup happens in onConnect.
  const isValidConnection = useCallback(
    (connection: Connection): boolean => {
      // Basic check: prevent self-connections (can be enhanced)
      return connection.source !== connection.target;
    },
    [] // No dependencies needed for this simple check
  );

  const onConnect: OnConnect = useCallback(
    (connection: Connection) => {
      // Remove any existing edge connected to the same target handle or source handle
      setEdges((prevEdges) => {
        const edgesToRemove = prevEdges.filter(
          edge => (edge.target === connection.target && edge.targetHandle === connection.targetHandle) ||
                  (edge.source === connection.source && edge.sourceHandle === connection.sourceHandle)
        );
        const updatedEdges = applyEdgeChanges(edgesToRemove.map(edge => ({ id: edge.id, type: 'remove' })), prevEdges);
        return addEdge({ ...connection, animated: true }, updatedEdges);
      });
    },
    [setEdges]
  );


  const addNode = useCallback((newNode: Node) => {
    setNodes((nds) => nds.concat(newNode));
  }, [setNodes]);

  return {
    nodes,
    edges,
    setNodes,
    setEdges,
    onNodesChange,
    onEdgesChange,
    onConnect,
    isValidConnection,
    addNode,
  };
}