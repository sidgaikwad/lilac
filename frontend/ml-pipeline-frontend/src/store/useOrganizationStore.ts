import { createWithEqualityFn } from 'zustand/traditional';

interface OrganizationState {
  selectedOrganizationId?: string;
  selectedProjectId?: string;
  setSelectedOrganizationId: (orgId?: string) => void;
  setSelectedProjectId: (projectId?: string) => void;
}

const useOrganizationStore = createWithEqualityFn<OrganizationState>((set) => ({
  selectedOrganizationId: undefined,
  selectedProjectId: undefined,

  setSelectedOrganizationId: (orgId) =>
    set((state) => {
      if (state.selectedOrganizationId !== orgId) {
        return { selectedOrganizationId: orgId, selectedProjectId: undefined };
      }
      return { selectedOrganizationId: orgId };
    }),
  setSelectedProjectId: (projectId) => set({ selectedProjectId: projectId }),
}));

export default useOrganizationStore;
