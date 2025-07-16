import { queryOptions, useQuery } from '@tanstack/react-query'; // Import UseQueryOptions
import { QueryKeys } from '../constants';
import { ServiceError, DatasetSummary } from '@/types';
import { useEffect } from 'react';
import { getHttp } from '@/lib/fetch';
import { camelCaseObject } from '@/lib/utils';
import { SnakeCasedPropertiesDeep as Sn } from 'type-fest';

export interface ListDatasetsResponse {
  datasets: {
    datasetId: string;
    datasetName: string;
    datasetDescription?: string;
    sourceType: string;
  }[];
}

export async function listDatasets(
  projectId: string
): Promise<ListDatasetsResponse> {
  const resp = await getHttp<Sn<ListDatasetsResponse>>(
    `/projects/${projectId}/datasets`
  );
  return camelCaseObject(resp);
}

export function listDatasetsQuery(projectId?: string, enabled: boolean = true) {
  return queryOptions({
    queryKey: [QueryKeys.LIST_DATASETS, projectId],
    queryFn: () => listDatasets(projectId!),
    enabled: !!projectId && enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data.datasets as DatasetSummary[],
  });
}

interface UseListDatasetsProps {
  projectId?: string;
  enabled?: boolean;
  onSuccess?: (datasets: DatasetSummary[]) => void;
  onError?: (error: ServiceError) => void;
}

export function useListDatasets(props?: UseListDatasetsProps) {
  const query = useQuery(listDatasetsQuery(props?.projectId, props?.enabled));

  useEffect(() => {
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
