import { useQuery } from "@tanstack/react-query";
import { QueryKeys } from "../constants";
import { User } from "@/types"; // Use main User type
import useAuthStore from "@/store/authStore"; // Import auth store to check if enabled

// Mock function to return user data
const fetchUserMock = async (): Promise<User> => {
    console.log("Mock API: Fetching user details...");
    await new Promise(res => setTimeout(res, 100)); // Simulate delay
    // Return mock user data consistent with login bypass
    const mockUser: User = { id: 'dev-user', name: 'Dev User', email: 'dev@example.com' };
    console.log("Mock API: Returning user details:", mockUser);
    return mockUser;
};

export interface UseGetUserProps {}

export function useGetUser(_props?: UseGetUserProps) { // Made props optional
    const isAuthenticated = useAuthStore(state => state.isAuthenticated); // Get auth state

    return useQuery({
        queryKey: [QueryKeys.GET_USER],
        queryFn: fetchUserMock, // Use the mock function
        // Only enable the query if the user is authenticated (based on the store)
        enabled: isAuthenticated,
        staleTime: Infinity, // User data is unlikely to change without re-auth
        gcTime: Infinity, // Keep cached indefinitely while session is active
    });
}