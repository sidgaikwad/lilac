import { useQuery, UseQueryOptions } from "@tanstack/react-query"; // Import UseQueryOptions
import { QueryKeys } from "./constants";
import { MOCK_ORGS, simulateDelay } from "./mocks";
import { Organization } from "@/types";

// Define options type, making 'enabled' optional
interface UseListOrganizationsOptions {
    enabled?: boolean;
}

// Mock function simulating API call
const fetchOrganizationsMock = async (): Promise<Organization[]> => {
    console.log("Mock API: Fetching organizations...");
    await simulateDelay(200); // Simulate network latency
    console.log("Mock API: Returning organizations:", MOCK_ORGS);
    return [...MOCK_ORGS]; // Return a copy
};

// Accept optional options object
export function useListOrganizations(options?: UseListOrganizationsOptions) {
    return useQuery<Organization[], Error>({
        queryKey: [QueryKeys.LIST_ORGANIZATIONS],
        queryFn: fetchOrganizationsMock,
        staleTime: 1000 * 60 * 5, // 5 minutes
        // Spread the options, allowing 'enabled' to be passed through
        ...(options || {}), // Use empty object if options is undefined
    });
}