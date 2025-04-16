import { useState, useCallback } from 'react';
import { Node, useReactFlow } from 'reactflow';
import { toast } from "sonner";

/**
 * Custom hook to manage the state and actions for the parameter editing dialog.
 */
export function useParameterDialog() {
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [configuringNode, setConfiguringNode] = useState<Node | null>(null);
  // Get setNodes directly from React Flow context to update node data
  const { setNodes } = useReactFlow();

  const openDialog = useCallback((node: Node) => {
    setConfiguringNode(node);
    setIsDialogOpen(true);
  }, []);

  const closeDialog = useCallback(() => {
    setIsDialogOpen(false);
    setConfiguringNode(null);
  }, []);

  const handleSaveParameters = useCallback((updatedParams: Record<string, any>) => {
    if (!configuringNode) return;

    setNodes((nds) =>
      nds.map((node) => {
        if (node.id === configuringNode.id) {
          // Preserve existing data and only update parameters
          const originalData = node.data || {};
          return {
            ...node,
            data: {
              ...originalData,
              parameters: updatedParams
            }
          };
        }
        return node;
      })
    );

    toast.success("Parameters Saved", {
      description: `Parameters for '${configuringNode.data.label}' updated.`,
    });

    // Dialog is closed by its own internal actions (Save/Cancel buttons)
  }, [configuringNode, setNodes]);

  return {
    isDialogOpen,
    configuringNode,
    openDialog,
    closeDialog,
    handleSaveParameters,
  };
}