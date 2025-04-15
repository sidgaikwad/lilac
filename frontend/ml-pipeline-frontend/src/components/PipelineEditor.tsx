// src/components/PipelineEditor.tsx
import React, { useState, useRef, useCallback } from 'react';
import ReactFlow, {
  ReactFlowProvider,
  addEdge,
  useNodesState,
  useEdgesState,
  Controls,
  Background,
  MiniMap,
  Connection,
  Edge,
  Node,
  useReactFlow,
  ReactFlowInstance,
} from 'reactflow';
import 'reactflow/dist/style.css'; // Import React Flow styles

import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'; // Assuming shadcn setup
import { Separator } from '@/components/ui/separator'; // Assuming shadcn setup
import { Button } from '@/components/ui/button'; // Assuming shadcn setup
import { cn } from '@/lib/utils'; // Assuming shadcn setup

// Define initial nodes and edges (can be empty)
const initialNodes: Node[] = [
  { id: '1', type: 'input', data: { label: 'Start (Input)' }, position: { x: 100, y: 100 } },
];
const initialEdges: Edge[] = [];

// Dummy data for available pipe types
const availablePipeTypes = [
  { type: 'default', label: 'Resize Image' },
  { type: 'default', label: 'Blur Detector' },
  { type: 'output', label: 'Save Output' },
  // Add more pipe types as needed
];

let id = 2; // Start IDs for new nodes after the initial 'Start' node
const getId = () => `${id++}`;

// --- Sidebar Component ---
interface SidebarProps {
  pipeTypes: { type: string; label: string }[];
}

const Sidebar: React.FC<SidebarProps> = ({ pipeTypes }) => {
  const onDragStart = (event: React.DragEvent, nodeType: string, nodeLabel: string) => {
    // Store data about the node being dragged
    event.dataTransfer.setData('application/reactflow-type', nodeType);
    event.dataTransfer.setData('application/reactflow-label', nodeLabel);
    event.dataTransfer.effectAllowed = 'move';
  };

  return (
    <Card className="h-full rounded-none border-r border-l-0 border-t-0 border-b-0">
      <CardHeader>
        <CardTitle>Available Pipes</CardTitle>
      </CardHeader>
      <Separator />
      <CardContent className="p-4 space-y-2">
        {pipeTypes.map((pipe) => (
          <Button
            key={pipe.label}
            variant="outline"
            className="w-full justify-start cursor-grab"
            draggable // Make the button draggable
            onDragStart={(event) => onDragStart(event, pipe.type, pipe.label)}
          >
            {pipe.label} ({pipe.type})
          </Button>
        ))}
      </CardContent>
    </Card>
  );
};

// --- Main Editor Component ---
const PipelineEditorFlow: React.FC = () => {
  const reactFlowWrapper = useRef<HTMLDivElement>(null); // Ref for the wrapper div
  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);
  const [reactFlowInstance, setReactFlowInstance] = useState<ReactFlowInstance | null>(null);
  const { project } = useReactFlow(); // Hook to get projection functions

  // Callback for handling new connections
  const onConnect = useCallback(
    (params: Connection | Edge) => setEdges((eds) => addEdge(params, eds)),
    [setEdges]
  );

  // Prevent default drag behavior
  const onDragOver = useCallback((event: React.DragEvent) => {
    event.preventDefault();
    event.dataTransfer.dropEffect = 'move';
  }, []);

  // Handle dropping a node from the sidebar
  const onDrop = useCallback(
    (event: React.DragEvent) => {
      event.preventDefault();

      if (!reactFlowWrapper.current || !reactFlowInstance) {
        return;
      }

      const reactFlowBounds = reactFlowWrapper.current.getBoundingClientRect();
      const type = event.dataTransfer.getData('application/reactflow-type');
      const label = event.dataTransfer.getData('application/reactflow-label');

      // Check if the dropped element is valid
      if (typeof type === 'undefined' || !type) {
        return;
      }

      // Project screen position to flow pane position
      const position = project({
        x: event.clientX - reactFlowBounds.left,
        y: event.clientY - reactFlowBounds.top,
      });

      // Create the new node
      const newNode: Node = {
        id: getId(),
        type,
        position,
        data: { label: `${label}` }, // Use the label from drag data
      };

      // Add the new node to the state
      setNodes((nds) => nds.concat(newNode));
    },
    [reactFlowInstance, project, setNodes] // Include project and setNodes in dependencies
  );

  return (
    <div className="flex h-full w-full" ref={reactFlowWrapper}>
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange} // Handles node movement, selection, removal
        onEdgesChange={onEdgesChange} // Handles edge changes
        onConnect={onConnect} // Handles new connections
        onInit={setReactFlowInstance} // Store the react flow instance
        onDrop={onDrop} // Handles dropping elements
        onDragOver={onDragOver} // Necessary for onDrop to work
        fitView // Adjusts the view to fit all nodes initially
        className="bg-background flex-grow" // Use Tailwind background color
      >
        <Controls />
        <MiniMap />
        <Background gap={12} size={1} />
      </ReactFlow>
    </div>
  );
};

// --- Wrapper Component with Provider ---
const PipelineEditor: React.FC = () => {
  return (
    <div className="flex h-screen w-screen border-t"> {/* Full screen height, border top */}
      {/* Wrap with ReactFlowProvider */}
      <ReactFlowProvider>
        <div className="w-64 flex-shrink-0 border-r"> {/* Fixed width sidebar */}
          <Sidebar pipeTypes={availablePipeTypes} />
        </div>
        <div className="flex-grow h-full"> {/* Flow area takes remaining space */}
          <PipelineEditorFlow />
        </div>
      </ReactFlowProvider>
    </div>
  );
};

export default PipelineEditor;
