import { useMutation } from '@tanstack/react-query';
import { ServiceError } from '@/types';
import { postHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';
import { snakeCaseObject } from '@/lib/utils';
import { S3Source, SnowflakeSource } from './types';

export interface TestDatasetRequest {
  datasetName: string;
  description?: string;
  projectId: string;
  source: S3Source | SnowflakeSource;
}

export interface TestDatasetResponse {
  success: boolean;
}

async function testDataset(
  payload: TestDatasetRequest
): Promise<TestDatasetResponse> {
  const { projectId, ...request } = payload;

  const resp = await postHttp<
    Sn<Omit<TestDatasetRequest, 'projectId'>>,
    Sn<TestDatasetResponse>
  >(`/projects/${projectId}/datasets/test`, {
    dataset_name: request.datasetName,
    description: request.description,
    source: snakeCaseObject(request.source),
  });
  return {
    success: resp.success,
  };
}

export interface UseTestDatasetProps {
  onSuccess?: (data: TestDatasetResponse) => void;
  onError?: (error: ServiceError) => void;
}

export function useTestDataset(props?: UseTestDatasetProps) {
  return useMutation({
    mutationKey: [QueryKeys.TEST_DATASET_CONNECTION],
    mutationFn: testDataset,
    onSuccess: (data) => {
      if (props?.onSuccess !== undefined) {
        props.onSuccess(data);
      }
    },
    onError: props?.onError,
  });
}
