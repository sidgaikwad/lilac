import { create } from 'zustand';
import { Organization, Project } from '@/types';

// This store now ONLY holds the currently selected UI context.
// Data fetching is handled by TanStack Query hooks.

interface OrganizationState {
  selectedOrganization: Organization | null;
  selectedProject: Project | null;
  setSelectedOrganization: (org: Organization | null) => void;
  setSelectedProject: (project: Project | null) => void;
}

const useOrganizationStore = create<OrganizationState>((set) => ({
  selectedOrganization: null,
  selectedProject: null,

  setSelectedOrganization: (org) => set((state) => {
    // Reset project if organization changes
    if (state.selectedOrganization?.id !== org?.id) {
      return { selectedOrganization: org, selectedProject: null };
    }
    return { selectedOrganization: org };
  }),

  setSelectedProject: (project) => set({ selectedProject: project }),
}));

export default useOrganizationStore;