import { get } from '@/lib/fetch';
import { queryOptions, useQuery } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { ApiError, Dataset } from '@/types';
import { useEffect } from 'react';

export interface GetDatasetResponse {
  id: string;
  name: string;
  description?: string;
  projectId: string;
  files: {
    fileName: string;
    fileType: string;
    size: number;
    createdAt: string;
    url: string;
  }[];
}

export async function getDataset(
  datasetId: string
): Promise<GetDatasetResponse> {
  return get(`/datasets/${datasetId}`);
}

export function getDatasetQuery(datasetId?: string, enabled: boolean = true) {
  return queryOptions({
    queryKey: [QueryKeys.GET_DATASET, datasetId],
    queryFn: () => getDataset(datasetId!),
    enabled: !!datasetId && enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data as Dataset,
  });
}

interface UseGetDatasetProps {
  datasetId: string | undefined;
  enabled?: boolean;
  onSuccess?: (dataset: Dataset) => void;
  onError?: (error: ApiError) => void;
}

export function useGetDataset(props: UseGetDatasetProps) {
  const query = useQuery(getDatasetQuery(props.datasetId, props.enabled));

  useEffect(() => {
    if (props?.onSuccess && query.data !== undefined) {
      props.onSuccess(query.data);
    }
  }, [props, query.data]);

  useEffect(() => {
    if (props?.onError && query.error !== null) {
      props.onError(query.error);
    }
  }, [props, query.error]);

  return query;
}
