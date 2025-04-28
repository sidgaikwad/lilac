import { create } from 'zustand';
import { PipelineSummary, PipelineDefinition } from '@/types'; // Assuming types exist

interface PipelineState {
  pipelineList: PipelineSummary[];
  currentPipeline: PipelineDefinition | null;
  isLoadingList: boolean;
  isLoadingDetail: boolean;
  error: string | null;

  // TODO: Replace with TanStack Query or similar server state management
  fetchPipelineList: () => Promise<void>;
  fetchPipelineDetail: (id: string) => Promise<void>;
  createPipeline: (name: string) => Promise<string | null>; // Returns new ID or null
  // Add update/delete actions later
}

const usePipelineStore = create<PipelineState>((set) => ({
  pipelineList: [],
  currentPipeline: null,
  isLoadingList: false,
  isLoadingDetail: false,
  error: null,

  fetchPipelineList: async () => {
    set({ isLoadingList: true, error: null });
    try {
      // TODO: API Call - GET /pipeline
      await new Promise((res) => setTimeout(res, 500)); // Simulate API call
      // const response = await apiClient.get('/pipeline');
      // set({ pipelineList: response.data, isLoadingList: false });
      set({ pipelineList: [], isLoadingList: false }); // Placeholder
    } catch (err) {
      const errorMsg = (err as Error).message || 'Failed to fetch pipelines';
      set({ error: errorMsg, isLoadingList: false });
    }
  },

  fetchPipelineDetail: async (_id: string) => {
    set({ isLoadingDetail: true, error: null, currentPipeline: null });
    try {
      // TODO: API Call - GET /pipeline/{id}
      await new Promise((res) => setTimeout(res, 500)); // Simulate API call
      // const response = await apiClient.get(`/pipeline/${id}`);
      // set({ currentPipeline: response.data, isLoadingDetail: false });
      set({ currentPipeline: null, isLoadingDetail: false }); // Placeholder
    } catch (err) {
      const errorMsg =
        (err as Error).message || 'Failed to fetch pipeline details';
      set({ error: errorMsg, isLoadingDetail: false });
    }
  },

  createPipeline: async (_name: string): Promise<string | null> => {
    set({ isLoadingList: true, error: null }); // Indicate loading on list potentially
    try {
      // TODO: API Call - POST /pipeline { name, organization_id }
      await new Promise((res) => setTimeout(res, 500)); // Simulate API call
      // const response = await apiClient.post('/pipeline', { name, organization_id: 'TODO' });
      // const newId = response.data.id;
      // await get().fetchPipelineList(); // Refresh list after creation
      // return newId;
      set({ isLoadingList: false }); // Placeholder
      return null; // Placeholder
    } catch (err) {
      const errorMsg = (err as Error).message || 'Failed to create pipeline';
      set({ error: errorMsg, isLoadingList: false });
      return null;
    }
  },
}));

export default usePipelineStore;
