import { useQuery } from '@tanstack/react-query';
import { QueryKeys } from './constants';
import { PipelineSummary } from '@/types';
import { ListPipelinesResponse } from './types';
import { get } from '@/lib/fetch';

interface UseListPipelinesProps {
  projectId: string | undefined; // Accept undefined to disable query
}

// Mock function simulating API call
const fetchPipelines = async (
  projectId?: string
): Promise<PipelineSummary[]> => {
  const resp = await get<object, ListPipelinesResponse>(
    `/projects/${projectId}/pipelines`
  );
  return resp.pipelines.map((pipeline) => ({
    id: pipeline.id,
    name: pipeline.name,
    description: pipeline.description,
  }));
};

export function useListPipelines({ projectId }: UseListPipelinesProps) {
  return useQuery<PipelineSummary[], Error>({
    // Query key depends on the projectId
    queryKey: [QueryKeys.LIST_PIPELINE, projectId], // Use LIST_PIPELINE from enum
    // Pass projectId to the mock function
    queryFn: () => fetchPipelines(projectId!), // Non-null assertion ok due to 'enabled'
    // Only run the query if projectId is truthy
    enabled: !!projectId,
    staleTime: 1000 * 60 * 2, // Keep fresh for 2 minutes
  });
}
