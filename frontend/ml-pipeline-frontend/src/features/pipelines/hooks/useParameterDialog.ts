import React, { useState, useCallback } from 'react';
import { Node } from 'reactflow';
import { toast } from "sonner";

// Correct type for the setNodes function from useState<Node[]>
type SetNodesAction = React.Dispatch<React.SetStateAction<Node[]>>;

interface UseParameterDialogProps {
  setNodes: SetNodesAction; // Use the correct type
}

/**
 * Custom hook to manage the state and actions for the parameter editing dialog.
 */
export function useParameterDialog({ setNodes }: UseParameterDialogProps) {
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [configuringNode, setConfiguringNode] = useState<Node | null>(null);

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

    setNodes((nds: Node[]) => // Add type for nds
      nds.map((node: Node) => { // Add type for node
        if (node.id === configuringNode.id) {
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

  }, [configuringNode, setNodes]);

  return {
    isDialogOpen,
    configuringNode,
    openDialog,
    closeDialog,
    handleSaveParameters,
  };
}