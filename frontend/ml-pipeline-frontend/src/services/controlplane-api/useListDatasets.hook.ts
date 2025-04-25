import { useQuery } from "@tanstack/react-query";
import { QueryKeys } from "./constants"; // Assuming you add LIST_DATASETS to QueryKeys
import { MOCK_DATASETS, simulateDelay } from "./mocks";
import { DataSetStorageEntry } from "@/lib/localStorageUtils"; // Using this type for mock structure

interface UseListDatasetsProps {
    projectId: string | undefined; // Accept undefined to disable query
}

// Mock function simulating API call
const fetchDatasetsMock = async (projId: string): Promise<DataSetStorageEntry[]> => {
    console.log(`Mock API: Fetching datasets for projectId: ${projId}...`);
    await simulateDelay(350); // Simulate network latency
    const filtered = MOCK_DATASETS.filter(ds => ds.projectId === projId);
    console.log(`Mock API: Returning ${filtered.length} datasets for projectId ${projId}:`, filtered);
    return [...filtered]; // Return a copy
};

// TODO: Add LIST_DATASETS to QueryKeys enum in constants.ts
// Query key uses the enum

export function useListDatasets({ projectId }: UseListDatasetsProps) {
    return useQuery<DataSetStorageEntry[], Error>({
        // Query key depends on the projectId
        queryKey: [QueryKeys.LIST_DATASETS, projectId],
        // Pass projectId to the mock function
        queryFn: () => fetchDatasetsMock(projectId!), // Non-null assertion ok due to 'enabled'
        // Only run the query if projectId is truthy
        enabled: !!projectId,
        staleTime: 1000 * 60 * 2, // Keep fresh for 2 minutes
    });
}