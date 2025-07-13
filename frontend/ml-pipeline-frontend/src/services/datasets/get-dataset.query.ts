import { getHttp } from '@/lib/fetch';
import { queryOptions, useQuery } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { ServiceError } from '@/types';
import { useEffect } from 'react';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';
import { camelCaseObject } from '@/lib/utils';

export interface GetDatasetResponse {
  id: string;
  name: string;
  description?: string;
  projectId: string;
  datasetSource: { sourceType: 'S3'; bucketName: string; region: string };
}

export async function getDataset(
  datasetId: string
): Promise<GetDatasetResponse> {
  const resp = await getHttp<Sn<GetDatasetResponse>>(`/datasets/${datasetId}`);
  return camelCaseObject(resp);
}

export function getDatasetQuery(datasetId?: string, enabled: boolean = true) {
  return queryOptions({
    queryKey: [QueryKeys.GET_DATASET, datasetId],
    queryFn: () => getDataset(datasetId!),
    enabled: !!datasetId && enabled,
    staleTime: 1000 * 60 * 5,
  });
}

interface UseGetDatasetProps {
  datasetId: string | undefined;
  enabled?: boolean;
  onSuccess?: (dataset: GetDatasetResponse) => void;
  onError?: (error: ServiceError) => void;
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
