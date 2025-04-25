import { useQuery } from "@tanstack/react-query";
import { QueryKeys } from "./constants";
import { MOCK_PIPELINES, simulateDelay } from "./mocks";
import { PipelineListItem } from "@/types";

interface UseListPipelinesProps {
    projectId: string | undefined; // Accept undefined to disable query
}

// Mock function simulating API call
const fetchPipelinesMock = async (projId: string): Promise<PipelineListItem[]> => {
    console.log(`Mock API: Fetching pipelines for projectId: ${projId}...`);
    await simulateDelay(400); // Simulate network latency
    const filtered = MOCK_PIPELINES.filter(p => p.projectId === projId);
    console.log(`Mock API: Returning ${filtered.length} pipelines for projectId ${projId}:`, filtered);
    // Sort by date descending (example)
    const sorted = [...filtered].sort((a, b) => new Date(b.lastModified).getTime() - new Date(a.lastModified).getTime());
    return sorted;
};

export function useListPipelines({ projectId }: UseListPipelinesProps) {
    return useQuery<PipelineListItem[], Error>({
        // Query key depends on the projectId
        queryKey: [QueryKeys.LIST_PIPELINE, projectId], // Use LIST_PIPELINE from enum
        // Pass projectId to the mock function
        queryFn: () => fetchPipelinesMock(projectId!), // Non-null assertion ok due to 'enabled'
        // Only run the query if projectId is truthy
        enabled: !!projectId,
        staleTime: 1000 * 60 * 2, // Keep fresh for 2 minutes
    });
}