import { useQuery } from '@tanstack/react-query';
import { QueryKeys } from './constants';
import { get } from '@/lib/fetch';
import { JobOutputSummary, JobOutputImages } from './types';
import { ApiError } from '@/types';

const fetchJobOutputs = async (
  projectId?: string
): Promise<JobOutputSummary[]> => {
  if (!projectId) {
    // Or handle as an error, or return empty array if projectId is truly optional for some views
    // For now, assuming projectId is required for this specific hook usage as per plan
    return Promise.reject(new Error('Project ID is required to fetch job outputs.'));
  }
  return get(`/api/job-outputs?projectId=${projectId}`); // Restored original call
};

export function useListJobOutputs(projectId?: string) {
  return useQuery<JobOutputSummary[], ApiError>({
    queryKey: [QueryKeys.LIST_JOB_OUTPUTS, projectId],
    queryFn: () => fetchJobOutputs(projectId),
    enabled: !!projectId, // Only run query if projectId is available
  });
}

const fetchJobOutputImages = async (
  jobId: string
): Promise<JobOutputImages> => {
  return get(`/api/job-outputs/${jobId}/images`); // Restored original call
};

export function useListJobOutputImages(jobId: string) {
  return useQuery<JobOutputImages, ApiError>({
    queryKey: [QueryKeys.LIST_JOB_OUTPUT_IMAGES, jobId],
    queryFn: () => fetchJobOutputImages(jobId),
    enabled: !!jobId, // Only run query if jobId is available
  });
}