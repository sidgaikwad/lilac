import { useMutation, useQueryClient } from "@tanstack/react-query";
import { QueryKeys } from "./constants";
import { simulateDelay } from "./mocks";
import { toast } from "sonner";
import { FlowData } from "@/lib/localStorageUtils"; // Keep type for payload structure for now
import { PipelineDefinition } from "@/types"; // For return type if needed

// Define the expected payload for the mutation
interface SavePipelinePayload {
    pipelineId: string;
    flowData: FlowData;
    projectId?: string; // Include projectId for invalidation if possible
}

// Mock function simulating saving a pipeline version
// A real API might return the updated PipelineDefinition or just a success status
const savePipelineVersionMock = async (payload: SavePipelinePayload): Promise<PipelineDefinition | null> => {
    console.log(`Mock API: Saving new version for pipeline ${payload.pipelineId}...`);
    console.log("Mock API: Payload FlowData:", payload.flowData);
    await simulateDelay(700);
    console.log(`Mock API: Saved version for pipeline ${payload.pipelineId}.`);
    // Simulate returning updated pipeline data (or null if API just returns status)
    // For now, return null as we primarily rely on invalidation
    return null;
};

export function useSavePipelineVersion() {
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: savePipelineVersionMock,
        onSuccess: (data, variables) => {
            toast.success(`Pipeline saved (mock).`);
            // Invalidate the specific pipeline detail query to refetch it
            queryClient.invalidateQueries({ queryKey: [QueryKeys.GET_PIPELINE, variables.pipelineId] });
            // Also invalidate the list query for the project it belongs to
            if (variables.projectId) {
                queryClient.invalidateQueries({ queryKey: [QueryKeys.LIST_PIPELINE, variables.projectId] });
            } else {
                // Fallback if projectId wasn't passed
                queryClient.invalidateQueries({ queryKey: [QueryKeys.LIST_PIPELINE] });
            }
        },
        onError: (error, variables) => {
            console.error("Error saving pipeline version (mock):", error);
            toast.error(`Failed to save pipeline.`);
        },
    });
}