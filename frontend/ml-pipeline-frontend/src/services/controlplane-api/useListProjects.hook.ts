import { useQuery } from "@tanstack/react-query";
import { QueryKeys } from "./constants";
import { MOCK_PROJECTS, simulateDelay } from "./mocks";
import { Project } from "@/types";

interface UseListProjectsProps {
    organizationId: string | undefined; // Accept undefined to disable query
}

// Mock function simulating API call
const fetchProjectsMock = async (orgId: string): Promise<Project[]> => {
    console.log(`Mock API: Fetching projects for orgId: ${orgId}...`);
    await simulateDelay(300); // Simulate network latency
    const filtered = MOCK_PROJECTS.filter(p => p.organization_id === orgId);
    console.log(`Mock API: Returning ${filtered.length} projects for orgId ${orgId}:`, filtered);
    return [...filtered]; // Return a copy
};

export function useListProjects({ organizationId }: UseListProjectsProps) {
    return useQuery<Project[], Error>({
        // Query key depends on the organizationId
        queryKey: [QueryKeys.LIST_PROJECTS, organizationId],
        // Pass orgId to the mock function
        queryFn: () => fetchProjectsMock(organizationId!), // Non-null assertion ok due to 'enabled'
        // Only run the query if organizationId is truthy
        enabled: !!organizationId,
        staleTime: 1000 * 60 * 5, // 5 minutes
    });
}