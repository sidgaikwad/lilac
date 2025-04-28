import { useQuery } from '@tanstack/react-query';
import { QueryKeys } from './constants';
import { get } from '@/lib/fetch';
import { Pipeline } from '@/types';

interface UseGetPipelineProps {
  pipelineId: string | undefined;
}

// Mock function simulating API call for pipeline details
const fetchPipelineDetails = async (pipelineId: string): Promise<Pipeline> => {
  return await get(`/pipelines/${pipelineId}`);
};

export function useGetPipeline({ pipelineId }: UseGetPipelineProps) {
  return useQuery<Pipeline, Error>({
    queryKey: [QueryKeys.GET_PIPELINE, pipelineId],
    queryFn: () => fetchPipelineDetails(pipelineId!), // Assert pipelineId exists due to enabled flag
    enabled: !!pipelineId, // Only run if pipelineId is available
    staleTime: 1000 * 60 * 5, // 5 minutes
  });
}
