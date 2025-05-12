import { useQuery } from '@tanstack/react-query';
import { BASE_URL, QueryKeys } from './constants';
import { ApiError } from '@/types';

interface UseGetDatasetProps {
  projectId: string | undefined;
  datasetId: string | undefined;
}

interface GetDatasetResponse {
  files: {
    fileName: string,
    fileType: string,
    size: number,
    createdAt: string,
    url: string,
  }[],
}

// Mock function simulating API call for pipeline details
const fetchDataset = async (projectId: string, datasetId: string): Promise<GetDatasetResponse> => {
  const resp = await fetch(`${BASE_URL}/projects/${projectId}/datasets/${datasetId}`, {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${localStorage.getItem('token')}`,
    },
  });
  if (resp.status < 200 || resp.status >= 300) {
    return Promise.reject({
      statusCode: resp.status,
      ...(await resp.json()),
    });
  }
  const body: GetDatasetResponse = await resp.json();
  return body;
};

export function useGetDataset(props: UseGetDatasetProps) {
  const query = useQuery<GetDatasetResponse, ApiError, GetDatasetResponse['files']>({
    queryKey: [QueryKeys.GET_DATASET, props.projectId, props.datasetId],
    queryFn: () => fetchDataset(props.projectId!, props.datasetId!),
    enabled: !!props.projectId && !!props.datasetId,
    staleTime: 1000 * 60 * 5, // 5 minutes
    select: (data) => data.files,
  });

  return query;
}
