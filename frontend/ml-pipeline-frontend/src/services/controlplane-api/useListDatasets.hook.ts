import { useQuery } from '@tanstack/react-query';
import { QueryKeys } from './constants';
import { get } from '@/lib/fetch';
import { ListDatasetsResponse } from './types';
import { ApiError } from '@/types';


interface UseListDatasetsProps {
  projectId: string | undefined; // Accept undefined to disable query
}

const fetchDatasets = async (projectId: string): Promise<ListDatasetsResponse> => {
  return get(`/projects/${projectId}/datasets`);
};

export function useListDatasets(props: UseListDatasetsProps) {
  return useQuery<ListDatasetsResponse, ApiError, ListDatasetsResponse['datasets']>({
    queryKey: [QueryKeys.LIST_DATASETS, props.projectId],
    queryFn: () => fetchDatasets(props.projectId!),
    enabled: !!props.projectId,
    select: (data) => data.datasets,
  });
}