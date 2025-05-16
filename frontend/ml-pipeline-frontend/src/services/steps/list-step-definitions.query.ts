import { queryOptions, useQuery } from '@tanstack/react-query'; // Import UseQueryOptions
import { QueryKeys } from '../constants';
import { ApiError, Organization, StepDefinition } from '@/types';
import { useEffect } from 'react';
import { get } from '@/lib/fetch';

export interface ListStepDefinitionsResponse {
  stepDefinitions: {
    id: string;
    name: string;
    description?: string;
    category: string;
    stepType: string;
    schema: unknown;
    inputs: string[];
    outputs: string[];
  }[];
}

export async function listStepDefinitions(): Promise<ListStepDefinitionsResponse> {
  return await get('/step-definitions');
}

export function listStepDefinitionsQuery(enabled: boolean = true) {
  return queryOptions({
    queryKey: [QueryKeys.LIST_STEP_DEFINITIONS],
    queryFn: () => listStepDefinitions(),
    enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data.stepDefinitions as StepDefinition[],
  });
}

interface UseListStepDefinitionsProps {
  enabled?: boolean;
  onSuccess?: (stepDefinitions: Organization[]) => void;
  onError?: (error: ApiError) => void;
}

export function useListStepDefinitions(props?: UseListStepDefinitionsProps) {
  const query = useQuery(listStepDefinitionsQuery(props?.enabled));

  useEffect(() => {
    console.log('hook', query.data)
    if (props?.onSuccess && query.data != undefined) {
      props.onSuccess(query.data);
    }
  }, [props, query.data]);

  useEffect(() => {
    if (props?.onError && query.error != null) {
      props.onError(query.error);
    }
  }, [props, query.error]);

  return query;
}
