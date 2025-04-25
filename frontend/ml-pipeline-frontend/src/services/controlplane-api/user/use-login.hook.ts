import { useMutation } from "@tanstack/react-query";
import { QueryKeys } from "../constants";
import { AuthToken } from "@/types/api/auth";
import { useNavigate } from "react-router-dom";
import { toast } from "sonner";
import useAuthStore from "@/store/authStore"; // Import auth store
import { User } from "@/types"; // Import User type

export interface UseAuthProps {
    redirectTo?: string;
}

// Mock function for login - always succeeds for dev
const loginMock = async (input: {email: string, password: string}): Promise<AuthToken> => {
    console.log(`Mock API: Simulating login for ${input.email}...`);
    await new Promise(res => setTimeout(res, 300)); // Simulate delay
    const mockToken: AuthToken = {
        access_token: 'mock-dev-token',
        token_type: 'bearer',
        // Add other fields if your AuthToken type requires them
    };
    console.log("Mock API: Login successful, returning mock token.");
    return mockToken;
};

// Mock user data to set in store on successful mock login
const mockUser: User = { id: 'dev-user', name: 'Dev User', email: 'dev@example.com' };


export function useLogin(props: UseAuthProps) {
    const navigate = useNavigate();
    const setAuthState = useAuthStore(state => state.setAuthState); // Get setter from store

    return useMutation({
        mutationKey: [QueryKeys.LOGIN],
        mutationFn: loginMock, // Use the mock function
        onSuccess: (data, variables) => { // variables here is the input {email, password}
            // localStorage.setItem('token', data.access_token); // Don't store mock token
            console.log("Mock login success, setting auth state.");
            // Set auth state directly here as well, using email from input if needed
            const loggedInUser = { ...mockUser, email: variables.email }; // Use input email
            setAuthState(true, loggedInUser, data.access_token);

            if (props.redirectTo) {
                navigate(props.redirectTo);
            } else {
                navigate('/'); // Default redirect
            }
        },
        onError: (error) => {
            // This shouldn't happen with the mock function, but keep for safety
            console.error("Mock login error (should not occur):", error);
            toast.error("Login Failed (Mock Error)", { description: "Something went wrong with mock login." });
        }
    });
}