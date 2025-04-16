import { create } from 'zustand';
import { Organization, OrganizationMember } from '@/types'; // Assuming types exist

interface OrganizationState {
  currentOrganization: Organization | null;
  members: OrganizationMember[];
  isLoading: boolean;
  error: string | null;

  // TODO: Replace with TanStack Query or similar server state management
  fetchCurrentOrganization: (orgId: string) => Promise<void>;
  fetchMembers: (orgId: string) => Promise<void>;
  // Add update/invite/remove actions later
}

// Placeholder data
const MOCK_ORG: Organization = { id: 'org-123', name: 'Default Mock Org' };
const MOCK_MEMBERS: OrganizationMember[] = [
    { id: 'user-admin', name: 'Admin User', email: 'admin@example.com', role: 'Admin' },
    { id: 'user-456', name: 'Jane Doe', email: 'jane@example.com', role: 'Member' },
];


const useOrganizationStore = create<OrganizationState>((set) => ({
  currentOrganization: null,
  members: [],
  isLoading: false,
  error: null,

  fetchCurrentOrganization: async (orgId: string) => {
    set({ isLoading: true, error: null });
    try {
      // TODO: API Call - GET /organization/{orgId}
      await new Promise(res => setTimeout(res, 300)); // Simulate API call
      // const response = await apiClient.get(`/organization/${orgId}`);
      // set({ currentOrganization: response.data, isLoading: false });
      set({ currentOrganization: MOCK_ORG, isLoading: false }); // Placeholder
    } catch (err) {
      const errorMsg = (err as Error).message || 'Failed to fetch organization';
      set({ error: errorMsg, isLoading: false });
    }
  },

  fetchMembers: async (orgId: string) => {
     set({ isLoading: true, error: null }); // Could use a separate loading state
     try {
        // TODO: API Call - GET /organization/{orgId}/members
        await new Promise(res => setTimeout(res, 400)); // Simulate API call
        // const response = await apiClient.get(`/organization/${orgId}/members`);
        // set({ members: response.data, isLoading: false });
        set({ members: MOCK_MEMBERS, isLoading: false }); // Placeholder
     } catch (err) {
        const errorMsg = (err as Error).message || 'Failed to fetch members';
        set({ error: errorMsg, isLoading: false });
     }
  },

}));

export default useOrganizationStore;