// src/config/pipelineEditorConfig.ts
import { Node, Edge } from 'reactflow';

// Define initial nodes and edges
// Ensure the initial node uses the type that maps to your CustomNode
export const initialNodes: Node[] = [
  // Use 'pipelineNode' which maps to CustomNode with Left/Right handles
  {
    id: '1',
    type: 'pipelineNode',
    data: {
        label: 'Start Step',
        // Add dummy parameters for the initial node
        parameters: {
            param1: 'Initial Value A',
            param2: 100,
        }
    },
    position: { x: 50, y: 150 }
  },
];

// Initial edges can also be animated if needed
export const initialEdges: Edge[] = [];

// Dummy data for available pipe types in the sidebar
// Ensure all types here match a key in the nodeTypes map in PipelineEditorFlow.tsx
export const availablePipeTypes = [
  // Add default parameters structure for nodes created from sidebar
  { type: 'pipelineNode', label: 'Resize Image', defaultParameters: { width: 1024, height: 768, mode: 'stretch' } },
  { type: 'pipelineNode', label: 'Blur Detector', defaultParameters: { threshold: 0.5, method: 'laplacian' } },
  { type: 'pipelineNode', label: 'Save Output Step', defaultParameters: { format: 'png', path: '/output/images' } },
];

// Simple ID generator for new nodes
let idCounter = initialNodes.length + 1; // Start IDs after initial nodes
export const getNewNodeId = (): string => `${idCounter++}`;
