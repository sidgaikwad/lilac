import { useMutation, useQueryClient } from "@tanstack/react-query";
import { QueryKeys } from "./constants";
import { simulateDelay } from "./mocks";
import { toast } from "sonner"; // For user feedback

// --- Mock Mutation Functions ---

// Simulate creating a pipeline (returns new ID)
const createPipelineMock = async (payload: { name: string; projectId: string }): Promise<{ id: string }> => {
    console.log(`Mock API: Creating pipeline "${payload.name}" for project ${payload.projectId}...`);
    await simulateDelay(500);
    const newId = `pipeline-${Math.random().toString(36).substring(2, 9)}`; // Simple random ID
    console.log(`Mock API: Created pipeline with ID: ${newId}`);
    // In a real API, this would return the created pipeline object or just its ID
    return { id: newId };
};

// Simulate renaming a pipeline
const renamePipelineMock = async (payload: { pipelineId: string; newName: string }): Promise<boolean> => {
    console.log(`Mock API: Renaming pipeline ${payload.pipelineId} to "${payload.newName}"...`);
    await simulateDelay(200);
    console.log(`Mock API: Renamed pipeline ${payload.pipelineId}.`);
    // Simulate success - in real API, check response status
    return true;
};

// Simulate deleting a pipeline
const deletePipelineMock = async (pipelineId: string): Promise<boolean> => {
    console.log(`Mock API: Deleting pipeline ${pipelineId}...`);
    await simulateDelay(600);
    console.log(`Mock API: Deleted pipeline ${pipelineId}.`);
    // Simulate success
    return true;
};


// --- Mutation Hooks ---

export function useCreatePipeline() {
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: createPipelineMock,
        onSuccess: (data, variables) => {
            toast.success(`Pipeline "${variables.name}" created (mock).`);
            // Invalidate the list query for the specific project to refetch
            queryClient.invalidateQueries({ queryKey: [QueryKeys.LIST_PIPELINE, variables.projectId] });
        },
        onError: (error, variables) => {
            console.error("Error creating pipeline (mock):", error);
            toast.error(`Failed to create pipeline "${variables.name}".`);
        },
    });
}

export function useRenamePipeline() {
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: renamePipelineMock,
        // Invalidate the general pipeline list query key on success.
        onSuccess: (data, variables) => {
            toast.success(`Pipeline renamed to "${variables.newName}" (mock).`);
            console.warn("Invalidating general pipeline list query after rename.");
            queryClient.invalidateQueries({ queryKey: [QueryKeys.LIST_PIPELINE] });
        },
        onError: (error, variables) => {
            console.error("Error renaming pipeline (mock):", error);
            toast.error(`Failed to rename pipeline.`); // Original name might not be easily available here
        },
    });
}


export function useDeletePipeline() {
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: deletePipelineMock,
        // Invalidate the general pipeline list query key on success.
        onSuccess: (data, variables) => {
            toast.success(`Pipeline deleted (mock).`);
            console.warn("Invalidating general pipeline list query after delete.");
            queryClient.invalidateQueries({ queryKey: [QueryKeys.LIST_PIPELINE] });
        },
        onError: (error, variables) => {
            console.error("Error deleting pipeline (mock):", error);
            toast.error(`Failed to delete pipeline.`);
        },
    });
}