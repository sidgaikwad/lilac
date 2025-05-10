import { useQuery } from '@tanstack/react-query';
import { QueryKeys } from './constants';
import { get } from '@/lib/fetch';
import { ListDatasetsResponse } from './types';
import { ApiError } from '@/types';

const fetchDatasets = async (): Promise<ListDatasetsResponse> => {
  return get('/api/datasets');
};

export function useListDatasets() {
  return useQuery<ListDatasetsResponse, ApiError, string[], string[]>({
    queryKey: [QueryKeys.LIST_DATASETS], 
    queryFn: fetchDatasets,
    select: (data) => data.datasets,
  });
}