import { useQuery } from '@tanstack/react-query';
import { QueryKeys } from './constants';
import { ApiError } from '@/types';
import { get } from '@/lib/fetch';
import { ListStepDefinitionsResponse } from './types';

// Mock function simulating API call
const listStepDefinitions = async (): Promise<ListStepDefinitionsResponse> => {
  return get('/step_definitions');
};

export function useListStepDefinitions() {
  return useQuery<ListStepDefinitionsResponse, ApiError>({
    queryKey: [QueryKeys.LIST_STEP_DEFINITIONS],
    queryFn: listStepDefinitions,
    staleTime: 1000 * 60 * 5,
  });
}
