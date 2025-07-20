import { useMutation } from '@tanstack/react-query';
import { ServiceError } from '@/types';
import { postHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';
import { camelCaseObject, snakeCaseObject } from '@/lib/utils';

export interface TestClusterRequest {
  clusterConfig: {
    clusterType: 'aws_eks';
    clusterName: string;
  };
  credentialId: string;
}

export interface TestClusterResponse {
  success: boolean;
}

async function testCluster(
  payload: TestClusterRequest
): Promise<TestClusterResponse> {
  const resp = await postHttp<
    Sn<Omit<TestClusterRequest, 'projectId'>>,
    Sn<TestClusterResponse>
  >(`/clusters/test`, snakeCaseObject(payload));
  return camelCaseObject(resp);
}

export interface UseTestClusterProps {
  onSuccess?: (data: TestClusterResponse) => void;
  onError?: (error: ServiceError) => void;
}

export function useTestCluster(props?: UseTestClusterProps) {
  return useMutation({
    mutationKey: [QueryKeys.TEST_CLUSTER_CONNECTION],
    mutationFn: testCluster,
    onSuccess: (data) => {
      if (props?.onSuccess !== undefined) {
        props.onSuccess(data);
      }
    },
    onError: props?.onError,
  });
}
