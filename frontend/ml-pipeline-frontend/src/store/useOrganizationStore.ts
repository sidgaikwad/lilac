import { create } from 'zustand';

// This store now ONLY holds the currently selected UI context.
// Data fetching is handled by TanStack Query hooks.

interface OrganizationState {
  selectedOrganizationId?: string;
  selectedProjectId?: string;
  setSelectedOrganizationId: (orgId?: string) => void;
  setSelectedProjectId: (projectId?: string) => void;
}

const useOrganizationStore = create<OrganizationState>((set) => ({
  selectedOrganization: undefined,
  selectedProject: undefined,

  setSelectedOrganizationId: (orgId) =>
    set((state) => {
      // Reset project if organization changes
      if (state.selectedOrganizationId !== orgId) {
        return { selectedOrganizationId: orgId, selectedProjectId: undefined };
      }
      return { selectedOrganization: orgId };
    }),
  setSelectedProjectId: (projectId) => set({ selectedProjectId: projectId }),
}));

export default useOrganizationStore;
