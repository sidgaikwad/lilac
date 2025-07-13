import { queryOptions, useQuery } from '@tanstack/react-query'; // Import UseQueryOptions
import { QueryKeys } from '../constants';
import { ServiceError } from '@/types';
import { useEffect } from 'react';
import { getHttp } from '@/lib/fetch';

export interface ListDatasetsS3PrefixesResponse {
  prefixes: string[];
  objects: string[];
}

type S3QueryParams = {
  prefix: string;
  start_after_key?: string;
};

export async function listDatasetS3Prefixes(
  datasetId: string,
  params: S3QueryParams
): Promise<ListDatasetsS3PrefixesResponse> {
  return getHttp(`/datasets/${datasetId}/s3`, params);
}

export function listDatasetS3PrefixesQuery(
  datasetId?: string,
  params?: S3QueryParams,
  enabled: boolean = true
) {
  return queryOptions({
    queryKey: [QueryKeys.LIST_DATASET_S3_OBJECTS, datasetId, params?.prefix],
    queryFn: () => listDatasetS3Prefixes(datasetId!, params!),
    enabled: !!datasetId && !!params && enabled,
    staleTime: 1000 * 60 * 5,
  });
}

interface UseListDatasetS3PrefixesProps {
  datasetId?: string;
  params?: S3QueryParams;
  enabled?: boolean;
  onSuccess?: (prefixes: ListDatasetsS3PrefixesResponse) => void;
  onError?: (error: ServiceError) => void;
}

export function useListDatasetS3Prefixes(
  props?: UseListDatasetS3PrefixesProps
) {
  const query = useQuery(
    listDatasetS3PrefixesQuery(props?.datasetId, props?.params, props?.enabled)
  );

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
