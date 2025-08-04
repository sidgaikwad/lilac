import { createWithEqualityFn } from 'zustand/traditional';

interface ProjectState {
  selectedProjectId?: string;
  setSelectedProjectId: (projectId?: string) => void;
}

const useProjectStore = createWithEqualityFn<ProjectState>((set) => ({
  selectedProjectId: undefined,
  setSelectedProjectId: (projectId) => set({ selectedProjectId: projectId }),
}));

export default useProjectStore;
