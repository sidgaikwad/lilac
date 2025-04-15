// src/hooks/useParameterDialog.ts
import { useState, useCallback } from 'react';
import { Node } from 'reactflow';

// Type for the setNodes function passed from the component using this hook
type SetNodesType = React.Dispatch<React.SetStateAction<Node[]>>;

export function useParameterDialog(setNodes: SetNodesType) {
    const [isDialogOpen, setIsDialogOpen] = useState(false);
    const [configuringNode, setConfiguringNode] = useState<Node | null>(null);
    // State for the temporary parameter values being edited in the dialog
    const [paramValues, setParamValues] = useState<Record<string, any>>({});

    const openDialog = useCallback((node: Node) => {
        setConfiguringNode(node);
        // Initialize dialog state with the node's current parameters
        setParamValues(node.data.parameters || {});
        setIsDialogOpen(true);
    }, []); // No dependencies needed as it only sets state

    const closeDialog = useCallback(() => {
        setIsDialogOpen(false);
        // Optional: Delay resetting node if animations cause flicker
        // setTimeout(() => {
             setConfiguringNode(null);
             setParamValues({}); // Clear temporary values
        // }, 300);
    }, []);

    const handleParamChange = useCallback((paramName: string, value: string | number) => {
        // Logic to handle input changes and update temporary state
        const inputElement = document.getElementById(paramName) as HTMLInputElement;
        // Attempt conversion based on input type, default to original value if parsing fails
        const finalValue = inputElement?.type === 'number' ? (parseFloat(value as string) || value) : value;
        setParamValues(prev => ({ ...prev, [paramName]: finalValue }));
    }, []);

    const saveParameters = useCallback(() => {
        if (!configuringNode) return;

        // Use the passed setNodes function to update the main nodes state
        setNodes((nds) =>
          nds.map((node) => {
            if (node.id === configuringNode.id) {
              // Create a new node object with updated data
              return { ...node, data: { ...node.data, parameters: { ...paramValues } } };
            }
            return node;
          })
        );
        closeDialog(); // Close dialog after saving
    }, [configuringNode, paramValues, setNodes, closeDialog]);

    return {
        isDialogOpen,
        configuringNode,
        paramValues, // Pass current values to the dialog component
        openDialog,
        closeDialog,
        handleParamChange,
        saveParameters,
    };
}
