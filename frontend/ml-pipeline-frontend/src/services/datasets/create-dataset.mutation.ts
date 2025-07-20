import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ServiceError } from '@/types';
import { postHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';
import { snakeCaseObject } from '@/lib/utils';
import { S3Source, SnowflakeSource } from './types';

export interface CreateDatasetRequest {
  datasetName: string;
  datasetDescription?: string;
  projectId: string;
  datasetSource: S3Source | SnowflakeSource;
}

export interface CreateDatasetResponse {
  projectId: string;
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
    dataset_description: request.datasetDescription,
    dataset_source: snakeCaseObject(request.datasetSource),
  });
  return {
    projectId: resp.project_id,
  };
}

export interface UseCreateDatasetProps {
  onSuccess?: (data: CreateDatasetResponse) => void;
  onError?: (error: ServiceError) => void;
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
