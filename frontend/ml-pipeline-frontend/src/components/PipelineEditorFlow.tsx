    // src/components/PipelineEditorFlow.tsx
    import React, { useState, useRef, useCallback, useEffect } from 'react';
    import ReactFlow, {
      Controls,
      MiniMap,
      Connection,
      Edge,
      Node,
      useReactFlow,
      ReactFlowInstance,
      NodeTypes,
      addEdge,
    } from 'reactflow';
    import 'reactflow/dist/style.css';

    // Import Hooks and Components
    import { usePipelineNodesState } from '@/hooks/usePipelineNodesState';
    import ParameterEditDialog from '@/components/ParameterEditDialog';
    import CustomNode from './CustomNode';
    import { toast } from "sonner"; // Import toast function from sonner

    // Import Config
    import { getNewNodeId, availablePipeTypes } from '@/config/pipelineEditorConfig';

    // Define the custom node types mapping
    const nodeTypes: NodeTypes = {
      pipelineNode: CustomNode,
    };

    // Define Pro Options to hide attribution
    const proOptions = { hideAttribution: true };

    // --- Main Flow Component ---
    const PipelineEditorFlow: React.FC = () => {
      const reactFlowWrapper = useRef<HTMLDivElement>(null);
      const [reactFlowInstance, setReactFlowInstance] = useState<ReactFlowInstance | null>(null);
      const { project } = useReactFlow();
      // Removed useToast hook

      // --- Use Custom Hook for Nodes/Edges ---
      const { nodes, edges, setNodes, setEdges, onNodesChange, onEdgesChange } = usePipelineNodesState();

      // --- State for Parameter Editing Dialog (Managed Locally) ---
      const [isDialogOpen, setIsDialogOpen] = useState(false);
      const [configuringNode, setConfiguringNode] = useState<Node | null>(null);
      const [paramValues, setParamValues] = useState<Record<string, any>>({});

      // --- Dialog Control Callbacks (Defined Locally) ---
       const openDialog = useCallback((node: Node) => {
        setConfiguringNode(node);
        setParamValues(node.data.parameters || {});
        setIsDialogOpen(true);
      }, []);

      const closeDialog = useCallback(() => {
        setIsDialogOpen(false);
        setConfiguringNode(null);
        setParamValues({});
      }, []);

      const handleParamChange = useCallback((paramName: string, value: string | number) => {
        const inputElement = document.getElementById(paramName) as HTMLInputElement;
        const finalValue = inputElement?.type === 'number' ? (parseFloat(value as string) || value) : value;
        setParamValues(prev => ({ ...prev, [paramName]: finalValue }));
      }, []);

       const saveParameters = useCallback(() => {
        if (!configuringNode) return;
        const nodeLabel = configuringNode.data?.label ?? 'Node';
        const nodeId = configuringNode.id;

        setNodes((nds) =>
          nds.map((node) => {
            if (node.id === nodeId) {
              return { ...node, data: { ...node.data, parameters: paramValues } };
            }
            return node;
          })
        );

        // Trigger sonner toast notification after setting state
        toast.success("Parameters Saved", { // Use sonner's success method
           description: `Parameters for '${nodeLabel}' updated successfully.`,
           // You can add actions etc. here if needed
        });

        closeDialog();
      }, [configuringNode, paramValues, setNodes, closeDialog]); // Removed toast dependency


      // --- Connection Validation Logic ---
      const isValidConnection = useCallback(
        (connection: Connection): boolean => {
          const targetHasConnection = edges.some(
            (edge) => edge.target === connection.target && edge.targetHandle === connection.targetHandle
          );
          const sourceHasConnection = edges.some(
            (edge) => edge.source === connection.source && edge.sourceHandle === connection.sourceHandle
          );
          return !targetHasConnection && !sourceHasConnection;
        },
        [edges]
      );

      // --- Connect Logic (Adds Animation) ---
      const onConnect = useCallback(
        (params: Connection | Edge) => {
            const newEdge = { ...params, animated: true };
            setEdges((eds) => addEdge(newEdge, eds));
        },
        [setEdges]
      );

      // --- Drag/Drop Logic ---
      const onDragOver = useCallback((event: React.DragEvent) => {
        event.preventDefault();
        event.dataTransfer.dropEffect = 'move';
      }, []);

      const onDrop = useCallback(
        (event: React.DragEvent) => {
          event.preventDefault();
          if (!reactFlowWrapper.current || !reactFlowInstance) return;

          const reactFlowBounds = reactFlowWrapper.current.getBoundingClientRect();
          const type = event.dataTransfer.getData('application/reactflow-type');
          const label = event.dataTransfer.getData('application/reactflow-label');

          if (typeof type === 'undefined' || !type || !nodeTypes[type]) {
            console.error(`Invalid or undefined node type dropped: ${type}`);
            return;
          }

          const pipeConfig = availablePipeTypes.find(p => p.label === label);
          const defaultParams = pipeConfig?.defaultParameters ?? {};

          const position = project({
            x: event.clientX - reactFlowBounds.left - 75,
            y: event.clientY - reactFlowBounds.top - 20,
          });

          const newNode: Node = {
            id: getNewNodeId(),
            type,
            position,
            data: { label: `${label}`, parameters: { ...defaultParams } },
          };
          setNodes((nds) => nds.concat(newNode));
        },
        [reactFlowInstance, project, setNodes]
      );

      // --- Node Click Logic (Calls local openDialog) ---
      const onNodeClick = useCallback((event: React.MouseEvent, node: Node) => {
          event.stopPropagation();
          event.preventDefault();
          openDialog(node);
      }, [openDialog]);


      // --- Render ---
      return (
        <React.Fragment>
            {/* Main container for React Flow */}
            <div
              className="flex-grow h-full bg-neutral-50 dark:bg-neutral-900"
              ref={reactFlowWrapper}
            >
              <ReactFlow
                nodes={nodes}
                edges={edges}
                nodeTypes={nodeTypes}
                onNodesChange={onNodesChange}
                onEdgesChange={onEdgesChange}
                onConnect={onConnect}
                isValidConnection={isValidConnection}
                onInit={setReactFlowInstance}
                onDrop={onDrop}
                onDragOver={onDragOver}
                onNodeClick={onNodeClick}
                fitView
                proOptions={proOptions}
              >
                <Controls />
                <MiniMap />
              </ReactFlow>
            </div>

            {/* Render the ParameterEditDialog (using original shadcn ui components) */}
            <ParameterEditDialog
                isOpen={isDialogOpen}
                node={configuringNode}
                paramValues={paramValues}
                onClose={closeDialog}
                onSave={saveParameters} // This now triggers the sonner toast
                onChange={handleParamChange}
            />
        </React.Fragment>
      );
    };

    export default PipelineEditorFlow;
    