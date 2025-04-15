// src/hooks/usePipelineNodesState.ts
import { useState, useCallback } from 'react';
import {
    Node,
    Edge,
    OnNodesChange,
    OnEdgesChange,
    applyNodeChanges,
    applyEdgeChanges,
    addEdge,
    Connection,
} from 'reactflow';
import { initialNodes, initialEdges } from '@/config/pipelineEditorConfig';

export function usePipelineNodesState() {
    const [nodes, setNodes] = useState<Node[]>(initialNodes);
    const [edges, setEdges] = useState<Edge[]>(initialEdges);

    const onNodesChange: OnNodesChange = useCallback(
        (changes) => setNodes((nds) => applyNodeChanges(changes, nds)),
        [setNodes]
    );

    const onEdgesChange: OnEdgesChange = useCallback(
        (changes) => setEdges((eds) => applyEdgeChanges(changes, eds)),
        [setEdges]
    );

    // Keep onConnect logic separate for now, but provide setters
    // Or potentially include it here if preferred

    return {
        nodes,
        edges,
        setNodes,
        setEdges,
        onNodesChange,
        onEdgesChange,
    };
}
