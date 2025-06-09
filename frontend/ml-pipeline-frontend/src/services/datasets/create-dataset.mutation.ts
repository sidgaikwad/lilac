import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ApiError } from '@/types';
import { postHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';

export interface S3Source {
  sourceType: 'S3';
  bucketName: string;
}

export interface CreateDatasetRequest {
  datasetName: string;
  description?: string;
  projectId: string;
  source: S3Source;
}

export interface CreateDatasetResponse {
  id: string;
}

async function createDataset(
  payload: CreateDatasetRequest
): Promise<CreateDatasetResponse> {
  const { projectId, ...request } = payload;
  const resp = await postHttp<
    Sn<Omit<CreateDatasetRequest, 'projectId'>>,
    Sn<CreateDatasetResponse>
  >(`/projects/${projectId}/datasets`, {
    dataset_name: request.datasetName,
    description: request.description,
    source: {
      source_type: request.source.sourceType,
      bucket_name: request.source.bucketName,
    },
  });
  return {
    id: resp.id,
  };
}

export interface UseCreateDatasetProps {
  onSuccess?: (data: CreateDatasetResponse) => void;
  onError?: (error: ApiError) => void;
}

export function useCreateDataset(props?: UseCreateDatasetProps) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationKey: [QueryKeys.CREATE_DATASET],
    mutationFn: createDataset,
    onSuccess: (data) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_DATASETS],
      });
      if (props?.onSuccess !== undefined) {
        props.onSuccess(data);
      }
    },
    onError: props?.onError,
  });
}
