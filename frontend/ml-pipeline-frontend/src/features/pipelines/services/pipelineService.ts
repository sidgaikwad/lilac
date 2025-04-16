import apiClient from '@/services/apiClient';
import { PipelineListItem, PipelineDefinition } from '@/types'; // Assuming types exist
import { listStoredPipelines, getPipelineEntry } from '@/lib/localStorageUtils'; // Use local storage for mock data

// --- Pipeline List ---

// TODO: Replace mock with actual API call to GET /pipeline (with potential filters)
export const fetchPipelines = async (): Promise<PipelineListItem[]> => {
    console.log("Fetching pipelines via pipelineService (mocked)");
    // Simulate API delay
    await new Promise(res => setTimeout(res, 400));
    // Use localStorage data as mock source for now
    const mockData = listStoredPipelines();
    console.log("Mock pipeline list data:", mockData);
    // Simulate potential API error
    // if (Math.random() > 0.8) throw new Error("Failed to fetch pipelines (mock error)");
    return mockData;
};

// --- Pipeline Detail ---

// TODO: Replace mock with actual API call to GET /pipeline/{pipelineId}
export const fetchPipelineDetail = async (pipelineId: string): Promise<PipelineDefinition | null> => {
    console.log(`Fetching pipeline detail for ${pipelineId} via pipelineService (mocked)`);
    await new Promise(res => setTimeout(res, 500));
    const entry = getPipelineEntry(pipelineId);
    if (entry) {
        // Simulate returning the full definition structure (might differ from storage entry)
        const mockDetail: PipelineDefinition = {
            id: entry.id,
            name: entry.name,
            organization_id: 'org-placeholder', // Add placeholder org ID
            steps: entry.versions[0]?.flowData.nodes.map(n => ({ // Map nodes to step config
                id: n.id,
                step_type: n.data.stepType,
                parameters: n.data.parameters,
                position: n.position,
            })) || [],
            connections: entry.versions[0]?.flowData.edges.map(e => ({ // Map edges to connections
                from_step_id: e.source,
                to_step_id: e.target,
                // Add handle IDs if needed later
            })) || [],
            version_id: entry.versions[0]?.versionId,
            created_at: new Date(0).toISOString(), // Placeholder dates
            updated_at: entry.versions[0]?.timestamp || new Date(0).toISOString(),
        };
        console.log("Mock pipeline detail data:", mockDetail);
        return mockDetail;
    } else {
        console.error(`Mock pipeline detail not found for ${pipelineId}`);
        throw new Error("Pipeline not found (mock error)");
    }
};

// --- Mutations (Placeholders) ---

// TODO: Implement create pipeline mutation function
// export const createPipeline = async (payload: { name: string; organization_id: string }): Promise<{ id: string }> => { ... }

// TODO: Implement update pipeline / save version mutation function
// export const savePipelineVersion = async (pipelineId: string, payload: FlowData): Promise<PipelineVersion> => { ... }

// TODO: Implement rename pipeline mutation function
// export const renamePipelineApi = async (pipelineId: string, payload: { name: string }): Promise<void> => { ... }

// TODO: Implement delete pipeline mutation function
// export const deletePipelineApi = async (pipelineId: string): Promise<void> => { ... }