import { useQuery } from "@tanstack/react-query";
import { QueryKeys } from "./constants";
import { MOCK_PIPELINES, simulateDelay } from "./mocks"; // Assuming MOCK_PIPELINES exists
import { PipelineDefinition, PipelineStepConfig, PipelineConnectionConfig } from "@/types";
import { hardcodedStepDefinitions } from "@/config/stepDefinitions"; // For mock steps

interface UseGetPipelineProps {
    pipelineId: string | undefined;
}

// Mock function simulating API call for pipeline details
const fetchPipelineDetailMock = async (id: string): Promise<PipelineDefinition | null> => {
    console.log(`Mock API: Fetching pipeline detail for ID: ${id}...`);
    await simulateDelay(450);

    // Find the basic info from the list mock
    const pipelineInfo = MOCK_PIPELINES.find(p => p.id === id);

    if (!pipelineInfo) {
        console.error(`Mock API: Pipeline with ID ${id} not found.`);
        return null; // Or throw error
    }

    // --- Construct Mock Steps & Connections ---
    // This is highly simplified. A real API would return this structure.
    const stepDefMap = new Map(hardcodedStepDefinitions.map(def => [def.type, def]));
    const inputDef = hardcodedStepDefinitions.find(def => def.category === 'Input');
    const outputDef = hardcodedStepDefinitions.find(def => def.category === 'Output');
    let mockSteps: PipelineStepConfig[] = [];
    let mockConnections: PipelineConnectionConfig[] = [];

    if (inputDef) {
        mockSteps.push({
            id: 'input_node_mock',
            step_type: inputDef.type,
            parameters: {}, // Add default params if needed
            position: { x: 100, y: 150 }
        });
    }
     if (outputDef) {
        mockSteps.push({
            id: 'output_node_mock',
            step_type: outputDef.type,
            parameters: {},
            position: { x: 500, y: 150 }
        });
    }
    // Add a mock connection if both exist
    if (inputDef && outputDef) {
        mockConnections.push({
            from_step_id: 'input_node_mock',
            to_step_id: 'output_node_mock'
        });
    }
    // --- End Mock Steps & Connections ---


    const mockDetail: PipelineDefinition = {
        id: pipelineInfo.id,
        name: pipelineInfo.name,
        project_id: pipelineInfo.projectId || 'unknown-proj', // Use projectId from list mock
        organization_id: 'org-placeholder', // Add placeholder org ID
        steps: mockSteps,
        connections: mockConnections,
        // Mock version/timestamps
        version_id: 'mock-version-1',
        created_at: new Date(Date.now() - 86400000).toISOString(),
        updated_at: pipelineInfo.lastModified,
    };

    console.log("Mock API: Returning pipeline detail:", mockDetail);
    return mockDetail;
};

export function useGetPipeline({ pipelineId }: UseGetPipelineProps) {
    return useQuery<PipelineDefinition | null, Error>({
        queryKey: [QueryKeys.GET_PIPELINE, pipelineId],
        queryFn: () => fetchPipelineDetailMock(pipelineId!), // Assert pipelineId exists due to enabled flag
        enabled: !!pipelineId, // Only run if pipelineId is available
        staleTime: 1000 * 60 * 5, // 5 minutes
    });
}